# getimgs

getimgs is a command-line tool for bulk downloading images from a link list file.

## Usage

```sh
getimgs [FILE] [FLAGS]
```

## Arguments

- `FILE`: File with all the links of the images (one link per line).
- `--output` (optional): Directory to save all the images. Defaults to "images".
- `--threads` (optional): Number of threads to use for downloading. Defaults to 50.
- `--verbose` (optional): Enable verbose output. Defaults to false.

## How to Build

To build the `getimgs` tool, make sure you have the latest stable version of Rust installed and [just](https://github.com/casey/just). Then, run the following command:

```sh
just build
```

## How to Use

To use the `getimgs` tool, run the following command:

```sh
getimgs [FILE]
```

Replace `[FILE]` with the path to the file containing the links of the images you want to download.

## License

This project is licensed under the GNU GPL v3.0 License. See the [LICENSE](LICENSE) file for details.

