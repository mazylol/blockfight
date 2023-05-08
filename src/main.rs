mod types;

use raylib::prelude::*;

use types::Player;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;

fn main() {
    let (mut rl, thread) = init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Block Fight")
        .build();

    rl.set_target_fps(60);

    let i = Image::load_image("resources/sword.png").expect("could not load image sword");
    let mut sword = rl.load_texture_from_image(&thread, &i).unwrap();
    sword.height = 50;
    sword.width = 50;

    let mut player_one = Player {
        x: 0,
        y: SCREEN_HEIGHT,
        // health: 100,
        jumping: false,
        velocity: 0,
    };

    let mut player_two = Player {
        x: SCREEN_WIDTH - 50,
        y: SCREEN_HEIGHT,
        // health: 100,
        jumping: false,
        velocity: 0,
    };

    while !rl.window_should_close() {
        player_one.movement(&rl, KeyboardKey::KEY_A, KeyboardKey::KEY_D, KeyboardKey::KEY_W);
        player_two.movement(&rl, KeyboardKey::KEY_LEFT, KeyboardKey::KEY_RIGHT, KeyboardKey::KEY_UP);

        collisions(&mut player_one, &mut player_two);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        player_one.draw(&mut d, Color::RED);
        player_two.draw(&mut d, Color::LIME);

        d.draw_texture(&sword, 5, 5, Color::WHITE);
    }
}

fn collisions(p_one: &mut Player, p_two: &mut Player) {
    let player_one_rect = raylib::ffi::Rectangle {
        x: p_one.x as f32,
        y: (p_one.y - 55) as f32,
        width: 50 as f32,
        height: 50 as f32,
    };

    let player_two_rect = raylib::ffi::Rectangle {
        x: p_two.x as f32,
        y: (p_two.y - 55) as f32,
        width: 50 as f32,
        height: 50 as f32,
    };

    unsafe {
        if raylib::ffi::CheckCollisionRecs(player_one_rect, player_two_rect) {
            p_one.velocity = 0;
            p_two.velocity = 0;
        }
    }
}
