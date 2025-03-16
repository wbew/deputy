use std::path::PathBuf;

use clap::{Parser, arg, command};
use glob;

#[derive(Parser)]
#[command(name = "deputy")]
#[command(about = "Organize files")]
struct Cli {
    /// Files or directories to organize. Both direct paths and glob patterns are supported.
    #[arg(required = true)]
    paths: Vec<String>,

    /// Preview only, don't commit changes
    #[arg(short, long)]
    dry_run: bool,
}

impl Cli {
    fn paths(&self) -> Vec<PathBuf> {
        // Collect all paths, expanding any glob patterns
        let mut all_paths: Vec<PathBuf> = Vec::new();

        for path_str in &self.paths {
            match glob::glob(path_str) {
                Ok(entries) => {
                    let paths: Vec<PathBuf> = entries.filter_map(Result::ok).collect();
                    if paths.is_empty() {
                        println!(
                            "Warning: Glob pattern '{}' did not match any files",
                            path_str
                        );
                    }
                    all_paths.extend(paths);
                }
                Err(e) => println!("Error parsing glob pattern '{}': {}", path_str, e),
            }
        }

        return all_paths;
    }
}

fn main() {
    let cli = Cli::parse();

    let paths = cli.paths();
    if paths.is_empty() {
        println!("No files found.")
    }

    for path in paths {
        println!("{}", path.display())
    }
}
