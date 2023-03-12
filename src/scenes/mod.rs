use bevy::app::App;
use bevy::prelude::Plugin;
use crate::scenes::game::GamePlugin;
use crate::scenes::menu::MenuPlugin;

mod menu;
mod game;

pub use game::*;

pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugin(MenuPlugin)
			.add_plugin(GamePlugin)
		;
	}
}
