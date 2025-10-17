//! Package render.

use humanode_distribution_schema::manifest::Package;

/// Rendering params.
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Renderer {
    /// Display name only.
    DisplayName,
    /// YAML format.
    Yaml,
    /// JSON format.
    Json,
}

impl Renderer {
    /// Render the package into a writer according to the renderer config.
    pub fn render(
        &self,
        mut writer: impl std::io::Write,
        package: &Package,
    ) -> Result<(), eyre::Error> {
        match self {
            Self::DisplayName => writer
                .write_all(package.display_name.as_bytes())
                .map_err(Into::into),
            Self::Yaml => {
                writer.write_all(b"---\n")?;
                serde_yaml_bw::to_writer(writer, package).map_err(Into::into)
            }
            Self::Json => serde_json::to_writer_pretty(writer, package).map_err(Into::into),
        }
    }

    /// Render the package into a string according to the renderer config.
    pub fn render_to_string(&self, package: &Package) -> Result<String, eyre::Error> {
        let mut buf = Vec::new();
        self.render(&mut buf, package)?;
        Ok(String::from_utf8(buf)?)
    }
}
