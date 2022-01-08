use std::vec::Vec;
use sdl2::render::WindowCanvas;
use sdl2::rect::{Rect};
use sdl2::pixels::Color;

pub struct Cellground {
    cells: Vec<Vec<bool>>,
    next_cells: Vec<Vec<bool>>,
    rows: usize,
    columns: usize,
}

impl Cellground {
    pub fn new(rows: usize, columns: usize) -> Self {
        let mut cells = Vec::with_capacity(rows);
        let mut next_cells = Vec::with_capacity(rows);
        for _ in 0..rows {
            cells.push(vec![false; columns]);
            next_cells.push(vec![false; columns]);
        }
        Self {
            cells,
            next_cells,
            rows,
            columns,
        }
    }

    pub fn set_cell(&mut self, row: usize, column: usize) {
        self.cells[row][column] = !self.cells[row][column];
    }

    pub fn clear_cells(&mut self) {
        let mut cells = Vec::with_capacity(self.rows);
        for _ in 0..self.rows {
            cells.push(vec![false; self.columns]);
        }
        self.cells = cells;
        self.next_cells = self.cells.clone();
    }

    pub fn next_gen(&mut self) {
        for x in 0..self.columns {
            for y in 0..self.rows {
                if self.cells[y][x] {
                    self.next_cells[y][x] = self.cells[y][x];
                    if self.get_neighbours_cnt(x, y) < 2 || self.get_neighbours_cnt(x, y) > 3 {
                        self.next_cells[y][x] = false;
                    }
                } else {
                    if self.get_neighbours_cnt(x, y) == 3 {
                        self.next_cells[y][x] = true;
                    }
                }
            }
        }
        self.cells = self.next_cells.clone();
    }

    fn get_neighbours_cnt(&self, x: usize, y: usize) -> i32 {
        let mut cnt :i32 = 0;
        if x == 0 && y == 0 {
            cnt += self.cells[1][1] as i32;
            cnt += self.cells[0][1] as i32;
            cnt += self.cells[1][0] as i32;
            return cnt;
        }

        if x == self.columns - 1 && y == self.rows - 1 {
            cnt += self.cells[self.rows - 2][self.columns - 2] as i32;
            cnt += self.cells[self.rows - 1][self.columns - 2] as i32;
            cnt += self.cells[self.rows - 2][self.columns - 1] as i32;
            return cnt;
        }

        if x == self.columns - 1 && y == 0 {
            cnt += self.cells[0][x - 1] as i32;
            cnt += self.cells[1][x - 1] as i32;
            cnt += self.cells[1][x] as i32;
            return cnt;
        }

        if x == 0 && y == self.rows - 1 {
            cnt += self.cells[y - 1][0] as i32;
            cnt += self.cells[y - 1][1] as i32;
            cnt += self.cells[y][1] as i32;
            return cnt;
        }

        if x == 0 && (y < self.rows - 1 || y > 0) {
            cnt += self.cells[y][1] as i32;
            cnt += self.cells[y - 1][1] as i32;
            cnt += self.cells[y + 1][1] as i32;
            cnt += self.cells[y - 1][0] as i32;
            cnt += self.cells[y + 1][0] as i32;
            return cnt;
        }

        if y == 0 && (x < self.columns - 1 || x > 0) {
            cnt += self.cells[1][x] as i32;
            cnt += self.cells[1][x - 1] as i32;
            cnt += self.cells[1][x + 1] as i32;
            cnt += self.cells[0][x - 1] as i32;
            cnt += self.cells[0][x + 1] as i32;
            return cnt;
        }

        if x == self.columns - 1 && (y < self.rows - 1 || y > 0) {
            cnt += self.cells[y][self.columns - 2] as i32;
            cnt += self.cells[y - 1][self.columns - 2] as i32;
            cnt += self.cells[y + 1][self.columns - 2] as i32;
            cnt += self.cells[y - 1][self.columns - 1] as i32;
            cnt += self.cells[y + 1][self.columns - 1] as i32;
            return cnt;
        }

        if y == self.rows - 1 && (x < self.columns - 1 || x > 0) {
            cnt += self.cells[self.rows - 2][x] as i32;
            cnt += self.cells[self.rows - 2][x - 1] as i32;
            cnt += self.cells[self.rows - 2][x + 1] as i32;
            cnt += self.cells[self.rows - 1][x - 1] as i32;
            cnt += self.cells[self.rows - 1][x + 1] as i32;
            return cnt;
        }

        cnt += self.cells[y][x - 1] as i32;
        cnt += self.cells[y - 1][x - 1] as i32;
        cnt += self.cells[y - 1][x] as i32;
        cnt += self.cells[y - 1][x + 1] as i32;
        cnt += self.cells[y][x + 1] as i32;
        cnt += self.cells[y + 1][x + 1] as i32;
        cnt += self.cells[y + 1][x] as i32;
        cnt += self.cells[y + 1][x - 1] as i32;
        cnt
    }
}

pub trait Renderable {
    fn render_thing(&self, _: &mut WindowCanvas);
}

impl Renderable for Cellground {
    fn render_thing(&self, canvas: &mut WindowCanvas) {
        for x in 0..self.columns {
            for y in 0..self.rows {
                if self.cells[y][x] {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.fill_rect(Rect::new((x as i32) * 10 + 1, (y as i32) * 10 + 1, 9, 9)).unwrap();
                } else {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                    canvas.fill_rect(Rect::new((x as i32) * 10 + 1, (y as i32) * 10 + 1, 9, 9)).unwrap();
                }
            }
        }
    }
}