// Game components in Rust

struct Player {
    health: i32,
    position: (f32, f32),
}

struct Enemy {
    health: i32,
    position: (f32, f32),
}

struct Projectile {
    position: (f32, f32),
    direction: (f32, f32),
}

struct GameState {
    players: Vec<Player>,
    enemies: Vec<Enemy>,
    projectiles: Vec<Projectile>,
}