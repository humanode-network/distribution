//! The CLI for operating on a Humanode distribution.

#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]

use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};
use humanode_distribution_config::load::SourcesLoadingResult;
use humanode_distribution_resolver::resolve::Contextualized;
use humanode_distribution_schema::manifest::Package;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// List all installable packages.
    List(List),
    /// Evaluate the parameters and print the package that would be installed.
    Eval(Eval),
    /// Install the distribution into a given directory.
    Install(Install),
    /// Display the sources.
    Sources(Sources),
}

#[derive(Debug, Args)]
struct SourcesArgs {
    /// Do not add the built-in sources.
    #[arg(long, default_value_t = false)]
    no_built_in_sources: bool,

    /// Do not load the config files.
    #[arg(long, default_value_t = false)]
    no_config_files: bool,

    /// The list of URLs to fetch the repos from.
    #[arg(short, long)]
    repo_urls: Vec<String>,

    /// The list of URLs to fetch the manifests from; in addition to repos.
    #[arg(short, long)]
    manifest_urls: Vec<String>,
}

#[derive(Debug, Args)]
struct ResolutionArgs {
    #[clap(flatten)]
    sources_args: SourcesArgs,

    /// The platform to use, current system's platform will be used by default.
    #[arg(short, long)]
    platform: Option<String>,

    /// The arch to use, current system's arch will be used by default.
    #[arg(short, long)]
    arch: Option<String>,
}

#[derive(Debug, Args)]
struct SelectionArgs {
    /// The package display name to select.
    #[arg(long)]
    package_display_name: Option<String>,
}

#[derive(Debug, Args)]
struct RenderingArgs {
    #[arg(long, default_value = "display-name")]
    renderer: humanode_distribution::package_render::Renderer,
}

#[derive(Debug, Parser)]
struct List {
    #[clap(flatten)]
    resolution_args: ResolutionArgs,

    #[clap(flatten)]
    rendering_args: RenderingArgs,
}

#[derive(Debug, Parser)]
struct Eval {
    #[clap(flatten)]
    resolution_args: ResolutionArgs,

    #[clap(flatten)]
    selection_args: SelectionArgs,

    #[clap(flatten)]
    rendering_args: RenderingArgs,
}

#[derive(Debug, Parser)]
struct Install {
    #[clap(flatten)]
    resolution_args: ResolutionArgs,

    #[clap(flatten)]
    selection_args: SelectionArgs,

    /// The directory to install to.
    #[arg(short, long, default_value = ".")]
    dir: String,
}

