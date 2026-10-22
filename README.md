# down

A simple CLI file downloader built on top of `trauma`.

## Installation

```bash
cargo install --git https://github.com/bircoder432/down
```

## Usage

```bash
down <links...> [OPTIONS]
```

### Examples

Download a single file:

```bash
down https://example.com/file.zip
```

Download multiple files concurrently:

```bash
down -c 4 https://example.com/a.zip https://example.com/b.zip
```

Specify output directory and custom filenames:

```bash
down -o ./downloads -f first.zip,second.zip \
    https://example.com/a.zip https://example.com/b.zip
```

Download from a file (one URL per line, optional filename after a space):

```bash
down -F urls.txt
```

### CLI options

| Flag | Description |
|------|-------------|
| `<links...>` | List of URLs to download |
| `-c, --concurrents <N>` | Number of concurrent downloads |
| `-F, --downloads-file <FILE>` | Path to a file containing URLs |
| `-o, --output-dir <DIR>` | Output directory for downloaded files |
| `-f, --file-names <NAMES>` | Comma-separated custom filenames |

## Configuration

The configuration file is located at:

- Linux: `~/.config/down/config.toml`
- macOS: `~/Library/Application Support/down/config.toml`
- Windows: `%APPDATA%\down\config.toml`

CLI arguments take priority over configuration values.

### Example `config.toml`

```toml
[downloader]
concurrent_downloads = 3
retries = 3
resumable = true

[downloader.headers]
User-Agent = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36"
Accept = "*/*"

[downloader.progress_bar]
hidden = false
template = "{elapsed} [{bar:40.cyan}] {pos}/{len}"
progress_chars = "=>-"
clear_on_finish = false

[messages]
success = "Done: {url}"
error = "Failed: {url} -> {error}"
skipped = "Skipped: {url} ({reason})"
not_started = "Not started: {url}"
```

### Configuration reference

#### `[downloader]`

- `concurrent_downloads` - maximum number of parallel downloads (default: `1`)
- `retries` - number of retries per failed download (default: `3`)
- `resumable` - resume interrupted downloads when possible (default: `true`)

#### `[downloader.headers]`

Custom HTTP headers sent with every request. Useful for setting `User-Agent` to bypass server-side restrictions.

#### `[downloader.progress_bar]`

- `hidden` - hide progress bars entirely (default: `false`)
- `template` - `indicatif`-style template string. Supported placeholders: `{bar}`, `{pos}`, `{len}`, `{percent}`, `{elapsed}`, `{eta}`, `{bytes}`, `{total_bytes}`, `{bytes_per_sec}`, `{msg}`
- `progress_chars` - three characters for filled, current, and empty states of the bar
- `clear_on_finish` - remove the bar from the terminal after completion (default: `false`)

#### `[messages]`

Templates for status output. Available placeholders:

- `{url}` - the download URL
- `{error}` - error description (only in `error`)
- `{reason}` - skip reason (only in `skipped`)

## License

WTFPL. See `LICENSE` for details.
