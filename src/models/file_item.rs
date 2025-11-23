use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileItemError {
    #[error("Filename cannot be empty or whitespace")]
    EmptyFilename,
    #[error("Filename contains invalid characters: {0}")]
    InvalidCharacters(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileItem {
    pub filename: String,
    pub level: usize,
    pub comment: String,
    pub line_number: usize,
}

impl FileItem {
    pub fn new(filename: String, level: usize, comment: String, line_number: usize) -> Result<Self, FileItemError> {
        Self::validate_filename(&filename)?;
        Ok(Self {
            filename,
            level,
            comment,
            line_number,
        })
    }

    fn validate_filename(filename: &str) -> Result<(), FileItemError> {
        if filename.trim().is_empty() {
            return Err(FileItemError::EmptyFilename);
        }

        let invalid_chars = ['<', '>', ':', '"', '|', '?', '*'];
        if filename.chars().any(|c| invalid_chars.contains(&c)) {
            return Err(FileItemError::InvalidCharacters(format!("{:?}", invalid_chars)));
        }

        Ok(())
    }

    pub fn is_directory(&self) -> bool {
        !self.filename.contains('.') || self.filename.ends_with('/')
    }

    pub fn name(&self) -> &str {
        self.filename.trim_end_matches('/')
    }

    pub fn extension(&self) -> Option<&str> {
        if self.is_directory() {
            None
        } else {
            self.filename.split('.').last().filter(|ext| !ext.is_empty() && self.filename.contains('.'))
        }
    }

    pub fn name_without_extension(&self) -> &str {
        if self.is_directory() {
            self.filename.trim_end_matches('/')
        } else if let Some(idx) = self.filename.rfind('.') {
             &self.filename[..idx]
        } else {
            &self.filename
        }
    }
}

impl fmt::Display for FileItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_filename() {
        let item = FileItem::new("test.txt".to_string(), 0, "".to_string(), 1).unwrap();
        assert_eq!(item.filename, "test.txt");
    }

    #[test]
    fn test_invalid_filename() {
        assert!(FileItem::new("".to_string(), 0, "".to_string(), 1).is_err());
        assert!(FileItem::new("test?.txt".to_string(), 0, "".to_string(), 1).is_err());
    }

    #[test]
    fn test_is_directory() {
        let dir = FileItem::new("folder/".to_string(), 0, "".to_string(), 1).unwrap();
        assert!(dir.is_directory());
        
        let dir2 = FileItem::new("folder".to_string(), 0, "".to_string(), 1).unwrap();
        assert!(dir2.is_directory());

        let file = FileItem::new("file.txt".to_string(), 0, "".to_string(), 1).unwrap();
        assert!(!file.is_directory());
    }
}
