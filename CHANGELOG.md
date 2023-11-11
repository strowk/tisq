# Changelog

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->

## [Unreleased]

### Added

- Status line to show loading while communicating with database.
- Custom snippets support.

## [0.1.12] - 2023-11-02

### Added

- Command `server add` to add server from command line.
- If no snippet is found in line, then show list of snippets to choose from.

### Changed

- Esc now is bound to `GlobalCancel` instead of `GlobalExit`.

## [0.1.11] - 2023-10-31

### Added

- Builds for Linux arm64 and for musl.

## [0.1.10] - 2023-10-31

### Added

- Snippets expansion for standard SQL and some Postgres queries.
- Toggle line comment with Ctrl+/.

## [0.1.9] - 2023-10-29

### Added

- List schemas in database tree view.
- List tables in database tree view.

### Fixed

- Auto reconnect to database if connection is missing after opening restored editor tab.

## [0.1.8] - 2023-10-26

### Added

- Close editor tab with Ctrl+W.
- Save and restore editors tabs content.

## [0.1.7] - 2023-10-21

### Added

- Customize keybindings in configuration file.

### Changed

- Use home directory `~/.tisq` for storage instead of current.

### Fixed

- Fixed navigating 'left' from query results.

## [0.1.6] - 2023-10-14

### Added

- Cycle navigation shortcut Ctrl+n.
- Support arrays.

### Fixed

- Process capital letters in query editor.

## [0.1.5] - 2023-10-11

### Added

- Shortcuts to switch between editor and query result.
- Query result table can be navigated with arrow keys - up/down move selected line and scroll if needed.
- Query result columns can be scrolled by Ctrl+Left, Ctrl+Right.
- Query result rows can be scrolled with PageUp, PageDown page-by-page.

### Fixed

- Fixed failing on nulls in query result.

## [0.1.4] - 2023-10-11

### Added

- Supported more postgres types, such as bool, char, smallint, bigint, float4, float8, bytea and similar to them.

## [0.1.3] - 2023-10-08

### Added

- New `ctrl+r` shortcut to execute (to avoid clashing with VS Code).
- Title for execute error panel.
- Only log errors to tisq-errors.log file.
- Log everything in debug.log if `--debug` flag is passed.
- Add Ctrl+Alt+Left and Ctrl+Alt+Right shortcuts to switch between panels (to avoid clashing with VS Code).

## [0.1.2] - 2023-10-08

### Added

- New `ctrl+e` shortcut to execute.

## [0.1.1] - 2023-10-08

### Added

- `--version` flag to print version.

## [0.1.0] - 2023-10-08

### Added

- Initial release.
- Supported limited set of Postgres types.
- Tree view to browse servers and databases.
- Adding and deleting servers with 'a' and 'delete' keys.
- Launching query editor by 'q' key.
- Query editor to write queries and execute them by ctrl+alt+enter.
- Execution results view supporting only successful fetched results.

<!-- next-url -->
[Unreleased]: https://github.com/strowk/tisq/compare/v0.1.12...HEAD
[0.1.12]: https://github.com/strowk/tisq/compare/v0.1.11...v0.1.12
[0.1.11]: https://github.com/strowk/tisq/compare/v0.1.10...v0.1.11
[0.1.10]: https://github.com/strowk/tisq/compare/v0.1.9...v0.1.10
[0.1.9]: https://github.com/strowk/tisq/compare/v0.1.8...v0.1.9
[0.1.8]: https://github.com/strowk/tisq/compare/v0.1.7...v0.1.8
[0.1.7]: https://github.com/strowk/tisq/compare/v0.1.6...v0.1.7
[0.1.6]: https://github.com/strowk/tisq/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/strowk/tisq/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/strowk/tisq/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/strowk/tisq/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/strowk/tisq/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/strowk/tisq/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/strowk/tisq/releases/tag/v0.1.0
