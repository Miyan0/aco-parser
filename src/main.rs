use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use clap::Parser;

use parser::parse_aco;

mod colors;
mod parser;

const DEFAULT_OUTPUT_FILE_NAME: &str = "default";

// ---------------------------------------------------------------
// CLI Args
// ---------------------------------------------------------------

/// Adobe color swatch parser. Converts one '.aco' file to
/// css, scss and css variable files. Converted files are
/// exported to a directory named 'output'.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Location of the .aco file
    #[arg(short, long)]
    input_name: String,

    /// name of the css output file
    #[arg(short, long, default_value_t = (DEFAULT_OUTPUT_FILE_NAME.to_string()))]
    output_name: String,
}

// ---------------------------------------------------------------
// entry point
// ---------------------------------------------------------------
// in dev, to run with args ->
//          cargo run -- --my_args myValue
fn main() {
    let args = Args::parse();
    let aco_path = build_path(&args.input_name);
    let output_filename: &str;

    if args.output_name == DEFAULT_OUTPUT_FILE_NAME {
        println!("using default output name");
        output_filename = aco_path.file_stem().unwrap().to_str().unwrap();
    } else {
        output_filename = &args.output_name;
    }
    run(&aco_path, &output_filename);
}

// ---------------------------------------------------------------
// File read/write functions
// ---------------------------------------------------------------

/// Build a path in current directory
fn build_path(filename: &str) -> PathBuf {
    let current_dir = env::current_dir().unwrap();
    let result = Path::new(&current_dir).join(filename);
    if !result.exists() {
        panic!("Could not create path")
    }
    result
}

// ---------------------------------------------------------------
// export string to file
// ---------------------------------------------------------------
fn export_to_file(filename: &str, data: &str) -> std::io::Result<()> {
    let current_dir = env::current_dir().unwrap();
    let output_dir = "output";
    let out_path = Path::new(&current_dir).join(output_dir);
    if !out_path.exists() {
        fs::create_dir_all(&out_path)?;
    }

    let path = out_path.join(filename);
    let mut file = File::create(path).unwrap();
    write!(file, "{}", data)
}

/// execute cli
fn run(aco_path: &PathBuf, output_filename: &str) {
    let hex_colors = parse_aco(&aco_path);
    let mut css = String::new();
    let mut scss = String::new();
    let mut css_vars = String::new();
    css_vars.push_str(":root {\n");

    for item in hex_colors {
        let c_css = format!("{}\n", item.to_css());
        css.push_str(&c_css);

        let c_scss = format!("{}\n", item.to_scss());
        scss.push_str(&c_scss);

        let c_css_vars = format!("{}\n", item.to_css_variables());
        css_vars.push_str(&c_css_vars);
    }
    let css_filename = format!("{}.css", output_filename);
    let scss_filename = format!("{}.scss", output_filename);
    let css_vars_filename = format!("{}-vars.css", output_filename);

    export_to_file(&css_filename, &css).expect("Could not export file");
    export_to_file(&scss_filename, &scss).expect("Could not export file");

    css_vars.push_str("\n}");
    export_to_file(&css_vars_filename, &css_vars).expect("Could not export file");
}
