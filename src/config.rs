use std::path::{Path, PathBuf};
use anyhow::Result;

pub fn db_path() -> Result<PathBuf> { 
    let mut p = dirs::config_dir().ok_or(anyhow::anyhow!("Cannot open default config dir"))?;
    // TODO: make configurable 
    p.push("data.db");
    Ok(p)
}

pub fn test_db_path() -> Result<PathBuf> { 
    let mut p = PathBuf::new();
    p.push("test.db");
    Ok(p)
}