#[derive(Debug, Parser)]
struct Sources {
    #[clap(flatten)]
    sources_args: SourcesArgs,
}

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt::init();
    color_eyre::install().unwrap();
    let cli = Cli::parse();

    let result = match cli.command {
        Command::List(args) => list(args).await,
        Command::Eval(args) => eval(args).await,
        Command::Install(args) => install(args).await,
        Command::Sources(args) => sources(args).await,
    };

    if let Err(error) = result {
        eprintln!("Error: {error:?}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn add_built_in_sources(sources: &mut humanode_distribution_config::Sources) {
    let extend = |what: &mut Vec<String>, with_what: &[&str]| {
        what.extend(with_what.iter().map(|&item| item.to_owned()));
    };
    extend(
        &mut sources.repo_urls,
        humanode_distribution_built_in_sources::REPO_URLS,
    );
    extend(
        &mut sources.manifest_urls,
        humanode_distribution_built_in_sources::MANIFEST_URLS,
    );
}

// Load the configs, print encountered errors.
async fn load_configs(all_sources: &mut humanode_distribution_config::Sources) {
    let config_paths = humanode_distribution_config::paths::configs();
    for config_path in config_paths {
        let SourcesLoadingResult { sources, errors } =
            humanode_distribution_config::load::sources(config_path).await;
        for error in errors.all() {
            // Skip the directories that were not found.
            let is_boring_error = matches!(
                &error,
                humanode_distribution_config::load::LoadingError::DirReading(inner, _)
                    if inner.kind() == std::io::ErrorKind::NotFound
            );
            if is_boring_error {
                continue;
            }
            eprintln!("Loading the config files: {}", error);
        }
        all_sources.extend(sources);
    }
}

/// Common CLI logic to process the source args and load the sources from
/// the configs.
async fn prepare_sources(sources_args: SourcesArgs) -> humanode_distribution_config::Sources {
    let SourcesArgs {
        no_built_in_sources,
        no_config_files,
        repo_urls,
        manifest_urls,
    } = sources_args;

    let mut sources = humanode_distribution_config::Sources::default();

    if !no_built_in_sources {
        add_built_in_sources(&mut sources);
    }

    if !no_config_files {
        load_configs(&mut sources).await;
    }

    sources.repo_urls.extend(repo_urls);
    sources.manifest_urls.extend(manifest_urls);

    sources
}

/// Common CLI logic to run the resolver from the given args.
async fn resolve(
    resolution_args: ResolutionArgs,
) -> Result<Vec<Contextualized<Package>>, eyre::Error> {
    let ResolutionArgs {
        sources_args,
        platform,
        arch,
    } = resolution_args;

    let humanode_distribution_config::Sources {
        manifest_urls,
        repo_urls,
    } = prepare_sources(sources_args).await;

    // Detect platform and arch if not specified.
    let (platform, arch) = match (platform, arch) {
        (Some(platform), Some(arch)) => (platform, arch),
        (platform, arch) => {
            let detected = humanode_distribution_detection::detect()?;
            (
                platform.unwrap_or(detected.platform),
                arch.unwrap_or(detected.arch),
            )
        }
    };

    let client = reqwest::Client::new();

    let filter = humanode_distribution_resolver::filter::Params { platform, arch };

    let packages = humanode_distribution_resolver::resolve::resolve(
        client,
        humanode_distribution_resolver::resolve::Params {
            manifest_urls,
            repo_urls,
        },
        humanode_distribution::issue_printer::Stderr,
        |package| filter.matches(package),
    )
    .await;

    Ok(packages)
}

fn select(
    args: SelectionArgs,
    packages: Vec<Contextualized<Package>>,
) -> Result<Contextualized<Package>, eyre::Error> {
    let SelectionArgs {
        package_display_name,
    } = args;

    let selector = humanode_distribution::selector::Selector {
        package_display_name,
    };
    let selected = selector.select(packages)?;

    Ok(selected)
}

/// List command.
async fn list(args: List) -> Result<(), eyre::Error> {
    let List {
        resolution_args,
        rendering_args,
    } = args;
    let RenderingArgs { renderer } = rendering_args;
    let packages = resolve(resolution_args).await?;

    for package in packages {
        println!("{}", renderer.render_to_string(&package.value)?);
    }

    Ok(())
}

/// Eval command.
async fn eval(args: Eval) -> Result<(), eyre::Error> {
    let Eval {
        resolution_args,
        selection_args,
        rendering_args,
    } = args;
    let RenderingArgs { renderer } = rendering_args;
    let packages = resolve(resolution_args).await?;
    let selected = select(selection_args, packages)?;
    println!("{}", renderer.render_to_string(&selected.value)?);
    Ok(())
}

/// Install command.
async fn install(args: Install) -> Result<(), eyre::Error> {
    let Install {
        resolution_args,
        selection_args,
        dir,
    } = args;
    let packages = resolve(resolution_args).await?;
    let selected = select(selection_args, packages)?;

    println!(
        "Installing {:?} to {:?}...",
        selected.value.display_name, dir
    );

    let client = reqwest::Client::new();

    let params = humanode_distribution_installer::install::Params {
        client,
        dir,
        base_url: selected.manifest_url,
        package: selected.value,
    };

    humanode_distribution_installer::install::install(params).await?;

    Ok(())
}

/// Sources command.
async fn sources(args: Sources) -> Result<(), eyre::Error> {
    let Sources { sources_args } = args;
    let sources = prepare_sources(sources_args).await;
    println!("{}", &serde_yaml_bw::to_string(&sources)?);
    Ok(())
}
