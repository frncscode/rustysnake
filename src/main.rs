use macroquad::prelude::*;
use snake::*;

mod snake;

fn config() -> Conf {
    Conf {
        window_width: 500,
        window_height: 500,
        window_title: "Snake".to_string(),
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(config)]
async fn main() {
    let texture = load_texture("C:/Users/fr3nc/dev/rust/snake/Assets/segment.png").await.unwrap();
    let food_textures = vec![
        load_texture("C:/Users/fr3nc/dev/rust/snake/Assets/apple.png").await.unwrap(),
        load_texture("C:/Users/fr3nc/dev/rust/snake/Assets/cherry.png").await.unwrap(),
    ];
    let head_texture = load_texture("C:/Users/fr3nc/dev/rust/snake/Assets/head.png").await.unwrap();
    let mut game = SnakeGame::new(20, 20, 25, texture, food_textures, head_texture);
    let mut ticks: u32 = 0;
    let interval: u32 = 10;
    let mut direction = game.direction.clone();

    loop {
        // update
        if ticks >= interval {
            game.input(direction);
            game.tick();
            ticks = 0;
        }
        if is_key_pressed(KeyCode::Up) {
            direction = DIRECTION_UP;
        } else if is_key_pressed(KeyCode::Down) {
            direction = DIRECTION_DOWN;
        } else if is_key_pressed(KeyCode::Left) {
            direction = DIRECTION_LEFT;
        } else if is_key_pressed(KeyCode::Right) {
            direction = DIRECTION_RIGHT;
        }
        game.eat();

        // render
        clear_background(BLACK);
        game.draw();
        next_frame().await;
        ticks += 1;
    } 
}
