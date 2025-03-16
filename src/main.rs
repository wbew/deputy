use std::path::PathBuf;

use clap::{Parser, arg, command};
use glob;
use time::{self, format_description};

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
        let metadata = match path.metadata() {
            Ok(md) => md,
            Err(e) => {
                println!("Error reading metadata for {}: {}", path.display(), e);
                continue;
            }
        };

        if metadata.is_file() {
            let modified = metadata.modified().unwrap();
            let modified = time::OffsetDateTime::from(modified);
            let date_format = format_description::parse("[year]-[month]-[day]").unwrap();
            let date_prefix = modified.format(&date_format).unwrap();

            if path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with(&date_prefix)
            {
                println!("no changes: {}", path.display());
                continue;
            }

            let new_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .map(|name| format!("{}__{}", date_prefix, name));

            if let Some(new_name) = new_name {
                let new_path = path.with_file_name(new_name);
                if cli.dry_run {
                    println!("old {}", path.display());
                    println!("new {}\n", new_path.display());
                } else {
                    if let Err(e) = std::fs::rename(&path, &new_path) {
                        println!("Error renaming {}: {}", path.display(), e);
                        continue;
                    }
                }
            }
        }
    }
}
