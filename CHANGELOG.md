# Change Log

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [v0.1.8] - 2023-10-26

### Added

- Close editor tab with Ctrl+W.
- Save and restore editors tabs content.

## [v0.1.7] - 2023-10-21

### Added

- Customize keybindings in configuration file.

### Changed

- Use home directory `~/.tisq` for storage instead of current.

### Fixed

- Fixed navigating 'left' from query results.

## [v0.1.6] - 2023-10-14

### Added

- Cycle navigation shortcut Ctrl+n.
- Support arrays.

### Fixed

- Process capital letters in query editor.

## [v0.1.5] - 2023-10-11

### Added

- Shortcuts to switch between editor and query result.
- Query result table can be navigated with arrow keys - up/down move selected line and scroll if needed.
- Query result columns can be scrolled by Ctrl+Left, Ctrl+Right.
- Query result rows can be scrolled with PageUp, PageDown page-by-page.

### Fixed

- Fixed failing on nulls in query result.

## [v0.1.4] - 2023-10-11

### Added

- Supported more postgres types, such as bool, char, smallint, bigint, float4, float8, bytea and similar to them.

## [v0.1.3] - 2023-10-08

### Added

- New `ctrl+r` shortcut to execute (to avoid clashing with VS Code).
- Title for execute error panel.
- Only log errors to tisq-errors.log file.
- Log everything in debug.log if `--debug` flag is passed.
- Add Ctrl+Alt+Left and Ctrl+Alt+Right shortcuts to switch between panels (to avoid clashing with VS Code).

## [v0.1.2] - 2023-10-08

### Added

- New `ctrl+e` shortcut to execute.

## [v0.1.1] - 2023-10-08

### Added

- `--version` flag to print version.

## [v0.1.0] - 2023-10-08

### Added

- Initial release.
- Supported limited set of Postgres types.
- Tree view to browse servers and databases.
- Adding and deleting servers with 'a' and 'delete' keys.
- Launching query editor by 'q' key.
- Query editor to write queries and execute them by ctrl+alt+enter.
- Execution results view supporting only successful fetched results.

<!-- next-url -->
[Unreleased]: https://github.com/strowk/tisq/compare/v0.1.8...HEAD
[v0.1.8]: https://github.com/strowk/tisq/compare/v0.1.7...v0.1.8
[v0.1.7]: https://github.com/strowk/tisq/compare/v0.1.6...v0.1.7
[v0.1.6]: https://github.com/strowk/tisq/compare/v0.1.5...v0.1.6
[v0.1.5]: https://github.com/strowk/tisq/compare/v0.1.4...v0.1.5
[v0.1.4]: https://github.com/strowk/tisq/compare/v0.1.3...v0.1.4
[v0.1.3]: https://github.com/strowk/tisq/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/strowk/tisq/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/strowk/tisq/compare/v0.1.0...v0.1.1
[v0.1.0]: https://github.com/strowk/tisq/releases/tag/v0.1.0
