use crate::errors::Error;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub struct InitDbConfig {
    statements: Vec<String>,
}

pub struct MtSqLiteClient {
    /// Use case: 1 connection shared between threads
    pub connection: Arc<Mutex<Connection>>,
}

impl MtSqLiteClient {

    /// Creates a client for SqLite.
    ///
    /// # Examples
    ///
    /// - **Empty db**
    ///  ```
    /// use sqlite_client::client::MtSqLiteClient;
    /// let client = MtSqLiteClient::new("my_db.db").unwrap();
    /// ```
    ///
    ///
    pub fn new(db: &str) -> Result<Self, Error> {
        Self::_new(Some(db))
    }

    /// Creates a client for SqLite (In-Memory).
    ///
    /// # Examples
    ///
    /// - **Empty db**
    ///  ```
    /// use sqlite_client::client::MtSqLiteClient;
    /// let client = MtSqLiteClient::new_in_memory().unwrap();
    /// ```
    ///
    pub fn new_in_memory() -> Result<Self, Error> {
        Self::_new(None)
    }
    fn _new(db: Option<&str>) -> Result<Self, Error> {
        let connection = Self::get_connection(db)?;

        let connection = Arc::new(Mutex::new(connection));

        Ok(Self { connection })
    }

    fn get_connection(db: Option<&str>) -> Result<Connection, Error> {
        let connection = if let Some(db) = db {
            Connection::open(db)
        } else {
            Connection::open_in_memory()
        };
        Ok(connection.map_err(Error::ConnectionError)?)
    }
}