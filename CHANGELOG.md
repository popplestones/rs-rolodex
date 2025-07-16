# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Status bar component with context-sensitive keyboard shortcuts and contact counts
- Backup and restore functionality via CLI options (`--backup` and `--restore`)
- Comprehensive database operations including search, duplicate detection, and JSON import/export
- Enhanced contact validation and error handling
- Test suite for database operations

### Changed
- Improved application layout with dedicated status bar area
- Enhanced App component to follow Elm architecture more consistently
- Updated AppMode enum to support error messages
- Refactored contact management with better state tracking

### Technical Details
- Added `StatusBar` component following Elm architecture pattern
- Implemented `StatusBarMsg` enum for state updates
- Enhanced database layer with search, backup/restore, and duplicate detection capabilities
- Added comprehensive test coverage for database operations
- Improved error handling and validation throughout the application

## [0.1.0] - Initial Release

### Added
- Basic contact management TUI application
- SQLite database backend
- Contact browsing with search functionality
- Add, edit, and delete contact operations
- JSON output for selected contacts
- Keyboard navigation and shortcuts