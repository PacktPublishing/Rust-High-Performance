extern crate phf_codegen;

use std::path::Path;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("phf.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let map = [("key1", "\"value1\""), ("key2", "\"value2\"")];

    write!(
        &mut file,
        "static MAP: phf::Map<&'static str, &'static str> =\n"
    ).unwrap();

    let mut phf_map = phf_codegen::Map::new();
    for &(key, value) in &map {
        phf_map.entry(key, value);
    }

    phf_map.build(&mut file).unwrap();
    write!(&mut file, ";\n").unwrap();
}
