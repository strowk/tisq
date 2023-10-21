# TisQ - terminal UI for SQL databases

TisQ [tɪz-kjuː] stands for **T**erminal **I**nterface for **S**QL **Q**ueries.

!["gif recording"](./vhs/base.gif)

Features:
- Browse servers and databases in tree view.
- Write and execute SQL queries.
- View query execution results in table.
- Switch between multiple query tabs with different connections.

## Status

In active development. Not ready for production use, there are no tests, incomplete error handling, program can crash on some inputs.

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

Some of following keybindings are configurable and could be adjusted in configuration 
file that is located under home folder in `.tisq/config.toml`.

The format for this file could be illustrated by the following example:

```toml
# Firstly specify the section for keybindings
[keybindings.globals] # word "globals" here is a name of the keybinding config section
# Then use keybinding name as a key and list of possible key presses as a value
GlobalExit = [
    # This would allow to use Esc key without any modifiers
    { modifiers = "", key = { type = "Esc" } },
    # Or, alternatively use Ctrl+C combination, both would work
    { modifiers = "Ctrl", key = { type = "Char", args = "c" } },
]
```

In this example it is demonstrated how to specify special keys, such as Esc,
using simple `type = "Esc"` and how characters are specified using `type = "Char"` with `args` field that contains the character.

### Global

Config section: `globals`.

| Default Keybindings        | Description    | Config name           | 
| ---                        | ---            | ---                   |
| Ctrl+c, Esc                | Quit           | `GlobalExit`          |
| Alt+Left / Ctrl+Alt+Left   | Navigate Left  | `GlobalNavigateLeft`  |
| Alt+Right / Ctrl+Alt+Right | Navigate Right | `GlobalNavigateRight` |
| Alt+Up / Ctrl+Alt+Up       | Navigate Up    | `GlobalNavigateUp`    |
| Alt+Down / Ctrl+Alt+Down   | Navigate Down  | `GlobalNavigateDown`  |

### Browser (tree view)

Config section: `browser`.

| Default Keybindings  | Description                              | Config name                       | 
| ---                  | ---                                      | ---                               |
| a                    | Add new server                           | `BrowserAddServer`                |
| Delete               | Delete server                            | -                                 |
| q                    | Open query editor for selected database  | `BrowserDatabaseOpenQueryEditor`  |
| Up, Down             | Navigate                                 | -                                 |
| Right, Left          | Open,close node                          | -                                 |

### Query editor

Config section: `editor`.

| Default Keybindings                | Description                  | Config name        | 
| ---                                | ---                          | ---                |
| Ctrl+PageUp                        | Previous query editor tab    | `EditorPrevTab`    |
| Ctrl+PageDown                      | Next query editor tab        | `EditorNextTab`    |
| Ctrl+Alt+Enter / Ctrl+E / Ctrl+R   | Execute query                | `EditorExecute`    |

### Query result

Config section: `result`.

| Default Keybindings    | Description                 | Config name               | 
| ---                    | ---                         | ---                       |
| Ctrl+Left              | Scroll columns to left      | `ResultOffsetColumnLeft`  |
| Ctrl+Right             | Scroll columns to right     | `ResultOffsetColumnRight` |
| Up, Down               | Move selected line pointer  | -                         |
| PageUp, PageDown       | Move by page                | -                         |

## Roadmap

- [x] Customizable keybindings by config TOML file
- [ ] Save query editors content on exit and restore on start
- [ ] Add support for more Postgres types (from https://docs.rs/sqlx-postgres/0.7.2/sqlx_postgres/types/index.html )
- [ ] Error handling: remove all unwrap() calls and anything else that can panic
- [ ] Add schemas to tree view
- [ ] Add other objects to tree view (tables, views, functions, etc)
- [ ] Show table columns in tree view
- [ ] Customizable keybindings with UI
- [ ] themes: Customizeable style by config TOML file
- [ ] Add support for query snippets
- [ ] Add support for query parameters
- [ ] Add support for query history
- [ ] Add support for query execution plan
- [ ] status line: Show loading while executing query
- [ ] Support queries with big results by paging
- [ ] Limit query result size by amount of rows
- [ ] Better limit of query result by memory size (use https://docs.rs/datasize/latest/datasize/ )
- [ ] Syntax highlighting for query editor
- [ ] databases: Add support for MySQL
- [ ] databses: Add support for SQLite
- [ ] databases: Add support for MS SQL Server (via https://github.com/prisma/tiberius )
- 

