use bevy::prelude::*;
use crate::sprite_animation::{SpriteAnimationIndices, SpriteAnimationTimer};
use crate::AppState;
use crate::assets::SpriteSheet;
use crate::transitions::{TransitionState, TransitionTo};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_system(menu_setup.in_schedule(OnEnter(AppState::Menu)))
			.add_system(menu_loop.in_set(OnUpdate(AppState::Menu)))
			.add_system(menu_teardown.in_schedule(OnExit(AppState::Menu)))
		;
	}
}

// Components
// =========================================================================

#[derive(Component)]
struct MenuRoot;

// Systems
// =========================================================================

fn menu_setup (
	mut commands : Commands,
	sprite_sheet : Res<SpriteSheet>,
) {
	commands.spawn((
		MenuRoot,
		Visibility::default(),
		ComputedVisibility::default(),
		Transform::default(),
		GlobalTransform::default(),
	))
	.with_children(|commands| {
		commands.spawn((
			SpriteSheetBundle {
				texture_atlas: sprite_sheet.handle.clone(),
				sprite: sprite_sheet.get("tap"),
				..default()
			},
			SpriteAnimationIndices::new(vec![
				sprite_sheet.get("tap").index,
				sprite_sheet.get("tapTick").index,
			]),
			SpriteAnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
		));
	});
}

fn menu_loop (
	mouse : Res<Input<MouseButton>>,
	touch : Res<Touches>,
	mut to_state : ResMut<TransitionTo<AppState>>,
	mut transition_state: ResMut<NextState<TransitionState>>,
) {
	if mouse.just_pressed(MouseButton::Left) || touch.any_just_pressed() {
		to_state.0 = Some(AppState::Game);
		transition_state.set(TransitionState::Start);
	}
}

fn menu_teardown (
	mut commands : Commands,
	query : Query<Entity, With<MenuRoot>>,
) {
	for entity in &query {
		commands.entity(entity).despawn_recursive();
	}
}
