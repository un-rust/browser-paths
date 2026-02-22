use browser_paths::{BrowserKind, get_browser_path};

fn main() {
    let chrome_path = get_browser_path(BrowserKind::Chrome);
    println!("chrome_path: {:?}", chrome_path);

    let edge_path = get_browser_path(BrowserKind::Edge);
    println!("edge_path: {:?}", edge_path);
}
