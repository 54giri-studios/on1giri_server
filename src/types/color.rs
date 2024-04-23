use diesel::{deserialize::FromSqlRow, prelude::*};
use regex::Regex;
use once_cell::sync::Lazy;

use crate::Db;

static HEX_REGEX: Lazy<Regex> = Lazy::new(|| 
    Regex::new(r#"^#(?P<red>[0-9a-fA-F]{2})(?P<green>[0-9a-fA-F]{2})(?P<blue>[0-9a-fA-F]{2})$"#).unwrap()
);

#[derive(Debug)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue}
    }

    pub fn from_hex(hex: String) -> Option<Self> {
        let capture = HEX_REGEX.captures_iter(&hex).next()?;
        let red = &capture["red"];
        let green = &capture["green"];
        let blue = &capture["blue"];

        Some(Self::new(
            u8::from_str_radix(red, 16).ok()?, 
            u8::from_str_radix(green, 16).ok()?,
            u8::from_str_radix(blue, 16).ok()?
        ))
    }
    pub fn to_hex_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}
