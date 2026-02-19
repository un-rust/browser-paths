/// A unified platform
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Platform {
    Linux,
    MacOS,
    Windows,
}

/// A unified browser kind
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BrowserKind {
    Chrome,
    Edge,
}

/// A unified browser release channel
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BrowserChannel {
    Stable,
    Dev,
    Beta,
    Canary,
}

#[derive(Clone, Copy, Debug)]
pub struct BrowserName(pub Platform, pub BrowserKind, pub BrowserChannel);

impl BrowserName {
    pub fn to_str(&self) -> &'static str {
        match (self.0, self.1, self.2) {
            (Platform::Linux, BrowserKind::Chrome, BrowserChannel::Beta) => "google-chrome-beta",
            (Platform::Linux, BrowserKind::Chrome, BrowserChannel::Stable) => {
                "google-chrome-stable"
            }
            (Platform::Linux, BrowserKind::Chrome, BrowserChannel::Dev) => "google-chrome-unstable",
            (Platform::Linux, BrowserKind::Chrome, BrowserChannel::Canary) => {
                "google-chrome-canary"
            }
            (Platform::MacOS, BrowserKind::Chrome, BrowserChannel::Beta) => "Google Chrome Beta",
            (Platform::MacOS, BrowserKind::Chrome, BrowserChannel::Stable) => "Google Chrome",
            (Platform::MacOS, BrowserKind::Chrome, BrowserChannel::Dev) => "Google Chrome Dev",
            (Platform::MacOS, BrowserKind::Chrome, BrowserChannel::Canary) => {
                "Google Chrome Canary"
            }
            (Platform::Windows, BrowserKind::Chrome, BrowserChannel::Beta) => "Chrome Beta",
            (Platform::Windows, BrowserKind::Chrome, BrowserChannel::Stable) => "Chrome",
            (Platform::Windows, BrowserKind::Chrome, BrowserChannel::Dev) => "Chrome Dev",
            (Platform::Windows, BrowserKind::Chrome, BrowserChannel::Canary) => "Chrome Canary",
            (Platform::Linux, BrowserKind::Edge, BrowserChannel::Beta) => "microsoft-edge-beta",
            (Platform::Linux, BrowserKind::Edge, BrowserChannel::Stable) => "microsoft-edge-stable",
            (Platform::Linux, BrowserKind::Edge, BrowserChannel::Dev) => "microsoft-edge-dev",
            (Platform::Linux, BrowserKind::Edge, BrowserChannel::Canary) => "microsoft-edge-canary",
            (Platform::MacOS, BrowserKind::Edge, BrowserChannel::Beta) => "Microsoft Edge Beta",
            (Platform::MacOS, BrowserKind::Edge, BrowserChannel::Stable) => "Microsoft Edge",
            (Platform::MacOS, BrowserKind::Edge, BrowserChannel::Dev) => "Microsoft Edge Dev",
            (Platform::MacOS, BrowserKind::Edge, BrowserChannel::Canary) => "Microsoft Edge Canary",
            (Platform::Windows, BrowserKind::Edge, BrowserChannel::Beta) => "Edge Beta",
            (Platform::Windows, BrowserKind::Edge, BrowserChannel::Stable) => "Edge",
            (Platform::Windows, BrowserKind::Edge, BrowserChannel::Dev) => "Edge Dev",
            (Platform::Windows, BrowserKind::Edge, BrowserChannel::Canary) => "Edge Canary",
        }
    }
}
