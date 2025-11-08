use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn find_in_path(cmd: &str) -> Option<PathBuf> {
    let path = Path::new(cmd);
    if path.components().count() > 1 {
        if is_executable(&path) {
            return Some(path.to_path_buf());
        }
        return None;
    }
    let path_var = env::var("PATH").unwrap_or_default();
    for dir in path_var.split(':') {
        let cand = Path::new(dir).join(cmd);
        if is_executable(&cand) {
            return Some(cand);
        }
    }
    None
}

pub fn is_executable(path: &Path) -> bool {
    if let Ok(meta) = fs::metadata(path) {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = meta.permissions().mode();
            (mode & 0o111) != 0
        }
        #[cfg(not(unix))]
        {
            // fallback: treat any file as executable
            meta.is_file()
        }
    } else {
        false
    }
}

pub fn expand_tilde(path: &str) -> String {
    if !path.starts_with('~') {
        return path.to_string();
    }
    let home = env::var("HOME").unwrap_or_else(|_| "/".to_string());

    if path == "~" {
        return home;
    }

    if let Some(rest) = path.strip_prefix("~/") {
        let mut pb = PathBuf::from(home);
        pb.push(rest);
        return pb.to_string_lossy().to_string();
    }

    path.to_string()
}
