use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use parser::parse_aco;

mod colors;
mod parser;

// ---------------------------------------------------------------
// entry point
// ---------------------------------------------------------------

fn main() {
    let filename = "test.aco";
    // let output_name = "test.css";
    let aco_path = build_path(filename);
    // let out_path = build_path(output_name);
    run(&aco_path);
    let args: Vec<_> = std::env::args().collect(); // get all arguements passed to app
    println!("{:?}", args);
}

// ---------------------------------------------------------------
// File read/write functions
// ---------------------------------------------------------------

/// Build a path in current directory
fn build_path(filename: &str) -> PathBuf {
    let current_dir = env::current_dir().unwrap();
    let result = Path::new(&current_dir).join("data").join(filename);
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
    let path = Path::new(&current_dir).join("data").join(filename);
    let mut file = File::create(path).unwrap();
    write!(file, "{}", data)
}

/// execute cli
fn run(aco_path: &PathBuf) {
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
    export_to_file("export.css", &css).expect("Could not export file");
    export_to_file("export.scss", &scss).expect("Could not export file");

    css_vars.push_str("\n}");
    export_to_file("export_vars.css", &css_vars).expect("Could not export file");

    // dbg!(&result);
    // match result.last() {
    //     None => println!("result is empty"),
    //     Some(c) => println!("colors: {}", c.to_web_colors()),
    // };
}
