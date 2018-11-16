|f: &mut ::std::fmt::Formatter| {
  f.write_str("hello world ")?;
  (&( 3.0e10 ) as &DisplayAs<HTML>).fmt(f)?;
  f.write_str("\nThis is cool.\n")?;
Ok(()) }
