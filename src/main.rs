use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::input::mouse::MouseButton;
use bevy::input::ButtonInput;
use bevy::sprite::ColorMaterial;

struct Board {
    tiles: Vec<Tile>,
    height: u32,
    width: u32,
}

impl Board {
    pub fn new(h: u32, w: u32) -> Board {
        Board {
            tiles: vec![Tile {
                is_mine: false,
                is_covered: true,
                value: 0,
            }; (h * w).try_into().unwrap()],
            height: h,
            width: w,
        }
    }

    pub fn initialize(&mut self, i: u32, m: u32) {
        let mut v: Vec<u32> = Vec::new();
        for _ in 0..m {
            let mut random_tile = rand::random::<u32>() % (self.height * self.width);
            while random_tile == i || v.contains(&random_tile) {
                random_tile = rand::random::<u32>() % (self.height * self.width);
            }
            v.push(random_tile);
        }
        for u in v {
            self.tiles[u as usize].is_mine = true;

            if u >= self.width {
                if u % self.width != 0 {
                    self.tiles[u as usize - self.width as usize - 1].value += 1;
                }
                self.tiles[u as usize - self.width as usize].value += 1;
                if (u + 1) % self.width != 0 {
                    self.tiles[u as usize - self.width as usize + 1].value += 1;
                }
            }
            
            if u % self.width != 0 {
                self.tiles[u as usize as usize - 1].value += 1;
            }
            if (u + 1) % self.width != 0 {
                self.tiles[u as usize as usize + 1].value += 1;
            }
            
            if u < self.width * (self.height - 1) {
                if u % self.width != 0 {
                    self.tiles[u as usize + self.width as usize - 1].value += 1;
                }
                self.tiles[u as usize + self.width as usize].value += 1;
                if (u + 1) % self.width != 0 {
                    self.tiles[u as usize + self.width as usize + 1].value += 1;
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut output: String = String::new();
        let mut index: u32 = 0;
        for i in &self.tiles {
            match i.is_mine {
                true => output.push('M'),
                false => match i.value {
                    0 => output.push(' '),
                    1 => output.push('1'),
                    2 => output.push('2'),
                    3 => output.push('3'),
                    4 => output.push('4'),
                    5 => output.push('5'),
                    6 => output.push('6'),
                    7 => output.push('7'),
                    8 => output.push('8'),
                    _ => output.push('?'),
                }
            }
            if (index + 1) % self.width == 0 {
                output.push('\n');
            } else {
                output.push(' ');
            }
            index += 1;
        }
        output
    }
}

// Tile component
#[derive(Component)]
struct DisplayTile {
    x: u32,
    y: u32,
}

// Plugin to set up the game
pub struct TileGridPlugin;

impl Plugin for TileGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tiles)
            .add_systems(Update, tile_click_system);
    }
}

#[derive(Clone, Copy)]
struct Tile {
    is_mine: bool,
    is_covered: bool,
    value: u8,
}

impl Tile {
    pub fn uncover(&mut self) {
        self.is_covered = false;
        if self.is_mine {
            fail();
        }
    }

    pub fn get_value(&self) -> TileValue {
        match self.is_mine {
            true => TileValue::Mine,
            false => TileValue::Clear(self.value)
        }
    }
}

enum TileValue {
    Mine,
    Clear(u8),
}

fn main() {
    let mut board = Board::new(16, 30);
    board.initialize(0, 99);
    println!("{}", board.to_string());
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_tiles)
        .add_systems(Update, tile_click_system)
        .run();
}


// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn(Camera2dBundle::default());
//     commands.spawn(NodeBundle {
//         style: Style {
//             width: Val::Percent(100.),
//             ..default()
//         },
//         ..default()
//     })
//     .with_children(|root| {
//         // Text where we display current resolution
//         root.spawn((
//             ImageBundle {
//                 style: Style {
//                     ..default()
//                 },
//                 image: UiImage::new(asset_server.load("tile_covered.png")),
//                 ..default()
//             },
//         ));
//     });
// }

fn fail() {
    println!("You lost!");
}

fn spawn_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile_size = 64.0;
    let grid_width = 8;
    let grid_height = 8;

    for y in 0..grid_height {
        for x in 0..grid_width {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("tile_covered.png"),
                    transform: Transform::from_xyz(
                        x as f32 * tile_size,
                        y as f32 * tile_size,
                        0.0,
                    ),
                    ..default()
                },
                DisplayTile { x: x as u32, y: y as u32 },
            ));
        }
    }
}

fn tile_click_system(
    buttons: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut tile_query: Query<(Entity, &Transform, &DisplayTile, &mut Sprite)>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.single();
    
    if let Some(cursor_position) = window.cursor_position() {
        if buttons.just_pressed(MouseButton::Left) {
            let (camera, camera_transform) = camera_query.single();
            
            if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                let world_position = ray.origin.truncate();
                for (entity, transform, tile, mut sprite) in tile_query.iter_mut() {
                    let tile_pos = transform.translation.truncate();
                    let tile_size = 64.0;
                    let tile_min = tile_pos - Vec2::splat(tile_size / 2.0);
                    let tile_max = tile_pos + Vec2::splat(tile_size / 2.0);
                    
                    if world_position.cmpge(tile_min).all() && world_position.cmple(tile_max).all() {
                        println!("Clicked tile at ({}, {})", tile.x, tile.y);
                        // Change the tile color to red
                        sprite.color = Color::RED;
                    }
                }
            }
        }
    }
}