use crate::music::Music;
use std::fs::{self};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Sender};
use std::thread;

pub fn index_directory(path: PathBuf) -> Result<Vec<Music>, ()> {
    if !path.is_dir() {
        return Err(());
    }

    let (sender, receiver) = channel();
    thread::spawn(move || {
        visit_dirs(&path, &sender).unwrap();
    });

    let mut items: Vec<Music> = Vec::new();
    for pb in receiver {
        if let Some(path) = pb.to_str() {
            items.push(Music {
                path: path.to_string(),
            });
        } else {
            // XXX log etc.
        }
    }
    Ok(items)
}

// borrowed/adapted from the rust docs
fn visit_dirs(dir: &Path, sender: &Sender<PathBuf>) -> Result<(), ()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let pb = entry.path();
            if pb.is_dir() {
                visit_dirs(&pb, sender)?;
            } else {
                sender.send(pb).unwrap();
            }
        }
    }
    Ok(())
}
