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

Make sure you have [Rust](https://rust-lang.org) and its package manager Cargo installed, then run
```Bash
cargo install pasty
```
## VIM Integration

Add the following to your `.vimrc` file to add commands for uploading the current buffer:
```VIMscript
:command Haste :w !pasty upload --open 
:command Gist :w !pasty upload --open --service github --name "%:t"
```
