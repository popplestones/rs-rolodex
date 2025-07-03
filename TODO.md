# Rolodex TODO

## Modes
- [ ] **Add mode**
  - Display input form for name, company, phone
  - Validate required fields (e.g. name, phone)
  - Append new contact to list
  - Persist to file

- [ ] **Edit mode**
  - Enter edit mode for selected contact
  - Show pre-filled fields
  - Save updated contact
  - Persist to file

- [ ] **Delete mode**
  - Confirm deletion
  - Remove selected contact
  - Persist to file

## File Persistence
- [ ] Write back changes (add/edit/delete) to disk
- [ ] Format file as readable JSON

## Configuration Path
- [ ] Store `contacts.json` at XDG-compliant location:
  - Prefer `$XDG_CONFIG_HOME/rolodex/contacts.json`
  - Fallback to `$HOME/.config/rolodex/contacts.json`
  - Create path if missing

## Command-Line UX
- [ ] Add `--file` flag to override contacts path
- [ ] Add `--version` / `--help`

## Extra
- [ ] Export full contact list as JSON
- [ ] Add support for multi-select mode
- [ ] Option to show phone-only output

