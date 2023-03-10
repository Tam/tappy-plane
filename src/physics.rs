use bevy::prelude::*;

const GRAVITY : f32 = -800.;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_system(input)
			.add_system(apply_velocity.after(input))
			.add_system(resolve_collisions.after(apply_velocity))
		;
	}
}

// Components
// =========================================================================

#[derive(Component, Default)]
pub struct Velocity (f32);

#[derive(Component)]
pub struct Collider (pub Vec2);

// Systems
// =========================================================================

fn input (
	mut query : Query<&mut Velocity>,
	keyboard : Res<Input<KeyCode>>,
) {
	if keyboard.just_pressed(KeyCode::Space) {
		query.single_mut().0 = 400.;
	}
}

fn apply_velocity (
	mut query : Query<(&mut Velocity, &mut Transform)>,
	time : Res<Time>,
) {
	for (mut velocity, mut transform) in query.iter_mut() {
		velocity.0 += GRAVITY * time.delta_seconds();
		transform.translation.y += velocity.0 * time.delta_seconds();
		let rot = 30. * if velocity.0 > 0. { 1.0_f32 } else { -1.0_f32 };
		transform.rotation = transform.rotation.slerp(
			Quat::from_rotation_z(rot.to_radians()),
			time.delta_seconds(),
		);
	}
}

fn resolve_collisions (
	mut player_query : Query<(&GlobalTransform, &Collider, &mut TextureAtlasSprite), With<Velocity>>,
	collider_query : Query<(&GlobalTransform, &Collider), Without<Velocity>>,
) {
	let (
		player_transform,
		player_collider,
		mut texture,
	) = player_query.single_mut();
	let half = player_collider.0 * 0.5;
	let pos = player_transform.translation().truncate();
	let player_min = pos - half;
	let player_max = pos + half;
	
	texture.color = Color::WHITE;
	
	for (transform, collider) in &collider_query {
		let half = collider.0 * 0.5;
		let pos = transform.translation().truncate();
		let min = pos - half;
		let max = pos + half;
		
		if aabb(player_min, player_max, min, max) {
			texture.color = Color::RED;
		}
	}
}

// Helpers
// =========================================================================

fn aabb (a_min : Vec2, a_max : Vec2, b_min : Vec2, b_max : Vec2) -> bool {
	   a_min.x < b_max.x
	&& a_max.x > b_min.x
	&& a_min.y < b_max.y
	&& a_max.y > b_min.y
}
