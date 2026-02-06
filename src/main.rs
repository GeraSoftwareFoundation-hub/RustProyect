use bevy::prelude::*;

struct Player;
struct Enemy;
struct Score(u32);
struct GameOver(bool);

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "2D Game".to_string(),
            ..Default::default()
        })
        .insert_resource(Gravity::default())
        .add_startup_system(setup.system())
        .add_system(player_movement.system())
        .add_system(enemy_spawn.system())
        .add_system(collision_detection.system())
        .add_system(game_over.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
        ..Default::default()
    }).insert(Player);
    commands.insert_resource(Score(0));
    commands.insert_resource(GameOver(false));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>, 
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (_, mut transform) in query.iter_mut() {
        let mut direction = Vec3::zero();
        if keyboard_input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }
        transform.translation += direction.normalize() * 2.0; // move speed
    }
}

fn enemy_spawn(mut commands: Commands, time: Res<Time>, mut timer: ResMut<Timer>, mut score: ResMut<Score>) {
    timer.tick(time.delta());
    if timer.finished() {
        commands.spawn_bundle(SpriteBundle {
            material: Color::rgb(1.0, 0.0, 0.0).into(),
            transform: Transform::from_translation(Vec3::new(random::<f32>() * 800.0 - 400.0, random::<f32>() * 600.0 - 300.0, 0.0)),
            ..Default::default()
        }).insert(Enemy);
        score.0 += 1; // increase score for each enemy spawned
    }
}

fn collision_detection(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>, 
    enemy_query: Query<(Entity, &Transform), With<Enemy>>, 
    mut game_over: ResMut<GameOver>,
) {
    for (player_entity, player_transform) in player_query.iter() {
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            if player_transform.translation.distance(enemy_transform.translation) < 50.0 { // collision radius
                println!("Game Over!");
                game_over.0 = true;
                commands.entity(player_entity).despawn(); // remove player
            }
        }
    }
}

fn game_over(mut commands: Commands, game_over: Res<GameOver>, score: Res<Score>) {
    if game_over.0 {
        // Clean up everything and show score
        println!("Final Score: {}", score.0);
        // Here you could add logic to restart the game or exit
    }
}