extern crate eb_core;
extern crate ggez;

use std::env;
use std::path;
use std::io::Read;

use ggez::conf;
use ggez::{Context, GameResult};
use ggez::event;
use ggez::graphics::{self, Rect, Point2};
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::filesystem;
use ggez::timer;

struct MainState {
    spritebatch: graphics::spritebatch::SpriteBatch
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {

        let image = graphics::Image::new(ctx, "/track-parts.png").unwrap();
        let mut spritebatch = graphics::spritebatch::SpriteBatch::new(image);
//        1.1517857, 0, 0.008928572, 0.125
        let p1 = graphics::DrawParam {
            src: Rect::new(1.1517857, 0., 0.008928572, 0.125),
            dest: Point2::new(0. , 0.),

            ..Default::default()
        };

        let p2 = graphics::DrawParam {
            src: Rect::new(0.25, 0., 0.25, 1.0),
            dest: Point2::new(16. , 0.),

            ..Default::default()
        };

        let p3 = graphics::DrawParam {
            src: Rect::new(0.5, 0., 0.25, 1.0),
            dest: Point2::new(32. , 0.),

            ..Default::default()
        };

        let p4 = graphics::DrawParam {
            src: Rect::new(0.75, 0., 0.25, 1.0),
            dest: Point2::new(48. , 0.),

            ..Default::default()
        };

        spritebatch.add(p1);
//        spritebatch.add(p2);
//        spritebatch.add(p3);
//        spritebatch.add(p4);

        let s = MainState {
            spritebatch
        };

        Ok(s)
    }
}


impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if timer::get_ticks(ctx) % 100 == 0 {
            println!("Delta frame time: {:?} ", timer::get_delta(ctx));
            println!("Average FPS: {}", timer::get_fps(ctx));
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::draw(ctx, &self.spritebatch, graphics::Point2::new(0., 0.), 0.)?;
        graphics::present(ctx);
        Ok(())
    }
}


pub fn main() {
    let c = conf::Conf {
        window_mode: conf::WindowMode {
            width: 640,
            height: 480,
            ..Default::default()
        },
        window_setup: conf::WindowSetup { title: "numbers".into(), ..Default::default() },
        ..Default::default()
    };

    println!("Starting with default config: {:#?}", c);
    let ctx = &mut Context::load_from_conf("spritebatch", "ggez", c).unwrap();

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut MainState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
