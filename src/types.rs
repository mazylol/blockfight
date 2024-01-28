use raylib::prelude::*;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub velocity: i32,
    pub jumping: bool
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
            self.y -= 5;
        } else {
            self.jumping = false;
        }

        if rl.is_key_down(left) {
            self.velocity = -5;
        } else if rl.is_key_down(right) {
            self.velocity = 5;
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
