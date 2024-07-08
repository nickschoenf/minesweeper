use bevy::prelude::*;

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
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            ..default()
        },
        ..default()
    })
    .with_children(|root| {
        // Text where we display current resolution
        root.spawn((
            ImageBundle {
                style: Style {
                    ..default()
                },
                image: UiImage::new(asset_server.load("tile_covered.png")),
                ..default()
            },
        ));
    });
}

fn fail() {
    println!("You lost!");
}