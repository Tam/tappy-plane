mod assets;
mod sprite_animation;
mod physics;
#[cfg(feature = "debug")]
mod debug;
mod shaders;
mod obstacle;
mod scenes;
mod transitions;

use bevy::prelude::*;
use bevy_tweening::TweeningPlugin;
use crate::assets::AssetsPlugin;
#[cfg(feature = "debug")]
use crate::debug::DebugPlugin;
use crate::obstacle::{ObstaclePlugin, ObstacleSpawner};
use crate::physics::PhysicsPlugin;
use crate::scenes::ScenesPlugin;
use crate::shaders::ShadersPlugin;
use crate::sprite_animation::SpriteAnimationPlugin;
use crate::transitions::TransitionsPlugin;

// Constants
// =========================================================================

// Screen size
// -------------------------------------------------------------------------

const SCREEN_WIDTH  : f32 = 800.;
const SCREEN_HEIGHT : f32 = 480.;

// Z-indexes
// -------------------------------------------------------------------------

pub mod z {
	pub const BACKGROUND : f32 = 0.;
	pub const OBSTACLE   : f32 = 1.;
	pub const GROUND     : f32 = 2.5;
	pub const PLANE      : f32 = 3.;
	pub const GAME_TEXT  : f32 = 4.;
	pub const TRANSITION : f32 = 100.;
}

// Tween Events
// -------------------------------------------------------------------------

const GAME_IN_ANIM_COMPLETE     : u64 = 1;
const GAME_OVER_ANIM_COMPLETE   : u64 = 2;
const TRANSITION_START_COMPLETE : u64 = 3;
const TRANSITION_END_COMPLETE   : u64 = 4;

// States
// =========================================================================

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
	#[default]
	Menu,
	Game,
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
	#[default]
	PreEnter,
	Enter,
	Play,
	Exit,
	Dead,
}

// Structs
// =========================================================================

pub enum LevelTheme {
	Grass,
	Stone,
	Snow,
}

#[derive(Resource)]
pub struct Level {
	pub theme    : LevelTheme,
	pub distance : f32, // Distance the player needs to travel to "complete" the level
	pub spawner  : ObstacleSpawner,
}

// Game
// =========================================================================

fn main() {
	let mut app = App::new();
	
	app
		.insert_resource(Level {
			theme: LevelTheme::Grass,
			distance: 100.,
			spawner: ObstacleSpawner {
				speed: 150.,
				timer: Timer::from_seconds(2., TimerMode::Repeating),
				gap_min: 150.,
				gap_max: 200.,
			},
		})
		.add_state::<AppState>()
		.add_state::<GameState>()
		.insert_resource(ClearColor(Color::hex("#D9ECF6").unwrap()))
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "Tappy Plane".into(),
				resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
				canvas: Some("#canvas".into()),
				..default()
			}),
			..default()
		}))
		.add_plugin(TweeningPlugin)
		.add_plugin(ShadersPlugin)
		.add_plugin(AssetsPlugin)
		.add_plugin(TransitionsPlugin)
		.add_plugin(SpriteAnimationPlugin)
		.add_plugin(PhysicsPlugin)
		.add_plugin(ObstaclePlugin)
		.add_plugin(ScenesPlugin)
		.add_system(setup.on_startup())
	;
	
	#[cfg(feature = "debug")]
	app.add_plugin(DebugPlugin);
	
	app.run();
}

fn setup (
	mut commands : Commands,
	asset_server : Res<AssetServer>,
	audio : Res<Audio>
) {
	// Camera
	commands.spawn(Camera2dBundle::default());
	
	// Bangin' tunes
	let music = asset_server.load("audio/Bavarian Goat.ogg");
	audio.play_with_settings(music, PlaybackSettings::LOOP);
}

