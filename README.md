# 🧐 browser-paths

<!-- automdrs:badges showCrateVersion="true" showCrateDownloads="true" showCrateDocs="true" showCommitActivity="true" showRepoStars="true" -->
![Crates.io Version](https://img.shields.io/crates/v/browser-paths)
![Crates.io Total Downloads](https://img.shields.io/crates/d/browser-paths)
![docs.rs](https://img.shields.io/docsrs/browser-paths)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/un-rust/browser-paths)
![GitHub Repo stars](https://img.shields.io/github/stars/un-rust/browser-paths)
<!-- /automdrs -->

<!-- automdrs:description -->

Get the path of the browser, support Chrome and Edge.

<!-- /automdrs -->

**[Full documentation →](https://docs.rs/browser-paths/)**

## Quick start

<!-- automdrs:cargo-add -->

```sh
cargo add browser-paths
```

<!-- /automdrs -->

## Usage

<!-- automdrs:file src="./src/main.rs" -->
```rust
use browser_paths::{BrowserKind, get_browser_path};

fn main() {
    let chrome_path = get_browser_path(BrowserKind::Chrome);
    println!("chrome_path: {:?}", chrome_path);

    let edge_path = get_browser_path(BrowserKind::Edge);
    println!("edge_path: {:?}", edge_path);
}
```
<!-- /automdrs -->

## License

<!-- automdrs:contributors author="UnRUST" license="Apache-2.0" -->
Published under the [Apache-2.0](./LICENSE) license.
Made by [@UnRUST](https://github.com/un-rust) 💛
<br><br>
<a href="https://github.com/un-rust/browser-paths/graphs/contributors">
<img src="https://contrib.rocks/image?repo=un-rust/browser-paths" />
</a>
<!-- /automdrs -->

<!-- automdrs:with-automdrs -->

---

_🛠️ auto updated with [automd-rs](https://github.com/betterhyq/automd-rs)_

<!-- /automdrs -->