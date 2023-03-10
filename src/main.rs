mod assets;
mod animation;
mod physics;
#[cfg(feature = "debug")]
mod debug;

use bevy::prelude::*;
use crate::animation::{AnimationIndices, AnimationPlugin, AnimationTimer};
use crate::assets::{AssetsPlugin, SpriteSheet};
#[cfg(feature = "debug")]
use crate::debug::DebugPlugin;
use crate::physics::{Collider, PhysicsPlugin, Velocity};

const SCREEN_WIDTH : f32 = 800.;
const SCREEN_HEIGHT : f32 = 480.;

fn main() {
    let mut app = App::new();
    
    app
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tappy Plane".into(),
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(AssetsPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(PhysicsPlugin)
        .add_startup_system(setup)
    ;
    
    #[cfg(feature = "debug")]
    app.add_plugin(DebugPlugin);
    
    app.run();
}

fn setup (
    mut commands : Commands,
    sprite_sheet: Res<SpriteSheet>,
) {
    // Camera
    // =========================================================================
    
    commands.spawn(Camera2dBundle::default());
    
    // Sprites
    // =========================================================================
    
    // Background
    // -------------------------------------------------------------------------
    
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: sprite_sheet.handle.clone(),
            sprite: sprite_sheet.get("background"),
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        },
    ));
    
    // Ceiling Collider
    // -------------------------------------------------------------------------
    
    commands.spawn((
        Transform::from_xyz(0., SCREEN_HEIGHT * 0.5 + 15., 0.),
        GlobalTransform::default(),
        Collider(Vec2::new(SCREEN_WIDTH, 30.)),
    ));
    
    // Ground
    // -------------------------------------------------------------------------
    
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: sprite_sheet.handle.clone(),
            sprite: sprite_sheet.get("groundGrass"),
            transform: Transform::from_xyz(0., (SCREEN_HEIGHT - 71.) / 2. * -1., 1.),
            ..default()
        },
        Collider(Vec2::new(SCREEN_WIDTH, 30.)),
    ));
    
    // Plane
    // -------------------------------------------------------------------------
    
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: sprite_sheet.handle.clone(),
            sprite: sprite_sheet.get("planeBlue1"),
            transform: Transform::from_xyz(SCREEN_WIDTH * -0.2, 0., 2.),
            ..default()
        },
        AnimationIndices::new(vec![
            sprite_sheet.get("planeBlue1").index,
            sprite_sheet.get("planeBlue2").index,
            sprite_sheet.get("planeBlue3").index,
        ]),
        AnimationTimer(Timer::from_seconds(0.04, TimerMode::Repeating)),
        Velocity::default(),
        Collider(Vec2::new(88., 73.) * 0.6),
    ));
    
}
