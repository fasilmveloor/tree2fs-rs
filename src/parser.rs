use std::cell::RefCell;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;
use thiserror::Error;
use log::warn;

use crate::models::{FileItem, Node};

#[derive(Error, Debug)]
pub enum TreeParseError {
    #[error("Failed to read tree file: {0}")]
    IoError(#[from] io::Error),
    #[error("Tree file is empty")]
    EmptyFile,
    #[error("No valid nodes found in tree file")]
    NoNodesFound,
    #[error("Line {line}: Level {level} has no parent (previous max level was {max_level})")]
    OrphanNode { line: usize, level: usize, max_level: usize },
    #[error("Invalid filename at line {line}: {content}")]
    InvalidFilename { line: usize, content: String },
}

pub struct TreeParser {
    symbol_length: usize,
}

impl TreeParser {
    pub fn new(symbol_length: usize) -> Self {
        Self { symbol_length }
    }

    pub fn parse_line(&self, line: &str, line_num: usize) -> Option<FileItem> {
        let line = line.trim_end();
        if line.trim().is_empty() {
            return None;
        }

        let parts: Vec<&str> = line.splitn(2, '#').collect();
        let file_part = parts[0].trim_end();
        let comment = if parts.len() > 1 {
            parts[1].trim().to_string()
        } else {
            String::new()
        };

        let filename = file_part.trim_start_matches(|c| "│└├─ ".contains(c));
        
        if filename.is_empty() {
             warn!("Invalid line format at line {}: '{}'", line_num, line);
             return None;
        }

        let indent_chars = file_part.chars().count() - filename.chars().count();
        if indent_chars % self.symbol_length != 0 {
            warn!(
                "Inconsistent indentation at line {}: expected multiple of {}, got {}",
                line_num, self.symbol_length, indent_chars
            );
        }

        let level = indent_chars / self.symbol_length;

        match FileItem::new(filename.to_string(), level, comment, line_num) {
            Ok(item) => Some(item),
            Err(e) => {
                warn!("Invalid filename at line {}: {}", line_num, e);
                None
            }
        }
    }

    pub fn build_tree<P: AsRef<Path>>(&self, tree_file: P) -> Result<(Rc<RefCell<Node>>, Option<String>), TreeParseError> {
        let file = File::open(tree_file)?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

        if lines.is_empty() {
            return Err(TreeParseError::EmptyFile);
        }

        let mut level_stack: Vec<Rc<RefCell<Node>>> = Vec::new();
        let mut root: Option<Rc<RefCell<Node>>> = None;
        let mut root_name_to_skip: Option<String> = None;

        for (i, line) in lines.iter().enumerate() {
            let line_num = i + 1;
            if let Some(file_item) = self.parse_line(line, line_num) {
                let level = file_item.level;
                let node = Node::new(file_item.clone());

                if level == 0 {
                    if root.is_none() {
                        root = Some(node.clone());
                        level_stack = vec![node.clone()];
                        root_name_to_skip = Some(file_item.name().to_string());
                    } else {
                        warn!("Multiple root-level nodes found at line {}", line_num);
                        if !level_stack.is_empty() {
                            level_stack[0] = node.clone();
                        }
                    }
                } else {
                    if level > level_stack.len() {
                        return Err(TreeParseError::OrphanNode {
                            line: line_num,
                            level,
                            max_level: level_stack.len().saturating_sub(1),
                        });
                    }

                    level_stack.truncate(level);
                    
                    if level > 0 && level_stack.len() >= level {
                        let parent = &level_stack[level - 1];
                        Node::add_child(parent, node.clone());
                    }

                    if level == level_stack.len() {
                        level_stack.push(node);
                    } else {
                        level_stack[level] = node;
                    }
                }
            }
        }

        root.ok_or(TreeParseError::NoNodesFound).map(|r| (r, root_name_to_skip))
    }
}
