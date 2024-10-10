use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use notify::{
    event::{AccessKind, AccessMode},
    Event, EventKind, RecursiveMode, Watcher,
};

#[allow(unused)]
#[derive(Debug, Clone)]
struct File {
    file_name: PathBuf,
    path: PathBuf,
    exists: bool,
    is_file: bool,
}

impl From<Event> for File {
    fn from(event: Event) -> Self {
        let file_name = event
            .paths
            .first()
            .and_then(|p| p.file_name())
            .and_then(|p| p.to_str())
            .and_then(|p| PathBuf::from_str(p).ok())
            .unwrap_or_default();
        let path = event.paths.first().unwrap_or(&PathBuf::new()).to_owned();
        let exists = path.exists();
        let is_file = path.is_file();

        Self {
            file_name,
            path,
            exists,
            is_file,
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SavedFile {
    pub file_name: PathBuf,
    pub path: PathBuf,
}

impl From<File> for SavedFile {
    fn from(file: File) -> Self {
        Self {
            file_name: file.file_name,
            path: file.path,
        }
    }
}

pub fn build_watcher(
    folder: &str,
) -> (
    notify::INotifyWatcher,
    tokio::sync::mpsc::Receiver<SavedFile>,
) {
    let (tx, rx) = tokio::sync::mpsc::channel::<SavedFile>(10);

    let event_handler = move |res| match res {
        Ok(event) => handle_event(event, tx.clone()),
        Err(e) => println!("watch error: {:?}", e),
    };

    let config = notify::Config::default();
    let mut watcher = notify::RecommendedWatcher::new(event_handler, config).unwrap();
    watcher
        .watch(Path::new(folder), RecursiveMode::Recursive)
        .unwrap();

    (watcher, rx)
}

fn handle_event(event: Event, tx: tokio::sync::mpsc::Sender<SavedFile>) {
    if let EventKind::Access(AccessKind::Close(AccessMode::Write)) = event.kind {
        let file: File = event.clone().into();

        if file.exists && file.is_file && tx.blocking_send(file.into()).is_err() {
            println!("Error Sending FileEvent Message")
        }
    }
}
