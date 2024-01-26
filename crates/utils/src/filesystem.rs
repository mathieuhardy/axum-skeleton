//! This file provided utility functions related to filesystem.

use std::path::PathBuf;
use std::str::FromStr;

use crate::prelude::*;

/// Returns a path relative to the current directory.
///
/// # Arguments:
/// * `path` - Path to add to the current directory.
///
/// # Returns:
/// The relative path or an error.
pub fn relative_path(path: &str) -> Res<PathBuf> {
    let base_path = match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(dir) => PathBuf::from_str(&dir).map_err(Error::Unexpected),
        Err(_) => std::env::current_dir().map_err(Error::Filesystem),
    }?;

    let path = base_path.join(path);

    let exists = path.try_exists().map_err(Error::Filesystem)?;

    if !exists {
        return Err(Error::PathNotFound(path));
    }

    Ok(path)
}

/// Returns a path relative to the root directory.
///
/// # Arguments:
/// * `path` - Path to add to the root directory.
///
/// # Returns:
/// The relative path or an error.
pub fn root_relative_path(path: &str) -> Res<PathBuf> {
    let base_path = project_root::get_project_root().map_err(Error::Filesystem)?;
    let path = base_path.join(path);
    let exists = path.try_exists().map_err(Error::Filesystem)?;

    if !exists {
        return Err(Error::PathNotFound(path));
    }

    Ok(path)
}
