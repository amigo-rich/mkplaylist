mod indexer;
use indexer::index_directory;
mod music;
use music::Music;
pub mod operation;
use operation::Operation;
mod store;
use store::Store;

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
        Operation::PlayList(Some(filter)) => {
            let music = store.select_filter(filter).unwrap();
            if let Some(music) = music {
                display(music.iter());
            }
        }
        Operation::PlayList(None) => {
            let music = store.select().unwrap();
            if let Some(music) = music {
                display(music.iter());
            }
        }
    }
    Ok(())
}
