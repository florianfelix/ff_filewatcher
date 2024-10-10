# Simple file watching for tokio

provides an INotifiWatcher and a tokio mpsc Receiver to watch for file save events.

Best just to simply copy the watcher module. Its really small.

Dependencies:
```toml
[dependencies]
notify = { version = "6.1.1", default-features = false }
tokio = { version = "1.40.0", features = ["rt", "rt-multi-thread", "macros", "sync"] }
```

Usage:
```rust
let (_watcher, mut rx) = watcher::build_watcher("watched");

loop {
    tokio::select! {
        Some(saved_file) = rx.recv() => {println!("{:#?}", &saved_file)},
    }
}
```
Returns saved_file:
```rust
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SavedFile {
    pub file_name: PathBuf,
    pub path: PathBuf,
}
```
