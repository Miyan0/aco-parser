use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum ColorSpace {
    RGB = 0,
    HSB = 1,
    CMYK = 2,
    PANTONE = 3,
    FOCOLTONE = 4,
    TRUMATCH = 5,
    TOYO = 6,
    LAB = 7,
    GRAYSCALE = 8,
    HKS = 10,
}

impl ColorSpace {
    pub fn from_u16(value: u16) -> ColorSpace {
        match value {
            0 => ColorSpace::RGB,
            1 => ColorSpace::HSB,
            2 => ColorSpace::CMYK,
            3 => ColorSpace::PANTONE,
            4 => ColorSpace::FOCOLTONE,
            5 => ColorSpace::TRUMATCH,
            6 => ColorSpace::TOYO,
            7 => ColorSpace::LAB,
            8 => ColorSpace::GRAYSCALE,
            10 => ColorSpace::HKS,
            _ => panic!("Unknown value: {}", value),
        }
    }

    pub fn as_u16(&self) -> u16 {
        *self as u16
    }
}

impl fmt::Display for ColorSpace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_u16())
    }
}

/// Stores Adobe Color Swatches (v2 only)
/// See adobe spec here:
/// https://www.adobe.com/devnet-apps/photoshop/fileformatashtml/#50577411_31265
#[derive(Debug)]
pub struct HexColor {
    pub name: String,
    pub color_space: ColorSpace,
    pub color_hex: String, // stores 16 bits color values.
}

impl HexColor {
    /// Converts from 16 bits to 8 bits color values
    /// This is not a lossless translation as 16bit contains way
    /// more data, but is good enough for web colors.
    pub fn to_web_colors(&self) -> String {
        let mut css_color = String::new();
        css_color.push('#');
        for (i, chunk) in self.color_hex.chars().enumerate() {
            if [0, 1, 4, 5, 8, 9, 12, 13].contains(&i) {
                css_color.push(chunk);
            }
        }
        css_color
    }

    pub fn to_css(&self) -> String {
        let slug = self.name.replace(" ", "_").to_lowercase();
        format!(
            ".{} {{\n    background: {};\n}}",
            slug,
            self.to_web_colors()
        )
    }

    pub fn to_scss(&self) -> String {
        let slug = self.name.replace(" ", "_").to_lowercase();
        format!("${}:    {};", slug, self.to_web_colors())
    }

    pub fn to_css_variables(&self) -> String {
        let slug = self.name.replace(" ", "-").to_lowercase();
        format!("    --{}:    {};", slug, self.to_web_colors())
    }

    /// keep for reference
    #[allow(dead_code)]
    pub fn count_components(&self) -> usize {
        match self.color_space {
            ColorSpace::RGB => 3,
            ColorSpace::HSB => 3,
            ColorSpace::CMYK => 4,
            ColorSpace::GRAYSCALE => 1,
            _ => panic!("Unsupported color space: {}", self.color_space),
        }
    }
}

impl fmt::Display for HexColor {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "name: {}, color space: {}, color hex: {}",
            self.name, self.color_space, self.color_hex
        )
    }
}

pub fn to_rgb(c1: u16, c2: u16, c3: u16) -> String {
    format!("{:04x}{:04x}{:04x}", c1, c2, c3)
}

fn to_hsb(c1: u16, c2: u16, c3: u16) -> String {
    format!("{:04x}{:04x}{:04x}", c1, c2, c3)
}

pub fn to_cmyk(c1: u16, c2: u16, c3: u16, c4: u16) -> String {
    format!("{:04x}{:04x}{:04x}{:04x}", c1, c2, c3, c4)
}

pub fn to_grayscale(c1: u16) -> String {
    format!("{:04x}", c1)
}

pub fn map_to_hex_color(
    color_space: ColorSpace,
    comp1: u16,
    comp2: u16,
    comp3: u16,
    comp4: u16,
    name: String,
) -> HexColor {
    let color_hex = match color_space {
        ColorSpace::RGB => to_rgb(comp1, comp2, comp3),
        ColorSpace::HSB => to_hsb(comp1, comp2, comp3),
        ColorSpace::CMYK => to_cmyk(comp1, comp2, comp3, comp4),
        ColorSpace::GRAYSCALE => to_grayscale(comp1),
        _ => panic!("Unsupported color space: {}", color_space),
    };
    HexColor {
        name,
        color_space,
        color_hex,
    }
}
