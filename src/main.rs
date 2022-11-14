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

fn read_aco_file(path: PathBuf) -> (Vec<RawColorV1>, Vec<RawColorV2>) {
    let mut input = BufReader::new(File::open(path).expect("Failed to open file"));
    let mut buffer_u16 = [0u8; std::mem::size_of::<u16>()];

    input.read_exact(&mut buffer_u16).expect("read error");
    let mut version_byte = u16::from_be_bytes(buffer_u16);
    if version_byte != 1 {
        panic!("Version byte should be 1")
    }

    input.read_exact(&mut buffer_u16).expect("read error");
    let mut color_count = u16::from_be_bytes(buffer_u16);
    if color_count != 7 {
        panic!("Color count should be 7 in our test file")
    }

    let mut v1_vec = Vec::new();
    let mut v2_vec = Vec::new();

    for idx in 1..=color_count {
        input.read_exact(&mut buffer_u16).expect("read error");
        let color_space = ColorSpace::from_u16(u16::from_be_bytes(buffer_u16));

        input.read_exact(&mut buffer_u16).expect("read error");
        let component_1 = u16::from_be_bytes(buffer_u16);

        input.read_exact(&mut buffer_u16).expect("read error");
        let component_2 = u16::from_be_bytes(buffer_u16);

        input.read_exact(&mut buffer_u16).expect("read error");
        let component_3 = u16::from_be_bytes(buffer_u16);

        input.read_exact(&mut buffer_u16).expect("read error");
        let component_4 = u16::from_be_bytes(buffer_u16);

        v1_vec.push(RawColorV1 {
            color_space,
            component_1,
            component_2,
            component_3,
            component_4,
        });
        // dbg!(raw_color);
    }

    // version 2
    input.read_exact(&mut buffer_u16).expect("read error");
    version_byte = u16::from_be_bytes(buffer_u16);
    if version_byte != 2 {
        panic!("Version byte should be 2 but was {}", version_byte)
    }

    input.read_exact(&mut buffer_u16).expect("read error");
    color_count = u16::from_be_bytes(buffer_u16);

    let mut buffer_u32 = [0u8; std::mem::size_of::<u32>()];
    for idx in 1..=color_count {
        input.read_exact(&mut buffer_u16).expect("read error");
        let color_space = ColorSpace::from_u16(u16::from_be_bytes(buffer_u16));

        input.read_exact(&mut buffer_u16).expect("read error");
        let component_1 = u16::from_be_bytes(buffer_u16);

        input.read_exact(&mut buffer_u16).expect("read error");
        let component_2 = u16::from_be_bytes(buffer_u16);

        input.read_exact(&mut buffer_u16).expect("read error");
        let component_3 = u16::from_be_bytes(buffer_u16);

        input.read_exact(&mut buffer_u16).expect("read error");
        let component_4 = u16::from_be_bytes(buffer_u16);

        input.read_exact(&mut buffer_u32).expect("read error");
        let name_length = u32::from_be_bytes(buffer_u32);
        let len = name_length * 2;

        let mut str_buffer = vec![0u8; len as usize];
        input.read_exact(&mut str_buffer).expect("read error here");
        let name = str::from_utf8(&str_buffer).unwrap();
        // let name = str::from_utf8(&str_buffer).unwrap();

        v2_vec.push(RawColorV2 {
            name: name.to_string().replace("\0", ""),
            color_space,
            component_1,
            component_2,
            component_3,
            component_4,
        });
    }
    (v1_vec, v2_vec)
}

fn build_path(filename: &str) -> PathBuf {
    let current_dir = env::current_dir().unwrap();
    let result = Path::new(&current_dir).join("data").join(filename);
    if !result.exists() {
        panic!("Could not create path")
    }
    result
}

fn to_css(color: &RawColorV2) -> String {
    let slug = color.name.replace(" ", "_").to_lowercase();
    format!(
        ".{} {{\n    background: #{};\n}}",
        slug,
        color.to_8bit_rgb()
    )
}

fn export_to_file(filename: &str, data: &str) -> std::io::Result<()> {
    let current_dir = env::current_dir().unwrap();
    let path = Path::new(&current_dir).join("data").join(filename);
    let mut file = File::create(path).unwrap();
    write!(file, "{}", data)
}

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
