#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum HardmodeOreType {
  UnknownOre,
  Cobalt = 107,
  Mythril = 108,
  Adamantite = 111,
  Palladium = 221,
  Orichalcum = 222,
  Titanium = 223,
}
