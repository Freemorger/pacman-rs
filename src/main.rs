use ggez::event::{self, EventHandler, EventLoop};
use ggez::filesystem::resources_dir;
use ggez::graphics::{self, set_window_title, Color, DrawParam};
use ggez::graphics::{GraphicsContext, Image};
use ggez::input::keyboard::{self, KeyCode, KeyInput, KeyMods};
use ggez::mint::Point2;
use ggez::{Context, ContextBuilder, GameResult};
use glam;
use std::path::PathBuf;
use std::{env, fs};
mod blocks;
use blocks::widgets::Player;

const VERSION: &str = "v0.3";
const AUTHOR: &str = "freemorger";

fn main() {
    let curdir = env::current_dir().unwrap();
    let (mut ctx, event_loop) = ContextBuilder::new("pacman-rs", "freemorger")
        .add_resource_path(curdir)
        .build()
        .expect("cant create context");

    set_window_title(&ctx, &format!("Pacman-rs, {} by {}", VERSION, AUTHOR));
    let game = Game::new(&mut ctx);

    event::run(ctx, event_loop, game);
}

struct Game {
    imgpaths: Vec<PathBuf>,
    player: blocks::widgets::Player,
    //game_map: Vec<Vec<u16>>
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        let player_start_texture: &str = "pacman.png";

        let mut imgs: Vec<PathBuf> = vec![];
        let img_dir = PathBuf::from("img");
        let files = fs::read_dir(img_dir).unwrap();
        let curdir = env::current_dir().unwrap();

        for file in files {
            match file {
                Ok(f) => imgs.push(curdir.join(f.path().to_str().unwrap())),
                Err(e) => eprintln!("Error reading images: {:?}", e),
            }
        }

        let player_texture = imgs
            .iter()
            .find(|s| {
                s.as_os_str()
                    .to_str()
                    .unwrap()
                    .contains(player_start_texture)
            })
            .unwrap();
        let playerPos: Point2<f32> = Point2 { x: 0.0, y: 0.0 };
        let mut player = blocks::widgets::Player::new(player_texture.clone(), playerPos, 0);

        //println!("{:#?}", player_texture);

        Game {
            imgpaths: imgs,
            player: player,
        }
    }

    /*
    fn checkMoves(&mut self, ctx: &mut Context) -> (&mut Self, &mut Context) {
        let k_ctx = &ctx.keyboard;
        if k_ctx.is_key_pressed(KeyCode::Up) {
            self.player.pos.x += 2.0;
        }
        if k_ctx.is_key_pressed(KeyCode::Down) {
            self.player.pos.x -= 2.0;
        }
        if k_ctx.is_key_pressed(KeyCode::Left) {
            self.player.pos.y -= 2.0;
        }
        if k_ctx.is_key_pressed(KeyCode::Right) {
            self.player.pos.y += 2.0;
        }
        if k_ctx.is_key_just_released(KeyCode::Q) {
            println!("{}, {}", self.player.pos.x, self.player.pos.y);
        }

        (self, ctx)
    }
    */
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let k_ctx = &ctx.keyboard;
        if k_ctx.is_key_pressed(KeyCode::Up) {
            self.player.pos.y += 2.0;
        }
        if k_ctx.is_key_pressed(KeyCode::Down) {
            self.player.pos.y -= 2.0;
        }
        if k_ctx.is_key_pressed(KeyCode::Left) {
            self.player.pos.x -= 2.0;
        }
        if k_ctx.is_key_pressed(KeyCode::Right) {
            self.player.pos.x += 2.0;
        }
        if k_ctx.is_key_just_released(KeyCode::Q) {
            println!("{}, {}", self.player.pos.x, self.player.pos.y);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        let curdir = env::current_dir().unwrap();

        //let mut testimage = graphics::Image::from_path(ctx, self.imgpaths[0].clone()).unwrap();
        let mut playerimg =
            graphics::Image::from_path(ctx, curdir.join("/img/pacman.png")).unwrap(); // self.player.texture_path.clone()
        let test_dest: Point2<f32> = Point2 {
            x: (self.player.pos.x.clone()),
            y: (self.player.pos.y.clone()),
        }; // some cords
        canvas.draw(&playerimg, DrawParam::new().dest(test_dest));

        canvas.finish(ctx)
    }
}
