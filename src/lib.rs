mod minesweeper;

use minesweeper::*;
use wasm_bindgen::prelude::*;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref MINESWEEPER: Mutex<Minesweeper> = Mutex::new(Minesweeper::new(10, 10, 5));
}

#[wasm_bindgen(js_name = startGame)]
pub fn start_game(width: usize, height: usize, mines: usize) -> String {
    let mut ms = MINESWEEPER.lock().unwrap();
    *ms = Minesweeper::new(width, height, mines);
    ms.to_string()
}

#[wasm_bindgen(js_name = openField)]
pub fn open_fields(x: usize, y: usize) -> String {
    let mut ms = MINESWEEPER.lock().unwrap();
    (*ms).open((x, y));
    ms.to_string()
}

#[wasm_bindgen(js_name = toggleFlag)]
pub fn toggle_flag(x: usize, y: usize) -> String {
    let mut ms = MINESWEEPER.lock().unwrap();
    (*ms).toggle_flag((x, y));
    ms.to_string()
}