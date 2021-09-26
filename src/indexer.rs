use crate::error::Error;
use crate::music::Music;

use std::fs::{self};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Sender};
use std::thread;

pub fn index_directory(path: PathBuf) -> Result<Vec<Music>, Error> {
    if !path.is_dir() {
        return Err(Error::IndexerPathNotDir(path));
    }

    let (sender, receiver) = channel();
    let visitor = thread::spawn(move || -> Result<(), Error> { visit_dirs(&path, &sender) });

    let mut items: Vec<Music> = Vec::new();
    for pb in receiver {
        if let Some(path) = pb.to_str() {
            items.push(Music {
                path: path.to_string(),
            });
        } else {
            // using the debug formatter for pb is kind of yuck.
            eprintln!("Skipping path '{:?}' due to invalid UTF8", pb);
        }
    }
    // Has the child panicked?
    let maybe_err = match visitor.join() {
        Ok(maybe_err) => maybe_err,
        // we can't really do anything useful here.. just note that an error
        // occurred.
        Err(_) => return Err(Error::IndexerThread),
    };
    // Was an error returned from the closure?
    match maybe_err {
        Ok(_) => Ok(items),
        Err(e) => Err(e),
    }
}

// borrowed/adapted from the rust docs
fn visit_dirs(dir: &Path, sender: &Sender<PathBuf>) -> Result<(), Error> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let pb = entry.path();
            if pb.is_dir() {
                visit_dirs(&pb, sender)?;
            } else {
                sender.send(pb)?;
            }
        }
    }
    Ok(())
}
