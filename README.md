# pasty - a client for hastebin-like sites
Pasty is a CLI client for hastebin.com and other similar services written in Rust.
Currently, it only supports uploading of files. You can optionally open newly opened files in a new browser window using `--open`.
## Supported services
Currently, hastebin and GitHub gist are supported. By default, Hastebin is used, but you can specify the service using the `--service` option.

## Example Usage
```Bash
pasty upload ./file.txt --open
```
The command above uploads ./file.txt to Hastebin and uploads the newly created haste in a new browser window.

## Installing

If you don't want to build from source, head over to the [releases](https://github.com/joek13/pasty/releases/latest) page and grab the latest binary build. If there isn't one available for your platform, you'll have to build from source - instructions for that are below.

## Building from source

1. Make sure you have [Rust](https://www.rust-lang.org/en-US/) installed.
2. Clone the repo.
3. Enter the directory and run `cargo install`
4. You're done! Optionally, add the following to your `vim` config file to create a custom command to upload the currently open file.
```
:command Haste :w !pasty upload --open 
```
