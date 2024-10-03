use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
use std::collections::HashSet;
use std::fs;

const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 720;
const SCREEN_OFFSET: f32 = 15.0;

const CARD_WIDTH: f32 = 300.0;
const CARD_HEIGHT: f32 = 180.0;
const MAX_CARDS: usize = 9;

#[macroquad::main("memory")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);
    //set_window_size(SCREEN_WIDTH, SCREEN_HEIGHT);

    // control
    let mut found_pairs: HashSet<String> = HashSet::with_capacity(MAX_CARDS);
    let mut current_opens: Vec<Card> = Vec::with_capacity(2);
    let mut game_over = false;

    // loading images
    let mut images = load_images().await;

    // create cards
    let mut cards: Vec<Card> = make_cards(images);

    loop {
        if game_over && is_key_pressed(KeyCode::Space) {
            images = load_images().await;
            found_pairs.clear();
            current_opens.clear();
            game_over = false;
            cards.clear();
            cards = make_cards(images);
        }

        // check click
        if !game_over {
            if is_mouse_button_pressed(MouseButton::Left) {
                let (x_mouse, y_mouse) = mouse_position();
                let clicked = cards.clone().into_iter().find(|c| c.clicked_at(x_mouse, y_mouse));

                if clicked.is_some() {
                    if current_opens.len() == 2 {
                        current_opens.clear()
                    }
                    let to_add = clicked.unwrap().clone();
                    current_opens.push(to_add);
                }
            }
        }

        // draw
        for c in &cards {
            let is_open = current_opens.iter().find(|co| co.is_equal(c)).is_some();
            let is_found = found_pairs.contains(&c.id);
            c.draw(is_open, is_found);
        }

        if !game_over && current_opens.len() == 2 {
            let a = current_opens[0].clone();
            let b = current_opens[1].clone();

            if a.is_other_pair(&b) {
                found_pairs.insert(a.id.clone());
            }
        }

        if found_pairs.len() == MAX_CARDS + 1 {
            game_over = true;
            draw_text("YOU WIN!!!", (SCREEN_WIDTH / 7) as f32, (SCREEN_HEIGHT / 2) as f32, (SCREEN_WIDTH / 7) as f32, GREEN);
        }

        next_frame().await;
    }
}

fn make_cards(images: Vec<(String, Texture2D)>) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::with_capacity(MAX_CARDS * 2);
    let mut initial_x = SCREEN_OFFSET;
    let mut initial_y = SCREEN_OFFSET;

    for (id, img) in images {
        // reset if higher
        if (initial_x + SCREEN_OFFSET + CARD_WIDTH) >= (SCREEN_WIDTH as f32) {
            initial_y += CARD_HEIGHT + SCREEN_OFFSET;
            initial_x = SCREEN_OFFSET;
        }

        let data = img.get_texture_data();
        let card = Card {
            id: id.clone(),
            texture: img.clone(),
            x: initial_x,
            y: initial_y,
            width: data.width as f32,
            height: data.height as f32,
        };
        cards.push(card);

        // bump x coord
        initial_x += CARD_WIDTH + SCREEN_OFFSET;
    }
    cards
}

async fn load_images() -> Vec<(String, Texture2D)> {
    let mut images_path: Vec<String> = fs::read_dir("images/round")
        .unwrap()
        .map(|r| r.unwrap().path())
        .map(|r| r.clone().to_str().unwrap().to_owned())
        .collect();

    images_path.shuffle();

    let mut images = Vec::with_capacity(MAX_CARDS);
    for i in &images_path[0..=MAX_CARDS] {
        let img = load_texture(i).await.unwrap();
        images.push((i.clone(), img.clone()));
        images.push((i.clone(), img.clone()));
    }

    images.shuffle();
    images
}

#[derive(Debug, Clone)]
struct Card {
    id: String,
    texture: Texture2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}


impl Card {
    fn draw(self: &Self, is_open: bool, is_found: bool) {
        if is_open || is_found {
            let x_middle = self.x + CARD_WIDTH / 2.0;
            let y_middle = self.y + CARD_HEIGHT / 2.0;

            let x_img = x_middle - (self.width / 2.0);
            let y_img = y_middle - (self.height / 2.0);

            if is_found {
                draw_rectangle(self.x, self.y, CARD_WIDTH, CARD_HEIGHT, PINK); // open
            } else {
                draw_rectangle(self.x, self.y, CARD_WIDTH, CARD_HEIGHT, SKYBLUE); // open
            }
            draw_texture(&self.texture, x_img, y_img, WHITE);
        } else {
            draw_rectangle(self.x, self.y, CARD_WIDTH, CARD_HEIGHT, LIGHTGRAY); // closed
        }
    }

    fn is_other_pair(self: &Self, other: &Card) -> bool {
        self.id == other.id && (self.x != other.x || self.y != other.y)
    }

    fn is_equal(self: &Self, other: &Card) -> bool {
        self.id == other.id && self.x == other.x && self.y == other.y
    }

    fn clicked_at(self: &Self, x_target: f32, y_target: f32) -> bool {
        let mouse_rect = Rect { x: x_target, y: y_target, w: 1.0, h: 1.0 };
        let card_rect = Rect { x: self.x, y: self.y, w: CARD_WIDTH, h: CARD_HEIGHT };
        card_rect.intersect(mouse_rect).is_some()
    }
}
