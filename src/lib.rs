pub mod error;
use error::Error;
mod indexer;
use indexer::index_directory;
mod music;
use music::Music;
pub mod operation;
use operation::{Operation, PlayList};
mod rating;
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

pub fn run(operation: Operation) -> Result<(), Error> {
    let path = Path::new("test.sqlite");
    let mut store = match path.is_file() {
        false => Store::create(path)?,
        true => Store::open(path)?,
    };
    match operation {
        Operation::Create(playlist) => {
            let music = match playlist {
                PlayList::Standard | PlayList::Shuffled => store.select()?,
                PlayList::Filtered(filter) | PlayList::ShuffledFiltered(filter) => {
                    store.select_filter(filter)?
                }
                PlayList::Rated(ref rating) => store.select_by_rating(rating.as_i64())?,
            };
            if let Some(mut music) = music {
                match playlist {
                    PlayList::Shuffled | PlayList::ShuffledFiltered(_) => {
                        let mut rng = thread_rng();
                        music.shuffle(&mut rng);
                    }
                    _ => (),
                }
                display(music.iter());
            }
        }
        Operation::Index(path) => {
            let music = index_directory(path)?;
            let _ = store.insert(music.into_iter());
        }
        Operation::Rate(music_id, rating_value) => {
            let inserts = vec![(music_id, rating_value)];
            let _ = store.insert_rating(inserts.into_iter())?;
        }
    }
    Ok(())
}
