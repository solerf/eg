use macroquad::prelude::next_frame;
use macroquad::prelude::*;
use macroquad::window::clear_background;
use std::collections::HashMap;

const IMG_NEUTRAL_1: &str = "images/neutral_1.png";
const IMG_NEUTRAL_2: &str = "images/neutral_2.png";
const IMG_WHAT: &str = "images/what.png";
const IMG_OK: &str = "images/ok.png";
const IMG_NOK: &str = "images/nok.png";

const NUM_LIMIT: i32 = 20;

#[macroquad::main("surprise-math")]
async fn main() {
    let mut key_codes = HashMap::with_capacity(10);
    key_codes.insert(KeyCode::Key0, 0);
    key_codes.insert(KeyCode::Key1, 1);
    key_codes.insert(KeyCode::Key2, 2);
    key_codes.insert(KeyCode::Key3, 3);
    key_codes.insert(KeyCode::Key4, 4);
    key_codes.insert(KeyCode::Key5, 5);
    key_codes.insert(KeyCode::Key6, 6);
    key_codes.insert(KeyCode::Key7, 7);
    key_codes.insert(KeyCode::Key8, 8);
    key_codes.insert(KeyCode::Key9, 9);

    let neutral_1 = load_texture(IMG_NEUTRAL_1).await.unwrap();
    let neutral_2 = load_texture(IMG_NEUTRAL_2).await.unwrap();
    let what = load_texture(IMG_WHAT).await.unwrap();
    let ok = load_texture(IMG_OK).await.unwrap();
    let nok = load_texture(IMG_NOK).await.unwrap();

    let mut calc = Calc::new();
    let mut answer = "".to_owned();

    let mut game_over = false;

    loop {
        rand::srand(miniquad::date::now() as u64);
        clear_background(WHITE);

        if game_over {
            let cat = if calc.equals_result(answer.as_str()) { &ok } else { &nok };
            print_cat_image(cat);
        }

        if game_over && is_key_pressed(KeyCode::Escape) {
            game_over = false;
            calc = Calc::new();
            answer = "".to_string();
        }

        if !game_over {
            let cat = if get_last_key_pressed().is_none() { &neutral_1 } else { &neutral_2 };
            print_cat_image(cat);

            if is_key_pressed(KeyCode::Space) {
                match calc {
                    _ if calc.a.is_none() => {
                        let n = rand::gen_range(1, NUM_LIMIT);
                        calc.a = Some(n);
                    }
                    _ if calc.operation.is_none() => {
                        let rnd = rand::gen_range(0, 100);
                        calc.operation = if rnd % 2 == 0 { Some(Operation::PLUS) } else { Some(Operation::SUB) };
                    }
                    _ if calc.b.is_none() => {
                        let limit = calc.a.unwrap();
                        let n = rand::gen_range(1, limit);
                        calc.b = Some(n);
                        calc.calculate();
                    }
                    _ =>
                        print_cat_image(&what)
                }
            }

            let last_key = get_last_key_pressed();
            if calc.not_empty() &&
                last_key.is_some() &&
                key_codes.contains_key(last_key.as_ref().unwrap()) {
                let v = key_codes.get(last_key.as_ref().unwrap()).unwrap().to_string();
                answer.push_str(v.as_str());
            }
        }

        let card_width = screen_width() / 8.0;
        let card_height = screen_height() / 3.0;
        let card_gap = card_width / 4.0;
        let card_font_size = card_width.min(card_height);

        for i in 0..5 {
            let initial_x = ((i as f32) * (card_width + card_gap)) + card_gap;
            let screen_y = screen_height() / 2.5;

            match i {
                0 if calc.a.is_some() => {
                    let value = format!("{}", calc.a.as_ref().unwrap());
                    draw_text(value.as_str(), initial_x, screen_y + (card_height / 1.5), card_font_size, GOLD);
                }
                1 if calc.operation.is_some() => {
                    draw_text(calc.operation.as_ref().unwrap().value().as_str(), initial_x, screen_y + (card_height / 1.5), card_font_size, BLACK);
                }
                2 if calc.b.is_some() => {
                    let value = format!("{}", calc.b.as_ref().unwrap());
                    draw_text(value.as_str(), initial_x, screen_y + (card_height / 1.5), card_font_size, GOLD);
                }
                3 => {
                    draw_text("=", initial_x, screen_y + (card_height / 1.5), card_font_size, BLACK);
                }
                4 if !answer.is_empty() => {
                    if answer.len() == calc.result.len() {
                        if answer == calc.result {
                            draw_text(answer.as_str(), initial_x, screen_y + (card_height / 1.5), card_font_size, GREEN);
                        } else {
                            let dimensions = draw_text(answer.as_str(), initial_x, screen_y + (card_height / 1.5), card_font_size, RED);
                            let correct = format!("({})", calc.result);
                            draw_text(&correct, initial_x + dimensions.width, screen_y + (card_height / 1.5), card_font_size / 2.0, GREEN);
                        }
                        game_over = true;
                    } else {
                        draw_text(answer.as_str(), initial_x, screen_y + (card_height / 1.5), card_font_size, GOLD);
                    }
                }
                i => {
                    if i == 4 {
                        let question = "?";
                        draw_text(&question.repeat(calc.result.len()), initial_x, screen_y + (card_height / 1.5), card_font_size, BLACK);
                    } else {
                        draw_text("?", initial_x, screen_y + (card_height / 1.5), card_font_size, BLACK);
                    }
                }
            }
        }

        next_frame().await;
    }
}

fn print_cat_image(img_target: &Texture2D) {
    draw_texture(img_target, screen_width() / 2.5, 10.0, WHITE)
}

struct Calc {
    a: Option<i32>,
    b: Option<i32>,
    operation: Option<Operation>,
    result: String,
}

impl Calc {
    fn new() -> Self {
        Calc {
            a: None,
            b: None,
            operation: None,
            result: "".to_string(),
        }
    }

    fn equals_result(&self, answer: &str) -> bool {
        self.result == answer
    }

    fn not_empty(&self) -> bool {
        self.a.is_some() && self.operation.is_some() && self.b.is_some()
    }

    fn calculate(&mut self) {
        match self.operation.as_ref().unwrap() {
            Operation::PLUS => {
                self.result = (self.a.unwrap() + self.b.unwrap()).to_string();
            }
            Operation::SUB => {
                self.result = (self.a.unwrap() - self.b.unwrap()).to_string();
            }
        }
    }
}

enum Operation {
    PLUS,
    SUB,
}

impl Operation {
    fn value(&self) -> String {
        match self {
            Operation::PLUS => "+".to_string(),
            Operation::SUB => "-".to_string()
        }
    }
}
