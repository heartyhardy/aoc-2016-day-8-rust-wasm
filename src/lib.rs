mod data;
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Pixel {
    Off = 0,
    On = 1,
}

#[wasm_bindgen]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TinyLCD {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

#[wasm_bindgen]
impl TinyLCD {
    // Gets the width of the LCD
    pub fn width(&self) -> u32 {
        self.width
    }

    // Gets the height of the LCD
    pub fn height(&self) -> u32 {
        self.height
    }

    // Gets the LCD matrix as u8 liner array
    pub fn pixels(&self) -> *const Pixel {
        self.pixels.as_ptr()
    }

    //Get no of instructions
    pub fn instructions_count(&self) -> u32 {
        let instructions_list = data::get_instructions();
        let instructions: Vec<&str> = instructions_list.lines().collect();
        instructions.len() as u32
    }

    // Create New LCD
    pub fn new() -> TinyLCD {
        let width = 50;
        let height = 6;

        let pixels = (0..width * height).map(|_i| Pixel::Off).collect();

        TinyLCD {
            width,
            height,
            pixels,
        }
    }

    // Process each instruction
    pub fn on_instruction_recieved(&mut self, next: usize) {
        let instructions_list = data::get_instructions();
        let instructions: Vec<&str> = instructions_list.lines().collect();

        if next >= instructions_list.len() {
            self.reset_pixels();
        }

        let instruction = instructions[next % instructions.len()];

        let params: Vec<&str> = instruction.split_whitespace().collect();

        match params[0] {
            "rect" => {
                let ops: Vec<u32> = params[1].split("x").map(|i| i.parse().unwrap()).collect();
                for r in 0..ops[1] {
                    for c in 0..ops[0] {
                        let idx = self.get_index(r, c);
                        self.pixels[idx] = Pixel::On;
                    }
                }
            }
            "rotate" => match params[1] {
                "row" => {
                    let r: u32 = params[2][2..].parse().unwrap();
                    let offset: u32 = params[4].parse().unwrap();
                    let row = self.copy_row(r);

                    self.turn_off_row(r);

                    for c in 0..self.width {
                        let idx = self.get_index(r, c + offset) % self.width as usize;
                        self.pixels[(r * self.width) as usize + idx] = row[c as usize];
                    }
                }
                "column" => {
                    let c: u32 = params[2][2..].parse().unwrap();
                    let offset: u32 = params[4].parse().unwrap();
                    let column = self.copy_column(c);

                    self.turn_off_column(c);

                    for r in 0..self.height {
                        let idx =
                            self.get_index(r + offset, c) % (self.width * self.height) as usize;
                        self.pixels[idx] = column[r as usize];
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }

    //Get Index by given row and column
    pub fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    // Copy the entire row
    fn copy_row(&self, r: u32) -> Vec<Pixel> {
        let mut row: Vec<Pixel> = Vec::new();
        for c in 0..self.width {
            let idx = self.get_index(r, c);
            row.push(self.pixels[idx]);
        }
        row
    }

    // Copy entire column
    fn copy_column(&self, col: u32) -> Vec<Pixel> {
        let mut column: Vec<Pixel> = Vec::new();
        for r in 0..self.height {
            let idx = self.get_index(r, col);
            column.push(self.pixels[idx]);
        }
        column
    }

    // Resets all pixels in row to default
    fn turn_off_row(&mut self, r: u32) {
        for c in 0..self.width {
            let idx = self.get_index(r, c);
            self.pixels[idx] = Pixel::Off;
        }
    }

    // Resets all pixels in column to default
    fn turn_off_column(&mut self, c: u32) {
        for r in 0..self.height {
            let idx = self.get_index(r, c);
            self.pixels[idx] = Pixel::Off;
        }
    }

    // Get Pixel On/Off count
    pub fn get_count(&self, state: Pixel) -> u32 {
        let mut count: u32 = 0;
        for p in 0..self.width * self.height {
            if self.pixels[p as usize] == state {
                count += 1;
            }
        }
        count
    }

    //Reset all pixels to default
    fn reset_pixels(&mut self) {
        for p in 0..self.width * self.height {
            self.pixels[p as usize] = Pixel::Off;
        }
    }
}
