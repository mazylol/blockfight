use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Block Fight")
        .build();

    rl.set_target_fps(60);

    let i = Image::load_image("resources/sword.png").expect("could not load image sword");
    let mut sword = rl.load_texture_from_image(&thread, &i).unwrap();
    sword.height = 50;
    sword.width = 50;

    let mut player_one = Player {
        posx: 0,
        posy: SCREEN_HEIGHT,
        // health: 100,
        isjumping: false,
        velocity: 0
    };

    let mut player_two = Player {
        posx: SCREEN_WIDTH - 50,
        posy: SCREEN_HEIGHT,
        // health: 100,
        isjumping: false,
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
    posx: i32,
    posy: i32,
    // health: i32,
    isjumping: bool,
    velocity: i32
}

impl Player {
    pub fn draw(&self, d: &mut RaylibDrawHandle, color: Color) {
        d.draw_rectangle(self.posx, self.posy - 55, 50, 50, color);
    }

    pub fn movement(&mut self, rl: &RaylibHandle, left: KeyboardKey, right: KeyboardKey, jump: KeyboardKey) {
        if self.posx <= 0 {
            self.posx = 0;
        }

        if self.posx >= 1280-50 {
            self.posx = 1280-50;
        }

        if self.posy < 720 && !self.isjumping {
            self.posy += 5;
        }

        if self.isjumping && self.posy != 720-150 {
            self.posy -= 10;
        } else {
            self.isjumping = false;
        }

        if rl.is_key_down(left) {
            self.velocity = -10;
        } else if rl.is_key_down(right) {
            self.velocity = 10;
        } else {
            self.velocity = 0;
        }

        if rl.is_key_pressed(jump) {
            if self.posy > 719 {
                self.isjumping = true;
            }
        }

        self.posx += self.velocity;
    }
}

fn collisions(p_one: &mut Player, p_two: &mut Player) {
    let player_one_rect = raylib::ffi::Rectangle {
        x: p_one.posx as f32,
        y: (p_one.posy - 55) as f32,
        width: 50 as f32,
        height: 50 as f32
    };

    let player_two_rect = raylib::ffi::Rectangle {
        x: p_two.posx as f32,
        y: (p_two.posy - 55) as f32,
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
