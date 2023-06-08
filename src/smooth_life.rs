use nannou::prelude::*;
use rand::Rng;
use rayon::prelude::*;

// Constants
pub const WIDTH: usize = 500;
pub const HEIGHT: usize = 500;
const RA: isize = 11;
const RI: isize = RA / 3;
const ALPHA_N: Float = 0.028;
const ALPHA_M: Float = 0.147;
const B1: Float = 0.278;
const B2: Float = 0.365;
const D1: Float = 0.267;
const D2: Float = 0.445;
const DT: Float = 0.5;

// Type aliasses
pub type Float = f32;
pub type Grid = Vec<Vec<Float>>;

/// Generate a random grid
pub fn rand_grid() -> Grid {
    let mut grid = vec![vec![0.0; WIDTH]; HEIGHT];
    let mut rng = rand::thread_rng();

    // Create noise blobs aXb pixels all over the grid ranmom places but when generated 2 must overlap 30% of the area
    let a = (WIDTH as Float * 0.1) as usize;
    let b = (HEIGHT as Float * 0.1) as usize;
    let area = a * b;
    let mut blobs: Vec<(usize, usize)> = Vec::new();
    let mut i = 0;

    while i < 2 {
        let x = rng.gen_range(0..WIDTH - a);
        let y = rng.gen_range(0..HEIGHT - b);

        let mut overlap = false;
        for blob in &blobs {
            let x1 = x.max(blob.0);
            let y1 = y.max(blob.1);
            let x2 = (x + a).min(blob.0 + a);
            let y2 = (y + b).min(blob.1 + b);

            if (x1 < x2) && (y1 < y2) {
                let overlap_area = (x2 - x1) * (y2 - y1);
                if overlap_area as Float > area as Float * 0.3 {
                    overlap = true;
                    break;
                }
            }
        }

        if !overlap {
            blobs.push((x, y));
            i += 1;
        }

        // Add the blob to the grid
        for y in y..y + b {
            for x in x..x + a {
                grid[y][x] = rng.gen_range(0.0..1.0);
            }
        }
    }

    grid
}

pub fn compute_grid_diff(grid: &Grid) -> Grid {
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

pub fn update_grid(grid: &mut Grid, grid_diff: &Grid) {
    grid.par_iter_mut().enumerate().for_each(|(y, row)| {
        for x in 0..WIDTH {
            row[x] += grid_diff[y][x] * DT;
            clamp(&mut row[x], 0.0, 1.0);
        }
    });
}

fn get_m_n(x: usize, y: usize, grid: &Grid) -> (Float, Float) {
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

pub fn get_color(value: Float) -> Rgb {
    let value = value * 255.0;
    Rgb::new(value, value, value)
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
