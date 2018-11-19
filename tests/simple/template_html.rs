 impl DisplayAs<HTML> for Simple  {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
  f.write_str("here is a simple template\n")?;
Ok(()) }
}
