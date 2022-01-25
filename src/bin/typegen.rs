use std::{
    fs::{self, File},
    io::Read,
};

use clap::Parser;
use rust_elm_typegen::{ElmFile, RustFile};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    input: String,
    #[clap(short, long)]
    output: String,
    #[clap(short, long)]
    module: String,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);

    // Read input file
    let mut rust_file = File::open(args.input).expect("Failed to open file");
    let mut rust_file_content = String::new();
    rust_file
        .read_to_string(&mut rust_file_content)
        .expect("Failed to read file");

    let ast = syn::parse_file(&rust_file_content).expect("Failed to parse file");
    let rust_file = RustFile::parse(&ast).expect("Failed to parse file");

    let elm_file_object = ElmFile {
        name: args.module,
        structs: rust_file.export_structs,
        enums: rust_file.export_enums,
    };

    let output = elm_file_object.generate_file_content();

    fs::write(args.output, output).expect("Failed to write file");
}
