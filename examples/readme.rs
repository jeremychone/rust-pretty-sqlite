use rusqlite::{Connection, params};

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

fn main() -> Result<()> {
	let conn = Connection::open_in_memory()?; // for file: Connection::open(path)?
	create_test_schema(&conn)?;
	seed_test_data(&conn)?;

	// -- Print table
	// pretty_sqlite::print_table(&conn, "person")?;

	// Same as:
	// let content = pretty_sqlite::pretty_table(&conn, "person")?;
	// println!("{content}");

	// -- Print a select
	pretty_sqlite::print_select(&conn, "select id, name, yob from person where id > ?", (2,))?;

	// Same as:
	// let content = pretty_sqlite::pretty_select(&conn, "select id, name, yob from person where id > ?", (2,))?;
	// println!("{content}");

	Ok(())
}

// region:    --- Support

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

	for i in 1..=5 {
		// start at 1 to match sqlite id
		let name = format!("Person {i}");
		let yob = 1950 + (i % 50);
		let data_t = format!("Data {i}");
		let data_b = vec![0u8; 10]; // Example binary data

		stmt.execute(params![name, yob, data_t, data_b])?;
	}

	Ok(())
}
// endregion: --- Support
