// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.

use deno_core::anyhow::Context;
use deno_core::error::AnyError;
pub use deno_core::normalize_path;
use std::env::current_dir;
use std::io::Error;
use std::path::Path;
use std::path::PathBuf;

/// Similar to `std::fs::canonicalize()` but strips UNC prefixes on Windows.
pub fn canonicalize_path(path: &Path) -> Result<PathBuf, Error> {
  Ok(deno_core::strip_unc_prefix(path.canonicalize()?))
}

#[inline]
pub fn resolve_from_cwd(path: &Path) -> Result<PathBuf, AnyError> {
  if path.is_absolute() {
    Ok(normalize_path(path))
  } else {
    let cwd =
      current_dir().context("Failed to get current working directory")?;
    Ok(normalize_path(cwd.join(path)))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn resolve_from_cwd_child() {
    let cwd = current_dir().unwrap();
    assert_eq!(resolve_from_cwd(Path::new("a")).unwrap(), cwd.join("a"));
  }

  #[test]
  fn resolve_from_cwd_dot() {
    let cwd = current_dir().unwrap();
    assert_eq!(resolve_from_cwd(Path::new(".")).unwrap(), cwd);
  }

  #[test]
  fn resolve_from_cwd_parent() {
    let cwd = current_dir().unwrap();
    assert_eq!(resolve_from_cwd(Path::new("a/..")).unwrap(), cwd);
  }

  #[test]
  fn test_normalize_path() {
    assert_eq!(normalize_path(Path::new("a/../b")), PathBuf::from("b"));
    assert_eq!(normalize_path(Path::new("a/./b/")), PathBuf::from("a/b/"));
    assert_eq!(
      normalize_path(Path::new("a/./b/../c")),
      PathBuf::from("a/c")
    );

    if cfg!(windows) {
      assert_eq!(
        normalize_path(Path::new("C:\\a\\.\\b\\..\\c")),
        PathBuf::from("C:\\a\\c")
      );
    }
  }

  #[test]
  fn resolve_from_cwd_absolute() {
    let expected = Path::new("a");
    let cwd = current_dir().unwrap();
    let absolute_expected = cwd.join(expected);
    assert_eq!(resolve_from_cwd(expected).unwrap(), absolute_expected);
  }
}
