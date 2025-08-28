use macroquad::prelude::*;

use macroquad::rand::ChooseRandom;
use std::collections::LinkedList;
use std::fs;

const SQUARES: i16 = 18;

type Point = (i16, i16);

struct Snake {
    head: Point,
    body: LinkedList<Point>,
    dir: Point,
}

#[macroquad::main("snake")]
async fn main() {
    let mut snake = Snake {
        head: (0, 0),
        dir: (1, 0),
        body: LinkedList::new(),
    };
    let mut fruit: Point = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
    let mut score = 0;
    let mut speed = 0.3;
    let mut last_update = get_time();
    let mut navigation_lock = false;
    let mut game_over = false;

    let up = (0, -1);
    let down = (0, 1);
    let right = (1, 0);
    let left = (-1, 0);

    // loading images
    let images = load_images().await;
    let mut current_target_texture = images.get(rand::gen_range(0, images.len() - 1)).unwrap();

    loop {
        if !game_over {
            if is_key_down(KeyCode::Right) && snake.dir != left && !navigation_lock {
                snake.dir = right;
                navigation_lock = true;
            } else if is_key_down(KeyCode::Left) && snake.dir != right && !navigation_lock {
                snake.dir = left;
                navigation_lock = true;
            } else if is_key_down(KeyCode::Up) && snake.dir != down && !navigation_lock {
                snake.dir = up;
                navigation_lock = true;
            } else if is_key_down(KeyCode::Down) && snake.dir != up && !navigation_lock {
                snake.dir = down;
                navigation_lock = true;
            }

            if get_time() - last_update > speed {
                last_update = get_time();
                snake.body.push_front(snake.head);
                snake.head = (snake.head.0 + snake.dir.0, snake.head.1 + snake.dir.1);

                // eat fruit
                if snake.head == fruit {
                    fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
                    current_target_texture =
                        images.get(rand::gen_range(0, images.len() - 1)).unwrap();
                    score += 100;
                    speed *= 0.9;
                } else {
                    snake.body.pop_back();
                }

                // out of bounds
                if snake.head.0 < 0
                    || snake.head.1 < 0
                    || snake.head.0 >= SQUARES
                    || snake.head.1 >= SQUARES
                {
                    game_over = true;
                }

                for (x, y) in &snake.body {
                    if *x == snake.head.0 && *y == snake.head.1 {
                        game_over = true;
                    }
                }
                navigation_lock = false;
            }
        }

        if !game_over {
            clear_background(BLACK);

            let (w, h) = (screen_width(), screen_height());
            let game_size = w.min(h);
            let offset_x = (screen_width() - game_size) / 2. + 10.;
            let offset_y = (screen_height() - game_size) / 2. + 10.;
            let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;

            draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

            for i in 1..SQUARES {
                draw_line(
                    offset_x,
                    offset_y + sq_size * i as f32,
                    screen_width() - offset_x,
                    offset_y + sq_size * i as f32,
                    2.,
                    LIGHTGRAY,
                );
            }

            for i in 1..SQUARES {
                draw_line(
                    offset_x + sq_size * i as f32,
                    offset_y,
                    offset_x + sq_size * i as f32,
                    screen_height() - offset_y,
                    2.,
                    LIGHTGRAY,
                );
            }

            // snake head
            draw_rectangle(
                offset_x + snake.head.0 as f32 * sq_size,
                offset_y + snake.head.1 as f32 * sq_size,
                sq_size,
                sq_size,
                DARKGREEN,
            );

            // snake body
            for (x, y) in &snake.body {
                draw_rectangle(
                    offset_x + *x as f32 * sq_size,
                    offset_y + *y as f32 * sq_size,
                    sq_size,
                    sq_size,
                    LIME,
                );
            }

            // target
            draw_texture_ex(
                current_target_texture,
                offset_x + fruit.0 as f32 * sq_size,
                offset_y + fruit.1 as f32 * sq_size,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(sq_size, sq_size)),
                    ..Default::default()
                },
            );
            // draw_rectangle(
            //     offset_x + fruit.0 as f32 * sq_size,
            //     offset_y + fruit.1 as f32 * sq_size,
            //     sq_size,
            //     sq_size,
            //     GOLD,
            // );

            draw_text(format!("SCORE: {score}").as_str(), 10., 25., 40., LIGHTGRAY);
        } else {
            clear_background(WHITE);
            let text = "GAME OVER! Press [enter] to play again.";
            let font_size = 30.;
            let text_size = measure_text(text, None, font_size as _, 1.0);

            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. + text_size.height / 2.,
                font_size,
                RED,
            );

            if is_key_down(KeyCode::Enter) {
                snake = Snake {
                    head: (0, 0),
                    dir: (1, 0),
                    body: LinkedList::new(),
                };
                fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
                score = 0;
                speed = 0.3;
                last_update = get_time();
                game_over = false;
            }
        }
        next_frame().await;
    }
}

async fn load_images() -> Vec<Texture2D> {
    let images_path: Vec<String> = fs::read_dir("images/square")
        .unwrap()
        .map(|r| r.unwrap().path())
        .map(|r| r.clone().to_str().unwrap().to_owned())
        .collect();

    let mut images = Vec::with_capacity(images_path.len());
    for i in &images_path {
        let img = load_texture(i).await.unwrap();
        images.push(img.clone());
        images.push(img.clone());
    }
    images.shuffle();
    images
}
