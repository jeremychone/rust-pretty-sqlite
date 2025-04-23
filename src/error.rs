use derive_more::{Display, From};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From, Display)]
#[display("{self:?}")]
pub enum Error {
	CantPrintRowsHasNoStatement,
	SQLiteTextCellIsNotUtf8,

	// -- Externals
	#[from]
	Rusqlite(rusqlite::Error), // as example
}

// region:    --- Error Boilerplate

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
