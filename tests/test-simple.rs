mod support;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

use crate::support::{create_test_schema, seed_test_data, XVec};
use pretty_sqlite::pretty_table;
use rusqlite::Connection;

#[test]
fn test_pretty_table() -> Result<()> {
	// -- Setup & Fixtures
	let conn = Connection::open_in_memory()?; // for file: Connection::open(path)?
	create_test_schema(&conn)?;
	seed_test_data(&conn)?;

	// -- Exec
	let content = pretty_table(&conn, "person")?;
	let lines: Vec<&str> = content.split('\n').collect();

	// -- Check
	// header row
	let header_row = lines.x_get(2).ok_or("get 2 should be first row, header")?;
	assert!(
		header_row.starts_with("│ id  │ name"),
		"Should have started with '│ id │ name'"
	);
	// first data row
	let first_data_row = lines.x_get(4).ok_or("get 3 should be first data row")?;
	assert!(
		first_data_row.starts_with(r#"│ 1   │ "Person 1""#),
		r#"Should have started with '│ 1   │ "Person 1"'"#
	);

	// last data row
	let last_row = lines.x_get(-2).ok_or("get -2 should have been the last row (id 300)")?;
	assert!(last_row.starts_with("│ 300"), "Should have started with '│ 300'");

	// -- Check

	Ok(())
}
