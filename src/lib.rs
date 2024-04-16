mod utils;

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
    alives: Vec<(isize, isize)>,
    camera: (isize, isize),
}

#[wasm_bindgen]
impl LifeOfGame {
    pub fn new(size: isize) -> Self {
        Self {
            alives: vec![],
            camera: (0, 0),
        }
    }

    pub fn toggle(&mut self, i: isize, j: isize) {
        if self.alives.contains(&(i, j)) {
            self.set(i, j, Cell::Dead)
        } else {
            self.set(i, j, Cell::Alive)
        }
    }
    fn set(&mut self, i: isize, j: isize, state: Cell) {
        if state == Cell::Dead {
            if self.alives.contains(&(i, j)) {
                self.alives.remove(
                    self.alives
                        .iter()
                        .enumerate()
                        .find(|x| x.1 == &(i, j))
                        .unwrap()
                        .0,
                );
            }
        } else {
            if self.alives.contains(&(i, j)) {
                return;
            }
            self.alives.push((i, j));
        }
    }
    pub fn move_camera(&mut self, x: isize, y: isize) {
        self.camera.0 += x;
        self.camera.1 += y;
    }
    pub fn draw(&self, width: isize, height: isize) {
        let doc = web_sys::window().unwrap().document().unwrap();
        let game = doc.get_element_by_id("game").unwrap();
        // alert(&format!("{:?} {:?}", self.camera, (width, height)));
        let mut html = String::new();
        for i in self.camera.1..(height + self.camera.1) {
            html.push_str("<div>");
            for j in self.camera.0..(width + self.camera.0) {
                html.push_str(&format!(
                    "<div type='button' id='{i},{j}' class='tile' {}></div>",
                    if self.alives.contains(&(i as isize, j as isize)) {
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
                    {
                        let mut count = 0;
                        for k in -1..=1 {
                            for l in -1..=1 {
                                let xx = x + k;
                                let yy = y + l;

                                count += self.alives.contains(&(xx as isize, yy as isize)) as i32;
                            }
                        }

                        if count == 3 {
                            changes.push((x as isize, y as isize, Cell::Alive));
                        }
                    }
                    count += self.alives.contains(&(x as isize, y as isize)) as i32;
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
}
