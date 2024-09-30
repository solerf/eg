use macroquad::prelude::*;
use macroquad::rand;

#[macroquad::main("number-rain")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    loop {
        clear_background(PINK);

        next_frame();
    }
}
