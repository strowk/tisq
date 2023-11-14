# TisQ - terminal UI for SQL databases

TisQ [tɪz-kjuː] stands for **T**erminal **I**nterface for **S**QL **Q**ueries.

!["gif recording"](./vhs/base.gif)

Features:

- Browse servers, databases, schemas and tables in tree view.
- Write and execute SQL queries.
- View query execution results in table.
- Switch between multiple query tabs with different connections.
- Save query editors content on exit and restore on start.
- Customizable keybindings by config TOML file.
- Customizable snippets expansion with a set of predefined queries.

## Status

In active development. Not ready for production use, there are no tests, incomplete error handling, program can crash on some inputs.

### Database support

Only Postgres is supported at the moment and with limited set of types. If type is not yet supported, value would be displayed as `not supported` in a table.

## Installation

### With script

In bash shell run:

```bash
curl -s https://raw.githubusercontent.com/strowk/tisq/main/install.sh | bash
```

Tested on Linux bash and Windows Git Bash. Should work for MacOS too.

#### Disabling sudo

By default the script would try to install TisQ to `/usr/local/bin` and would require sudo rights for that,
but you can disable this behavior by setting `NO_SUDO` environment variable:

```bash
curl -s https://raw.githubusercontent.com/strowk/tisq/main/install.sh | NO_SUDO=1 bash
```

Sudo is disabled by default for Windows Git Bash.

### From sources

If your system/architecture is not supported by the script above,
you can install Rust and install TisQ from sources:

```bash
git clone https://github.com/strowk/tisq
cargo install --path ./tisq
```

### Using Docker

You can run TisQ in Docker container:

```bash
docker run -it --rm ghcr.io/strowk/tisq:main-debian
# or
docker run -it --rm ghcr.io/strowk/tisq:main-alpine
```

Note that the version in `main-*` tags would be from the latest commit in `main` branch.
Builds are provided based on Debian and Alpine Linux with x86_64 and aarch64 architectures.

Under certain conditions commands above might fail like this:

> thread 'main' panicked at 'Cannot initialize terminal: CannotConnectStdout'.

This seems to be an issue in docker, which does not prepare tty properly before starting the process.
To bypass this issue you can add a small delay like this:

```bash
docker run -ti --rm --entrypoint sh ghcr.io/strowk/tisq:main-alpine -c 'sleep 1; exec tisq'
```

This issue was only noticed with Alpine so far.

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

For sptecifying several modifiers, you can list them separated by `+`, like this: `modifiers = "Ctrl+Alt"`.

### Global

Config section: `globals`.

| Default Keybindings        | Description    | Config name           |
| -------------------------- | -------------- | --------------------- |
| Ctrl+c                     | Quit           | `GlobalExit`          |
| Esc                        | Cancel or quit | `GlobalCancel`        |
| Alt+Left / Ctrl+Alt+Left   | Navigate Left  | `GlobalNavigateLeft`  |
| Alt+Right / Ctrl+Alt+Right | Navigate Right | `GlobalNavigateRight` |
| Alt+Up / Ctrl+Alt+Up       | Navigate Up    | `GlobalNavigateUp`    |
| Alt+Down / Ctrl+Alt+Down   | Navigate Down  | `GlobalNavigateDown`  |

`GlobalCancel` is used to cancel some of operations, such as applying snippet.
In case if no such operation is in progress, it would serve as a quit command.

### Browser (tree view)

Config section: `browser`.

| Default Keybindings | Description                             | Config name                      |
| ------------------- | --------------------------------------- | -------------------------------- |
| a                   | Add new server                          | `BrowserAddServer`               |
| Delete              | Delete server                           | -                                |
| q                   | Open query editor for selected database | `BrowserDatabaseOpenQueryEditor` |
| Up, Down            | Navigate                                | -                                |
| Right, Left         | Open,close node                         | -                                |

### Query editor

Config section: `editor`.

