//! The CLI for operating on a Humanode distribution.

#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]

use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};
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
}

#[derive(Debug, Args)]
struct ResolutionArgs {
    /// The list of URLs to fetch the repos from.
    #[arg(short, long)]
    repo_urls: Vec<String>,

    /// The list of URLs to fetch the manifests from; in addition to repos.
    #[arg(short, long)]
    manifest_urls: Vec<String>,

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

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::List(args) => list(args).await,
        Command::Eval(args) => eval(args).await,
        Command::Install(args) => install(args).await,
    };

    if let Err(error) = result {
        eprintln!("Error: {error:?}");
        eprintln!("{}", error.backtrace());
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

/// Common CLI logic to run the resolver from the given args.
async fn resolve(
    resolution_args: ResolutionArgs,
) -> Result<Vec<Contextualized<Package>>, anyhow::Error> {
    let ResolutionArgs {
        repo_urls,
        manifest_urls,
        platform,
        arch,
    } = resolution_args;

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
) -> Result<Contextualized<Package>, anyhow::Error> {
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
async fn list(args: List) -> Result<(), anyhow::Error> {
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
async fn eval(args: Eval) -> Result<(), anyhow::Error> {
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
async fn install(args: Install) -> Result<(), anyhow::Error> {
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
