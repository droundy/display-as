//! Helper routines for formatting floating point numbers.

//! The standard library does nice exact conversions to decimal, but
//! lacks a nice output format, so this module helps to do that.

use std::str::FromStr;
use std::fmt::{Display, Formatter, Error};

/// This represents an f32 or f64 that has been converted to a string,
/// but which we have not yet decided for certain how to represent
/// (e.g. how many digits to show, or whether to use `e` or `E`
/// notation).
#[derive(Eq, PartialEq, Debug)]
pub enum Floating {
    /// A normal floating point number
    Normal {
        /// The exponent
        exponent: i16,
        /// The mantissa, without any decimal point
        mantissa: String,
        /// Is it negative?
        is_negative: bool,
    },
    /// This is a NaN or an infinity
    Abnormal(String),
}

impl From<f64> for Floating {
    fn from(x: f64) -> Self {
        if !x.is_normal() {
            return Floating::Abnormal(format!("{}", x));
        }
        let is_negative = x < 0.;
        let x = if is_negative { -x } else { x };
        let x = format!("{:e}", x);
        let mut parts = x.splitn(2, "e");
        if let Some(mantissa) = parts.next() {
            let mut mantissa = mantissa.to_string();
            if mantissa.len() > 1 {
                mantissa.remove(1);
            }
            let exponent = i16::from_str(parts.next()
                                         .expect("float repr should have exponent")
            ).expect("exponent should be integer");
            Floating::Normal { exponent, mantissa, is_negative }
        } else {
            panic!("I think thi sis impossible...");
        }
    }
}
impl From<f32> for Floating {
    fn from(x: f32) -> Self {
        if !x.is_normal() {
            return Floating::Abnormal(format!("{}", x));
        }
        let is_negative = x < 0.;
        let x = if is_negative { -x } else { x };
        let x = format!("{:e}", x);
        let mut parts = x.splitn(2, "e");
        if let Some(mantissa) = parts.next() {
            let mut mantissa = mantissa.to_string();
            if mantissa.len() > 1 {
                mantissa.remove(1);
            }
            let exponent = i16::from_str(parts.next()
                                         .expect("float repr should have exponent")
            ).expect("exponent should be integer");
            Floating::Normal { exponent, mantissa, is_negative }
        } else {
            panic!("I think thi sis impossible...");
        }
    }
}

#[test]
fn to_floating() {
    assert_eq!(Floating::from(1.0),
               Floating::Normal {
                   exponent: 0,
                   mantissa: "1".to_string(),
                   is_negative: false
               });
    assert_eq!(Floating::from(1.2e10),
               Floating::Normal {
                   exponent: 10,
                   mantissa: "12".to_string(),
                   is_negative: false
               });
}

impl Floating {
    /// Format this floating point number nicely, using `e` and
    /// `after_e` to delimit the exponent in case we decide to format
    /// it using scientific notation.  `e_waste` is the number
    /// of characters we consider wasted when using scientific
    /// notation.
    pub fn fmt_with(&self, f: &mut Formatter,
                    e: &str, after_e: &str, e_waste: usize,
                    power_ten: Option<&str>) -> Result<(), Error> {
        match self {
            Floating::Abnormal(s) => f.write_str(&s),
            Floating::Normal { exponent, mantissa, is_negative } => {
                let e_waste = e_waste as i16;
                if *is_negative { f.write_str("-")?; }
                if *exponent > 1 + e_waste || *exponent < -2 - e_waste {
                    if mantissa.len() > 1 {
                        let (a,r) = mantissa.split_at(1);
                        f.write_str(a)?;
                        f.write_str(".")?;
                        f.write_str(r)?;
                        f.write_str(e)?;
                        exponent.fmt(f)?;
                        f.write_str(after_e)
                    } else if mantissa == "1" && power_ten.is_some() {
                        // We can omit the mantissa, keeping things
                        // pretty and compact.
                        f.write_str(power_ten.unwrap())?;
                        exponent.fmt(f)?;
                        f.write_str(after_e)
                    } else {
                        f.write_str(mantissa)?;
                        f.write_str(e)?;
                        exponent.fmt(f)?;
                        f.write_str(after_e)
                    }
                } else {
                    if *exponent+1 > mantissa.len() as i16 {
                        f.write_str(mantissa)?;
                        for _ in 0 .. *exponent as usize + 1 - mantissa.len() {
                            f.write_str("0")?;
                        }
                        Ok(())
                    } else if *exponent < 0 {
                        f.write_str("0.")?;
                        for _ in 0 .. -exponent-1 {
                            f.write_str("0")?;
                        }
                        f.write_str(&mantissa)
                    } else if *exponent+1 == mantissa.len() as i16 {
                        f.write_str(mantissa)
                    } else {
                        let (a,b) = mantissa.split_at(*exponent as usize+1);
                        f.write_str(a)?;
                        f.write_str(".")?;
                        f.write_str(b)
                    }
                }
            }
        }
    }
}

impl Display for Floating {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.fmt_with(f, "e", "", 1, None)
    }
}

#[test]
fn display() {
    assert_eq!(&format!("{}", Floating::from(1.0)), "1");
    assert_eq!(&format!("{}", Floating::from(0.1)), "0.1");
    assert_eq!(&format!("{}", Floating::from(1e-10)), "1e-10");
    assert_eq!(&format!("{}", Floating::from(1.2e-10)), "1.2e-10");
    assert_eq!(&format!("{}", Floating::from(120.)), "120");
    assert_eq!(&format!("{}", Floating::from(123.)), "123");
    assert_eq!(&format!("{}", Floating::from(123.4)), "123.4");
    assert_eq!(&format!("{}", Floating::from(1.2e6)), "1.2e6");
    assert_eq!(&format!("{}", Floating::from(0.001)), "0.001");
    assert_eq!(&format!("{}", Floating::from(0.0001)), "1e-4");
    assert_eq!(&format!("{}", Floating::from(0.001234)), "0.001234");
}
