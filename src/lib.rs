mod indexer;
use indexer::index_directory;
mod music;
pub mod operation;
use operation::Operation;
mod store;
use store::Store;

use std::path::Path;

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
                for m in music {
                    println!("{}", m);
                }
            }
        }
        Operation::PlayList(None) => {
            let music = store.select().unwrap();
            if let Some(music) = music {
                for m in music {
                    println!("{}", m);
                }
            }
        }
    }
    Ok(())
}
