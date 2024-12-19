use anyhow::Result;
use std::path::PathBuf;

pub struct File {
    pub path: String,
    pub contents: String,
    pub rows: Vec<String>,
}

impl File {
    pub fn new(path: PathBuf) -> Result<Self> {
        let contents = std::fs::read_to_string(&path)?;
        let path = path.display().to_string();
        let rows = contents.lines().map(|s| s.to_string()).collect();
        Ok(Self {
            path,
            contents,
            rows,
        })
    }
}
