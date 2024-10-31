use ggez::event::{self, EventHandler, EventLoop};
use ggez::filesystem::resources_dir;
use ggez::graphics::{self, Color, DrawParam};
use ggez::graphics::{GraphicsContext, Image};
use ggez::mint::Point2;
use ggez::{Context, ContextBuilder, GameResult};
use glam;
use std::path::PathBuf;
use std::{env, fs};
mod blocks;

fn main() {
    let curdir = env::current_dir().unwrap();
    let (mut ctx, event_loop) = ContextBuilder::new("pacman-rs", "freemorger")
        .add_resource_path(curdir)
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
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        let curdir = env::current_dir().unwrap();

        //let mut testimage = graphics::Image::from_path(ctx, self.imgpaths[0].clone()).unwrap();
        let mut testimage =
            graphics::Image::from_path(ctx, curdir.join("/img/pacman.png")).unwrap();
        let test_dest: Point2<f32> = Point2 {
            x: (55.0),
            y: (44.0),
        };
        canvas.draw(&testimage, DrawParam::new().dest(test_dest));

        canvas.finish(ctx)
    }
}
