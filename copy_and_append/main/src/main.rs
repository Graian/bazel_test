use clap::{App, Arg};
use lib::copy_and_append;

fn main() {
    let matches = App::new("My Program")
        .arg(Arg::new("input_file")
            .required(true)
            .takes_value(true)
            .short('i')
            .long("input_file"))
        .arg(Arg::new("output_file")
            .required(true)
            .takes_value(true)
            .short('o')
            .long("output_file"))
        .get_matches();

    let input_file = matches.value_of("input_file").unwrap();
    let output_file = matches.value_of("output_file").unwrap();

    copy_and_append(input_file, output_file).expect("[*MT:F] Failed to copy and append");
}