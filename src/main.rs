extern crate wld;

fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
  wld::mounts()?;
  Ok(())
}

// use nom::{dbg_dmp, bytes::complete::tag};

// fn dump(i: &[u8]) -> IResult<&[u8], &[u8]> {
//   dbg_dmp(tag("abcd"), "tag")(i)
// }
