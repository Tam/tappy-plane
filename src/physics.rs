use bevy::prelude::*;
use crate::{AppState, GameState};
use crate::obstacle::ObstacleSpawner;
use crate::scenes::{DeathSpeed, GroundSpeed};

const GRAVITY : f32 = -800.;
const UP_AMOUNT: f32 = 300.;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(
				(
					input,
					apply_velocity.after(input),
					resolve_collisions.after(apply_velocity),
				).in_set(OnUpdate(AppState::Game))
				 .in_set(OnUpdate(GameState::Play))
			)
		;
	}
}

// Components
// =========================================================================

#[derive(Component, Default)]
pub struct Velocity (f32);

#[derive(Component)]
pub struct AABBCollider(pub Vec2, pub Option<Vec2>);

#[derive(Component)]
pub struct SATCollider (pub Vec<Vec2>);

// Systems
// =========================================================================

fn input (
	mut query : Query<&mut Velocity>,
	mouse : Res<Input<MouseButton>>,
	touch : Res<Touches>,
) {
	if mouse.just_pressed(MouseButton::Left) || touch.any_just_pressed() {
		query.single_mut().0 = UP_AMOUNT;
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
	mut player_query : Query<(&GlobalTransform, &AABBCollider), With<Velocity>>,
	aabb_collider_query : Query<(&GlobalTransform, &AABBCollider), Without<Velocity>>,
	sat_collider_query : Query<(&GlobalTransform, &SATCollider), Without<Velocity>>,
	spawner : Res<ObstacleSpawner>,
	ground_speed : Res<GroundSpeed>,
	mut state : ResMut<NextState<GameState>>,
	mut death_speed : ResMut<DeathSpeed>,
) {
	let (
		player_transform,
		player_collider,
	) = player_query.single_mut();
	let half = player_collider.0 * 0.5;
	let player_pos = player_transform.translation().truncate() + player_collider.1.unwrap_or(Vec2::ZERO);
	
	let player_min = player_pos - half;
	let player_max = player_pos + half;
	
	for (transform, collider) in &aabb_collider_query {
		let half = collider.0 * 0.5;
		let pos = transform.translation().truncate();
		let min = pos - half;
		let max = pos + half;
		
		if aabb(player_min, player_max, min, max) {
			death_speed.0 = ground_speed.0 * 0.8;
            state.set(GameState::Dead);
        }
	}
	
	let player_points = vec![
		player_min,
		Vec2::new(player_min.x, player_max.y),
		player_max,
		Vec2::new(player_max.x, player_min.y),
	];
	
	for (transform, collider) in &sat_collider_query {
		let t = transform.translation().truncate();
		let points : Vec<Vec2> = collider.0.clone().into_iter().map(|f| f + t).collect();
		
		if sat(&player_points, &points) {
			death_speed.0 = spawner.speed;
			state.set(GameState::Dead);
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

fn sat (
	a : &Vec<Vec2>,
	b : &Vec<Vec2>,
) -> bool {
	debug_assert!(a.len() > 2, "a must have at least 3 points");
	debug_assert!(b.len() > 2, "b must have at least 3 points");
	
	let mut poly_1 = a;
	let mut poly_2 = b;
	
	for i in 0..=1 {
		if i == 1 {
			poly_1 = b;
			poly_2 = a;
		}
		
		for a in 0..poly_1.len() {
			let b = (a + 1) % poly_1.len();
			
			let axis_proj = Vec2::new(
				-(poly_1[b].y - poly_1[a].y),
				poly_1[b].x - poly_1[a].x,
			);
			
			let mut min_r1 = f32::MAX;
			let mut max_r1 = f32::MIN;
			
			for p in poly_1 {
				let q = p.x * axis_proj.x + p.y * axis_proj.y;
				min_r1 = f32::min(min_r1, q);
				max_r1 = f32::max(max_r1, q);
			}
			
			let mut min_r2 = f32::MAX;
			let mut max_r2 = f32::MIN;
			
			for p in poly_2 {
				let q = p.x * axis_proj.x + p.y * axis_proj.y;
				min_r2 = f32::min(min_r2, q);
				max_r2 = f32::max(max_r2, q);
			}
			
			if !(max_r2 >= min_r1 && max_r1 >= min_r2) {
				return false;
			}
		}
	}
	
	true
}
