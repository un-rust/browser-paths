use crate::browser_name::{BrowserChannel, BrowserKind, BrowserName, Platform};
use std::path::PathBuf;
use which::which;

#[cfg(target_os = "linux")]
fn get_browser_path_on_linux(
    browser_kind: BrowserKind,
    channel: BrowserChannel,
) -> Option<PathBuf> {
    let browser_name = BrowserName(Platform::Linux, browser_kind, channel);
    which(browser_name.to_str()).ok()
}

#[cfg(target_os = "windows")]
fn get_browser_path_on_windows(
    browser_kind: BrowserKind,
    channel: BrowserChannel,
) -> Option<PathBuf> {
    let browser_name = BrowserName(Platform::Windows, browser_kind, channel);
    let suffix = match browser_kind {
        BrowserKind::Chrome => {
            format!("Google\\{}\\Application\\chrome.exe", browser_name.to_str())
        }
        BrowserKind::Edge => format!(
            "Microsoft\\{}\\Application\\msedge.exe",
            browser_name.to_str()
        ),
    };
    [
        std::env::var_os("LOCALAPPDATA").map(PathBuf::from),
        std::env::var_os("PROGRAMFILES").map(PathBuf::from),
        std::env::var_os("ProgramFiles(x86)").map(PathBuf::from),
    ]
    .into_iter()
    .flatten()
    .map(|prefix| prefix.join(&suffix))
    .find(|path| path.exists())
}

#[cfg(target_os = "macos")]
fn get_browser_path_on_macos(
    browser_kind: BrowserKind,
    channel: BrowserChannel,
) -> Option<PathBuf> {
    let browser_name = BrowserName(Platform::MacOS, browser_kind, channel);
    let default_path = PathBuf::from(format!(
        "/Applications/{}.app/Contents/MacOS/{}",
        browser_name.to_str(),
        browser_name.to_str()
    ));
    default_path.exists().then_some(default_path)
}

#[cfg(target_os = "linux")]
fn get_browser_path_for_channel(
    browser_kind: BrowserKind,
    channel: BrowserChannel,
) -> Option<PathBuf> {
    get_browser_path_on_linux(browser_kind, channel)
}

#[cfg(target_os = "windows")]
fn get_browser_path_for_channel(
    browser_kind: BrowserKind,
    channel: BrowserChannel,
) -> Option<PathBuf> {
    get_browser_path_on_windows(browser_kind, channel)
}

#[cfg(target_os = "macos")]
fn get_browser_path_for_channel(
    browser_kind: BrowserKind,
    channel: BrowserChannel,
) -> Option<PathBuf> {
    get_browser_path_on_macos(browser_kind, channel)
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
fn get_browser_path_for_channel(
    _browser_kind: BrowserKind,
    _channel: BrowserChannel,
) -> Option<PathBuf> {
    None
}

pub fn get_browser_path(browser_kind: BrowserKind) -> Option<PathBuf> {
    get_browser_path_for_channel(browser_kind, BrowserChannel::Stable)
}

pub fn get_browser_dev_path(browser_kind: BrowserKind) -> Option<PathBuf> {
    get_browser_path_for_channel(browser_kind, BrowserChannel::Dev)
}

pub fn get_browser_beta_path(browser_kind: BrowserKind) -> Option<PathBuf> {
    get_browser_path_for_channel(browser_kind, BrowserChannel::Beta)
}

pub fn get_browser_canary_path(browser_kind: BrowserKind) -> Option<PathBuf> {
    get_browser_path_for_channel(browser_kind, BrowserChannel::Canary)
}

pub fn get_any_browser_latest(browser_kind: BrowserKind) -> Option<PathBuf> {
    [
        BrowserChannel::Canary,
        BrowserChannel::Dev,
        BrowserChannel::Beta,
        BrowserChannel::Stable,
    ]
    .into_iter()
    .find_map(|channel| get_browser_path_for_channel(browser_kind, channel))
}

pub fn get_any_browser_stable(browser_kind: BrowserKind) -> Option<PathBuf> {
    [
        BrowserChannel::Stable,
        BrowserChannel::Beta,
        BrowserChannel::Dev,
        BrowserChannel::Canary,
    ]
    .into_iter()
    .find_map(|channel| get_browser_path_for_channel(browser_kind, channel))
}
