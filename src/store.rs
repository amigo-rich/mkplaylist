use crate::error::Error;
use crate::music::Music;

use rusqlite::{params, Connection};
use std::path::Path;

#[derive(Debug)]
pub struct Store {
    con: Connection,
}

impl Store {
    pub fn create(path: &Path) -> Result<Self, Error> {
        let con = Connection::open(path)?;
        let schema = r#"
            CREATE TABLE music (
                id INTEGER PRIMARY KEY,
                path TEXT NOT NULL UNIQUE
            )
        "#;
        let _ = con.execute(schema, params![])?;
        Ok(Store { con })
    }
    pub fn open(path: &Path) -> Result<Self, Error> {
        if !path.is_file() {
            return Err(Error::StoreOpenNoFile(path.to_path_buf()));
        }
        let con = Connection::open(path)?;
        Ok(Store { con })
    }
    pub fn insert<I>(&mut self, iter: I) -> Result<(), Error>
    where
        I: Iterator<Item = Music>,
    {
        let sql = r#"
            INSERT INTO music (path)
            VALUES (?1)
        "#;

        let tx = self.con.transaction()?;
        for item in iter {
            match tx.execute(sql, params![item.path]) {
                Ok(_) => (),
                Err(e) => match e {
                    rusqlite::Error::SqliteFailure(ec, _) => match ec.code {
                        rusqlite::ErrorCode::ConstraintViolation => {
                            continue;
                        }
                        _ => return Err(Error::StoreRusqlite(e)),
                    },
                    _ => return Err(Error::StoreRusqlite(e)),
                },
            }
        }
        tx.commit()?;
        Ok(())
    }
    pub fn select(&self) -> Result<Option<Vec<Music>>, Error> {
        let sql = r#"
            SELECT path
            FROM music
        "#;

        let mut statement = self.con.prepare(sql)?;
        let iter = statement.query_map(params![], |row| Ok(Music { path: row.get(0)? }))?;
        let mut music: Vec<Music> = Vec::new();
        for item in iter {
            music.push(item?);
        }
        if music.is_empty() {
            return Ok(None);
        }
        Ok(Some(music))
    }
    pub fn select_filter(&self, filter: &str) -> Result<Option<Vec<Music>>, Error> {
        let sql = r#"
            SELECT path
            FROM music
            WHERE LIKE(?1, path)
        "#;

        let mut statement = self.con.prepare(sql)?;
        // work around an issue with parameters and LIKE
        let filter = format!("%{}%", filter);
        let iter = statement.query_map(params![&filter], |row| Ok(Music { path: row.get(0)? }))?;
        let mut music: Vec<Music> = Vec::new();
        for item in iter {
            music.push(item?);
        }
        if music.is_empty() {
            return Ok(None);
        }
        Ok(Some(music))
    }
}
