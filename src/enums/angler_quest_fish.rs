use num_traits::FromPrimitive;
use scroll::{
  ctx::{
    SizeWith,
    TryFromCtx,
    TryIntoCtx,
  },
  Endian,
  Error as ScrollError,
  Pread,
  LE,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(C)]
pub enum AnglerQuestFish {
  Batfish = 0,
  BumblebeeTuna = 1,
  Catfish = 2,
  Cloudfish = 3,
  Cursedfish = 4,
  Dirtfish = 5,
  DynamiteFish = 6,
  EaterOfPlankton = 7,
  FallenStarfish = 8,
  TheFishOfCthulhu = 9,
  Fishotron = 10,
  Harpyfish = 11,
  Hungerfish = 12,
  Ichorfish = 13,
  Jewelfish = 14,
  MirageFish = 15,
  MutantFlinxfin = 16,
  Pengfish = 17,
  Pixiefish = 18,
  Spiderfish = 19,
  TundraTrout = 20,
  UnicornFish = 21,
  GuideVoodooFish = 22,
  Wyverntail = 23,
  ZombieFish = 24,
  AmanitiaFungifin = 25,
  Angelfish = 26,
  BloodyManowar = 27,
  Bonefish = 28,
  Bunnyfish = 29,
  CapnTunabeard = 30,
  Clownfish = 31,
  DemonicHellfish = 32,
  Derpfish = 33,
  Fishron = 34,
  InfectedScabbardfish = 35,
  Mudfish = 36,
  Slimefish = 37,
  TropicalBarracuda = 38,
}

impl SizeWith<Endian> for AnglerQuestFish {
  fn size_with(_: &Endian) -> usize {
    i32::size_with(&LE)
  }
}

impl<'a> TryFromCtx<'a, Endian> for AnglerQuestFish {
  type Error = ScrollError;

  fn try_from_ctx(
    buf: &'a [u8],
    _: Endian,
  ) -> Result<(Self, usize), Self::Error> {
    let offset = &mut 0;
    let value = match Self::from_i32(buf.gread_with::<i32>(offset, LE)?) {
      Some(v) => v,
      None => {
        return Err(Self::Error::Custom(
          "failed to convert i32 to AnglerQuestFish".to_string(),
        ))
      }
    };
    Ok((value, *offset))
  }
}

impl<'a> TryIntoCtx<Endian> for &'a AnglerQuestFish {
  type Error = ScrollError;

  fn try_into_ctx(
    self,
    buf: &mut [u8],
    ctx: Endian,
  ) -> Result<usize, Self::Error> {
    let mut size = 0;
    let value = *self as i32;
    size += value.try_into_ctx(&mut buf[size..], ctx)?;
    Ok(size)
  }
}
