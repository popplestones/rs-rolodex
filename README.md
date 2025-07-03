# Rolodex 🗂️

A fast and minimal terminal-based contact manager built in Rust using the Elm architecture, Crossterm, and Ratatui.

## Features

- 🧭 Interactive TUI with real-time filtering by name or company
- ⌨️ Keyboard navigation (↑/↓, Home/End)
- 🔍 Search-as-you-type filtering
- 🧵 JSON output for scripting (pipe into `jq` or CLI dialers)
- 📁 Contact data loaded from `contacts.json`

## Usage

```bash
./rolodex | jq -r '.phone'
```

Use arrow keys to select a contact. Press Enter to emit the contact as JSON, which you can then pipe into another tool.

## Keyboard shortcuts
| **Key** | Action |
|---|---|
| `↑  / ↓` | Move selection |
| `Home / End` | Jump to start/end |
| `Ctrl + Q` | Quit |
| `Esc` | Clear search |
| `Enter` | Output selected contact |


## JSON Format

```json
{
  "name": "Alice Anderson",
  "company": "Acme Co.",
  "email": "alice@acme.co",
  "phone": "0412 345 678"
}
```

## Installation

```bash
git clone https://github.com/popplestones/rolodex.git
cd rolodex
cargo build --release
```

Then run it:

```bash
./target/release/rolodex
```

## Development Notes

For planned features like Add/Edit/Delete modes, file persistence, XDG path support, and CLI improvements, see [TODO.md](TODO.md). Contributions and ideas welcome!

## License

MIT
