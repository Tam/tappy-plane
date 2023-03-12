mod assets;
mod animation;
mod physics;
#[cfg(feature = "debug")]
mod debug;
mod shaders;
mod obstacle;
mod scenes;

use bevy::prelude::*;
use crate::animation::AnimationPlugin;
use crate::assets::AssetsPlugin;
#[cfg(feature = "debug")]
use crate::debug::DebugPlugin;
use crate::obstacle::ObstaclePlugin;
use crate::physics::PhysicsPlugin;
use crate::scenes::ScenesPlugin;
use crate::shaders::ShadersPlugin;

const SCREEN_WIDTH : f32 = 800.;
const SCREEN_HEIGHT : f32 = 480.;

const Z_BACKGROUND : f32 = 0.;
const Z_OBSTACLE : f32 = 1.;
const Z_GROUND : f32 = 2.;
const Z_PLANE : f32 = 3.;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash, States)]
enum AppState {
	#[default]
	Menu,
	Game,
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash, States)]
enum GameState {
	#[default]
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
				..default()
			}),
			..default()
		}))
		.add_plugin(ShadersPlugin)
		.add_plugin(AssetsPlugin)
		.add_plugin(AnimationPlugin)
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
) {
	commands.spawn(Camera2dBundle::default());
}

