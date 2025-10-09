# extscan

extscan detects and reports mismatches between file extensions and file contents.

## Note

When using libmagic as the file detection engine, false positives may occur, particularly with text files, due to its dependence on libmagic’s database. Magika provides similar functionality but offers better accuracy at the expense of increased scan time.

## Install

### Linux (Debian)

#### libmagic

```sh
sudo apt install libmagic-dev
```

#### Magika

```sh
pip install magika
```

If you plan to use Magika as the detection engine, you will also need to install [ONNX Runtime](https://github.com/microsoft/onnxruntime/).

1. Download the appropriate ONNX Runtime release for your system from the [official repository](https://github.com/microsoft/onnxruntime/releases).

2. Extract the archive, for example:

   ```sh
   tar xzf onnxruntime-linux-x64-*.tgz
   ```

3. Set the `ORT_LIB_LOCATION` environment variable to the path of the extracted `lib` directory. For example:

   ```sh
   export ORT_LIB_LOCATION=/path/to/onnxruntime-linux-x64/lib
   ```

    You can add this line to your shell configuration file (e.g., `~/.bashrc` or `~/.zshrc`) to make it persistent.

## Usage

```text
$ cargo run -- --help
Usage: extscan [OPTIONS] <PATH>

Arguments:
  <PATH>  Input path

Options:
  -e, --engine <ENGINE>          File type detection engine to use [default: libmagic] [possible values: libmagic, magika]
      --magic-file <MAGIC_FILE>  Use the specified magic file for file type detection
  -r, --recursive                Check files and directories recursively
  -h, --help                     Print help
  -V, --version                  Print version
```

### Example

```sh
cargo run tests/data -r -e magika
```
