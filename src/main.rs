
extern crate ggez;
use ggez::conf;
use ggez::event;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, Point};
use std::time::Duration;

const SPEED: f32 = 250.0;
const SIZE: f32 = 50.0;

struct InputState {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl InputState {
    fn new() -> InputState {
        return InputState {
                   up: false,
                   down: false,
                   left: false,
                   right: false,
               };
    }

    fn reset(&mut self) {
        self.up = false;
        self.down = false;
        self.left = false;
        self.right = false;
    }
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
            input_state: InputState::new(),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {
        let float_duration = dt.as_secs() as f64 + dt.subsec_nanos() as f64 * 1e-9;

        if self.input_state.up {
            self.y = self.y - (SPEED * float_duration as f32);
        }
        if self.input_state.down {
            self.y = self.y + (SPEED * float_duration as f32);
        }
        if self.input_state.left {
            self.x = self.x - (SPEED * float_duration as f32);
        }
        if self.input_state.right {
            self.x = self.x + (SPEED * float_duration as f32);
        }
        self.input_state.reset();

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

    fn key_down_event(&mut self, keycode: event::Keycode, keymod: event::Mod, repeat: bool) {
        match keycode {
            event::Keycode::Up => self.input_state.up = true,
            event::Keycode::Down => self.input_state.down = true,
            event::Keycode::Left => self.input_state.left = true,
            event::Keycode::Right => self.input_state.right = true,
            _ => {
                println!("Key pressed: {:?}, modifier {:?}, repeat: {}",
                         keycode,
                         keymod,
                         repeat);
            }
        }
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
