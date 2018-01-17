# waitforfile
Simple rust application to wait until a given file exists. By utilizing [inotify](http://man7.org/linux/man-pages/man7/inotify.7.html), `waitforfile` does not use polling.

## Usage
```
waitforfile <filename>
```

where `filename` is a relative or absolute path. If `filename` is an absolute path (e.g. `/foo/bar/baz`), the directory portion (e.g. `/foo/bar`) must already exist prior to running the command.
`waitforfile` will return with exit code 0 if the file already exists or has been created during
execution. If the directory is deleted while `waitforfile` is running, it will return with exit code
1.

## Installation

Arch Linux:
A package is available on the [AUR](https://aur.archlinux.org/packages/waitforfile/).

Void Linux:
```
xbps-install -S waitforfile
```

For all other distributions, you may either download a [binary release](https://github.com/nroi/waitforfile/releases), or build from source. To install from source, you need to have cargo installed. Then, clone this repository and run:
```
cargo build --release
```
the binary is then available at `target/release/waitforfile`.
