extern crate ggez;
extern crate minimax;

use std::path::PathBuf;

use ggez::*;
use ggez::event::*;
use ggez::graphics::{Font};

use minimax::board::*;
use minimax::minimax::AI;

struct Game {
    ai: AI,
    board: Board,
    font: Font,
    next: u8,
    ready: bool,
    done: bool,
}

const WIDTH: u32 = 500;
const HEIGHT: u32 = 430;

const STROKE: f32 = 2.0;
const RADIUS: f32 = 30.0;
const OFFSET: f32 = 40.0;

impl Game {
    fn new(ctx: &mut Context) -> GameResult<Game> {
        graphics::set_background_color(ctx, (70, 114, 186, 255).into());

        Ok(Game {
            ai: AI::new(),
            board: Board::new(),
            font: graphics::Font::new(ctx, "/OpenSans-Regular.ttf", 48)?,
            next: 0,
            ready: false,
            done: true,
        }) 
    }

}

fn to_point(row: u8, col: u8) -> graphics::Point2 {
    let col = col as f32;
    let row = (ROWS - row - 1) as f32;
    graphics::Point2::new(
        OFFSET + (OFFSET + RADIUS)*col,
        OFFSET + (OFFSET + RADIUS)*row,)
}

impl event::EventHandler for Game {
    fn update(&mut self, _: &mut Context) -> GameResult<()> {
        if self.board.was_won().is_some() {
            return Ok(())
        } else if self.ready && self.done {
            if self.board.valid_moves().contains(&self.next) {
                self.board.make_move(self.next);
                self.ready = false;
                self.done = false;
                self.next = 8;
            }
        } else if !self.done {
            // let next = self.ai.solve(&mut self.board);
            // self.board.make_move(self.next);
            self.ai.reset();
            self.done = true;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        
        for row in (0..ROWS).rev() {
            for col in 0..COLS {
                let color = match self.board.get(row, col) {
                    Some(_) => graphics::Color::from_rgb(0, 0, 0),
                _           => graphics::Color::from_rgb(90, 154, 254),
                };
                let fill = match self.board.get(row, col) {
                    Some(WHITE) => graphics::DrawMode::Line(STROKE),
                    _           => graphics::DrawMode::Fill,
                };
                graphics::set_color(ctx, graphics::Color::from_rgb(255, 255, 255))?;
                graphics::circle(ctx, graphics::DrawMode::Fill, to_point(row, col), RADIUS, 0.01)?;
                graphics::set_color(ctx, color)?;
                graphics::circle(ctx, fill, to_point(row, col), RADIUS, 0.01)?
            } 
        }
        if let Some(color) = self.board.was_won() {
            let winner     = if color == WHITE { "White" } else { "Black "};
            let over_text  = graphics::Text::new(ctx, "Game over!", &self.font)?;
            let win_text   = graphics::Text::new(ctx, &format!("{} won!", winner), &self.font)?;
            let (x, y)     = ((WIDTH / 2) as f32, (HEIGHT / 2) as f32);
            let (ox, oy)   = ((over_text.width() / 2) as f32, (over_text.height() / 2) as f32);
            let (wx, wy)   = ((win_text.width() / 2) as f32, (win_text.height() / 2) as f32);
            let over_point = graphics::Point2::new(x - ox, y - (oy * 2.25));
            let win_point  = graphics::Point2::new(x - wx, y + (wy * 0.75));
            let paint = if color == WHITE {
                graphics::Color::from_rgb(255, 255, 255)
            } else {
                graphics::Color::from_rgb(0, 0, 0)
            };
            graphics::set_color(ctx, paint)?;
            graphics::draw(ctx, &over_text, over_point, 0.0)?; 
            graphics::draw(ctx, &win_text, win_point, 0.0)?; 
        }
        graphics::present(ctx);
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self,
                        ctx: &mut Context,
                        keycode: Keycode,
                        _keymod: Mod,
                        _repeat: bool) {
        match keycode {
            Keycode::Num1 => {
                self.next = 0;
            }
            Keycode::Num2 => {
                self.next = 1;
            }
            Keycode::Num3 => {
                self.next = 2;
            }
            Keycode::Num4 => {
                self.next = 3;
            }
            Keycode::Num5 => {
                self.next = 4;
            }
            Keycode::Num6 => {
                self.next = 5;
            }
            Keycode::Num7 => {
                self.next = 6;
            }
            Keycode::Escape => ctx.quit().unwrap(),
            _ => return, // Do nothing
        }
        self.ready = true;
    }
}


pub fn main() {
    let cb = ContextBuilder::new("connect-four", "nwtnni")
        .window_setup(conf::WindowSetup::default()
                      .title("Connect Four"))
        .window_mode(conf::WindowMode::default()
                     .dimensions(WIDTH, HEIGHT));

    let ctx = &mut cb.build().unwrap();

    // Mount resources directory
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let mut path = PathBuf::from(manifest_dir);
    path.push("resources");
    ctx.filesystem.mount(&path, true);

    match Game::new(ctx) {
        Err(_) => {
            println!("Could not load game.");
        }
        Ok(ref mut game) => {
            run(ctx, game).unwrap();
        }
    }
}
