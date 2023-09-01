mod cell;
mod gameoflife;
use std::{thread::sleep, time::Duration};

use clap::Parser;

/// Conway's Game of Life
///
/// Rules:
///
/// 1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
/// 2. Any live cell with two or three live neighbours lives on to the next generation.
/// 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
/// 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
///
/// These rules, which compare the behaviour of the automaton to real life, can be condensed into the following:
/// 1. Any live cell with two or three live neighbours survives.
/// 2. Any dead cell with three live neighbours becomes a live cell.
/// 3. All other live cells die in the next generation. Similarly, all other dead cells stay dead.
///
/// Source: https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The width of the grid
    #[arg(long, default_value_t = 32)]
    width: u8,

    /// The height of the grid
    #[arg(long, default_value_t = 32)]
    height: u8,

    /// The time between updates in milliseconds
    #[arg(long, short, default_value_t = 3)]
    time_between_updates: u64,

    /// The starting cells that are alive on the first generation (gen-zero) in the format: x,y;x,y;x,y
    #[arg(long)]
    alive_starting_cells: Option<String>,

    /// Randomly generate the starting cells
    #[arg(short, long)]
    random: bool,
}

fn main() {
    let args = Args::parse();

    let mut alive_starting_cells: Vec<[u8; 2]> = Vec::new();

    if !args.random && args.alive_starting_cells.is_some() {
        alive_starting_cells = args
            .alive_starting_cells
            .unwrap()
            .split(';')
            .map(|cell| {
                let mut cell = cell.split(',');
                let x = cell.next().unwrap().parse::<u8>().unwrap();
                let y = cell.next().unwrap().parse::<u8>().unwrap();

                [x, y]
            })
            .collect::<Vec<[u8; 2]>>();
    } else {
        for y in 0..args.height {
            for x in 0..args.width {
                let alive = rand::random::<bool>();

                if alive {
                    alive_starting_cells.push([x, y]);
                }
            }
        }
    }

    let mut game = gameoflife::Game::new(args.width, args.height, alive_starting_cells);

    game.display();

    loop {
        sleep(Duration::from_millis(args.time_between_updates.into()));

        game.update();

        game.display();
    }
}
