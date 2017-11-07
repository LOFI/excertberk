
extern crate ggez;
use ggez::conf;
use ggez::event;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::graphics::{DrawMode, Point};
use std::time::Duration;

const SPEED: f32 = 250.0;
const SIZE: f32 = 50.0;

struct InputState {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

struct MainState {
    x: f32,
    y: f32,
    input_state: InputState,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let window = ctx.gfx_context.get_window();
        let (width, height) = window.size();
        let s = MainState {
            x: width as f32 * 0.5,
            y: height as f32 * 0.5,
            input_state: InputState {
                up: false,
                down: false,
                left: false,
                right: false,
            },
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {
        let window = ctx.gfx_context.get_window();
        let (width, height) = window.size();
        let float_duration = dt.as_secs() as f64 + dt.subsec_nanos() as f64 * 1e-9;

        if self.input_state.up {
            let temp_up = self.y - (SPEED * float_duration as f32);
            if temp_up - (SIZE / 2.) > 0. {
                self.y = temp_up;
            }
        }
        if self.input_state.down {
            let temp_down = self.y + (SPEED * float_duration as f32);
            if temp_down + (SIZE / 2.) < (height as f32) {
                self.y = temp_down;
            }
        }
        if self.input_state.left {
            let temp_left = self.x - (SPEED * float_duration as f32);
            if temp_left - (SIZE / 2.) > 0. {
                self.x = temp_left;
            }
        }
        if self.input_state.right {
            let temp_right = self.x + (SPEED * float_duration as f32);
            if temp_right + (SIZE / 2.) < (width as f32) {
                self.x = temp_right;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::circle(ctx,
                         DrawMode::Fill,
                         Point {
                             x: self.x,
                             y: self.y,
                         },
                         SIZE,
                         32)?;
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, keycode: event::Keycode, _keymod: event::Mod, _repeat: bool) {
        match keycode {
            event::Keycode::Up => self.input_state.up = true,
            event::Keycode::Down => self.input_state.down = true,
            event::Keycode::Left => self.input_state.left = true,
            event::Keycode::Right => self.input_state.right = true,
            _ => {}
        }
    }

    fn key_up_event(&mut self, keycode: event::Keycode, _keymod: event::Mod, _repeat: bool) {
        match keycode {
            event::Keycode::Up => self.input_state.up = false,
            event::Keycode::Down => self.input_state.down = false,
            event::Keycode::Left => self.input_state.left = false,
            event::Keycode::Right => self.input_state.right = false,
            _ => {}
        }
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
