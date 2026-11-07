//! Internal helper code required for [display_floats_as].
//!
//! The standard library does nice exact conversions to decimal, but
//! lacks a nice output format, so this module helps to do that.  **Do
//! not use this code direcly, but instead call [display_floats_as]!**

use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;

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

impl Floating {
    /// Build a `Normal` from the `{:e}` representation of the absolute
    /// value of a finite float, which is always of the form `1.234e5`
    /// or `1e5`.
    fn normal(is_negative: bool, lower_exp: &str) -> Self {
        let (mantissa, exponent) = lower_exp
            .split_once('e')
            .expect("float repr should have exponent");
        Floating::Normal {
            exponent: i16::from_str(exponent).expect("exponent should be integer"),
            mantissa: mantissa.replace('.', ""),
            is_negative,
        }
    }
}

macro_rules! floating_from {
    ($($float:ty),*) => {
        $(
            impl From<$float> for Floating {
                fn from(x: $float) -> Self {
                    if x.is_normal() {
                        Floating::normal(x < 0., &format!("{:e}", x.abs()))
                    } else {
                        Floating::Abnormal(format!("{}", x))
                    }
                }
            }
        )*
    };
}
floating_from!(f32, f64);

#[test]
fn to_floating() {
    assert_eq!(
        Floating::from(1.0),
        Floating::Normal {
            exponent: 0,
            mantissa: "1".to_string(),
            is_negative: false
        }
    );
    assert_eq!(
        Floating::from(1.2e10),
        Floating::Normal {
            exponent: 10,
            mantissa: "12".to_string(),
            is_negative: false
        }
    );
}

impl Floating {
    /// Format this floating point number nicely, using `e` and
    /// `after_e` to delimit the exponent in case we decide to format
    /// it using scientific notation.  `e_waste` is the number
    /// of characters we consider wasted when using scientific
    /// notation.
    pub fn fmt_with(
        &self,
        f: &mut Formatter,
        e: &str,
        after_e: &str,
        e_waste: usize,
        power_ten: Option<&str>,
    ) -> Result<(), Error> {
        match self {
            Floating::Abnormal(s) => f.write_str(&s),
            Floating::Normal {
                exponent,
                mantissa,
                is_negative,
            } => {
                let e_waste = e_waste as i16;
                if *is_negative {
                    f.write_str("-")?;
                }
                if *exponent > 1 + e_waste || *exponent < -2 - e_waste {
                    if mantissa.len() > 1 {
                        let (a, r) = mantissa.split_at(1);
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
                    if *exponent + 1 > mantissa.len() as i16 {
                        f.write_str(mantissa)?;
                        for _ in 0..*exponent as usize + 1 - mantissa.len() {
                            f.write_str("0")?;
                        }
                        Ok(())
                    } else if *exponent < 0 {
                        f.write_str("0.")?;
                        for _ in 0..-exponent - 1 {
                            f.write_str("0")?;
                        }
                        f.write_str(&mantissa)
                    } else if *exponent + 1 == mantissa.len() as i16 {
                        f.write_str(mantissa)
                    } else {
                        let (a, b) = mantissa.split_at(*exponent as usize + 1);
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

#[test]
fn to_floating_f32() {
    assert_eq!(
        Floating::from(1.0f32),
        Floating::Normal {
            exponent: 0,
            mantissa: "1".to_string(),
            is_negative: false
        }
    );
    assert_eq!(
        Floating::from(1.2e10f32),
        Floating::Normal {
            exponent: 10,
            mantissa: "12".to_string(),
            is_negative: false
        }
    );
    assert_eq!(
        Floating::from(-0.1f32),
        Floating::Normal {
            exponent: -1,
            mantissa: "1".to_string(),
            is_negative: true
        }
    );
}

#[test]
fn to_floating_negative_and_abnormal() {
    assert_eq!(
        Floating::from(-123.4),
        Floating::Normal {
            exponent: 2,
            mantissa: "1234".to_string(),
            is_negative: true
        }
    );
    assert_eq!(Floating::from(0.0), Floating::Abnormal("0".to_string()));
    assert_eq!(Floating::from(-0.0), Floating::Abnormal("-0".to_string()));
    assert_eq!(Floating::from(f64::NAN), Floating::Abnormal("NaN".to_string()));
    assert_eq!(
        Floating::from(f64::INFINITY),
        Floating::Abnormal("inf".to_string())
    );
    assert_eq!(
        Floating::from(f64::NEG_INFINITY),
        Floating::Abnormal("-inf".to_string())
    );
    assert_eq!(Floating::from(0.0f32), Floating::Abnormal("0".to_string()));
    assert_eq!(Floating::from(f32::NAN), Floating::Abnormal("NaN".to_string()));
    assert_eq!(
        Floating::from(f32::NEG_INFINITY),
        Floating::Abnormal("-inf".to_string())
    );
    // Subnormal numbers are not "normal", and are printed in full.
    assert_eq!(
        Floating::from(f64::MIN_POSITIVE / 2.0),
        Floating::Abnormal(format!("{}", f64::MIN_POSITIVE / 2.0))
    );
    assert_eq!(
        Floating::from(f32::MIN_POSITIVE / 2.0),
        Floating::Abnormal(format!("{}", f32::MIN_POSITIVE / 2.0))
    );
}

#[test]
fn extremes() {
    assert_eq!(
        Floating::from(f64::MAX),
        Floating::Normal {
            exponent: 308,
            mantissa: "17976931348623157".to_string(),
            is_negative: false
        }
    );
    assert_eq!(
        Floating::from(f64::MIN_POSITIVE),
        Floating::Normal {
            exponent: -308,
            mantissa: "22250738585072014".to_string(),
            is_negative: false
        }
    );
    assert_eq!(
        Floating::from(f32::MAX),
        Floating::Normal {
            exponent: 38,
            mantissa: "34028235".to_string(),
            is_negative: false
        }
    );
    assert_eq!(
        Floating::from(f32::MIN_POSITIVE),
        Floating::Normal {
            exponent: -38,
            mantissa: "11754944".to_string(),
            is_negative: false
        }
    );
}

#[test]
fn display_f32() {
    assert_eq!(&format!("{}", Floating::from(1.0f32)), "1");
    assert_eq!(&format!("{}", Floating::from(0.1f32)), "0.1");
    assert_eq!(&format!("{}", Floating::from(-0.1f32)), "-0.1");
    assert_eq!(&format!("{}", Floating::from(1e-10f32)), "1e-10");
    assert_eq!(&format!("{}", Floating::from(1.2e-10f32)), "1.2e-10");
    assert_eq!(&format!("{}", Floating::from(120.0f32)), "120");
    assert_eq!(&format!("{}", Floating::from(123.4f32)), "123.4");
    assert_eq!(&format!("{}", Floating::from(1.2e6f32)), "1.2e6");
    assert_eq!(&format!("{}", Floating::from(0.0001f32)), "1e-4");
    assert_eq!(&format!("{}", Floating::from(0.001234f32)), "0.001234");
}
