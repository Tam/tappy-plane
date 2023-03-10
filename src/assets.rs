use bevy::prelude::*;
use bevy::utils::HashMap;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(SpriteSheet::default())
			.add_startup_system(load_sprite_sheet)
		;
	}
}

// Resources
// =========================================================================

#[derive(Resource, Default)]
pub struct SpriteSheet {
	pub sprites : HashMap<String, usize>,
	pub handle : Handle<TextureAtlas>,
}

impl SpriteSheet {
	pub fn get (&self, name : &str) -> TextureAtlasSprite {
		TextureAtlasSprite::new(*self.sprites.get(name).unwrap())
	}
}

// Systems
// =========================================================================

fn load_sprite_sheet (
	asset_server : Res<AssetServer>,
	mut texture_atlases : ResMut<Assets<TextureAtlas>>,
	mut spritesheet : ResMut<SpriteSheet>,
) {
	let texture_handle = asset_server.load("sheet.png");
	let mut texture_atlas = TextureAtlas::new_empty(
		texture_handle,
		Vec2::new(1024., 2048.),
	);
	
	let mut sprites = HashMap::new();
	
	let mut add = |
		name : &str,
		x : f32,
		y : f32,
		width : f32,
		height : f32,
	| {
		sprites.insert(name.to_string(), texture_atlas.add_texture(Rect {
			min: Vec2::new(x, y),
			max: Vec2::new(x + width, y + height),
		}));
	};
	
	add("UIbg", 0., 986., 264., 264.);
	add("background", 0., 355., 800., 480.);
	add("buttonLarge", 0., 1250., 196., 70.);
	add("buttonSmall", 0., 1320., 136., 80.);
	add("groundDirt", 0., 0., 808., 71.);
	add("groundGrass", 0., 142.3, 808., 71.);
	add("groundIce", 0., 71., 808., 71.);
	add("groundRock", 0., 284., 808., 71.);
	add("groundSnow", 0., 213., 808., 71.);
	add("letterA", 412., 835., 61., 64.);
	add("letterB", 487., 1537., 50., 66.);
	add("letterC", 460., 977., 52., 66.);
	add("letterD", 432., 1613., 54., 66.);
	add("letterE", 511., 1965., 45., 64.);
	add("letterF", 512., 963., 44., 64.);
	add("letterG", 460., 1107., 52., 66.);
	add("letterH", 473., 835., 51., 64.);
	add("letterI", 524., 835., 22., 64.);
	add("letterJ", 512., 1027., 42., 66.);
	add("letterK", 432., 1821., 53., 64.);
	add("letterL", 512., 899., 44., 64.);
	add("letterM", 392., 1967., 66., 64.);
	add("letterN", 432., 1679., 53., 64.);
	add("letterO", 418., 1284., 60., 66.);
	add("letterP", 489., 1427., 48., 65.);
	add("letterQ", 418., 1205., 60., 79.);
	add("letterR", 478., 1249., 51., 65.);
	add("letterS", 511., 1899., 46., 66.);
	add("letterT", 460., 1043., 52., 64.);
	add("letterU", 485., 1757., 51., 66.);
	add("letterV", 400., 913., 61., 64.);
	add("letterW", 136., 1320., 76., 64.);
	add("letterX", 418., 1409., 58., 64.);
	add("letterY", 432., 1473., 57., 64.);
	add("letterZ", 486., 1613., 50., 64.);
	add("medalBronze", 0., 1400., 114., 119.);
	add("medalGold", 0., 1519., 114., 119.);
	add("medalSilver", 0., 1638., 114., 119.);
	add("number0", 432., 1743., 53., 78.);
	add("number1", 512., 1093., 37., 76.);
	add("number2", 477., 1350., 51., 77.);
	add("number3", 485., 1679., 51., 78.);
	add("number4", 432., 1537., 55., 76.);
	add("number5", 485., 1823., 50., 76.);
	add("number6", 432., 1885., 53., 77.);
	add("number7", 478., 1173., 51., 76.);
	add("number8", 461., 899., 51., 78.);
	add("number9", 458., 1962., 53., 77.);
	add("planeBlue1", 330., 1371., 88. - 0.4, 73. - 0.4);
	add("planeBlue2", 372., 1132., 88. - 0.4, 73. - 0.4);
	add("planeBlue3", 222., 1562., 88. - 0.4, 73. - 0.4);
	add("planeGreen1", 114., 1639., 88., 73.);
	add("planeGreen2", 216., 1951., 88., 73.);
	add("planeGreen3", 222., 1489., 88., 73.);
	add("planeRed1", 216., 1878., 88., 73.);
	add("planeRed2", 372., 1059., 88., 73.);
	add("planeRed3", 372., 986., 88., 73.);
	add("planeYellow1", 304., 1967., 88., 73.);
	add("planeYellow2", 330., 1298., 88., 73.);
	add("planeYellow3", 330., 1225., 88., 73.);
	add("puffLarge", 114., 1712., 42., 35.);
	add("puffSmall", 196., 1250., 25., 21.);
	add("rock", 114., 1400., 108., 239.);
	add("rockDown", 324., 1489., 108., 239.);
	add("rockGrass", 0., 1757., 108., 239.);
	add("rockGrassDown", 264., 986., 108., 239.);
	add("rockIce", 216., 1639., 108., 239.);
	add("rockIceDown", 222., 1250., 108., 239.);
	add("rockSnow", 324., 1728., 108., 239.);
	add("rockSnowDown", 108., 1757., 108., 239.);
	add("starBronze", 170., 1996., 39., 37.);
	add("starGold", 369., 1444., 39., 37.);
	add("starSilver", 330., 1444., 39., 37.);
	add("tap", 156., 1712., 40., 40.);
	add("tapLeft", 0., 1996., 85., 42.);
	add("tapRight", 85., 1996., 85., 42.);
	add("tapTick", 418., 1350., 59., 59.);
	add("textGameOver", 0., 835., 412., 78.);
	add("textGetReady", 0., 913., 400., 73.);
	
	let texture_atlas_handle = texture_atlases.add(texture_atlas);
	
	spritesheet.sprites = sprites;
	spritesheet.handle = texture_atlas_handle;
}
