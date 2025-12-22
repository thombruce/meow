# Catfood =^,^=

A modular utility suite for system management, built with Rust and Ratatui.

## Usage

### System Bar

Run the system bar:

Top of screen:
```sh
kitten panel catfood bar
```

Bottom of screen:
```sh
kitten panel --edge=bottom catfood bar
```

### Other Commands

```sh
catfood bar           # Run the system bar
catfood menu          # Run the menu system (coming soon)
catfood notifications # Run notification system (coming soon)
```

## Installation

### From Source

```sh
# Build and install all utilities
cargo install --path .

# Or build and run locally
cargo run -- bar
```

## Configuration

catfood supports live configuration via `~/.config/catfood/bar.json`. The first time you run the application, a default configuration file will be created.

### Hot-Reload

Configuration changes are automatically detected and applied without restarting the application. Simply edit your `bar.json` file and the bar layout will update in real-time!

### Error Logging

All errors are logged to `~/.local/share/catfood/logs/bar.log` in ISO format:
```
2025-12-21T03:45:12Z [ERROR] [COMPONENT_WORKSPACES] Error: Failed to get workspaces
2025-12-21T03:45:13Z [ERROR] [CONFIG] Failed to reload configuration: ...
2025-12-21T03:45:14Z [ERROR] [FILE_WATCHER] Failed to create file watcher: ...
```

The log file keeps only the last 1000 lines.

See [CONFIG_EXAMPLE.md](CONFIG_EXAMPLE.md) for detailed configuration options and examples.

## License

Copyright (c) Thom Bruce <thom@thombruce.com>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
