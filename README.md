# TisQ - terminal UI for SQL databases

TisQ [tɪz-kjuː] stands for **T**erminal **I**nterface for **S**QL **Q**ueries.

!["gif recording"](./vhs/base.gif)

Features:
- Browse servers and databases in tree view.
- Write and execute SQL queries.
- View query execution results in table.
- Switch between multiple query tabs with different connections.

## Status

In active development. Not ready for production use, there are no tests, incomplete error handling, program can crash on some inputs that are not yet completely supported (like f.e array types).

### Database support

Only Postgres is supported at the moment and with very limited set of types.

## Installation

### With script

In bash shell run:

```bash
curl -s https://raw.githubusercontent.com/strowk/tisq/main/install.sh | bash
```

Tested on Linux bash and Windows Git Bash. Should work for MacOS too.

### From sources

If your system/architecture is not supported by the script above, 
you can install Rust and install TisQ from sources:

```bash
git clone https://github.com/strowk/tisq
cargo install --path ./tisq
```

## Keybindings

### Global

- Escape - quit
- Alt+Left, Alt+Right, Alt+Up, Alt+Down / Ctrl+Alt+Left, Ctrl+Alt+Right, Ctrl+Alt+Up, Ctrl+Alt+Down - navigation

### Tree view

- Up, Down - navigate
- Right - open node
- Left - close node
- a - add server
- delete - delete server

### Query editor

- Ctrl+Alt+Enter / Ctrl+E / Ctrl+R - execute query
- Ctrl+PageUp, Ctrl+PageDown - switch between tabs


### Query result

- Up, Down - move selected line pointer
- PageUp, PageDown - move by page
- Ctrl+Left, Ctrl+Right - scroll columns

## Roadmap

- [] Add support for more Postgres types (from https://docs.rs/sqlx-postgres/0.7.2/sqlx_postgres/types/index.html )
- [] Add support for MySQL
- [] Add support for SQLite
- [] Add support for MS SQL Server (via https://github.com/prisma/tiberius )
- [] Add schemas to tree view
- [] Add other objects to tree view (tables, views, functions, etc)
- [] Save query editors content on exit and restore on start
- [] Add support for query parameters
- [] Add support for query history
- [] Add support for query snippets
- [] Add support for query templates
- [] Add support for query execution plan
- [] Show loading while executing query
- [] Support queries with big results by paging
- [] Syntax highlighting for query editor
- 

