extern crate ggez;
extern crate fallingball;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::timer;

use fallingball::bar::{self,Bar,TimerBar};

#[derive(Debug)]
struct MainState {
    font:graphics::Font,
    timer_bar: TimerBar,
    power_record_bar: TimerBar,
    bar: Bar,
    vel_bar: Bar,
}

impl MainState {
     fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(_ctx, "/DejaVuSerif.ttf", 16)?;
        let window_size = graphics::get_size(_ctx);
        let delta_width = (window_size.0 / 100 ) as f32;
        let mut s = MainState{
            font:font,
            timer_bar: TimerBar::new(5.0,delta_width*30f32,window_size.1 as f32 - 4.0 *delta_width,30f32*delta_width,2.0*delta_width ),
            power_record_bar:TimerBar::new(5.0,delta_width*30f32,window_size.1 as f32 - 36.0 *delta_width,30f32*delta_width,2.0*delta_width),
            bar: Bar::new(delta_width*89f32,window_size.1 as f32 - 2.0 *delta_width,10f32*delta_width,delta_width ),
            vel_bar: Bar::new(delta_width*98f32,window_size.1 as f32 - 15.0 *delta_width,10f32*delta_width,delta_width),
        };
        s.power_record_bar.set_mode(bar::TimeBarMode::Increase);
        s.power_record_bar.set_direction(bar::BarDirection::Vertical);
        s.vel_bar.set_direction(bar::BarDirection::Vertical);
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.timer_bar.get_event(){
            println!("timer stop{:?}",std::time::Instant::now());
        }
        self.timer_bar.update(ctx);
        self.power_record_bar.update(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_background_color(ctx, graphics::Color::new(1.0,1.0,1.0,1.0));
        self.timer_bar.draw(ctx)?;
        self.power_record_bar.draw(ctx)?;
        self.bar.draw(ctx)?;
        self.vel_bar.draw(ctx)?;
        let s = format!("Power Record: {}", self.power_record_bar.get_value());
        let text = graphics::Text::new(ctx, s.as_str(), &self.font)?;
        let window_size = graphics::get_size(ctx);
        graphics::set_color(ctx,graphics::Color::new(0f32,0f32,0f32,1f32))?;
        graphics::draw(ctx, &text, graphics::Point2::new(window_size.0 as f32 *0.7,0f32),0.0)?;
        graphics::present(ctx);
        timer::yield_now();
        Ok(())
    }
    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: event::MouseButton, _x: i32, _y: i32) {
        if button == event::MouseButton::Left{
            let window_size = graphics::get_size(ctx);
            self.bar.set_value(1f32 - _x as f32 /window_size.0 as f32);
            self.vel_bar.set_value(1f32 - _y as f32 /window_size.1 as f32);
            self.power_record_bar.start(ctx);
            println!("mouse_button_down:{:?}",std::time::Instant::now());
        }
    
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: event::MouseButton, _x: i32, _y: i32) {
        println!("mouse up{:?}",std::time::Instant::now());
        match button {
            event::MouseButton::Left => {
                if self.timer_bar.is_stopped(){
                    println!("timer start{:?}",std::time::Instant::now());
                    self.timer_bar.start(ctx);
                }
                self.power_record_bar.pause();
            },
            _ =>{},
        }
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("falling ball", "ggez", c).unwrap();
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}