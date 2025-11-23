use tree2fs_rs::parser::TreeParser;
use tree2fs_rs::builder::FilesystemBuilder;
use std::io::Write;
use tempfile::NamedTempFile;

fn main() -> anyhow::Result<()> {
    // Create a temporary tree file
    let mut tree_file = NamedTempFile::new()?;
    write!(tree_file, "example_project/\n├── src/\n│   └── main.rs\n└── Cargo.toml")?;
    
    let tree_path = tree_file.path().to_path_buf();
    println!("Created temporary tree file at: {:?}", tree_path);

    // 1. Parse the tree file
    println!("Parsing tree file...");
    let parser = TreeParser::new(4);
    let (root, root_name_to_skip) = parser.build_tree(&tree_path)?;

    // 2. Build the filesystem
    // We'll use a temporary directory for the output to avoid cluttering
    let output_dir = std::env::temp_dir().join("tree2fs_example_output");
    if output_dir.exists() {
        std::fs::remove_dir_all(&output_dir)?;
    }
    std::fs::create_dir_all(&output_dir)?;
    
    println!("Building filesystem at: {:?}", output_dir);
    
    // Determine skip_root logic (same as CLI)
    let skip_root = root_name_to_skip.is_some();

    let mut builder = FilesystemBuilder::new(&output_dir, false, true);
    let (dirs, files) = builder.build(&root, skip_root)?;

    println!("Successfully created {} directories and {} files.", dirs, files);
    
    // Cleanup
    std::fs::remove_dir_all(output_dir)?;
    println!("Cleaned up output directory.");

    Ok(())
}
