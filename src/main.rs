extern crate ggez;
extern crate minimax;

use ggez::*;
use ggez::event::*;
use ggez::graphics::{DrawMode, Point2};

use minimax::board::*;
use minimax::minimax::AI;
use minimax::engine::*;

struct Game {
    ai: AI,
    board: Board,
    ready: bool,
    done: bool,
    next: u8,
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
            ready: true,
            done: true,
            next: 8,
        }) 
    }

}

fn to_point(row: u8, col: u8) -> graphics::Point2 {
    let col = col as f32;
    let row = (ROWS - row - 1) as f32;
    graphics::Point2::new(
        OFFSET + (OFFSET + RADIUS)*col,
        OFFSET + (OFFSET + RADIUS)*row,
    )
}

impl event::EventHandler for Game {
    fn update(&mut self, _: &mut Context) -> GameResult<()> {
        if self.ready && self.done && self.next < 8 {
            self.board.make_move(self.next);
            self.ready = false;
            self.done = false;
            self.next = 8;
        } else if self.next < 8 {
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

    match Game::new(ctx) {
        Err(_) => {
            println!("Could not load game.");
        }
        Ok(ref mut game) => {
            run(ctx, game).unwrap();
        }
    }

}
