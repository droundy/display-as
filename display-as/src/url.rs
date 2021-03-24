//! [Format] as URL, with escaping using percent encoding.

use super::*;
use percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};

/// [Format] as URL.
pub struct URL;
impl Format for URL {
    fn escape(f: &mut Formatter, s: &str) -> Result<(), Error> {
        f.write_str(&utf8_percent_encode(s, DEFAULT_ENCODE_SET).to_string())
    }
    /// The MIME type for URL is [mime::TEXT_URL_UTF_8].
    fn mime() -> mime::Mime {
        return "text/x-url".parse().unwrap();
    }
    fn this_format() -> Self {
        URL
    }
}

display_integers_as!(URL);
display_floats_as!(URL, "e", "", 1, None);

#[test]
fn escaping() {
    assert_eq!(&format_as!(URL, ("&")).into_string(), "&");
    assert_eq!(
        &format_as!(URL, ("hello &>this is cool")).into_string(),
        "hello%20&%3Ethis%20is%20cool"
    );
    assert_eq!(
        &format_as!(URL, ("hello &>this is 'cool")).into_string(),
        "hello%20&%3Ethis%20is%20\'cool"
    );
}
