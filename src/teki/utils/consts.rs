pub const BASE_VOLUME: f32 = 1.0 / 4.0;
pub const CHANNEL_COUNT: u32 = 3;

pub const PADDING: i32 = 10;

pub const WINDOW_WIDTH: i32 = 350;
pub const WINDOW_HEIGHT: i32 = 288;

pub const GAME_WIDTH: i32 = 200;
pub const GAME_HEIGHT: i32 = WINDOW_HEIGHT - (PADDING * 2);

pub const CENTER_X: i32 = GAME_WIDTH / 2;
pub const PLAYER_Y: i32 = GAME_HEIGHT - 16;

pub const APP_NAME: &str = "Teki";
pub const PLAYER_SPEED: i32 = 10;
pub const MYSHOT_SPEED: i32 = 6;
pub const FPS: u32 = 60;

pub const FONTS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/font.png");

pub const WATER_TEXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/water.png");

pub const NEKO_SPRITE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/neko.png");
pub const CORGI_SPRITE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/corgi.png");
pub const HEART_SPRITE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/heart.png");

pub const BUBBLE_SOUND: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/audio/bubble.wav");
pub const BG_LOOP: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/audio/8bit.mp3");
