use std::cell::RefCell;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use thiserror::Error;

use crate::models::Node;

#[derive(Error, Debug)]
pub enum FilesystemBuildError {
    #[error("Failed to create directory {path}: {source}")]
    DirectoryCreationError { path: String, source: io::Error },
    #[error("Failed to create file {path}: {source}")]
    FileCreationError { path: String, source: io::Error },
}

pub struct FilesystemBuilder {
    base_dir: PathBuf,
    dry_run: bool,
    verbose: bool,
    created_dirs: HashSet<String>,
    created_files: HashSet<String>,
}

impl FilesystemBuilder {
    pub fn new<P: AsRef<Path>>(base_dir: P, dry_run: bool, verbose: bool) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
            dry_run,
            verbose,
            created_dirs: HashSet::new(),
            created_files: HashSet::new(),
        }
    }

    fn create_directory(&mut self, path: &Path, node: &Node) -> Result<(), FilesystemBuildError> {
        if !self.dry_run {
            fs::create_dir_all(path).map_err(|e| FilesystemBuildError::DirectoryCreationError {
                path: path.to_string_lossy().to_string(),
                source: e,
            })?;
        }

        self.created_dirs.insert(path.to_string_lossy().to_string());

        if self.verbose {
            let action = if self.dry_run { "[DRY RUN] Would create" } else { "Created" };
            println!("{} directory: {}", action, path.display());
            if !node.data.comment.is_empty() {
                println!("  → Comment: {}", node.data.comment);
            }
        }

        Ok(())
    }

    fn create_file(&mut self, path: &Path, node: &Node) -> Result<(), FilesystemBuildError> {
        if !self.dry_run {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|e| FilesystemBuildError::DirectoryCreationError {
                    path: parent.to_string_lossy().to_string(),
                    source: e,
                })?;
            }
            fs::File::create(path).map_err(|e| FilesystemBuildError::FileCreationError {
                path: path.to_string_lossy().to_string(),
                source: e,
            })?;
        }

        self.created_files.insert(path.to_string_lossy().to_string());

        if self.verbose {
            let action = if self.dry_run { "[DRY RUN] Would create" } else { "Created" };
            println!("{} file: {}", action, path.display());
            if !node.data.comment.is_empty() {
                println!("  → Comment: {}", node.data.comment);
            }
        }

        Ok(())
    }

    fn traverse_and_create(&mut self, node: &Rc<RefCell<Node>>, skip_root: bool) -> Result<(), FilesystemBuildError> {
        let node_borrowed = node.borrow();
        
        let node_path = node_borrowed.get_full_path();
        let final_path = if skip_root {
             let components: Vec<_> = node_path.components().skip(1).collect();
             if components.is_empty() {
                 self.base_dir.clone()
             } else {
                 self.base_dir.join(PathBuf::from_iter(components))
             }
        } else {
            self.base_dir.join(node_path)
        };
        
        if node_borrowed.data.is_directory() {
             self.create_directory(&final_path, &node_borrowed)?;
        } else {
             self.create_file(&final_path, &node_borrowed)?;
        }

        for child in &node_borrowed.children {
            self.traverse_and_create(child, skip_root)?;
        }

        Ok(())
    }

    pub fn build(&mut self, root: &Rc<RefCell<Node>>, skip_root: bool) -> Result<(usize, usize), FilesystemBuildError> {
        self.created_dirs.clear();
        self.created_files.clear();

        self.traverse_and_create(root, skip_root)?;

        Ok((self.created_dirs.len(), self.created_files.len()))
    }

    pub fn get_summary(&self) -> (usize, usize, usize, bool) {
        (
            self.created_dirs.len(),
            self.created_files.len(),
            self.created_dirs.len() + self.created_files.len(),
            self.dry_run,
        )
    }

    pub fn print_summary(&self) {
        let (dirs, files, total, dry_run) = self.get_summary();
        let prefix = if dry_run { "[DRY RUN] " } else { "" };

        println!("\n{}Summary:", prefix);
        println!("  📁 Directories: {}", dirs);
        println!("  📄 Files: {}", files);
        println!("  📊 Total: {}", total);

        if dry_run {
            println!("\n💡 Run without --dry-run to actually create the structure.");
        }
    }
}
