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

#[derive(Debug)]
pub struct RawColorV1 {
    pub color_space: ColorSpace,
    pub component_1: u16,
    pub component_2: u16,
    pub component_3: u16,
    pub component_4: u16,
}

impl RawColorV1 {
    fn to_rgb(&self) -> String {
        format!(
            "{:04x}{:04x}{:04x}",
            self.component_1, self.component_2, self.component_3
        )
        .to_uppercase()
    }

    fn to_hsb(&self) -> String {
        self.to_rgb()
    }
    fn to_cmyk(&self) -> String {
        format!(
            "{:04x}{:04x}{:04x}{:04x}",
            self.component_1, self.component_2, self.component_3, self.component_4
        )
        .to_uppercase()
    }
    fn to_grayscale(&self) -> String {
        format!("{:04x}", self.component_1).to_uppercase()
    }

    fn to_hex(&self) -> String {
        match self.color_space {
            ColorSpace::RGB => self.to_rgb(),
            ColorSpace::HSB => self.to_hsb(),
            ColorSpace::CMYK => self.to_cmyk(),
            // ColorSpace::PANTONE => four_c,
            // ColorSpace::FOCOLTONE => four_c,
            // ColorSpace::TRUMATCH => four_c,
            // ColorSpace::TOYO => four_c,
            // ColorSpace::LAB => four_c,
            ColorSpace::GRAYSCALE => self.to_grayscale(),
            // ColorSpace::HKS => four_c,
            _ => panic!("Unsupported color space {}", self.color_space),
        }
    }
}

impl fmt::Display for RawColorV1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v1 swatch {}", self.to_hex())
    }
}

#[derive(Debug)]
pub struct RawColorV2 {
    pub name: String,
    pub color_space: ColorSpace,
    pub component_1: u16,
    pub component_2: u16,
    pub component_3: u16,
    pub component_4: u16,
}
impl RawColorV2 {
    fn to_rgb(&self) -> String {
        format!(
            "{:04x}{:04x}{:04x}",
            self.component_1, self.component_2, self.component_3
        )
        .to_uppercase()
    }

    pub fn to_8bit_rgb(&self) -> String {
        let temp = self.to_rgb();
        let p1 = temp[0..2].to_string();
        let p2 = temp[4..6].to_string();
        let p3 = temp[8..10].to_string();
        format!("{}{}{}", p1, p2, p3)
    }

    fn to_hsb(&self) -> String {
        self.to_rgb()
    }

    pub fn to_8bit_hsb(&self) -> String {
        self.to_8bit_rgb()
    }

    fn to_cmyk(&self) -> String {
        format!(
            "{:04x}{:04x}{:04x}{:04x}",
            self.component_1, self.component_2, self.component_3, self.component_4
        )
        .to_uppercase()
    }

    fn to_grayscale(&self) -> String {
        format!("{:04x}", self.component_1).to_uppercase()
    }

    pub fn to_hex(&self) -> String {
        match self.color_space {
            ColorSpace::RGB => self.to_rgb(),
            ColorSpace::HSB => self.to_hsb(),
            ColorSpace::CMYK => self.to_cmyk(),
            // ColorSpace::PANTONE => four_c,
            // ColorSpace::FOCOLTONE => four_c,
            // ColorSpace::TRUMATCH => four_c,
            // ColorSpace::TOYO => four_c,
            // ColorSpace::LAB => four_c,
            ColorSpace::GRAYSCALE => self.to_grayscale(),
            // ColorSpace::HKS => four_c,
            _ => panic!("Unsupported color space {}", self.color_space),
        }
    }
}

impl fmt::Display for RawColorV2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {}, color: {}", self.name, self.to_hex())
    }
}
