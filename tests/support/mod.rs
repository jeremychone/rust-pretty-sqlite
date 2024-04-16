pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

use rusqlite::{params, Connection};

pub fn create_test_schema(conn: &Connection) -> Result<()> {
	conn.execute(
		"CREATE TABLE IF NOT EXISTS person (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            name    TEXT,
            yob     INTEGER, -- year of birth
            data_t  TEXT,
            data_b  BLOB
        ) STRICT",
		(), // empty list of parameters.
	)?;
	Ok(())
}

pub fn seed_test_data(con: &Connection) -> Result<()> {
	let mut stmt = con.prepare("INSERT INTO person (name, yob, data_t, data_b) VALUES (?, ?, ?, ?)")?;

	for i in 1..=1000 {
		// start at 1 to match sqlite id
		let name = format!("Person {}", i);
		let yob = 1950 + (i % 50);
		let data_t = format!("Data {}", i);
		let data_b = vec![0u8; 10]; // Example binary data

		stmt.execute(params![name, yob, data_t, data_b])?;
	}

	Ok(())
}

// region:    --- XVec

pub trait XVec<T> {
	fn x_get(&self, idx: isize) -> Option<&T>;
}

impl<T> XVec<T> for Vec<T> {
	fn x_get(&self, idx: isize) -> Option<&T> {
		if idx >= 0 {
			self.get(idx as usize)
		} else {
			self.get((self.len() as isize + idx) as usize)
		}
	}
}

// endregion: --- XVec
