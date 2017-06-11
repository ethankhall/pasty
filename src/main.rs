extern crate hyper;
extern crate hyper_native_tls;
extern crate clap;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod hastebin;

use clap::{App, Arg, SubCommand, ArgMatches};
use std::io::Write;
use std::process::exit;
use std::process::Command;

fn main() {
    let matches = App::new("Hastebin Client")
        .version("0.1")
        .author("Joe K.")
        .about("Uploads and downloads files from hastebin.com")
        .subcommand(SubCommand::with_name("upload")
                        .about("uploads a file to hastebin.")
                        .arg(Arg::with_name("file")
                                 .required(true)
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
    if let Some(matches) = matches.subcommand_matches("upload") {
        //upload file
        let file = matches.value_of("file").unwrap();
        //should crash if file isn't present, as it's a required argument
        match hastebin::upload_file(file) {
            Err(e) => {
                return Err(e.to_string());
            }
            Ok(id) => {
                let url = format!("https://hastebin.com/{}", id);
                println!("{}", url);
                if matches.is_present("open") {
                    //open with xdg-open
                    Command::new("xdg-open")
                        .arg(url)
                        .spawn()
                        .expect("Failed to open");
                }
            }
        }
    }
    Ok(())
}
