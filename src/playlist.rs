use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct PlaylistManager {
    files: Vec<PathBuf>,
    current_index: usize,
}

impl PlaylistManager {
    pub fn new(path: Option<&str>) -> Result<Self, io::Error> {
        let files = match path {
            Some(p) => {
                let path_obj = Path::new(p);
                if path_obj.is_file() {
                    vec![PathBuf::from(p)]
                } else if path_obj.is_dir() {
                    Self::scan_folder(p)?
                } else {
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        format!("Path not found: {}", p),
                    ));
                }
            }
            None => Self::scan_folder("./sgf")?,
        };

        if files.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No SGF files found",
            ));
        }

        Ok(PlaylistManager {
            files,
            current_index: 0,
        })
    }

    fn scan_folder(path: &str) -> Result<Vec<PathBuf>, io::Error> {
        let mut files: Vec<PathBuf> = fs::read_dir(path)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| {
                path.extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext.eq_ignore_ascii_case("sgf"))
                    .unwrap_or(false)
            })
            .collect();

        files.sort_by(|a, b| Self::natural_sort_compare(a, b));
        Ok(files)
    }

    fn natural_sort_compare(a: &Path, b: &Path) -> std::cmp::Ordering {
        let a_name = a.file_stem().unwrap().to_string_lossy();
        let b_name = b.file_stem().unwrap().to_string_lossy();

        let a_tokens = Self::tokenize(&a_name);
        let b_tokens = Self::tokenize(&b_name);

        a_tokens.cmp(&b_tokens)
    }

    fn tokenize(s: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut current_num = String::new();
        let mut current_text = String::new();

        for ch in s.chars() {
            if ch.is_numeric() {
                if !current_text.is_empty() {
                    tokens.push(Token::Text(current_text.to_lowercase()));
                    current_text.clear();
                }
                current_num.push(ch);
            } else {
                if !current_num.is_empty() {
                    if let Ok(num) = current_num.parse::<usize>() {
                        tokens.push(Token::Number(num));
                    }
                    current_num.clear();
                }
                current_text.push(ch);
            }
        }

        if !current_text.is_empty() {
            tokens.push(Token::Text(current_text.to_lowercase()));
        }
        if !current_num.is_empty() {
            if let Ok(num) = current_num.parse::<usize>() {
                tokens.push(Token::Number(num));
            }
        }

        tokens
    }

    pub fn current(&self) -> &Path {
        &self.files[self.current_index]
    }

    pub fn has_next(&self) -> bool {
        self.current_index + 1 < self.files.len()
    }

    pub fn next(&mut self) -> bool {
        if self.has_next() {
            self.current_index += 1;
            true
        } else {
            false
        }
    }

    pub fn peek_next(&self) -> Option<&Path> {
        if self.has_next() {
            Some(&self.files[self.current_index + 1])
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.current_index = 0;
    }

    pub fn is_single_file(&self) -> bool {
        self.files.len() == 1
    }

    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Token {
    Text(String),
    Number(usize),
}
