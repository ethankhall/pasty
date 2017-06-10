# hastebin-client
A CLI client for hastebin.com written in Rust.
Currently, it only supports uploading of files. You can optionally open newly opened files in a new browser window using `--open`.

## Installing

If you don't want to build from source, head over to the [releases](https://github.com/joek13/hastebin-client/releases/latest) page and grab the latest binary build. If there isn't one available for your platform, you'll have to build from source - instructions for that are below.

## Building from source

1. Make sure you have [Rust](https://www.rust-lang.org/en-US/) installed.
2. Clone the repo.
3. Enter the directory and run `cargo build --release` (the `--release` flag enables compile-time optimizations)
4. Enter the newly created `target/release` folder and copy the `hastebin-client` binary into `/usr/bin` or put it somewhere safe and add it to your PATH.
5. You're done! Optionally, add the following to your `vim` config file to create a custom command to upload the currently open file.
```
:command Haste !hastebin-client upload --open "%"
```
