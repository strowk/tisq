# TisQ - terminal UI for SQL databases

TisQ [tɪz-kjuː] stands for **T**erminal **I**nterface for **S**QL **Q**ueries.

!["gif recording"](./vhs/base.gif)

Features:
- Browse servers and databases in tree view.
- Write and execute SQL queries.
- View query execution results in table.
- Switch between multiple query tabs with different connections.

## Installation

### With script

In bash shell run:

```bash
curl -s https://raw.githubusercontent.com/strowk/tisq/main/install.sh | bash
```

Tested on Linux bash and Windows Git Bash.

### From sources

```bash
git clone https://github.com/strowk/tisq
cargo install --path ./tisq
```

## Database support

Only Postgres is supported at the moment and with very limited set of types.

## Keybindings

### Global

- Escape - quit
- Alt+Left, Alt+Right / Ctrl+Alt+Left, Ctrl+Alt+Right - navigation

### Tree view

- Up, Down - navigate
- Right - open node
- Left - close node
- a - add server
- delete - delete server

### Query editor

- Ctrl+Alt+Enter / Ctrl+E / Ctrl+R - execute query
- Ctrl+PageUp, Ctrl+PageDown - switch between tabs

