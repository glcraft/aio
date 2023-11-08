use std::borrow::Cow;

pub fn home_dir() -> &'static str {
    static HOME: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
        #[cfg(unix)]
        let path = std::env::var("HOME")
            .expect("Failed to resolve home path");
        
        #[cfg(windows)]
        let path = std::env::var("USERPROFILE")
            .expect("Failed to resolve user profile path");
        path
    });

    &HOME
}

pub fn config_dir() -> &'static str {
    static CONFIG: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
        std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
            format!("{}{}.config", home_dir(), std::path::MAIN_SEPARATOR)
        })
    });
    &CONFIG
}

pub fn cache_dir() -> &'static str {
    static CACHE: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
        std::env::var("XDG_CACHE_HOME").unwrap_or_else(|_| {
            format!("{}{}.cache", home_dir(), std::path::MAIN_SEPARATOR)
        })
    });
    &CACHE
}

pub fn resolve_path(path: &str) -> Cow<str> {
    if let Some(path) = path.strip_prefix("~/") {
        Cow::Owned(format!("{}{}{}", home_dir(), std::path::MAIN_SEPARATOR, path))
    } else {
        Cow::Borrowed(path)
    }
}

pub fn config_path(path: &std::path::Path) -> Option<Cow<'_, std::path::Path>> {
    if path.exists() {
        return Some(Cow::Borrowed(path))
    }
    let new_extension = match path.extension().and_then(|e| e.to_str()) {
        Some("yml") => "yaml",
        Some("yaml") => "yml",
        _ => return None
    };
    let new_path = path.with_extension(new_extension);
    if new_path.exists() {
        return Some(Cow::Owned(new_path));
    }
    None
}