#[macro_use]
extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("user")
                .help("The user to say hello to")
                .value_name("username")
                .short("u")
                .long("username")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let user = matches
        .value_of("user")
        .expect("somehow the user did not give the username");

    println!("Hello, {}", user);
}
