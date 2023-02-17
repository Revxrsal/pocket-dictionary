use sqlite::{ConnectionWithFullMutex, State, Result};

/// Stores the SQLite connection
pub struct Database {
    connection: ConnectionWithFullMutex,
}

impl Database {

    /// Create a new Database
    pub fn new(connection: ConnectionWithFullMutex) -> Self {
        Self { connection }
    }

    /// Return the HTML connected to the entry
    pub fn get_html(&self, entry: &str) -> Result<Option<String>> {
        let mut statement = self.connection.prepare("select html from words where entry = ? limit 1")?;
        statement.bind((1, entry)).unwrap();

        while let Ok(State::Row) = statement.next() {
            return Ok(Some(statement.read::<String, _>("html").unwrap()));
        }
        return Ok(None);
    }
}