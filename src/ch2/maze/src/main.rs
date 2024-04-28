use rand::Rng;

const MAP_N: usize = 25;

fn main() {
    let mut rng = rand::thread_rng(); // random number generater
    let mut maze = [[0; MAP_N]; MAP_N]; // init 2d array as 0

    // Initialize maze with walls
    for n in 0..MAP_N {
        maze[n][0] = 1;
        maze[0][n] = 1;
        maze[n][MAP_N - 1] = 1;
        maze[MAP_N - 1][n] = 1;
    }

    for y in 2..MAP_N - 2 {
        for x in 2..MAP_N - 2 {
            if x % 2 == 1 || y % 2 == 1 {
                continue;
            }
            maze[y][x] = 1; // wall

            let r = rng.gen_range(0..=3);
            match r {
                0 => maze[y - 1][x] = 1, // up
                1 => maze[y + 1][x] = 1, // down
                2 => maze[y][x - 1] = 1, // left
                3 => maze[y][x + 1] = 1, // right
                _ => {}
            }
        }
    }

    let tiles = ["  ", "ZZ"];
    for y in 0..MAP_N {
        for x in 0..MAP_N {
            print!("{}", tiles[maze[y][x]]);
        }
        println!("");
    }
}
