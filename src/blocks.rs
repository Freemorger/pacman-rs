pub mod widgets {
    use ggez::mint::Point2;
    use std::path::PathBuf;

    pub struct Player {
        pub texture_path: PathBuf,
        pub pos: Point2<f32>,
        pub points: i32,
    }

    impl Player {
        pub fn new(texture: PathBuf, position: Point2<f32>, points: i32) -> Player {
            let player = Player {
                texture_path: texture,
                pos: position,
                points: points,
            };
            player
        }
        pub fn default(texture: PathBuf, gtx: ggez::graphics::GraphicsContext) -> Player {
            let (sizeX, sizeY) = gtx.size();
            let playerpos: Point2<f32> = Point2 {
                x: (sizeX / 2.0),
                y: (sizeY / 2.0),
            };
            let player = Player {
                texture_path: texture,
                pos: playerpos,
                points: 0,
            };
            player
        }
    }
}
