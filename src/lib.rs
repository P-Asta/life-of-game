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
    cells: Vec<Vec<Cell>>,
}

#[wasm_bindgen]
impl LifeOfGame {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            cells: vec![vec![Cell::Dead; size]; size],
        }
    }

    pub fn toggle(&mut self, i: usize, j: usize) {
        self.cells[i][j].toggle();
    }
    pub fn draw(&self) {
        let doc = web_sys::window().unwrap().document().unwrap();
        let game = doc.get_element_by_id("game").unwrap();
        game.set_inner_html("");
        for i in 0..self.size {
            let mut html = format!("{}<div>", game.inner_html());
            for j in 0..self.size {
                html = format!(
                    "{}<div type='button' id='{i}-{j}' class='tile' {}></div>",
                    html,
                    if self.cells[i][j] == Cell::Alive {
                        "alive"
                    } else {
                        ""
                    },
                );
            }

            game.set_inner_html(&format!("{}</div>", html.clone()));
        }
    }
    pub fn step(&mut self) {
        let mut changes = vec![];
        for i in 0..self.size {
            for j in 0..self.size {
                let mut neighbor = 0;
                for k in 0..=2 {
                    for l in 0..=2 {
                        neighbor += self
                            .cells
                            .get(i + k - 1)
                            .unwrap_or(&Vec::new())
                            .get(j + l - 1)
                            .unwrap_or(&Cell::Dead)
                            .parse_int();
                    }
                    changes.push((
                        i,
                        j,
                        if neighbor == 3 {
                            Cell::Alive
                        } else {
                            Cell::Dead
                        },
                    ))
                }
            }
        }

        for data in changes {
            self.cells[data.0][data.1] = data.2;
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
