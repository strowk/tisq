# TisQ - terminal UI for SQL databases

TisQ [tɪz-kjuː] stands for **T**erminal **I**nterface for **S**QL **Q**ueries.

Features:
- Browse servers and databases in tree view.
- Write and execute SQL queries.
- View query execution results in table.
- Switch between multiple query tabs with different connections.

## Installation

Only installation from sources is supported at the moment.

```bash
git clone https://github.com/strowk/tisq
cargo install --path ./tisq
```

## Database support

Only Postgres is supported at the moment and with very limited set of types.

## Keybindings

### Global

Escape - quit
Alt+Left, Alt+Right - navigation

### Tree view

Up, Down - navigate
Right - open node
Left - close node
a - add server
delete - delete server

### Query editor

Ctrl+Alt+Enter - execute query
Cntrl+PageUp, Cntrl+PageDown - switch between tabs

