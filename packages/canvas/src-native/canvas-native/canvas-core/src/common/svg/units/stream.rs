use std::str::FromStr;

use crate::common::svg::error::{Error, Result};
use crate::common::svg::units::length::{Length, LengthUnit};

pub trait ByteExtension {
    fn is_space(&self) -> bool;
    fn is_digit(&self) -> bool;
    fn is_sign(&self) -> bool;
}

impl ByteExtension for u8 {
    fn is_space(&self) -> bool {
        matches!(*self, b' ' | b'\n' | b'\t' | b'\r')
    }
    fn is_digit(&self) -> bool {
        matches!(*self, b'0'..=b'9')
    }
    fn is_sign(&self) -> bool {
        matches!(*self, b'-' | b'+')
    }
}

pub struct Stream<'a> {
    txt: &'a str,
    pos: usize,
}

impl<'a> From<&'a str> for Stream<'a> {
    fn from(txt: &'a str) -> Self {
        Stream { txt, pos: 0 }
    }
}

impl<'a> Stream<'a> {
    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn parse_length(&mut self) -> Result<Length> {
        let value = self.parse_number()?;
        if self.at_end() {
            return Ok(Length::new(value, LengthUnit::None));
        }

        let u = if self.starts_with(b"%") {
            LengthUnit::Percent
        } else if self.starts_with(b"em") {
            LengthUnit::Em
        } else if self.starts_with(b"ex") {
            LengthUnit::Ex
        } else if self.starts_with(b"px") {
            LengthUnit::Px
        } else if self.starts_with(b"in") {
            LengthUnit::In
        } else if self.starts_with(b"cm") {
            LengthUnit::Cm
        } else if self.starts_with(b"mm") {
            LengthUnit::Mm
        } else if self.starts_with(b"pt") {
            LengthUnit::Pt
        } else if self.starts_with(b"pc") {
            LengthUnit::Pc
        } else {
            LengthUnit::None
        };

        match u {
            LengthUnit::Percent => self.advance(1),
            LengthUnit::None => {}
            _ => self.advance(2),
        }

        Ok(Length::new(value, u))
    }

    fn parse_number(&mut self) -> Result<f32> {
        self.skip_white_spaces();
        let start = self.pos();

        if self.at_end() {
            return Err(Error::InvalidNumber);
        }

        let mut c = self.current_byte()?;
        if c.is_sign() {
            self.advance(1);
            c = self.current_byte()?;
        }

        match c {
            b'0'..=b'9' => self.skip_digits(),
            b'.' => {}
            _ => {
                return Err(Error::InvalidNumber);
            }
        }

        if let Ok(b'.') = self.current_byte() {
            self.advance(1);
            self.skip_digits();
        }

        if let Ok(c) = self.current_byte() {
            if matches!(c, b'e' | b'E') && c != b'm' && c != b'x' {
                self.advance(1);
                match self.current_byte()? {
                    b'+' | b'-' => {
                        self.advance(1);
                        self.skip_digits();
                    }
                    b'0'..=b'9' => self.skip_digits(),
                    _ => {
                        return Err(Error::InvalidNumber);
                    }
                }
            }
        }

        let s = self.slice_back(start);
        if let Ok(val) = f32::from_str(s) {
            if val.is_finite() {
                return Ok(val);
            }
        }

        Err(Error::InvalidNumber)
    }

    fn skip_digits(&mut self) {
        self.skip_bytes(|_, c| c.is_digit())
    }

    fn skip_bytes<F>(&mut self, f: F)
        where
            F: Fn(&Stream, u8) -> bool,
    {
        while !self.at_end() {
            let c = self.current_byte_unchecked();
            if f(self, c) {
                self.advance(1);
            } else {
                break;
            }
        }
    }

    fn advance(&mut self, by: usize) {
        self.pos += by;
    }

    fn skip_white_spaces(&mut self) {
        while !self.at_end() && self.current_byte_unchecked().is_space() {
            self.advance(1)
        }
    }

    fn current_byte_unchecked(&self) -> u8 {
        self.txt.as_bytes()[self.pos]
    }

    fn current_byte(&self) -> Result<u8> {
        if self.at_end() {
            return Err(Error::UnexpectedEndOfStream);
        }
        Ok(self.txt.as_bytes()[self.pos])
    }

    pub fn at_end(&self) -> bool {
        self.pos >= self.txt.len()
    }

    pub fn starts_with(&self, text: &[u8]) -> bool {
        self.txt.as_bytes()[self.pos..].starts_with(text)
    }

    pub fn slice_back(&self, pos: usize) -> &str {
        &self.txt[pos..self.pos]
    }
}
