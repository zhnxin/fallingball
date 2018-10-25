extern crate ggez;
extern crate fallingball;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::timer;

use fallingball::bar::{HorizontalBar,TimerHorizontalBar};

#[derive(Debug)]
struct MainState {
    timer_bar: TimerHorizontalBar,
    bar: HorizontalBar,
}

impl MainState {
     fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let window_size = graphics::get_size(_ctx);
        let delta_width = (window_size.0 / 100 ) as f32;
        let s = MainState{
            timer_bar: TimerHorizontalBar::new(5.0,delta_width*30f32,window_size.1 as f32 - 4.0 *delta_width,30f32*delta_width,2.0*delta_width ),
            bar: HorizontalBar::new(delta_width*89f32,window_size.1 as f32 - 2.0 *delta_width,10f32*delta_width,delta_width ),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.timer_bar.get_event(){
            println!("timer stop{:?}",std::time::Instant::now());
        }
        self.timer_bar.update(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_background_color(ctx, graphics::Color::new(1.0,1.0,1.0,1.0));
        self.timer_bar.draw(ctx)?;
        self.bar.draw(ctx)?;
        graphics::present(ctx);
        timer::yield_now();
        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: event::MouseButton, x: i32, y: i32) {
        match button {
            event::MouseButton::Left => {
                if self.timer_bar.is_disable(){
                    println!("timer start{:?}",std::time::Instant::now());
                    self.timer_bar.start(_ctx);
                }
                let window_size = graphics::get_size(_ctx);
                self.bar.set_value(1f32 - y as f32 /window_size.1 as f32);
            },
            _ =>{},
        }
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("falling ball", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}