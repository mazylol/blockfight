use raylib::prelude::*;

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
        velocity: 0
    };

    let mut player_two = Player {
        x: SCREEN_WIDTH - 50,
        y: SCREEN_HEIGHT,
        // health: 100,
        jumping: false,
        velocity: 0
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

struct Player {
    x: i32,
    y: i32,
    // health: i32,
    jumping: bool,
    velocity: i32
}

impl Player {
    pub fn draw(&self, d: &mut RaylibDrawHandle, color: Color) {
        d.draw_rectangle(self.x, self.y - 55, 50, 50, color);
    }

    pub fn movement(&mut self, rl: &RaylibHandle, left: KeyboardKey, right: KeyboardKey, jump: KeyboardKey) {
        if self.x <= 0 {
            self.x = 0;
        }

        if self.x >= 1280-50 {
            self.x = 1280-50;
        }

        if self.y < 720 && !self.jumping {
            self.y += 5;
        }

        if self.jumping && self.y != 720-150 {
            self.y -= 10;
        } else {
            self.jumping = false;
        }

        if rl.is_key_down(left) {
            self.velocity = -10;
        } else if rl.is_key_down(right) {
            self.velocity = 10;
        } else {
            self.velocity = 0;
        }

        if rl.is_key_pressed(jump) {
            if self.y > 719 {
                self.jumping = true;
            }
        }

        self.x += self.velocity;
    }
}

fn collisions(p_one: &mut Player, p_two: &mut Player) {
    let player_one_rect = raylib::ffi::Rectangle {
        x: p_one.x as f32,
        y: (p_one.y - 55) as f32,
        width: 50 as f32,
        height: 50 as f32
    };

    let player_two_rect = raylib::ffi::Rectangle {
        x: p_two.x as f32,
        y: (p_two.y - 55) as f32,
        width: 50 as f32,
        height: 50 as f32
    };
    
    unsafe {
        if raylib::ffi::CheckCollisionRecs(player_one_rect, player_two_rect) {
            p_one.velocity = 0;
            p_two.velocity = 0;
        }
    }
}
