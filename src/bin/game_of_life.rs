use futures::stream::Collect;
use rand::Rng;
pub struct GameOfLife {
    width: usize,
    height: usize,
    grid: Vec<Vec<bool>>,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![false; width]; height];
        let mut rng = rand::thread_rng();
        let test: Vec<Vec<bool>> = (0..height)
            .map(|_| (0..width).map(|_| rng.gen_bool(0.2)).collect())
            .collect();

        Self {
            width,
            height,
            grid: test,
        }
    }

    pub fn toggle_cell(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.grid[y][x] = !self.grid[y][x];
        }
    }

    pub fn next_generation(&mut self) {
        let mut new_grid = vec![vec![false; self.width]; self.height];

        for y in 0..self.height {
            for x in 0..self.width {
                let alive_neighbors = self.alive_neighbors(x, y);
                let is_alive = self.grid[y][x];

                new_grid[y][x] = match (is_alive, alive_neighbors) {
                    (true, 2) | (_, 3) => true,
                    _ => false,
                };
            }
        }

        self.grid = new_grid;
    }

    fn alive_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = (x as isize + dx) as usize;
                let ny = (y as isize + dy) as usize;

                if nx < self.width && ny < self.height && self.grid[ny][nx] {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn get_grid(&self) -> &Vec<Vec<bool>> {
        &self.grid
    }
}
