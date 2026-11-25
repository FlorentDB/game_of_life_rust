/// Desktop entry point.
use life_game_rust::{Grid, life_cycle};
use minifb::{Key, Window, WindowOptions};
use std::thread;
use std::time::Duration;

fn main() {
    let width = 1000;
    let height = 600;
    let probability = 0.4;

    let mut grid = Grid::init_grid(width, height, probability);
    let mut buffer: Vec<u32> = grid.array.iter().map(|p| p.to_color()).collect();

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
