extern crate hyper;
extern crate hyper_native_tls;
extern crate clap;

use clap::{App, Arg, SubCommand, ArgMatches};
use std::io::Write;
use std::process::exit;

fn main() {
    let matches = App::new("Hastebin Client")
        .version("0.1")
        .author("Joe K.")
        .about("Uploads and downloads files from hastebin.com")
        .subcommand(SubCommand::with_name("upload")
                    .about("uploads a file to hastebin.")
                    .arg(Arg::with_name("file")
                         .required(true)
                         .help("The file to be uploaded.")
                    )
                    )
        .subcommand(SubCommand::with_name("download")
                    .about("downloads a file from hastebin.")
                    .arg(Arg::with_name("URL")
                         .required(true)
                         .help("the URL or id of the haste to download.")
                        )
                    )
        .get_matches();
    if let Err(e) = run(matches) {
        writeln!(&mut std::io::stderr(), "Error: {}", e).expect("Failed to write to stderr");
        exit(1);
    }
    exit(0);
}
fn run(matches: ArgMatches) -> Result<(), String> {
    Ok(())
}
