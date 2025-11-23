use std::fs;
use std::io::Write;
use tempfile::TempDir;
use tree2fs_rs::parser::TreeParser;
use tree2fs_rs::builder::FilesystemBuilder;

#[test]
fn test_end_to_end_creation() -> anyhow::Result<()> {
    // Setup
    let temp_dir = TempDir::new()?;
    let tree_file_path = temp_dir.path().join("tree.txt");
    let output_dir = temp_dir.path().join("output");
    fs::create_dir(&output_dir)?;

    // Create tree file
    let tree_content = r#"
my_project/
├── src/
│   ├── main.rs
│   └── utils.rs
└── README.md
"#;
    let mut f = fs::File::create(&tree_file_path)?;
    f.write_all(tree_content.as_bytes())?;

    // Execution
    let parser = TreeParser::new(4);
    let (root, root_name_to_skip) = parser.build_tree(&tree_file_path)?;
    
    let skip_root = root_name_to_skip.is_some();
    let mut builder = FilesystemBuilder::new(&output_dir, false, false);
    builder.build(&root, skip_root)?;

    // Verification
    // With skip_root=true (default), the root directory 'my_project' is skipped,
    // and its contents are created directly in output_dir.
    assert!(output_dir.join("src").exists());
    assert!(output_dir.join("src/main.rs").exists());
    assert!(output_dir.join("src/utils.rs").exists());
    assert!(output_dir.join("README.md").exists());

    Ok(())
}

#[test]
fn test_no_skip_root() -> anyhow::Result<()> {
    // Setup
    let temp_dir = TempDir::new()?;
    let tree_file_path = temp_dir.path().join("tree.txt");
    let output_dir = temp_dir.path().join("output");
    fs::create_dir(&output_dir)?;

    // Create tree file
    let tree_content = r#"
root_folder/
└── file.txt
"#;
    let mut f = fs::File::create(&tree_file_path)?;
    f.write_all(tree_content.as_bytes())?;

    // Execution
    let parser = TreeParser::new(4);
    let (root, _root_name_to_skip) = parser.build_tree(&tree_file_path)?;
    
    // Force skip_root = false
    let mut builder = FilesystemBuilder::new(&output_dir, false, false);
    builder.build(&root, false)?;

    // Verification
    // Should create root_folder inside output_dir
    assert!(output_dir.join("root_folder").exists());
    assert!(output_dir.join("root_folder/file.txt").exists());

    Ok(())
}
