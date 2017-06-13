extern crate hyper;
extern crate hyper_native_tls;
extern crate clap;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod uploader;

use clap::{App, Arg, SubCommand, ArgMatches};
use std::io::Write;
use std::io;
use std::process::exit;
use std::process::Command;

fn main() {
    let matches = App::new("pasty")
        .version("0.2")
        .author("Joe K.")
        .about("Uploads files to various paste sites")
        .subcommand(SubCommand::with_name("upload")
                        .about("uploads a file.")
                        .arg(Arg::with_name("file")
                                 .takes_value(true)
                                 .required(false)
                                 .help("The file to be uploaded."))
                        .arg(Arg::with_name("service")
                                 .long("service")
                                 .short("s")
                                 .takes_value(true)
                                 .required(false)
                                 .possible_values(&["hastebin", "github"])
                                 .help("Service to upload to. Defaults to hastebin."))
                        .arg(Arg::with_name("filename")
                                 .long("name")
                                 .short("n")
                                 .takes_value(true)
                                 .required(false)
                                 .help("Filename to use. Some services will display the filename with the pasted file"))
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
            let matches = matches.subcommand_matches("upload").unwrap();
            //ok to unwrap here, guaranteed some matches

            //upload file
            let url: String = match matches.value_of("service") {
                Some("github") => {
                    let url = match matches.value_of("file") {
                            Some(file) => {
                                uploader::github::upload_file(file,
                                                              matches
                                                                  .value_of("filename")
                                                                  .map(|s| s.to_owned()))
                            }
                            None => {
                                uploader::github::upload(&mut io::stdin(),
                                                         matches
                                                             .value_of("filename")
                                                             .unwrap_or("")
                                                             .to_owned())
                            }
                        }
                        .map_err(|e| e.to_string())?;
                    url
                }
                Some("hastebin") | None => {
                    let url = match matches.value_of("file") {
                            Some(file) => uploader::hastebin::upload_file(file), //read from file
                            None => uploader::hastebin::upload(&mut io::stdin()), //read from stdin if no file provided
                        }
                        .map_err(|e| e.to_string())?;
                    url
                }
                Some(_) => {
                    unreachable!(); //clap should prevent this from happening
                }
            };
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
