use std::path::PathBuf;
use lib::parse_sv_file;

fn main() {
    let path = PathBuf::from("./mytest.sv");
    let syntax_tree = parse_sv_file(&path).unwrap();

    println!("{:#?}", syntax_tree);
}