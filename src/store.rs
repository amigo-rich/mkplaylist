use crate::music::Music;
use rusqlite::{params, Connection};
use std::path::Path;

#[derive(Debug)]
pub struct Store {
    con: Connection,
}

impl Store {
    pub fn create(path: &Path) -> Result<Self, ()> {
        let con = Connection::open(path).unwrap();
        let schema = r#"
            CREATE TABLE music (
                id INTEGER PRIMARY KEY,
                path TEXT NOT NULL UNIQUE
            )
        "#;
        let _ = con.execute(schema, params![]).unwrap();
        Ok(Store { con })
    }
    pub fn open(path: &Path) -> Result<Self, ()> {
        if !path.is_file() {
            return Err(());
        }
        let con = Connection::open(path).unwrap();
        Ok(Store { con })
    }
    pub fn insert<I>(&mut self, iter: I) -> Result<(), ()>
    where
        I: Iterator<Item = Music>,
    {
        let sql = r#"
            INSERT INTO music (path)
            VALUES (?1)
        "#;

        let tx = self.con.transaction().unwrap();
        for item in iter {
            tx.execute(sql, params![item.path]).unwrap();
        }
        tx.commit().unwrap();
        Ok(())
    }
    pub fn select(&self) -> Result<Option<Vec<Music>>, ()> {
        let sql = r#"
            SELECT path
            FROM music
        "#;

        let mut statement = self.con.prepare(sql).unwrap();
        let iter = statement
            .query_map(params![], |row| {
                Ok(Music {
                    path: row.get(0).unwrap(),
                })
            })
            .unwrap();
        let mut music: Vec<Music> = Vec::new();
        for item in iter {
            music.push(item.unwrap());
        }
        if music.is_empty() {
            return Ok(None);
        }
        Ok(Some(music))
    }
    pub fn select_filter(&self, filter: &str) -> Result<Option<Vec<Music>>, ()> {
        let sql = r#"
            SELECT path
            FROM music
            WHERE LIKE(?1, path)
        "#;

        let mut statement = self.con.prepare(sql).unwrap();
        // work around an issue with parameters and LIKE
        let filter = format!("%{}%", filter);
        let iter = statement
            .query_map(params![&filter], |row| {
                Ok(Music {
                    path: row.get(0).unwrap(),
                })
            })
            .unwrap();
        let mut music: Vec<Music> = Vec::new();
        for item in iter {
            music.push(item.unwrap());
        }
        if music.is_empty() {
            return Ok(None);
        }
        Ok(Some(music))
    }
}
