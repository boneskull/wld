use lazy_static::lazy_static;
use std::fs::read;
use wld::model::properties::Properties;
use wld::{
  model::common::*,
  model::header::{Header, Offsets},
  model::properties::*,
  model::common::TBool::*,
  model::world::World,
  parse_world,
};

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
      header: Header {
        version: "1.3.5.3".to_string(),
        revision: 160,
        is_favorite: false,
        offsets: Offsets {
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
      },
      properties: Properties {
        tile_frame_importances: TBitVec::from(vec![
          false, false, false, true, true, true, false, false, false, false,
          true, true, true, true, true, true, true, true, true, true, true,
          true, false, false, true, false, true, true, true, true, false, true,
          false, true, true, true, true, false, false, false, false, false,
          true, false, false, false, false, false, false, false, true, false,
          false, false, false, true, false, false, false, false, false, true,
          false, false, false, false, false, false, false, false, false, true,
          true, true, true, false, false, true, true, true, false, true, true,
          true, true, true, true, true, true, true, true, true, true, true,
          true, true, true, true, true, true, true, true, true, true, true,
          true, true, false, false, false, true, false, false, true, true,
          false, false, false, false, false, false, false, false, false, false,
          true, true, false, true, true, false, false, true, true, true, true,
          true, true, true, true, false, true, true, true, true, false, false,
          false, false, true, false, false, false, false, false, false, false,
          false, false, false, false, false, false, false, false, true, false,
          false, false, false, false, true, true, true, true, false, false,
          false, true, false, false, false, false, false, true, true, true,
          true, false, false, false, false, false, false, false, false, false,
          false, false, false, false, true, false, false, false, false, false,
          true, false, true, true, false, true, false, false, true, true, true,
          true, true, true, false, false, false, false, false, false, true,
          true, false, false, true, false, true, false, true, true, true, true,
          true, true, true, true, true, true, true, true, true, false, false,
          false, false, false, false, true, false, false, false, false, false,
          false, false, false, false, false, false, false, false, false, true,
          true, true, false, false, false, true, true, true, true, true, true,
          true, true, true, false, true, true, true, true, true, true, true,
          true, true, true, true, true, true, true, true, true, true, true,
          true, true, true, true, true, true, true, true, false, false, false,
          true, false, true, true, true, true, true, false, false, true, true,
          false, false, false, false, false, false, false, false, false, true,
          true, false, true, true, true, false, false, false, false, false,
          false, false, false, false, true, false, false, false, false, true,
          true, true, false, true, true, true, true, true, true, true, false,
          false, false, false, false, false, false, true, true, true, true,
          true, true, true, false, true, false, false, false, false, false,
          true, true, true, true, true, true, true, true, true, true, false,
          false, false, false, false, false, false, false, false, true, true,
          false, false, false, true, true, true, true, true, false, false,
          false, false, true, true, false, false, true, true, true, false,
          true, true, true, false, false, false, false, false, true, true,
          true, true, true, true, true, true, true, true, true, false, false,
          false, false, false, false, true, true, true, true, true, true,
          false, false, false, true, true, true, true, true, true, true, true,
          true, false, false,
        ]),
        name: TString::from("Foon"),
        generator: GeneratorInfo {
          seed: TString::from("1451234789"),
          version: 833223655425,
        },
        uuid: TUuid::from(
          Uuid::parse_str("d578e106-3827-f648-a224-254c06ca78cb").unwrap()
        ),
        id: 1468463142,
        bounds: Rect {
          left: 0,
          right: 67200,
          top: 0,
          bottom: 19200,
        },
        size: Point { x: 4200, y: 1200 },
        is_expert: False,
        created_on: 9860045932737703464,
        style: WorldStyle {
          moon: 1,
          trees: QuadrantStyle {
            far_left: 4,
            near_left: 5,
            near_right: 0,
            far_right: 0,
            x1: 3072,
            x2: 4200,
            x3: 4200
          },
          moss: QuadrantStyle {
            far_left: 1,
            near_left: 0,
            near_right: 3,
            far_right: 0,
            x1: 1210,
            x2: 4200,
            x3: 4200
          },
          underground_snow: 3,
          underground_jungle: 0,
          hell: 0,
        },
        spawn_point: Point::new(2098, 229),
        underground_level: 300.0,
        cavern_level: 528.0,
        time: 34958.0,
        is_daytime: True,
        moon_phase: 1,
        is_blood_moon: False,
        is_eclipse: False,
        dungeon_point: Point::new(3426, 211),
        evil_type: EvilType::Corruption,
      }
    }
  );
}