| Default Keybindings              | Description               | Config name           |
| -------------------------------- | ------------------------- | --------------------- |
| Ctrl+PageUp                      | Previous query editor tab | `EditorPrevTab`       |
| Ctrl+PageDown                    | Next query editor tab     | `EditorNextTab`       |
| Ctrl+Alt+Enter / Ctrl+E / Ctrl+R | Execute query             | `EditorExecute`       |
| Ctrl+W                           | Close editor tab          | `EditorCloseTab`      |
| Ctrl+Space                       | Attempt to expand snippet | `EditorTryExpand`     |
| Ctrl+/                           | Comment or uncomment line | `EditorToggleComment` |

### Query result

Config section: `result`.

| Default Keybindings | Description                | Config name               |
| ------------------- | -------------------------- | ------------------------- |
| Left                | Scroll columns to left     | `ResultOffsetColumnLeft`  |
| Right               | Scroll columns to right    | `ResultOffsetColumnRight` |
| Up, Down            | Move selected line pointer | -                         |
| PageUp, PageDown    | Move by page               | -                         |

## Snippets

Snippets are small shortcuts that can be expanded into SQL code.

You can enter snippet shortcut and press `Ctrl+Space` to attempt to expand it.
In case if no snippets matched, you will see a table with available snippets to choose from.
Then you can use `Enter` key to aplly selected snippet or use `GlobalCancel` (defaults to `Esc`) to cancel selection of snippet.

### Standard Postgres snippets

| Shortcut | Expansion        |
| -------- | ---------------- |
| `cq`     | Current queries  |
| `ds`     | Databases sizes  |
| `ts`     | Tables sizes     |
| `cl`     | Current locks    |
| `sel`    | `SELECT * FROM`  |
| `ins`    | `INSERT INTO`    |
| `upd`    | `UPDATE`         |
| `del`    | `DELETE FROM`    |
| `cre`    | `CREATE TABLE`   |
| `alt`    | `ALTER TABLE`    |
| `dro`    | `DROP TABLE`     |
| `trun`   | `TRUNCATE TABLE` |

### Custom snippets

You can add your own snippets to `~/.tisq/config.toml` file like this:

```toml
[[snippets.Postgres]]
shortcut = "kc"
description = "kill connection"
query = """
SELECT pg_terminate_backend(pid) 
FROM pg_stat_activity 
WHERE pid =
"""
```

This would add a snippet with shortcut `kc` that would be expanded into the query that kills connection by its process id.

If you add a snippet with shortcut that already exists, it would override the existing one.

## Subcommands

### `tisq --version`

Show version of TisQ and exit.

### `tisq --help`

Show help message and exit.

### `tisq server add`

Add new server to the list of servers in storage under `~/.tisq`.

```bash
tisq server add [name] [connection-url]
```

## Roadmap

- [x] Customizable keybindings by config TOML file
- [x] Save query editors content on exit and restore on start
- [x] Add schemas and tables to tree view
- [x] Add standard postgres snippets expansion
- [x] Display available snippets to choose from
- [x] status line: Show loading while executing query
- [x] Allow to add custom snippets
- [x] Show table columns in tree view
- [ ] Optimize for queries with big results by paging
- [ ] Add other objects to tree view (views, functions, etc)
- [ ] Add support for more Postgres types (from https://docs.rs/sqlx-postgres/0.7.2/sqlx_postgres/types/index.html )
- [ ] Error handling: remove all unwrap() calls and anything else that can panic
- [ ] Add support for query history
- [ ] Limit query result size by amount of rows
- [ ] Better limit of query result by memory size (use https://docs.rs/datasize/latest/datasize/ )
- [ ] Syntax highlighting for query editor
- [ ] databases: Add support for MySQL
- [ ] databases: Add support for SQLite
- [ ] Add support for query parameters
- [ ] Add support for query execution plan
- [ ] databases: Add support for MS SQL Server (via https://github.com/prisma/tiberius )
- [ ] themes: Customizeable style by config TOML file
- [ ] Customize keybindings in UI
-
