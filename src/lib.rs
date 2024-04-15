mod utils;

use std::fmt::Display;

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}
impl Cell {
    pub fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        }
    }
    pub fn parse_int(&self) -> u8 {
        match self {
            Cell::Dead => 0,
            Cell::Alive => 1,
        }
    }
}

#[wasm_bindgen]
pub struct LifeOfGame {
    size: usize,
    alives: Vec<(usize, usize)>,
    cells: Vec<Vec<Cell>>,
}

#[wasm_bindgen]
impl LifeOfGame {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            alives: vec![],
            cells: vec![vec![Cell::Dead; size]; size],
        }
    }

    pub fn toggle(&mut self, i: usize, j: usize) {
        if self.cells[i][j] == Cell::Dead {
            self.set(i, j, Cell::Alive)
        } else {
            self.set(i, j, Cell::Dead)
        }
    }
    fn set(&mut self, i: usize, j: usize, state: Cell) {
        if state == Cell::Alive {
            if self.alives.contains(&(i, j)) {
                return;
            }
            self.alives.push((i, j));
            self.cells[i][j] = Cell::Alive;
        } else {
            self.alives.remove(
                self.alives
                    .iter()
                    .enumerate()
                    .find(|x| x.1 == &(i, j))
                    .unwrap()
                    .0,
            );
            self.cells[i][j] = Cell::Dead;
        }
        // alert(&format!("{:?}", self.alives))
    }
    pub fn draw(&self) {
        let doc = web_sys::window().unwrap().document().unwrap();
        let game = doc.get_element_by_id("game").unwrap();
        game.set_inner_html("");
        let mut html = String::new();
        for i in 0..self.size {
            html.push_str("<div>");
            for j in 0..self.size {
                html.push_str(&format!(
                    "<div type='button' id='{i}-{j}' class='tile' {}></div>",
                    if self.cells[i][j] == Cell::Alive {
                        "alive"
                    } else {
                        ""
                    }
                ));
            }
            html.push_str("</div>");
        }
        game.set_inner_html(&html);
    }
    pub fn step(&mut self) {
        let mut changes = vec![];
        for alive in &self.alives {
            let mut count = 0;
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    let x = alive.0 as i32 + i;
                    let y = alive.1 as i32 + j;
                    if x < 0 || y < 0 || x >= self.size as i32 || y >= self.size as i32 {
                        continue;
                    }
                    {
                        let mut count = 0;
                        for k in -1..=1 {
                            for l in -1..=1 {
                                let xx = x + k;
                                let yy = y + l;
                                if xx < 0
                                    || yy < 0
                                    || xx >= self.size as i32
                                    || yy >= self.size as i32
                                {
                                    continue;
                                }
                                count += self.cells[xx as usize][yy as usize].parse_int();
                            }
                        }

                        if count == 3 {
                            changes.push((x as usize, y as usize, Cell::Alive));
                        }
                    }
                    count += self.cells[x as usize][y as usize].parse_int();
                }
            }
            if 2 > count || count > 3 {
                changes.push((alive.0, alive.1, Cell::Dead));
            }
        }

        for data in &changes {
            self.set(data.0, data.1, data.2)
        }
    }
    pub fn render(&self) -> String {
        let mut result = String::new();
        for i in &self.cells {
            for j in i {
                match j {
                    Cell::Dead => result.push_str("◻️"),
                    Cell::Alive => result.push_str("◼️"),
                }
            }
            result.push_str("\n");
        }
        result
    }
}
