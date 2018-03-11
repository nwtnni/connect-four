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

const RADIUS: f32 = 30.0;
const OFFSET: f32 = 40.0;

impl Game {
    fn new(ctx: &mut Context) -> GameResult<Game> {
        graphics::set_background_color(ctx, (0, 0, 0, 255).into());

        Ok(Game {
            ai: AI::new(),
            board: Board::new(),
            ready: false,
            done: false,
            next: 0,
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
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.ready && self.done {
            // self.board.make_move(self.next);
            // self.ready = false;
            // self.done = false;
        } else {
            // let next = self.ai.solve(&mut self.board);
            // self.board.make_move(next);
            // self.ai.reset();
            // self.done = true;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        
        for row in (0..ROWS).rev() {
            for col in 0..COLS {
                graphics::circle(
                    ctx,
                    graphics::DrawMode::Fill,
                    to_point(row, col),
                    RADIUS,
                    0.01
                )?
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
            _ => (), // Do nothing
        }
        self.done = true;
    }
}


pub fn main() {
    let mut cb = ContextBuilder::new("connect-four", "nwtnni")
        .window_setup(conf::WindowSetup::default()
                      .title("Connect Four"))
        .window_mode(conf::WindowMode::default()
                     .dimensions(WIDTH, HEIGHT));

    let ctx = &mut cb.build().unwrap();

    match Game::new(ctx) {
        Err(e) => {
            println!("Could not load game.");
        }
        Ok(ref mut game) => {
            let result = run(ctx, game).unwrap();
        }
    }

}
