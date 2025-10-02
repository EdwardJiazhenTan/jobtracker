# Job Tracker

A terminal-based application for tracking job applications.

## Demo

https://github.com/user-attachments/assets/demo.mp4

## Features

- Record and manage job application information
- View applications in a table format
- Add, edit, and delete records
- Generate statistical charts by resume version, platform, and status
- Data persistence in JSON format

## Installation

Requires Rust 1.70 or later.

```bash
cargo build --release
```

## Usage

```bash
cargo run
```

Data is automatically saved to `applications.json` in the current directory.

## Data Fields

Each application record contains:

- Company name
- Application platform (LinkedIn, Indeed, company website, or custom)
- Resume modified (yes/no)
- Resume version
- Application status (Applied, Interview, Offer, Rejected)
- Application date
- Notes

## Keyboard Controls

### List View

- `↑/↓` or `j/k`: Navigate records
- `a`: Add new record
- `e`: Edit selected record
- `d`: Delete selected record
- `g`: View charts
- `q`: Quit

### Form View

- `↑/↓`: Navigate between fields
- `j/k`: Select options in dropdown menus
- `Enter`: Move to next field (saves on last field)
- `Esc`: Cancel

### Chart View

- `Tab`: Switch chart type
- `Esc`: Return to list view

## Technology

- **ratatui**: Terminal UI framework
- **crossterm**: Terminal input handling
- **serde/serde_json**: Data serialization
- **chrono**: Date/time handling
- **anyhow**: Error handling

## License

MIT
