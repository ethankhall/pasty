extern crate hyper;
extern crate hyper_native_tls;
extern crate clap;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod uploader;

use clap::{App, Arg, SubCommand, ArgMatches};
use std::io::Write;
use std::io;
use std::process::exit;
use std::process::Command;

fn main() {
    let matches = App::new("Hastebin Client")
        .version("0.2")
        .author("Joe K.")
        .about("Uploads and downloads files from hastebin.com")
        .subcommand(SubCommand::with_name("upload")
                        .about("uploads a file to hastebin.")
                        .arg(Arg::with_name("file")
                                 .long("file")
                                 .short("f")
                                 .takes_value(true)
                                 .required(false)
                                 .help("The file to be uploaded."))
                        .arg(Arg::with_name("open")
                                 .short("o")
                                 .long("open")
                                 .takes_value(false)
                                 .help("Opens the created paste after using xdg-open.")
                                 .required(false)))
        .get_matches();
    if let Err(e) = run(matches) {
        writeln!(&mut std::io::stderr(), "Error: {}", e).expect("Failed to write to stderr");
        exit(1);
    }
    exit(0);
}
fn run(matches: ArgMatches) -> Result<(), String> {
    match matches.subcommand_name() {
        Some("upload") => {
            let matches = matches.subcommand_matches("upload").unwrap(); //ok to unwrap here, guaranteed some matches

            //upload file
            let id = match matches.value_of("file") {
                    Some(file) => uploader::upload_file(file), //read from file
                    None => uploader::upload(&mut io::stdin()), //read from stdin if no file provided
                }
                .map_err(|e| e.to_string())?;

            let url = format!("https://hastebin.com/{}", id);
            println!("{}", url);

            if matches.is_present("open") {
                //open with xdg-open
                Command::new("xdg-open") //TODO: config file for other programs?
                    .arg(url)
                    .spawn()
                    .map_err(|e| {
                        format!("An error occured while attempting to open the new Haste: {}",
                                e.to_string())
                    })?;
            }
        }
        Some(_) => {
            unreachable!(); //clap should prevent any incorrect subcommands from matching
        }
        None => {
            return Err("A subcommand is required".to_owned());
        }
    }
    Ok(())
}
