use std::time::Duration;
use mini_moka::sync::{Cache, CacheBuilder};
use sqlite::{ConnectionWithFullMutex, State, Result};

/// Stores the SQLite connection
pub struct Database {
    connection: ConnectionWithFullMutex,
    cache: Cache<String, Option<String>>,
}

impl Database {
    /// Create a new Database
    pub fn new(connection: ConnectionWithFullMutex) -> Self {
        Self {
            connection,
            cache: CacheBuilder::new(1_000)
                .time_to_idle(Duration::from_secs(30 * 60))
                .build(),
        }
    }

    /// Return the HTML connected to the entry
    pub fn get_html(&mut self, entry: &str) -> Result<Option<String>> {
        let mut statement = self.connection.prepare("select html from words where entry = ? limit 1")?;
        statement.bind((1, entry)).unwrap();

        while let Ok(State::Row) = statement.next() {
            let html = statement.read::<String, _>("html").unwrap();
            return Ok(Some(html));
        }
        return Ok(None);
    }

    /// A fail-safe method for retrieving
    pub fn lookup(&mut self, entry: &str) -> Option<String> {
        let string = entry.to_owned();
        if let Some(result) = self.cache.get(&string) {
            return result.to_owned();
        }
        let html = self.get_html(entry);
        return match html {
            Ok(v) => {
                self.cache.insert(string, v.to_owned());
                v
            }
            Err(_) => {
                self.cache.insert(string, None);
                None
            }
        };
    }
}