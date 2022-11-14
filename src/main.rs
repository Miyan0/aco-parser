use std::env;
use std::io::BufReader;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::{self};
use std::{fs::File, io::Read};

use colors::ColorSpace;
use colors::RawColorV1;
use colors::RawColorV2;

mod colors;

// ---------------------------------------------------------------
// read 2 bytes
// ---------------------------------------------------------------

fn read_u16(reader: &mut BufReader<File>) -> u16 {
    let mut buffer_u16 = [0u8; std::mem::size_of::<u16>()];
    reader.read_exact(&mut buffer_u16).expect("read error");
    u16::from_be_bytes(buffer_u16)
}

// ---------------------------------------------------------------
// read swatch name
// ---------------------------------------------------------------

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
// read aco file
// ---------------------------------------------------------------

fn read_aco_file(path: PathBuf) -> (Vec<RawColorV1>, Vec<RawColorV2>) {
    let mut input = BufReader::new(File::open(path).expect("Failed to open file"));

    let mut version_byte = read_u16(&mut input);
    if version_byte != 1 {
        panic!("Version byte should be 1")
    }
    let mut color_count = read_u16(&mut input);
    let mut v1_vec = Vec::new();
    for _ in 1..=color_count {
        let color_space = read_u16(&mut input);
        let component_1 = read_u16(&mut input);
        let component_2 = read_u16(&mut input);
        let component_3 = read_u16(&mut input);
        let component_4 = read_u16(&mut input);
        v1_vec.push(RawColorV1 {
            color_space: ColorSpace::from_u16(color_space),
            component_1,
            component_2,
            component_3,
            component_4,
        });
    }

    // version 2
    version_byte = read_u16(&mut input);
    if version_byte != 2 {
        panic!("Version byte should be 2 but was {}", version_byte)
    }
    color_count = read_u16(&mut input);
    let mut v2_vec = Vec::new();
    for _ in 1..=color_count {
        let color_space = read_u16(&mut input);
        let component_1 = read_u16(&mut input);
        let component_2 = read_u16(&mut input);
        let component_3 = read_u16(&mut input);
        let component_4 = read_u16(&mut input);
        let name = read_swatch_name(&mut input);
        v2_vec.push(RawColorV2 {
            name,
            color_space: ColorSpace::from_u16(color_space),
            component_1,
            component_2,
            component_3,
            component_4,
        });
    }
    (v1_vec, v2_vec)
}

// ---------------------------------------------------------------
// build path for aco file
// ---------------------------------------------------------------

fn build_path(filename: &str) -> PathBuf {
    let current_dir = env::current_dir().unwrap();
    let result = Path::new(&current_dir).join("data").join(filename);
    if !result.exists() {
        panic!("Could not create path")
    }
    result
}

// ---------------------------------------------------------------
// convert one raw color (v2) to css
// ---------------------------------------------------------------

fn to_css(color: &RawColorV2) -> String {
    let slug = color.name.replace(" ", "_").to_lowercase();
    format!(
        ".{} {{\n    background: #{};\n}}",
        slug,
        color.to_8bit_rgb()
    )
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

// ---------------------------------------------------------------
// entry point
// ---------------------------------------------------------------

fn main() {
    let filename = "test.aco";

    let path = build_path(filename);
    let (_vec_raw_color_v1, vec_raw_color_v2) = read_aco_file(path);
    let mut data = String::new();
    for item in vec_raw_color_v2 {
        let tmp = format!("{}\n", to_css(&item));
        data.push_str(&tmp);
    }
    export_to_file("export.css", &data).expect("Could not export file");
}
