mod browser_name;
mod browser_path;

use std::path::PathBuf;

pub use browser_name::{BrowserChannel, BrowserKind, BrowserName, Platform};
pub use browser_path::{
    get_any_browser_latest, get_any_browser_stable, get_browser_beta_path, get_browser_canary_path,
    get_browser_dev_path, get_browser_path,
};

pub fn get_any_chrome_latest() -> Option<PathBuf> {
    get_any_browser_latest(BrowserKind::Chrome)
}

pub fn get_any_chrome_stable() -> Option<PathBuf> {
    get_any_browser_stable(BrowserKind::Chrome)
}

pub fn get_chrome_dev_path() -> Option<PathBuf> {
    get_browser_dev_path(BrowserKind::Chrome)
}

pub fn get_chrome_beta_path() -> Option<PathBuf> {
    get_browser_beta_path(BrowserKind::Chrome)
}

pub fn get_chrome_canary_path() -> Option<PathBuf> {
    get_browser_canary_path(BrowserKind::Chrome)
}

pub fn get_chrome_path() -> Option<PathBuf> {
    get_browser_path(BrowserKind::Chrome)
}

pub fn get_any_edge_latest() -> Option<PathBuf> {
    get_any_browser_latest(BrowserKind::Edge)
}

pub fn get_any_edge_stable() -> Option<PathBuf> {
    get_any_browser_stable(BrowserKind::Edge)
}

pub fn get_edge_dev_path() -> Option<PathBuf> {
    get_browser_dev_path(BrowserKind::Edge)
}

pub fn get_edge_beta_path() -> Option<PathBuf> {
    get_browser_beta_path(BrowserKind::Edge)
}

pub fn get_edge_canary_path() -> Option<PathBuf> {
    get_browser_canary_path(BrowserKind::Edge)
}

pub fn get_edge_path() -> Option<PathBuf> {
    get_browser_path(BrowserKind::Edge)
}

mod tests {
    #[test]
    fn test_get_chrome_dev_path() {
        let path = super::get_chrome_dev_path();
        assert!(path.is_some());
        assert!(path.unwrap().exists());
    }

    #[test]
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    fn test_get_chrome_beta_path() {
        let path = super::get_chrome_beta_path();
        assert!(path.is_some());
        assert!(path.unwrap().exists());
    }

    #[test]
    fn test_get_chrome_canary_path() {
        let path = super::get_chrome_canary_path();
        assert!(path.is_some());
        assert!(path.unwrap().exists());
    }

    #[test]
    fn test_get_chrome_stable_path() {
        let path = super::get_chrome_path();
        assert!(path.is_some());
        assert!(path.unwrap().exists());
    }

    #[test]
    fn test_get_edge_dev_path() {
        let path = super::get_edge_dev_path();
        assert!(path.is_some());
        assert!(path.unwrap().exists());
    }

    #[test]
    fn test_get_edge_beta_path() {
        let path = super::get_edge_beta_path();
        assert!(path.is_some());
        assert!(path.unwrap().exists());
    }

    #[test]
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    fn test_get_edge_canary_path() {
        let path = super::get_edge_canary_path();
        assert!(path.is_some());
        assert!(path.unwrap().exists());
    }

    #[test]
    fn test_get_edge_stable_path() {
        let path = super::get_edge_path();
        assert!(path.is_some());
        assert!(path.unwrap().exists());
    }
}
