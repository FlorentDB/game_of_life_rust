/// Inspired by https://blog.adamchalmers.com/grids-1/#defining-the-grid-trait and https://medium.com/@sunside/the-game-of-life-vs-convolutions-bc495c962de8

use minifb::{Key, Window, WindowOptions};
use rand::RngExt;
use std::thread;
use std::time::Duration;

/// An enum to store the pixel to_color
#[derive(Clone, Copy, Default)]
enum Cell {
    Alive,
    #[default]
    Dead,
}

impl Cell {

    fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }

    fn to_color(&self) -> u32 {
        match self {
            Cell::Alive => Cell::from_u8_rgb(0, 59, 0),
            Cell::Dead => Cell::from_u8_rgb(0, 0, 0),
        }
    }
}


/// A container which stores elements (pixels) in a grid
struct Grid<T> {
    width: usize,
    height: usize,
    array : Vec<T>,
}

/// Constructor of the grid (1D vec)
impl Grid<Cell> {
    fn init_grid(width: usize, height: usize, probability: f64) -> Self {
        let mut rng = rand::rng();
        let array = (0..width * height)
            .map(|_| if rng.random_bool(probability) { Cell::Alive } else {Cell::Dead })
            .collect();
        Self { array, width, height}
    }

    fn create_grid(width: usize, height: usize) -> Self {
        Self {
            array: vec![Cell::default(); width * height],
            width,
            height,
        }
    }
}

/// Iteration and transforamtion of the grid
/// Rule 1: living cell with < 2 neighbours dies
/// Rule 2: living cell with 2 or 3 neighbours lives
/// Rule 3: living cell with > 3 neighbours dies
/// Rule 4: dead cell with 3 neighbours becomes alive
fn life_cycle(grid: Grid<Cell>) -> Grid<Cell> {

    let width = grid.width;
    let height = grid.height;

    let mut new_grid: Grid<Cell> = Grid::create_grid(width, height);
    
    let width = grid.width as isize;
    let height = grid.height as isize;
    let kernel = [(-1isize, -1isize), (0, -1), (1, -1), (-1, 0),(1, 0),(-1, 1), (0, 1), (1, 1),];
    

    for i in 0..grid.array.len() {
        let x = (i % width as usize) as isize;
        let y = (i / width as usize) as isize;

        let mut neighbours = 0;
        
        for (dx, dy) in kernel {
            let nx = (x + dx).rem_euclid(width);
            let ny = (y + dy).rem_euclid(height);
            let n = ny * width + nx;
            
        if matches!(grid.array[n as usize], Cell::Alive) { neighbours += 1}
        }

        new_grid.array[i] = match (grid.array[i], neighbours) {
            (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive, // Rule 2
            (Cell::Dead, 3) => Cell::Alive, // Rule 4
            _ => Cell::Dead, // Rule 1, 3
        }
    }
    new_grid

}

fn main() {

    let width = 1000;
    let height = 600;
    let probability = 0.4;

    let mut grid: Grid<Cell> = Grid::init_grid(width, height, probability);
    let mut buffer : Vec<u32> = grid.array.iter().map(|p| p.to_color()).collect();

    let mut window = match Window::new("Life Game", width, height, WindowOptions::default()) {
        Ok(win) => win,
        Err(err) => {
        println!("Unable to create window {}", err);
        return;
        }
    };

    window.update_with_buffer(&buffer, width, height).unwrap(); 

    while window.is_open() && !window.is_key_down(Key::Escape) {
        grid = life_cycle(grid);
        buffer = grid.array.iter().map(|p| p.to_color()).collect(); 
        window.update_with_buffer(&buffer, width, height).unwrap();
        thread::sleep(Duration::from_millis(10));
            
    }
}
