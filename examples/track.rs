extern crate eb_core;
extern crate ggez;

use std::env;
use std::path;

use ggez::conf;
use ggez::{Context, GameResult};
use ggez::event;
use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::timer;
use eb_core::track::Track;


struct MainState {
    spritebatch: SpriteBatch,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {


        let args: Vec<String> = env::args().collect();

        let fp = &args[1];
        println!("reading: {}", fp);
        let track = Track::from_file(fp);

        let image = graphics::Image::new(ctx, "/track-parts.png").unwrap();
        let mut batch = graphics::spritebatch::SpriteBatch::new(image);

        for r in &track.rects() {
            println!("{:?}", r);

            // snagged some of the spriteback example for usage

//            let p = graphics::DrawParam {
//                dest: graphics::Point2::new(x * 10.0, y * 10.0),
//                // scale: graphics::Point::new(0.0625, 0.0625),
//                scale: graphics::Point2::new(((time % cycle * 2) as f32 / cycle as f32 * 6.28)
//                                                 .cos()
//                                                 .abs() *
//                                                 0.0625,
//                                             ((time % cycle * 2) as f32 / cycle as f32 * 6.28)
//                                                 .cos()
//                                                 .abs() *
//                                                 0.0625),
//                rotation: -2.0 * ((time % cycle) as f32 / cycle as f32 * 6.28),
//                ..Default::default()
//            };
//
//            batch.add(p);

        }

        let s = MainState { spritebatch: batch };
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

        let time = (timer::duration_to_f64(timer::get_time_since_start(ctx)) * 1000.0) as u32;

        graphics::draw(ctx, &self.spritebatch, graphics::Point2::origin(), 0.)?;
        self.spritebatch.clear();

        graphics::present(ctx);
        Ok(())
    }
}


pub fn main() {
    let c = conf::Conf::new();
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
