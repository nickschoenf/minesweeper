use std::sync::Arc;
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
            while random_tile != i && !v.contains(&random_tile) {
                random_tile = rand::random::<u32>() % (self.height * self.width);
            }
            v.push(random_tile);
        }
        for u in v {
            self.tiles[u as usize].is_mine = true;
        }

        // Initializing value of top left corner
        self.tiles[0_usize].value = 0;
        if self.tiles[1_usize].is_mine {
            self.tiles[0_usize].value += 1;
        }
        if self.tiles[self.width as usize].is_mine {
            self.tiles[0_usize].value += 1;
        }
        if self.tiles[self.width as usize + 1].is_mine {
            self.tiles[0_usize].value += 1;
        }

        // Initializing value of top right corner
        self.tiles[self.width as usize - 1].value = 0;
        if self.tiles[self.width as usize - 2].is_mine {
            self.tiles[self.width as usize - 1].value += 1;
        }
        if self.tiles[self.width as usize * 2 - 1].is_mine {
            self.tiles[self.width as usize - 1].value += 1;
        }
        if self.tiles[self.width as usize * 2 - 2].is_mine {
            self.tiles[self.width as usize - 1].value += 1;
        }

        // Initializing value of bottom left corner
        self.tiles[self.width as usize * (self.height as usize - 1)].value = 0;
        if self.tiles[self.width as usize * (self.height as usize - 2)].is_mine {
            self.tiles[self.width as usize * (self.height as usize - 1)].value += 1;
        }
        if self.tiles[self.width as usize * (self.height as usize - 2) + 1].is_mine {
            self.tiles[self.width as usize * (self.height as usize - 1)].value += 1;
        }
        if self.tiles[self.width as usize * (self.height as usize - 1) + 1].is_mine {
            self.tiles[self.width as usize * (self.height as usize - 1)].value += 1;
        }

        // Initializing value of bottom right corner
        self.tiles[self.width as usize * (self.height as usize) - 1].value = 0;
        if self.tiles[self.width as usize * (self.height as usize - 2)].is_mine {
            self.tiles[self.width as usize * (self.height as usize) - 1].value += 1;
        }
        if self.tiles[self.width as usize * (self.height as usize - 2) + 1].is_mine {
            self.tiles[self.width as usize * (self.height as usize) - 1].value += 1;
        }
        if self.tiles[self.width as usize * (self.height as usize) - 2].is_mine {
            self.tiles[self.width as usize * (self.height as usize) - 1].value += 1;
        }

        // Initializing values of top row
        for index in 1..self.width - 1 {
            if self.tiles[index as usize + 1].is_mine {
                self.tiles[index as usize].value += 1;
            }
            if self.tiles[self.width as usize * (self.height as usize) - 2].is_mine {
                self.tiles[self.width as usize * (self.height as usize) - 1].value += 1;
            }
        }

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
}

fn main() {
    let mut board = Board::new(16, 30);
    board.initialize(0, 99);
    for i in 0..16 {
        print!("[");
        for j in 0..30 {
            if board.tiles.get(i * board.width + j).unwrap() {

            }
            print!("M");
        }
        print!("]\n");
    }
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn fail() {
    println!("You lost!");
}