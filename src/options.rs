pub struct PrettyOptions {
	pub rows_limit: usize,

	pub cell_truncate_max: Option<u32>,
	// TODO: Implement cell_truncate_pos
	// pub cell_truncate_pos: TruncatePosition,
}

// TODO: to be implemented
// pub enum TruncatePosition {
// 	Start(u32),
// 	Middle,
// 	End(u32),
// }

// implement default
impl Default for PrettyOptions {
	fn default() -> Self {
		Self {
			rows_limit: 300,
			cell_truncate_max: None,
			// TODO: to be implemented
			// cell_truncate_pos: TruncatePosition::End(0),
		}
	}
}
