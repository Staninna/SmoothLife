use rand::Rng;
use rayon::prelude::*;

// Constants
const WIDTH: usize = 200;
const HEIGHT: usize = 200;
const RA: isize = 11;
const RI: isize = RA / 3;
const ALPHA_N: Float = 0.028;
const ALPHA_M: Float = 0.147;
const B1: Float = 0.278;
const B2: Float = 0.365;
const D1: Float = 0.267;
const D2: Float = 0.445;
const DT: Float = 0.1;

// Type aliasses
type Float = f32;

/// Generate a random grid
fn rand_grid() -> Vec<Vec<Float>> {
    let mut grid = vec![vec![0.0; WIDTH]; HEIGHT];
    let mut rng = rand::thread_rng();

    // Create noise in the grid

    // Center noice
    for y in HEIGHT / 2..HEIGHT {
        for x in WIDTH / 2..WIDTH {
            grid[y][x] = rng.gen();
        }
    }

    // Smaller noise blob in the center but offsetted bij offset to +x
    // let offset = 100;
    // let r = 40;
    // for y in 0..HEIGHT {
    //     for x in 0..WIDTH {
    //         let dx = x as isize - WIDTH as isize / 2 + offset;
    //         let dy = y as isize - HEIGHT as isize / 2;
    //         let d = dx * dx + dy * dy;
    //         if d < r * r {
    //             grid[y][x] = rng.gen();
    //         }
    //     }
    // }

    grid
}

/// Display the grid
#[allow(dead_code)]
fn display_grid(grid: &Vec<Vec<Float>>) {
    let gradient = [" ", ".", ":", "-", "=", "+", "*", "#", "%", "@"];

    let mut buffer = String::new();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let index = (grid[y][x] * (gradient.len() - 1) as Float) as usize;
            buffer.push_str(gradient[index]);
            buffer.push_str(gradient[index]);
        }
        buffer.push('\n');
    }

    // Move to the top left corner
    print!("\x1B[1;1H");

    print!("{}", buffer);
}

fn main() {
    let mut grid = rand_grid();

    // Clear the screen
    print!("\x1B[2J\x1B[1;1H");

    display_grid(&grid);
    loop {
        let grid_diff = compute_grid_diff(&grid);
        grid.par_iter_mut().enumerate().for_each(|(y, row)| {
            for x in 0..WIDTH {
                row[x] += grid_diff[y][x] * DT;
                clamp(&mut row[x], 0.0, 1.0);
            }
        });

        display_grid(&grid);
    }
}

fn compute_grid_diff(grid: &Vec<Vec<Float>>) -> Vec<Vec<Float>> {
    let mut grid_diff = vec![vec![0.0; WIDTH]; HEIGHT];

    grid_diff.par_iter_mut().enumerate().for_each(|(cy, row)| {
        row.iter_mut().enumerate().for_each(|(cx, cell)| {
            let (m, n) = get_m_n(cx, cy, &grid);
            let s = s(n, m);
            *cell = 2.0 * s - 1.0;
        });
    });

    grid_diff
}
fn get_m_n(x: usize, y: usize, grid: &Vec<Vec<Float>>) -> (Float, Float) {
    let mut m: Float = 0.0;
    let mut n: Float = 0.0;
    let mut m_area: usize = 0;
    let mut n_area: usize = 0;

    // Loop through the cells in the radius
    for dy in -(RA - 1)..=RA - 1 {
        for dx in -(RA - 1)..=RA - 1 {
            // Get the coordinates of the cell
            let x = emod(x as isize + dx, WIDTH as isize);
            let y = emod(y as isize + dy, HEIGHT as isize);

            // Distance from the center
            let d = (dx * dx + dy * dy) as Float;

            // Inner circle
            if d < (RI * RI) as Float {
                m += grid[y as usize][x as usize];
                m_area += 1;
            }
            // Outer circle
            else if d < (RA * RA) as Float {
                n += grid[y as usize][x as usize];
                n_area += 1;
            }
        }
    }

    // Normilize m and n
    m /= m_area as Float;
    n /= n_area as Float;

    (m, n)
}

fn sigma(x: Float, a: Float, alpha: Float) -> Float {
    1.0 / (1.0 + (-(x - a) * 4.0 / alpha).exp())
}

fn sigma_n(x: Float, a: Float, b: Float) -> Float {
    sigma(x, a, ALPHA_N) * (1.0 - sigma(x, b, ALPHA_N))
}

fn sigma_m(x: Float, y: Float, m: Float) -> Float {
    x * (1.0 - sigma(m, 0.5, ALPHA_M)) + y * sigma(m, 0.5, ALPHA_M)
}

fn s(n: Float, m: Float) -> Float {
    sigma_n(n, sigma_m(B1, D1, m), sigma_m(B2, D2, m))
}

/// Euclidean modulo
fn emod(a: isize, b: isize) -> isize {
    (a % b + b) % b
}

// Clamp a value between a minimum and a maximum using pointers
fn clamp(value: &mut Float, min: Float, max: Float) {
    if *value < min {
        *value = min;
    } else if *value > max {
        *value = max;
    }
}
