#[cfg(test)]
mod test_variables {
  use super::*;
  use crate::test_helpers::*;

  #[test]
  fn test_importances() {
    assert_eq!(
      // unwrap(importances(&WORLD[66..127]))[..32],
      // [
      //   false, false, false, true, true, true, false, false, false, false, true, true, true, true,
      //   true, true, true, true, true, true, true, true, false, false, true, false, true, true,
      //   true, true, false, true
      // ]
      unwrap(importances(
        &[
          &16u16.to_le_bytes()[..],
          &0u8.to_le_bytes()[..],
          &1u8.to_le_bytes()[..]
        ]
        .concat()
      )),
      [
        false, false, false, false, false, false, false, false, true, false, false, false, false,
        false, false, false
      ]
    );
  }
}
