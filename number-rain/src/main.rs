use macroquad::prelude::*;

const BUCKET_SPEED: f32 = 700.0;
const MAX_BUCKET_CAP: i32 = 30;

const MIN_NUMBER_SPEED: f32 = 200.0;
const MAX_NUMBER_SPEED: f32 = 400.0;
const NUMBER_PERCENT: i32 = 95;


const SCREEN_OFFSET: f32 = 30.0;


#[macroquad::main("number-rain")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let mut game_over = false;

    let mut bucket_x = screen_width() / 2.0;
    let mut bucket_cap = rand::gen_range(1, MAX_BUCKET_CAP);

    let mut numbers = vec![];

    loop {
        clear_background(LIGHTGRAY);

        let screen_x = screen_width() - SCREEN_OFFSET;
        let screen_y = screen_height() - SCREEN_OFFSET;

        // main
        let main_w = screen_x - SCREEN_OFFSET;
        let main_h = screen_y - SCREEN_OFFSET;

        // bucket
        let bucket_y = screen_y - (3.0 * SCREEN_OFFSET);
        let bucket_w = 4.0 * SCREEN_OFFSET;
        let bucket_h = 3.0 * SCREEN_OFFSET;
        let bucket_fs = bucket_w.min(bucket_h);

        let delta_time = get_frame_time();
        let bucket_mov = BUCKET_SPEED * delta_time;

        if game_over && is_key_pressed(KeyCode::Space) {
            numbers.clear();
            bucket_x = main_w / 2.0;
            bucket_cap = rand::gen_range(1, MAX_BUCKET_CAP);
            game_over = false;
        }

        if !game_over {
            // numbers
            if rand::gen_range(0, 100) >= NUMBER_PERCENT {
                numbers.push(Number {
                    value: rand::gen_range(1, MAX_BUCKET_CAP + 10),
                    speed: rand::gen_range(MIN_NUMBER_SPEED, MAX_NUMBER_SPEED),
                    x: rand::gen_range(SCREEN_OFFSET, screen_x),
                    y: SCREEN_OFFSET * 2.0,
                });
            }

            if is_key_down(KeyCode::Left) {
                bucket_x -= bucket_mov;
            }
            if is_key_down(KeyCode::Right) {
                bucket_x += bucket_mov;
            }
            bucket_x = clamp(bucket_x, SCREEN_OFFSET, screen_x - bucket_w);

            for n in &mut numbers {
                n.y += n.speed * delta_time;
            }

            numbers.retain(|n| n.y <= screen_y);
            numbers.retain(|n| {
                let collapsed = n.y >= bucket_y && n.y <= bucket_y + bucket_h &&
                    n.x >= bucket_x && n.x <= bucket_x + bucket_w;

                if collapsed {
                    bucket_cap -= n.value;
                }
                !collapsed
            });

            if bucket_cap <= 0 {
                game_over = true;
            }
        }

        // draw
        draw_rectangle(SCREEN_OFFSET, SCREEN_OFFSET, main_w, main_h, PINK);

        for n in &numbers {
            draw_text(n.value.to_string().as_str(), n.x, n.y, bucket_fs, BLACK);
        }

        draw_rectangle(
            bucket_x,
            bucket_y,
            bucket_w,
            bucket_h,
            DARKBLUE,
        );
        draw_text(
            bucket_cap.to_string().as_str(),
            bucket_x + (bucket_fs / 3.0),
            bucket_y + (bucket_fs / 2.0),
            bucket_fs / 1.3,
            WHITE,
        );

        if game_over {
            if bucket_cap < 0 {
                draw_text("GAME OVER!!!", screen_x / 7.0, screen_y / 2.0, screen_x / 7.0, RED);
            }

            if bucket_cap == 0 {
                draw_text("YOU WIN!!!", screen_x / 7.0, screen_y / 2.0, screen_x / 7.0, GREEN);
            }
        }

        next_frame().await;
    }
}

struct Number {
    speed: f32,
    x: f32,
    y: f32,
    value: i32,
}
