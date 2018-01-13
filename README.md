# waitforfile
Simple rust application to wait until a given file exists. By utilizing [inotify](http://man7.org/linux/man-pages/man7/inotify.7.html), `waitforfile` does not use polling.

## Usage
```
waitforfile <filename>
```

where `filename` is a relative or absolute path. If `filename` is an absolute path (e.g. `/foo/bar/baz`), the directory portion (e.g. `/foo/bar`) must already exist prior to running the command.

## Installation

Binaries are available under [releases](https://github.com/nroi/waitforfile/releases). If you happen to use Arch Linux, you'll find a package on the [AUR](https://aur.archlinux.org/packages/waitforfile/). If you want to compile from source instead, you need to have cargo installed. Clone the repository and run:
```
cargo build --release
```
the binary is then available at `target/release/waitforfile`.
