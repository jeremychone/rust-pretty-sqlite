# `pretty-sqlite` - Simple, Minimalistic Pretty Printing for SQLite

This library helps with testing and exploratory development of SQLite by conveniently printing the contents of a table or a select statement into a nicely formatted table using the [tabled](https://crates.io/crates/tabled) crate.

This library is based on the [rusqlite](https://crates.io/crates/rusqlite) crate.

IMPORTANT: By default, all queries/prints are limited to `300` records. Using the `pretty_...` functions with a [PrettyOptions](src/options.rs) argument can change this default behavior.

The API convention is as follows:

- Functions prefixed with `pretty_...`, like `pretty_table` and `pretty_select`, return a formatted string of the table or query content.
- Functions prefixed with `print_...`, like `print_table` and `print_select`, call `println!()` on the corresponding `pretty_...` function.

There is also a more advanced function:

- `pretty_select_with_options(conn, sql, params, pretty_options)` allows customization of the table output with `PrettyOptions`.

## Example: 

```rust
let conn = Connection::open_in_memory()?; // For a file: Connection::open(path)?
// ... seed db

// -- Print table
pretty_sqlite::print_table(&conn, "person")?;

// Same as:
// let content = pretty_sqlite::pretty_table(&conn, "person")?;
// println!("{content}");
```

This will print something like: 

```
 TABLE: person
┌────┬────────────┬──────┬──────────┬───────────────────┐
│ id │ name       │ yob  │ data_t   │ data_b            │
├────┼────────────┼──────┼──────────┼───────────────────┤
│ 1  │ "Person 1" │ 1951 │ "Data 1" │ BLOB (length: 10) │
├────┼────────────┼──────┼──────────┼───────────────────┤
│ 2  │ "Person 2" │ 1952 │ "Data 2" │ BLOB (length: 10) │
├────┼────────────┼──────┼──────────┼───────────────────┤
│ 3  │ "Person 3" │ 1953 │ "Data 3" │ BLOB (length: 10) │
├────┼────────────┼──────┼──────────┼───────────────────┤
│ 4  │ "Person 4" │ 1954 │ "Data 4" │ BLOB (length: 10) │
├────┼────────────┼──────┼──────────┼───────────────────┤
│ 5  │ "Person 5" │ 1955 │ "Data 5" │ BLOB (length: 10) │
└────┴────────────┴──────┴──────────┴───────────────────┘
```

```rust
let conn = Connection::open_in_memory()?; // For a file: Connection::open(path)?
// ... seed db

pretty_sqlite::print_select(&conn, "select id, name, yob from person where id > ?", (2,))?;

// Same as:
// let content = pretty_sqlite::pretty_select(&conn, "select id, name, yob from person where id > ?", (2,))?;
// println!("{content}");
```

```
┌────┬────────────┬──────┐
│ id │ name       │ yob  │
├────┼────────────┼──────┤
│ 3  │ "Person 3" │ 1953 │
├────┼────────────┼──────┤
│ 4  │ "Person 4" │ 1954 │
├────┼────────────┼──────┤
│ 5  │ "Person 5" │ 1955 │
└────┴────────────┴──────┘
```

See [examples/readme.rs](examples/readme.rs)