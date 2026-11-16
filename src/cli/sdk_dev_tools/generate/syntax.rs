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

pub(super) fn generate(args: Args) {
    let config_dir = args.config.parent().unwrap_or(Path::new("."));
    let config_file_text = read_to_string(&args.config)
        .unwrap_or_else(|_| panic!("failed to read file {}", args.config.display()));

    let config: Config = toml::from_str(&config_file_text).unwrap_or_else(|_| {
        panic!(
            "failed to parse syntax generation config {}",
            args.config.display()
        )
    });

    let grammar_path = config_dir.join(&config.grammar);
    let grammar = read_to_string(&grammar_path)
        .unwrap_or_else(|_| panic!("failed to read file {}", grammar_path.display()));

    let gen_sources = shakva::syntax::codegen::generate(&config.kinds, &grammar);
    let output_dir = config_dir.join(&config.output_dir);

    update_generated_file(
        &output_dir.join("generated.rs"),
        &gen_sources.kinds_src,
        args.check,
    );
    update_generated_file(
        &output_dir.join("ast/generated.rs"),
        &gen_sources.ast_src,
        args.check,
    );
}

fn update_generated_file(path: &Path, expected: &str, check: bool) {
    match fs::read(path) {
        Ok(actual) if actual == expected.as_bytes() => {
            return;
        }
        Ok(_) => {}
        Err(source) if source.kind() == io::ErrorKind::NotFound => {}
        Err(_) => {
            panic!("failed to read file {}", path.display());
        }
    }

    if check {
        eprintln!(
            "generated file {} is missing or out of date; run generation without --check",
            path.display()
        );
        return;
    }

    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)
            .unwrap_or_else(|_| panic!("failed to create directory {}", parent.display()));
    }

    fs::write(path, expected).unwrap_or_else(|_| panic!("failed to write file {}", path.display()));
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct Config {
    grammar: PathBuf,
    output_dir: PathBuf,
    kinds: shakva::syntax::codegen::KindsConfig,
}
