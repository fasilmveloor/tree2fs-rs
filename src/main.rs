use clap::Parser;
use std::path::PathBuf;
use anyhow::{Context, Result};
use env_logger::Env;

use tree2fs_rs::parser::TreeParser;
use tree2fs_rs::builder::FilesystemBuilder;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the tree file
    #[arg(name = "TREE_FILE")]
    tree_file: PathBuf,

    /// Base directory to create structure in
    #[arg(short, long, default_value = ".")]
    base_dir: PathBuf,

    /// Preview without creating files/directories
    #[arg(long)]
    dry_run: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Do not skip creating the root directory
    #[arg(long)]
    no_skip_root: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logger
    let env = Env::default().filter_or("RUST_LOG", if args.verbose { "debug" } else { "info" });
    env_logger::init_from_env(env);

    let parser = TreeParser::new(4);
    let (root, root_name_to_skip) = parser.build_tree(&args.tree_file)
        .context("Failed to parse tree file")?;

    // Determine if we should skip root
    // If user explicitly requested no-skip-root, we don't skip
    // Otherwise, if the tree has a single root directory (root_name_to_skip is Some), we skip it by default
    let skip_root = if args.no_skip_root {
        false
    } else {
        root_name_to_skip.is_some()
    };

    let mut builder = FilesystemBuilder::new(args.base_dir, args.dry_run, args.verbose);
    builder.build(&root, skip_root)
        .context("Failed to build filesystem")?;
    
    builder.print_summary();

    Ok(())
}
