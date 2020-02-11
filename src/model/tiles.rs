use crate::model::common::*;
use num_traits::FromPrimitive;
use rayon::prelude::*;
use scroll::{
  Pread,
  LE,
};
use std::convert::TryInto;

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum BlockShape {
  Normal = 0,
  HalfTile = 1,
  TopRightSlope = 2,
  TopLeftSlope = 3,
  BottomRightSlope = 4,
  BottomLeftSlope = 5,
}

impl From<&TBitVec> for BlockShape {
  fn from(flags: &TBitVec) -> Self {
    let value =
      ((flags[6] as u8) << 2) + ((flags[5] as u8) << 1) + flags[4] as u8;
    Self::from_u8(value).unwrap()
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum BlockType {
  Dirt = 0,
  Stone = 1,
  Grass = 2,
  Plants = 3,
  Torches = 4,
  Trees = 5,
  Iron = 6,
  Copper = 7,
  Gold = 8,
  Silver = 9,
  ClosedDoor = 10,
  OpenDoor = 11,
  Heart = 12,
  Bottles = 13,
  Tables = 14,
  Chairs = 15,
  Anvils = 16,
  Furnaces = 17,
  WorkBenches = 18,
  Platforms = 19,
  Saplings = 20,
  Containers = 21,
  Demonite = 22,
  CorruptGrass = 23,
  CorruptPlants = 24,
  Ebonstone = 25,
  DemonAltar = 26,
  Sunflower = 27,
  Pots = 28,
  PiggyBank = 29,
  WoodBlock = 30,
  ShadowOrbs = 31,
  CorruptThorns = 32,
  Candles = 33,
  Chandeliers = 34,
  Jackolanterns = 35,
  Presents = 36,
  Meteorite = 37,
  GrayBrick = 38,
  RedBrick = 39,
  ClayBlock = 40,
  BlueDungeonBrick = 41,
  HangingLanterns = 42,
  GreenDungeonBrick = 43,
  PinkDungeonBrick = 44,
  GoldBrick = 45,
  SilverBrick = 46,
  CopperBrick = 47,
  Spikes = 48,
  WaterCandle = 49,
  Books = 50,
  Cobweb = 51,
  Vines = 52,
  Sand = 53,
  Glass = 54,
  Signs = 55,
  Obsidian = 56,
  Ash = 57,
  Hellstone = 58,
  Mud = 59,
  JungleGrass = 60,
  JunglePlants = 61,
  JungleVines = 62,
  Sapphire = 63,
  Ruby = 64,
  Emerald = 65,
  Topaz = 66,
  Amethyst = 67,
  Diamond = 68,
  JungleThorns = 69,
  MushroomGrass = 70,
  MushroomPlants = 71,
  MushroomTrees = 72,
  Plants2 = 73,
  JunglePlants2 = 74,
  ObsidianBrick = 75,
  HellstoneBrick = 76,
  Hellforge = 77,
  ClayPot = 78,
  Beds = 79,
  Cactus = 80,
  Coral = 81,
  ImmatureHerbs = 82,
  MatureHerbs = 83,
  BloomingHerbs = 84,
  Tombstones = 85,
  Loom = 86,
  Pianos = 87,
  Dressers = 88,
  Benches = 89,
  Bathtubs = 90,
  Banners = 91,
  Lampposts = 92,
  Lamps = 93,
  Kegs = 94,
  ChineseLanterns = 95,
  CookingPots = 96,
  Safes = 97,
  SkullLanterns = 98,
  TrashCan = 99,
  Candelabras = 100,
  Bookcases = 101,
  Thrones = 102,
  Bowls = 103,
  GrandfatherClocks = 104,
  Statues = 105,
  Sawmill = 106,
  Cobalt = 107,
  Mythril = 108,
  HallowedGrass = 109,
  HallowedPlants = 110,
  Adamantite = 111,
  Ebonsand = 112,
  HallowedPlants2 = 113,
  TinkerersWorkbench = 114,
  HallowedVines = 115,
  Pearlsand = 116,
  Pearlstone = 117,
  PearlstoneBrick = 118,
  IridescentBrick = 119,
  Mudstone = 120,
  CobaltBrick = 121,
  MythrilBrick = 122,
  Silt = 123,
  WoodenBeam = 124,
  CrystalBall = 125,
  DiscoBall = 126,
  MagicalIceBlock = 127,
  Mannequin = 128,
  Crystals = 129,
  ActiveStoneBlock = 130,
  InactiveStoneBlock = 131,
  Lever = 132,
  AdamantiteForge = 133,
  MythrilAnvil = 134,
  PressurePlates = 135,
  Switches = 136,
  Traps = 137,
  Boulder = 138,
  MusicBoxes = 139,
  DemoniteBrick = 140,
  Explosives = 141,
  InletPump = 142,
  OutletPump = 143,
  Timers = 144,
  CandyCaneBlock = 145,
  GreenCandyCaneBlock = 146,
  SnowBlock = 147,
  SnowBrick = 148,
  HolidayLights = 149,
  AdamantiteBeam = 150,
  SandstoneBrick = 151,
  EbonstoneBrick = 152,
  RedStucco = 153,
  YellowStucco = 154,
  GreenStucco = 155,
  GrayStucco = 156,
  Ebonwood = 157,
  RichMahogany = 158,
  Pearlwood = 159,
  RainbowBrick = 160,
  IceBlock = 161,
  BreakableIce = 162,
  CorruptIce = 163,
  HallowedIce = 164,
  Stalactite = 165,
  Tin = 166,
  Lead = 167,
  Tungsten = 168,
  Platinum = 169,
  PineTree = 170,
  ChristmasTree = 171,
  Sinks = 172,
  PlatinumCandelabra = 173,
  PlatinumCandle = 174,
  TinBrick = 175,
  TungstenBrick = 176,
  PlatinumBrick = 177,
  ExposedGems = 178,
  GreenMoss = 179,
  BrownMoss = 180,
  RedMoss = 181,
  BlueMoss = 182,
  PurpleMoss = 183,
  LongMoss = 184,
  SmallPiles = 185,
  LargePiles = 186,
  LargePiles2 = 187,
  CactusBlock = 188,
  Cloud = 189,
  MushroomBlock = 190,
  LivingWood = 191,
  LeafBlock = 192,
  SlimeBlock = 193,
  BoneBlock = 194,
  FleshBlock = 195,
  RainCloud = 196,
  FrozenSlimeBlock = 197,
  Asphalt = 198,
  FleshGrass = 199,
  FleshIce = 200,
  FleshWeeds = 201,
  Sunplate = 202,
  Crimstone = 203,
  Crimtane = 204,
  CrimsonVines = 205,
  IceBrick = 206,
  WaterFountain = 207,
  Shadewood = 208,
  Cannon = 209,
  LandMine = 210,
  Chlorophyte = 211,
  SnowballLauncher = 212,
  Rope = 213,
  Chain = 214,
  Campfire = 215,
  Firework = 216,
  Blendomatic = 217,
  MeatGrinder = 218,
  Extractinator = 219,
  Solidifier = 220,
  Palladium = 221,
  Orichalcum = 222,
  Titanium = 223,
  Slush = 224,
  Hive = 225,
  LihzahrdBrick = 226,
  DyePlants = 227,
  DyeVat = 228,
  HoneyBlock = 229,
  CrispyHoneyBlock = 230,
  Larva = 231,
  WoodenSpikes = 232,
  PlantDetritus = 233,
  Crimsand = 234,
  Teleporter = 235,
  LifeFruit = 236,
  LihzahrdAltar = 237,
  PlanteraBulb = 238,
  MetalBars = 239,
  Painting3x3 = 240,
  Painting4x3 = 241,
  Painting6x4 = 242,
  ImbuingStation = 243,
  BubbleMachine = 244,
  Painting2x3 = 245,
  Painting3x2 = 246,
  Autohammer = 247,
  PalladiumColumn = 248,
  BubblegumBlock = 249,
  Titanstone = 250,
  PumpkinBlock = 251,
  HayBlock = 252,
  SpookyWood = 253,
  Pumpkins = 254,
  AmethystGemsparkOff = 255,
  TopazGemsparkOff = 256,
  SapphireGemsparkOff = 257,
  EmeraldGemsparkOff = 258,
  RubyGemsparkOff = 259,
  DiamondGemsparkOff = 260,
  AmberGemsparkOff = 261,
  AmethystGemspark = 262,
  TopazGemspark = 263,
  SapphireGemspark = 264,
  EmeraldGemspark = 265,
  RubyGemspark = 266,
  DiamondGemspark = 267,
  AmberGemspark = 268,
  Womannequin = 269,
  FireflyinaBottle = 270,
  LightningBuginaBottle = 271,
  Cog = 272,
  StoneSlab = 273,
  SandStoneSlab = 274,
  BunnyCage = 275,
  SquirrelCage = 276,
  MallardDuckCage = 277,
  DuckCage = 278,
  BirdCage = 279,
  BlueJay = 280,
  CardinalCage = 281,
  FishBowl = 282,
  HeavyWorkBench = 283,
  CopperPlating = 284,
  SnailCage = 285,
  GlowingSnailCage = 286,
  AmmoBox = 287,
  MonarchButterflyJar = 288,
  PurpleEmperorButterflyJar = 289,
  RedAdmiralButterflyJar = 290,
  UlyssesButterflyJar = 291,
  SulphurButterflyJar = 292,
  TreeNymphButterflyJar = 293,
  ZebraSwallowtailButterflyJar = 294,
  JuliaButterflyJar = 295,
  ScorpionCage = 296,
  BlackScorpionCage = 297,
  FrogCage = 298,
  MouseCage = 299,
  BoneWelder = 300,
  FleshCloningVat = 301,
  GlassKiln = 302,
  LihzahrdFurnace = 303,
  LivingLoom = 304,
  SkyMill = 305,
  IceMachine = 306,
  SteampunkBoiler = 307,
  HoneyDispenser = 308,
  PenguinCage = 309,
  WormCage = 310,
  DynastyWood = 311,
  RedDynastyShingles = 312,
  BlueDynastyShingles = 313,
  MinecartTrack = 314,
  Coralstone = 315,
  BlueJellyfishBowl = 316,
  GreenJellyfishBowl = 317,
  PinkJellyfishBowl = 318,
  ShipInAbottle = 319,
  SeaweedPlanter = 320,
  BorealWood = 321,
  PalmWood = 322,
  PalmTree = 323,
  BeachPiles = 324,
  TinPlating = 325,
  Waterfall = 326,
  Lavafall = 327,
  Confetti = 328,
  ConfettiBlack = 329,
  CopperCoinPile = 330,
  SilverCoinPile = 331,
  GoldCoinPile = 332,
  PlatinumCoinPile = 333,
  WeaponsRack = 334,
  FireworksBox = 335,
  LivingFire = 336,
  AlphabetStatues = 337,
  FireworkFountain = 338,
  GrasshopperCage = 339,
  LivingCursedFire = 340,
  LivingDemonFire = 341,
  LivingFrostFire = 342,
  LivingIchor = 343,
  LivingUltrabrightFire = 344,
  Honeyfall = 345,
  ChlorophyteBrick = 346,
  CrimtaneBrick = 347,
  ShroomitePlating = 348,
  MushroomStatue = 349,
  MartianConduitPlating = 350,
  ChimneySmoke = 351,
  CrimtaneThorns = 352,
  VineRope = 353,
  BewitchingTable = 354,
  AlchemyTable = 355,
  Sundial = 356,
  MarbleBlock = 357,
  GoldBirdCage = 358,
  GoldBunnyCage = 359,
  GoldButterflyCage = 360,
  GoldFrogCage = 361,
  GoldGrasshopperCage = 362,
  GoldMouseCage = 363,
  GoldWormCage = 364,
  SilkRope = 365,
  WebRope = 366,
  Marble = 367,
  Granite = 368,
  GraniteBlock = 369,
  MeteoriteBrick = 370,
  PinkSlimeBlock = 371,
  PeaceCandle = 372,
  WaterDrip = 373,
  LavaDrip = 374,
  HoneyDrip = 375,
  FishingCrate = 376,
  SharpeningStation = 377,
  TargetDummy = 378,
  Bubble = 379,
  PlanterBox = 380,
  LavaMoss = 381,
  VineFlowers = 382,
  LivingMahogany = 383,
  LivingMahoganyLeaves = 384,
  CrystalBlock = 385,
  TrapdoorOpen = 386,
  TrapdoorClosed = 387,
  TallGateClosed = 388,
  TallGateOpen = 389,
  LavaLamp = 390,
  CageEnchantedNightcrawler = 391,
  CageBuggy = 392,
  CageGrubby = 393,
  CageSluggy = 394,
  ItemFrame = 395,
  Sandstone = 396,
  HardenedSand = 397,
  CorruptHardenedSand = 398,
  CrimsonHardenedSand = 399,
  CorruptSandstone = 400,
  CrimsonSandstone = 401,
  HallowHardenedSand = 402,
  HallowSandstone = 403,
  DesertFossil = 404,
  Fireplace = 405,
  Chimney = 406,
  FossilOre = 407,
  LunarOre = 408,
  LunarBrick = 409,
  LunarMonolith = 410,
  Detonator = 411,
  LunarCraftingStation = 412,
  SquirrelOrangeCage = 413,
  SquirrelGoldCage = 414,
  LunarBlockSolar = 415,
  LunarBlockVortex = 416,
  LunarBlockNebula = 417,
  LunarBlockStardust = 418,
  LogicGateLamp = 419,
  LogicGate = 420,
  ConveyorBeltLeft = 421,
  ConveyorBeltRight = 422,
  LogicSensor = 423,
  WirePipe = 424,
  AnnouncementBox = 425,
  TeamBlockRed = 426,
  TeamBlockRedPlatform = 427,
  WeightedPressurePlate = 428,
  WireBulb = 429,
  TeamBlockGreen = 430,
  TeamBlockBlue = 431,
  TeamBlockYellow = 432,
  TeamBlockPink = 433,
  TeamBlockWhite = 434,
  TeamBlockGreenPlatform = 435,
  TeamBlockBluePlatform = 436,
  TeamBlockYellowPlatform = 437,
  TeamBlockPinkPlatform = 438,
  TeamBlockWhitePlatform = 439,
  GemLocks = 440,
  FakeContainers = 441,
  ProjectilePressurePad = 442,
  GeyserTrap = 443,
  BeeHive = 444,
  PixelBox = 445,
  SillyBalloonPink = 446,
  SillyBalloonPurple = 447,
  SillyBalloonGreen = 448,
  SillyStreamerBlue = 449,
  SillyStreamerGreen = 450,
  SillyStreamerPink = 451,
  SillyBalloonMachine = 452,
  SillyBalloonTile = 453,
  Pigronata = 454,
  PartyMonolith = 455,
  PartyBundleOfBalloonTile = 456,
  PartyPresent = 457,
  SandFallBlock = 458,
  SnowFallBlock = 459,
  SnowCloud = 460,
  SandDrip = 461,
  DjinnLamp = 462,
  DefendersForge = 463,
  WarTable = 464,
  WarTableBanner = 465,
  ElderCrystalStand = 466,
  Containers2 = 467,
  FakeContainers2 = 468,
  Tables2 = 469,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum WallType {
  Stone = 1,
  DirtUnsafe = 2,
  EbonstoneUnsafe = 3,
  Wood = 4,
  GrayBrick = 5,
  RedBrick = 6,
  BlueDungeonUnsafe = 7,
  GreenDungeonUnsafe = 8,
  PinkDungeonUnsafe = 9,
  GoldBrick = 10,
  SilverBrick = 11,
  CopperBrick = 12,
  HellstoneBrickUnsafe = 13,
  ObsidianBrickUnsafe = 14,
  MudUnsafe = 15,
  Dirt = 16,
  BlueDungeon = 17,
  GreenDungeon = 18,
  PinkDungeon = 19,
  ObsidianBrick = 20,
  Glass = 21,
  PearlstoneBrick = 22,
  IridescentBrick = 23,
  MudstoneBrick = 24,
  CobaltBrick = 25,
  MythrilBrick = 26,
  Planked = 27,
  PearlstoneBrickUnsafe = 28,
  CandyCane = 29,
  GreenCandyCane = 30,
  SnowBrick = 31,
  AdamantiteBeam = 32,
  DemoniteBrick = 33,
  SandstoneBrick = 34,
  EbonstoneBrick = 35,
  RedStucco = 36,
  YellowStucco = 37,
  GreenStucco = 38,
  Gray = 39,
  SnowWallUnsafe = 40,
  Ebonwood = 41,
  RichMaogany = 42,
  Pearlwood = 43,
  RainbowBrick = 44,
  TinBrick = 45,
  TungstenBrick = 46,
  PlatinumBrick = 47,
  AmethystUnsafe = 48,
  TopazUnsafe = 49,
  SapphireUnsafe = 50,
  EmeraldUnsafe = 51,
  RubyUnsafe = 52,
  DiamondUnsafe = 53,
  CaveUnsafe = 54,
  Cave2unsafe = 55,
  Cave3unsafe = 56,
  Cave4unsafe = 57,
  Cave5unsafe = 58,
  Cave6unsafe = 59,
  LivingLeaf = 60,
  Cave7unsafe = 61,
  SpiderUnsafe = 62,
  GrassUnsafe = 63,
  JungleUnsafe = 64,
  FlowerUnsafe = 65,
  Grass = 66,
  Jungle = 67,
  Flower = 68,
  CorruptGrassUnsafe = 69,
  HallowedGrassUnsafe = 70,
  IceUnsafe = 71,
  Cactus = 72,
  Cloud = 73,
  Mushroom = 74,
  Bone = 75,
  Slime = 76,
  Flesh = 77,
  LivingWood = 78,
  ObsidianBackUnsafe = 79,
  MushroomUnsafe = 80,
  CrimsonGrassUnsafe = 81,
  DiscWall = 82,
  CrimstoneUnsafe = 83,
  IceBrick = 84,
  Shadewood = 85,
  HiveUnsafe = 86,
  LihzahrdBrickUnsafe = 87,
  PurpleStainedGlass = 88,
  YellowStainedGlass = 89,
  BlueStainedGlass = 90,
  GreenStainedGlass = 91,
  RedStainedGlass = 92,
  RainbowStainedGlass = 93,
  BlueDungeonSlabUnsafe = 94,
  BlueDungeonTileUnsafe = 95,
  PinkDungeonSlabUnsafe = 96,
  PinkDungeonTileUnsafe = 97,
  GreenDungeonSlabUnsafe = 98,
  GreenDungeonTileUnsafe = 99,
  BlueDungeonSlab = 100,
  BlueDungeonTile = 101,
  PinkDungeonSlab = 102,
  PinkDungeonTile = 103,
  GreenDungeonSlab = 104,
  GreenDungeonTile = 105,
  WoodenFence = 106,
  MetalFence = 107,
  Hive = 108,
  PalladiumColumn = 109,
  BubblegumBlock = 110,
  TitanstoneBlock = 111,
  LihzahrdBrick = 112,
  Pumpkin = 113,
  Hay = 114,
  SpookyWood = 115,
  ChristmasTreeWallpaper = 116,
  OrnamentWallpaper = 117,
  CandyCaneWallpaper = 118,
  FestiveWallpaper = 119,
  StarsWallpaper = 120,
  SquigglesWallpaper = 121,
  SnowflakeWallpaper = 122,
  KrampusHornWallpaper = 123,
  BluegreenWallpaper = 124,
  GrinchFingerWallpaper = 125,
  FancyGrayWallpaper = 126,
  IceFloeWallpaper = 127,
  MusicWallpaper = 128,
  PurpleRainWallpaper = 129,
  RainbowWallpaper = 130,
  SparkleStoneWallpaper = 131,
  StarlitHeavenWallpaper = 132,
  BubbleWallpaper = 133,
  CopperPipeWallpaper = 134,
  DuckyWallpaper = 135,
  Waterfall = 136,
  Lavafall = 137,
  EbonwoodFence = 138,
  RichMahoganyFence = 139,
  PearlwoodFence = 140,
  ShadewoodFence = 141,
  WhiteDynasty = 142,
  BlueDynasty = 143,
  ArcaneRunes = 144,
  IronFence = 145,
  CopperPlating = 146,
  StoneSlab = 147,
  Sail = 148,
  BorealWood = 149,
  BorealWoodFence = 150,
  PalmWood = 151,
  PalmWoodFence = 152,
  AmberGemspark = 153,
  AmethystGemspark = 154,
  DiamondGemspark = 155,
  EmeraldGemspark = 156,
  AmberGemsparkOff = 157,
  AmethystGemsparkOff = 158,
  DiamondGemsparkOff = 159,
  EmeraldGemsparkOff = 160,
  RubyGemsparkOff = 161,
  SapphireGemsparkOff = 162,
  TopazGemsparkOff = 163,
  RubyGemspark = 164,
  SapphireGemspark = 165,
  TopazGemspark = 166,
  TinPlating = 167,
  Confetti = 168,
  ConfettiBlack = 169,
  CaveWall = 170,
  CaveWall2 = 171,
  Honeyfall = 172,
  ChlorophyteBrick = 173,
  CrimtaneBrick = 174,
  ShroomitePlating = 175,
  MartianConduit = 176,
  HellstoneBrick = 177,
  MarbleUnsafe = 178,
  MarbleBlock = 179,
  GraniteUnsafe = 180,
  GraniteBlock = 181,
  MeteoriteBrick = 182,
  Marble = 183,
  Granite = 184,
  Cave8unsafe = 185,
  Crystal = 186,
  Sandstone = 187,
  CorruptionUnsafe1 = 188,
  CorruptionUnsafe2 = 189,
  CorruptionUnsafe3 = 190,
  CorruptionUnsafe4 = 191,
  CrimsonUnsafe1 = 192,
  CrimsonUnsafe2 = 193,
  CrimsonUnsafe3 = 194,
  CrimsonUnsafe4 = 195,
  DirtUnsafe1 = 196,
  DirtUnsafe2 = 197,
  DirtUnsafe3 = 198,
  DirtUnsafe4 = 199,
  HallowUnsafe1 = 200,
  HallowUnsafe2 = 201,
  HallowUnsafe3 = 202,
  HallowUnsafe4 = 203,
  JungleUnsafe1 = 204,
  JungleUnsafe2 = 205,
  JungleUnsafe3 = 206,
  JungleUnsafe4 = 207,
  LavaUnsafe1 = 208,
  LavaUnsafe2 = 209,
  LavaUnsafe3 = 210,
  LavaUnsafe4 = 211,
  RocksUnsafe1 = 212,
  RocksUnsafe2 = 213,
  RocksUnsafe3 = 214,
  RocksUnsafe4 = 215,
  HardenedSand = 216,
  CorruptHardenedSand = 217,
  CrimsonHardenedSand = 218,
  HallowHardenedSand = 219,
  CorruptSandstone = 220,
  CrimsonSandstone = 221,
  HallowSandstone = 222,
  DesertFossil = 223,
  LunarBrickWall = 224,
  CogWall = 225,
  SandFall = 226,
  SnowFall = 227,
  SillyBalloonPinkWall = 228,
  SillyBalloonPurpleWall = 229,
  SillyBalloonGreenWall = 230,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum LiquidType {
  NoLiquid = 0,
  Water = 1,
  Lava = 2,
  Honey = 3,
}

impl From<&TBitVec> for LiquidType {
  fn from(flags: &TBitVec) -> Self {
    if flags[3] && flags[4] {
      LiquidType::Honey
    } else if flags[4] {
      LiquidType::Lava
    } else if flags[3] {
      LiquidType::Water
    } else {
      LiquidType::NoLiquid
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Liquid {
  pub liquid_type: LiquidType,
  pub volume: u8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Block {
  pub block_type: BlockType,
  pub shape: BlockShape,
  pub frame_data: Option<(u16, u16)>,
  pub block_paint: Option<u8>,
  pub is_block_active: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Wall {
  pub wall_type: WallType,
  pub wall_paint: Option<u8>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Wiring {
  pub red: bool,
  pub green: bool,
  pub blue: bool,
  pub yellow: bool,
  pub actuator: bool,
}

impl From<(&TBitVec, &TBitVec)> for Wiring {
  fn from(flags: (&TBitVec, &TBitVec)) -> Self {
    let (flags, more_flags) = flags;
    Self {
      red: flags[1],
      green: flags[2],
      blue: flags[3],
      yellow: more_flags[5],
      actuator: more_flags[1],
    }
  }
}

impl From<&TBitVec> for Wiring {
  fn from(flags: &TBitVec) -> Self {
    Self {
      red: flags[1],
      green: flags[2],
      blue: flags[3],
      yellow: false,
      actuator: false,
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Tile {
  pub block: Option<Block>,
  pub wall: Option<Wall>,
  pub liquid: Option<Liquid>,
  pub wiring: Option<Wiring>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum RLEType {
  NoCompression = 0,
  SingleByte = 1,
  DoubleByte = 2,
}

impl From<&TBitVec> for RLEType {
  fn from(flags: &TBitVec) -> Self {
    let value = ((flags[7] as u8) << 1) + flags[6] as u8;
    Self::from_u8(value).unwrap()
  }
}

fn read_tile(
  buf: &[u8],
  tile_frame_importances: &VariableTBitVec,
  offset: &mut usize,
) -> std::result::Result<(Tile, u16), Box<dyn std::error::Error>> {
  let flags = buf.gread_with::<TBitVec>(offset, LE)?;
  // println!("{}: {:?}", offset, flags);
  let has_more_flags = flags[0];
  let has_block = flags[1];
  let has_wall = flags[2];
  let liquid_type = LiquidType::from(&flags);
  let has_extended_block_id = flags[5];
  let rle_type = RLEType::from(&flags);
  let mut shape = BlockShape::Normal;
  let mut is_block_active = true;
  let mut is_wall_painted = false;
  let mut is_block_painted = false;
  let mut wiring: Option<Wiring> = None;
  let mut block: Option<Block> = None;
  let mut wall: Option<Wall> = None;
  let mut liquid: Option<Liquid> = None;
  if has_more_flags {
    let more_flags = buf.gread_with::<TBitVec>(offset, LE)?;
    let has_even_more_flags = more_flags[0];
    shape = BlockShape::from(&more_flags);
    if has_even_more_flags {
      let even_more_flags = buf.gread_with::<TBitVec>(offset, LE)?;
      is_block_active = !even_more_flags[2];
      wiring = Some(Wiring::from((&more_flags, &even_more_flags)));
      is_block_painted = even_more_flags[3];
      is_wall_painted = even_more_flags[4];
    } else {
      wiring = Some(Wiring::from(&more_flags));
    }
  }

  if has_block {
    let mut frame_data: Option<(u16, u16)> = None;
    let mut block_paint: Option<u8> = None;

    let block_id = if has_extended_block_id {
      buf.gread_with::<u16>(offset, LE)? as i64
    } else {
      buf.gread::<u8>(offset)? as i64
    };
    // #[allow(trivial_numeric_casts)]
    // println!(
    //   "{:?} is dirt? {:?}",
    //   block_id,
    //   block_id == BlockType::Dirt as i64
    // );
    let block_type = BlockType::from_i64(block_id).unwrap();
    // println!("block_type: {:?}", block_type);
    if tile_frame_importances[block_type as usize] {
      let frame_data_x = buf.gread_with::<u16>(offset, LE)?;
      let frame_data_y = buf.gread_with::<u16>(offset, LE)?;
      frame_data = Some((frame_data_x, frame_data_y));
    }
    if is_block_painted {
      block_paint = Some(buf.gread::<u8>(offset)?);
    }
    block = Some(Block {
      block_type,
      shape,
      frame_data,
      block_paint,
      is_block_active,
    });
  }

  if has_wall {
    println!("reading offset at {:?}", offset);
    let wall_id = buf.gread::<u8>(offset)? as i64;
    println!(
      "{:?} is {:?}",
      wall_id,
      wall_id == WallType::RocksUnsafe2 as i64
    );
    let res = WallType::from_i64(wall_id);
    let wall_type: WallType = res.unwrap();
    let wall_paint: Option<u8> = if is_wall_painted {
      Some(buf.gread::<u8>(offset)?)
    } else {
      None
    };
    wall = Some(Wall {
      wall_type,
      wall_paint,
    });
    // println!("{:?}", wall);
  }

  if liquid_type != LiquidType::NoLiquid {
    liquid = Some(Liquid {
      liquid_type,
      volume: buf.gread::<u8>(offset)?,
    });
    // println!("liquid: {:?}", liquid);
  }

  let multiply_by: u16 = match rle_type {
    RLEType::DoubleByte => buf.gread_with::<u16>(offset, LE)? + 1,
    RLEType::SingleByte => buf.gread::<u8>(offset)? as u16 + 1,
    _ => 1,
  };
  // println!("repeating {:?} times", multiply_by);
  let tile = Tile {
    block,
    wall,
    liquid,
    wiring,
  };
  Ok((tile, multiply_by))
}

pub fn parse_tile_matrix(
  buf: &[u8],
  offset: &mut usize,
  world_width: &i32,
  world_height: &i32,
  tile_frame_importances: &VariableTBitVec,
) -> Vec<Vec<Tile>> {
  let column_count: usize = (*world_height).try_into().unwrap();
  let row_count: usize = (*world_width).try_into().unwrap();
  let mut matrix: Vec<Vec<Tile>> = Vec::with_capacity(row_count);
  while matrix.len() < row_count {
    let mut column: Vec<Tile> = Vec::with_capacity(column_count);
    while column.len() < column_count {
      let (tile, repeat) =
        read_tile(buf, tile_frame_importances, offset).unwrap();
      for _ in 0..repeat {
        column.push(tile);
      }
    }
    matrix.push(column);
  }
  matrix
}

//  let mut offset = AtomicUsize::new(*offset);
//   (0..row_count)
//     .into_par_iter()
//     .map(|_| {
//       (0..column_count)
//         .into_par_iter()
//         .flat_map(|_| {
//           let mut tiles: Vec<Tile> = vec![];
//           let (tile, repeat) =
//             read_tile(buf, tile_frame_importances, offset).unwrap();
//           tiles.par_extend((0..repeat).into_par_iter().map(|_| tile));
//           tiles
//         })
//         .collect()
//     })
//     .collect_into_vec(&mut matrix);
