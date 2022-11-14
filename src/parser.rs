use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::PathBuf,
    str,
};

use crate::colors::{map_to_hex_color, ColorSpace, HexColor};

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

/// Parses an '.aco' file and creates a Vector of HexColor(s)
/// This does NOT use any part related to Adobe v1 stuff.
pub fn parse_aco(aco_path: &PathBuf) -> Vec<HexColor> {
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
pub fn aco_to_buffer(aco_path: &PathBuf) -> io::Result<BufReader<File>> {
    let f = File::open(aco_path)?;
    let reader = BufReader::new(f);
    Ok(reader)
}
