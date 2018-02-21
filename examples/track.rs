extern crate eb_core;
extern crate ggez;

use std::env;
use std::path;
use std::io::Read;

use ggez::conf;
use ggez::{Context, GameResult};
use ggez::event;
use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::filesystem;
use ggez::timer;
use eb_core::track::{ImageSize, TrackData};

struct MainState {
    spritebatch: SpriteBatch,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut reader: filesystem::File = ctx.filesystem.open("/test-track-01.json")?;

        let data = {
            let mut s = String::new();
            let _ = reader
                .read_to_string(&mut s)
                .expect("read tilemap data file");
            s
        };

        let image = graphics::Image::new(ctx, "/track-parts.png").unwrap();
        let track = TrackData::from_str(
            &data,
            ImageSize {
                width: image.width(),
                height: image.height(),
            },
        );

        let rects = track.uv_rects();
        let mut batch = graphics::spritebatch::SpriteBatch::new(image);
        println!("tile count: {}", rects.len());

        for (i, maybe_rect) in rects.iter().enumerate() {
            // when the rect is None, it's an empty tile (GID=0)
            if let &Some(uv) = maybe_rect {
                let (row, col) = eb_core::track::divmod(i as u32, track.tile_layer.width);

                let dest = {
                    let x = col as f32 * track.tile_width as f32;
                    let y = row as f32 * track.tile_height as f32;
                    graphics::Point2::new(x, y)
                };

                let p = graphics::DrawParam {
                    src: uv,
                    dest,
                    ..Default::default()
                };

                batch.add(p);
            }
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
        let _time = (timer::duration_to_f64(timer::get_time_since_start(ctx)) * 1000.0) as f32;
        graphics::draw(ctx, &self.spritebatch, graphics::Point2::new(0., 0.), 0.)?;
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf {
        window_mode: conf::WindowMode {
            width: 72 * 16,
            height: 16 * 16,
            ..Default::default()
        },
        window_setup: conf::WindowSetup {
            title: "excertberk".into(),
            ..Default::default()
        },
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
