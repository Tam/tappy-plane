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
use crate::obstacle::ObstaclePlugin;
use crate::physics::PhysicsPlugin;
use crate::scenes::ScenesPlugin;
use crate::shaders::ShadersPlugin;
use crate::sprite_animation::SpriteAnimationPlugin;
use crate::transitions::TransitionsPlugin;

const SCREEN_WIDTH : f32 = 800.;
const SCREEN_HEIGHT : f32 = 480.;

const Z_BACKGROUND : f32 = 0.;
const Z_OBSTACLE : f32 = 1.;
const Z_GROUND : f32 = 2.;
const Z_PLANE : f32 = 3.;
const Z_GAME_TEXT : f32 = 4.;
const Z_TRANSITION : f32 = 100.;

const GAME_IN_ANIM_COMPLETE : u64 = 1;
const GAME_OVER_ANIM_COMPLETE : u64 = 2;
const TRANSITION_START_COMPLETE : u64 = 3;
const TRANSITION_END_COMPLETE : u64 = 4;

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

fn main() {
	let mut app = App::new();
	
	app
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
	audio.play(music);
}

