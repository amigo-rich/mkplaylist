mod indexer;
use indexer::index_directory;
mod music;
use music::Music;
pub mod operation;
use operation::Operation;
mod store;
use store::Store;

use rand::{seq::SliceRandom, thread_rng};
use std::path::Path;

pub fn display<'a, I>(iter: I)
where
    I: Iterator<Item = &'a Music>,
{
    for music in iter {
        println!("{}", music);
    }
}

pub fn run(operation: Operation) -> Result<(), ()> {
    let path = Path::new("test.sqlite");
    let mut store = match path.is_file() {
        false => Store::create(path).unwrap(),
        true => Store::open(path).unwrap(),
    };
    match operation {
        Operation::Index(path) => {
            let music = index_directory(path).unwrap();
            let _ = store.insert(music.into_iter());
        }
        Operation::PlayList(maybe_filter, shuffle) => {
            let music = if let Some(filter) = maybe_filter {
                store.select_filter(filter).unwrap()
            } else {
                store.select().unwrap()
            };
            if let Some(mut music) = music {
                if shuffle {
                    let mut rng = thread_rng();
                    music.shuffle(&mut rng);
                }
                display(music.iter());
            }
        }
    }
    Ok(())
}
