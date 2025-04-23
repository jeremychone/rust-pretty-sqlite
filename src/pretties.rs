use crate::options::PrettyOptions;
use crate::{Error, Result};
use rusqlite::types::ValueRef;
use rusqlite::{Connection, Params, Rows};
use std::str::from_utf8;
use std::sync::OnceLock;
use tabled::settings::Style;

/// Default PRETTY OPTIONS
/// Note: Given the simplicy of the current PrettyOptions, this is probably not needed.
static DEFAULT_PRETTY_OPTIONS: OnceLock<PrettyOptions> = OnceLock::new();

pub fn print_table(conn: &Connection, table: &str) -> Result<()> {
	let table_content = pretty_table(conn, table)?;
	println!("{table_content}");

	Ok(())
}

pub fn pretty_table(conn: &Connection, table: &str) -> Result<String> {
	let pretty_options = DEFAULT_PRETTY_OPTIONS.get_or_init(PrettyOptions::default);

	let mut content = format!(" TABLE: {table}\n");

	let select_content = pretty_select_with_options(
		conn,
		&format!("SELECT * FROM {table} limit {}", pretty_options.rows_limit),
		[],
		pretty_options,
	)?;

	content.push_str(&select_content);

	Ok(content)
}

pub fn print_select(conn: &Connection, sql: &str, params: impl Params) -> Result<()> {
	let select_content = pretty_select(conn, sql, params)?;
	println!("{select_content}");
	Ok(())
}

pub fn pretty_select(conn: &Connection, sql: &str, params: impl Params) -> Result<String> {
	let pretty_options = DEFAULT_PRETTY_OPTIONS.get_or_init(PrettyOptions::default);

	let mut stmt = conn.prepare(sql)?;
	let rows = stmt.query(params)?;

	pretty_rows(rows, pretty_options)
}

pub fn pretty_select_with_options(
	conn: &Connection,
	sql: &str,
	params: impl Params,
	options: &PrettyOptions,
) -> Result<String> {
	let mut stmt = conn.prepare(sql)?;
	let rows = stmt.query(params)?;

	pretty_rows(rows, options)
}

pub fn print_rows(rows: Rows<'_>) -> Result<()> {
	let pretty_options = DEFAULT_PRETTY_OPTIONS.get_or_init(PrettyOptions::default);
	let rows_content = pretty_rows(rows, pretty_options)?;
	println!("{rows_content}");
	Ok(())
}

fn pretty_rows(mut rows: Rows<'_>, options: &PrettyOptions) -> Result<String> {
	enum SubType {
		Time,
		None,
	}

	let stmt = rows.as_ref().ok_or(Error::CantPrintRowsHasNoStatement)?;
	let names: Vec<String> = stmt.column_names().into_iter().map(|s| s.to_string()).collect();
	let sub_types: Vec<SubType> = names
		.iter()
		.map(|n| {
			if n.ends_with("time") {
				SubType::Time
			} else {
				SubType::None
			}
		})
		.collect();

	let mut table_builder = tabled::builder::Builder::new();
	table_builder.push_record(names.clone());

	let mut count = 0;

	while let Some(row) = rows.next()? {
		count += 1;
		if count > options.rows_limit {
			break;
		}
		// -- Extract row cells
		let mut cells: Vec<String> = Vec::new();
		for (i, _k) in names.iter().enumerate() {
			let v = row.get_ref(i)?;
			let v = match v {
				ValueRef::Null => "NULL".to_string(),
				ValueRef::Integer(num) => match sub_types[i] {
					SubType::Time => format!("{num}"), //epoch_us_to_rfc3339(num),
					SubType::None => format!("{num}"),
				},
				ValueRef::Real(num) => format!("{num}"),
				ValueRef::Text(bytes) => {
					let txt = format!("\"{}\"", from_utf8(bytes).map_err(|_| Error::SQLiteTextCellIsNotUtf8)?);
					truncate_string(txt, options)
				}
				ValueRef::Blob(blob) => format!("BLOB (length: {})", blob.len()),
			};

			cells.push(v);
		}

		// -- Add the row celles to the table builder
		table_builder.push_record(cells);
	}

	let table_content = table_builder.build().with(Style::modern()).to_string();

	Ok(table_content)
}

fn truncate_string(s: String, options: &PrettyOptions) -> String {
	let Some(truncate_cell_max) = options.cell_truncate_max else {
		return s;
	};

	let max_cell_len = truncate_cell_max as usize;

	let char_indices: Vec<(usize, char)> = s.char_indices().collect();
	if char_indices.len() > max_cell_len {
		let first_part_len = max_cell_len / 6; // TODO: Need to use the Truncate data
		let remaining_len = max_cell_len - first_part_len;
		let first_part = char_indices[..first_part_len].iter().map(|&(_, c)| c).collect::<String>();
		let second_part = char_indices[char_indices.len() - remaining_len..]
			.iter()
			.map(|&(_, c)| c)
			.collect::<String>();
		format!("{}...{}", first_part, second_part)
	} else {
		s.to_string()
	}
}
