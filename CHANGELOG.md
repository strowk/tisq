# Change Log

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->

## [Unreleased] - ReleaseDate

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

## [v0.1.0] - 2023-05-09

### Added

- Initial release.
- Supported limited set of Postgres types.
- Tree view to browse servers and databases.
- Adding and deleting servers with 'a' and 'delete' keys.
- Launching query editor by 'q' key.
- Query editor to write queries and execute them by ctrl+alt+enter.
- Execution results view supporting only successful fetched results.

<!-- next-url -->
[Unreleased]: https://github.com/strowk/tisq/compare/v0.1.2...HEAD
[v0.1.2]: https://github.com/strowk/tisq/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/strowk/tisq/compare/v0.1.0...v0.1.1
[v0.1.0]: https://github.com/strowk/tisq/releases/tag/v0.1.0
