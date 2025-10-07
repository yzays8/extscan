# extscan

extscan detects and reports mismatches between file extensions and file contents.

## Note

File detection relies on libmagic's database and automated workflows derived from it, and false positives may occur for certain files.

## Install

### Linux (Debian)

```sh
sudo apt install libmagic-dev
```

## Usage

```text
$ cargo run -- --help
Usage: extscan [OPTIONS] <PATH>

Arguments:
  <PATH>  Input path

Options:
      --magic-file <MAGIC_FILE>  Use the specified magic file for file type detection
  -r, --recursive                Check files and directories recursively
      --no-summary               Suppress summary output after checking
  -h, --help                     Print help
  -V, --version                  Print version
```

### Example

```sh
cargo run tests/data -r
```
