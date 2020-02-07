use lazy_static::lazy_static;
use nom::IResult;
use std::fs::read;
use wld::model::properties::Properties;
use wld::{
  model::common::*,
  model::header::{Header, Offsets},
  model::properties::*,
  model::variables::*,
  model::world::World,
  parse_world,
};

pub fn unwrap<T>(res: IResult<&[u8], T>) -> T {
  res.unwrap().1
}

lazy_static! {
  // paths are relative to root, I guess?
  pub static ref WORLD: std::vec::Vec<u8> =
    read("tests/fixtures/Foon.wld").expect("Unable to read file");
}

#[test]
fn test_parse() {
  assert_eq!(
    parse_world(&WORLD).unwrap(),
    World {
      header: Header::new(
        "1.3.5.3".to_string(),
        160,
        false,
        Offsets {
          header: 127,
          tiles: 2802,
          chests: 2860224,
          signs: 2879758,
          npcs: 2880141,
          tile_entities: 2880453,
          pressure_plates: 2880457,
          town_manager: 2880461,
          footer: 2880489
        }
      ),
      properties: Properties {
        tile_frame_importances: TBitVec::from(vec![
          true, false, true, false, true, false, true, false,
        ]),
        name: TString::from("boneskullandia"),
        generator: GeneratorInfo {
          seed: TString::from("herp"),
          version: 123456789,
        },
        uuid: TUuid::new(Uuid::NAMESPACE_DNS), // why not
        id: 12345678,
        bounds: Rect {
          left: 0,
          right: 67200,
          top: 0,
          bottom: 19200,
        },
        size: Point::new(4200, 1200),
        is_expert: TBool::from(false),
        created_on: 444444,
        style: WorldStyle {
          moon: 1,
          trees: QuadrantStyle::new(1, 2, 3, 4, 5, 6, 7),
          moss: QuadrantStyle::new(1, 2, 3, 4, 5, 6, 7),
          underground_ice: 1,
          underground_jungle: 0,
          hell: 1,
        },
        spawn_point: Point::new(2098, 229),
        underground_level: 300.0,
        cavern_level: 528.0,
      },
      variables: Variables {
        time: 0f32,
        is_daytime: TBool::from(true),
        moon_phase: 0u32,
        is_blood_moon: TBool::from(false),
        is_eclipse: TBool::from(true),
        dungeon_point: Point::new(0, 0),
        evil_type: EvilType::Crimson
      }
    }
  );
}
