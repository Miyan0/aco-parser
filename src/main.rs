use colors::ColorSpace;
use colors::{map_to_hex_color, HexColor};
use std::env;
use std::io;
use std::io::BufReader;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::{self};
use std::{fs::File, io::Read};

mod colors;

const V1_INFO_LEN: usize = 10;

// ---------------------------------------------------------------
// Read chunks from reader
// ---------------------------------------------------------------

/// Read 2 bytes from file buffer
fn read_u16(reader: &mut BufReader<File>) -> u16 {
    let mut buffer_u16 = [0u8; std::mem::size_of::<u16>()];
    reader.read_exact(&mut buffer_u16).expect("read error");
    u16::from_be_bytes(buffer_u16)
}

/// Read swatch name from file buffer (name is a UTF-8 string)
fn read_swatch_name(reader: &mut BufReader<File>) -> String {
    let mut buffer_u32 = [0u8; std::mem::size_of::<u32>()];
    reader.read_exact(&mut buffer_u32).expect("read error");
    let name_length = u32::from_be_bytes(buffer_u32);
    let len = name_length * 2;

    let mut str_buffer = vec![0u8; len as usize];
    reader.read_exact(&mut str_buffer).expect("read error here");

    let name = str::from_utf8(&str_buffer).unwrap();
    name.to_string().replace("\0", "")
}

// ---------------------------------------------------------------
// Skip chunks (advance file buffer cursor)
// ---------------------------------------------------------------

/// Advance the file reader cursor by 2 bytes
fn skip_u16(reader: &mut BufReader<File>) {
    _ = read_u16(reader);
}

/// Adbance the file reader cursor after adobe v1 stuff
fn skip_v1(reader: &mut BufReader<File>) {
    let mut buffer = [0u8; V1_INFO_LEN];
    reader.read_exact(&mut buffer).expect("read error");
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

/*
fn parse_using_buffer(filename: &str) {
    let path = build_path(filename);
    let vec_raw_color_v2 = parse_aco(&path);
    let mut data = String::new();
    for item in vec_raw_color_v2 {
        let tmp = format!("{}\n", to_css(&item));
        data.push_str(&tmp);
    }
    export_to_file("export.css", &data).expect("Could not export file");
}
*/

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

// ---------------------------------------------------------------
// entry point
// ---------------------------------------------------------------

fn main() {
    let filename = "test.aco";
    let output_name = "test.css";
    let aco_path = build_path(filename);
    // let out_path = build_path(output_name);
    run(&aco_path);
}

/// Parses an '.aco' file and creates a Vector of HexColor(s)
/// This does NOT use any part related to Adobe v1 stuff.
fn parse_aco(aco_path: &PathBuf) -> Vec<HexColor> {
    let mut input = match aco_to_buffer(aco_path) {
        Ok(buffer) => buffer,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    // NOTE: we don't care about v1 stuff
    skip_u16(&mut input); // v1 version bytes
    let color_count = read_u16(&mut input);
    for _ in 1..=color_count {
        skip_v1(&mut input);
    }
    // version 2. This we DO care!
    let version_byte = read_u16(&mut input);
    if version_byte != 2 {
        panic!("Version byte should be 2 but was {}", version_byte)
    }

    let mut v2_vec = Vec::new();

    let color_count = read_u16(&mut input);
    for _ in 1..=color_count {
        let hex_color = map_to_hex_color(
            ColorSpace::from_u16(read_u16(&mut input)),
            read_u16(&mut input),
            read_u16(&mut input),
            read_u16(&mut input),
            read_u16(&mut input),
            read_swatch_name(&mut input),
        );
        v2_vec.push(hex_color);
    }
    v2_vec
}

/// creates a buffer for reading an '.aco' file
fn aco_to_buffer(aco_path: &PathBuf) -> io::Result<BufReader<File>> {
    let f = File::open(aco_path)?;
    let reader = BufReader::new(f);
    Ok(reader)
}
