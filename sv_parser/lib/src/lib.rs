use std::{collections::HashMap, path::PathBuf, io};
use sv_parser::{parse_sv, SyntaxTree};

pub fn parse_sv_file(file_path: &PathBuf) -> io::Result<SyntaxTree> {
    let defines = HashMap::new();
    let vec_includes: Vec<PathBuf> = Vec::new();

    match parse_sv(file_path, &defines, &vec_includes, false, false) {
        Ok((syntax_tree, _)) => Ok(syntax_tree),
        Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Failed to parse the file")),
    }
}