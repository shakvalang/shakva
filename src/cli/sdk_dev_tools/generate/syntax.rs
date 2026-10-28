use std::{
    fs::{self, read_to_string},
    io,
    path::{Path, PathBuf},
};

#[derive(Debug, clap::Args)]
pub(super) struct Args {
    /// Path to the syntax generation TOML configuration file.
    ///
    /// Relative paths in the configuration are resolved against its directory.
    config: PathBuf,

    /// Check that generated files are up to date without modifying them.
    ///
    /// Exit with a non-zero status if any generated file is missing or differs
    /// from the expected output.
    #[arg(long)]
    check: bool,
}

pub(super) fn generate(args: Args) -> Result<(), Error> {
    let config_dir = args.config.parent().unwrap_or(Path::new("."));
    let config_file_text =
        read_to_string(&args.config).map_err(|source| Error::FileReadFailed {
            path: args.config.clone(),
            source,
        })?;

    let config: Config =
        toml::from_str(&config_file_text).map_err(|source| Error::ConfigParseFailed {
            path: args.config.clone(),
            source,
        })?;

    let grammar_path = config_dir.join(&config.grammar);
    let grammar = read_to_string(&grammar_path).map_err(|source| Error::FileReadFailed {
        path: grammar_path,
        source,
    })?;

    let gen_sources = shakva::syntax::codegen::generate(&config.kinds, &grammar)?;
    let output_dir = config_dir.join(&config.output_dir);

    update_generated_file(
        &output_dir.join("generated.rs"),
        &gen_sources.kinds_src,
        args.check,
    )
    // update_generated_file(
    //     &output_dir.join("ast/generated.rs"),
    //     &gen_sources.ast_src,
    //     args.check,
    // )
}

fn update_generated_file(path: &Path, expected: &str, check: bool) -> Result<(), Error> {
    match fs::read(path) {
        Ok(actual) if actual == expected.as_bytes() => return Ok(()),
        Ok(_) => {}
        Err(source) if source.kind() == io::ErrorKind::NotFound => {}
        Err(source) => {
            return Err(Error::FileReadFailed {
                path: path.to_path_buf(),
                source,
            });
        }
    }

    if check {
        return Err(Error::GeneratedFileOutOfDate {
            path: path.to_path_buf(),
        });
    }

    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent).map_err(|source| Error::DirectoryCreateFailed {
            path: parent.to_path_buf(),
            source,
        })?;
    }

    fs::write(path, expected).map_err(|source| Error::FileWriteFailed {
        path: path.to_path_buf(),
        source,
    })
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("failed to read file {}", path.display())]
    FileReadFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("generated file {} is missing or out of date; run generation without --check", path.display())]
    GeneratedFileOutOfDate { path: PathBuf },

    #[error("failed to create directory {}", path.display())]
    DirectoryCreateFailed {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("failed to write file {}", path.display())]
    FileWriteFailed {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("failed to parse syntax generation config {}", path.display())]
    ConfigParseFailed {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },

    #[error(transparent)]
    Codegen(#[from] shakva::syntax::codegen::Error),
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct Config {
    grammar: PathBuf,
    output_dir: PathBuf,
    kinds: shakva::syntax::codegen::KindsConfig,
}
