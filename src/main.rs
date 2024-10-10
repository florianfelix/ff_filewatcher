mod watcher;

#[tokio::main]
async fn main() {
    let (_watcher, mut rx) = watcher::build_watcher("watched");

    loop {
        tokio::select! {
            Some(saved_file) = rx.recv() => {println!("{:#?}", &saved_file)},
        }
    }
}
