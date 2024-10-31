use ggez::event::{self, EventHandler, EventLoop};
use ggez::graphics::{self, Color, DrawParam};
use ggez::graphics::{GraphicsContext, Image};
use ggez::{Context, ContextBuilder, GameResult};
use glam;
use std::fs;
use std::path::PathBuf;
mod blocks;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("pacman-rs", "freemorger")
        .build()
        .expect("cant create context");

    let game = Game::new(&mut ctx);

    event::run(ctx, event_loop, game);
}

struct Game {
    imgpaths: Vec<String>,
    //game_map: Vec<Vec<u16>>
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        let mut imgs: Vec<String> = vec![];
        let img_dir = PathBuf::from("img");
        let files = fs::read_dir(img_dir).unwrap();

        for file in files {
            match file {
                Ok(f) => imgs.push(f.path().to_str().unwrap().to_string()),
                Err(e) => eprintln!("Error reading images: {:?}", e),
            }
        }

        Game { imgpaths: imgs }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        /*
        let mut testimage = graphics::Image::from_path(ctx, self.imgpaths[0].clone()).unwrap();
        let my_dest = glam::vec2(13.0, 37.0);
        canvas.draw(&testimage, DrawParam::default());
        */
        canvas.finish(ctx)
    }
}
