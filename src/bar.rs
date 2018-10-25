use ggez::graphics::{self, DrawMode};
use ggez::{Context, GameResult};
use ggez::timer;
use std::time;

#[derive(Debug)]
pub struct HorizontalBar {
    rim_rect:graphics::Rect,
    value_rect:graphics::Rect,
    value:f32,
}

impl HorizontalBar {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> HorizontalBar {
        HorizontalBar{
            rim_rect:graphics::Rect::new(x,y,w,h),
            value_rect:graphics::Rect::new(x+1f32,y+1f32,w-2f32,h-2f32),
            value:1.0,
        }
    }

    pub fn restore(&mut self,x: f32, y: f32, w: f32, h: f32){
        self.value = 1.0;
        self.rim_rect.x =x;self.rim_rect.y = y;
        self.rim_rect.w =w;self.rim_rect.h = h;
        self.rim_rect.x =x +1f32;self.rim_rect.y = y +1f32;
        self.rim_rect.w =w -2f32;self.rim_rect.h = h -2f32;

    }

    pub fn set_value(&mut self,value:f32){
        self.value = value;
        if value > 0f32 {
            self.value_rect.w = (self.rim_rect.w-1.0)*self.value;
        }
    }

    pub fn get_value(&self) -> f32{
        self.value
    }

    pub fn draw(&self,ctx:&mut Context) ->GameResult<()>{
        graphics::set_color(ctx, graphics::Color::new(0.74218,0.74218,0.74218,1.0))?;
        graphics::rectangle(ctx,DrawMode::Fill,self.rim_rect)?;
        if self.value > 0f32{
            graphics::set_color(ctx, graphics::Color::new(0.0,1.0,0.0,1.0))?;
            graphics::rectangle(ctx,DrawMode::Fill,self.value_rect)?;
        }
        Ok(())
    }

}

#[derive(Debug)]
pub struct TimerHorizontalBar {
    bar : HorizontalBar,
    duration : f64,
    started : time::Duration,
    event_flag: (bool,bool),
}

impl TimerHorizontalBar{
    pub fn new(duration:f64,x: f32, y: f32, w: f32, h: f32)-> TimerHorizontalBar {
        TimerHorizontalBar{
            bar: HorizontalBar::new(x,y,w,h),
            duration: duration,
            started: time::Duration::new(0,0),
            event_flag:(true,true),
        }
    }

    fn set_value(&mut self,value:f32){
        self.event_flag.0 = value <= 0f32;
        self.bar.set_value(value);
    }

    pub fn get_event(&mut self) ->bool{
        if self.event_flag.0 && ! self.event_flag.1{
            self.event_flag.1 = true;
            return true;
        }
        return false;
    }


    pub fn is_disable(&self) ->bool{
        self.event_flag.0 && self.event_flag.1
    }

    pub fn start(&mut self,ctx:&Context){
        self.event_flag.0 = false;
        self.event_flag.1 = false;
        self.started = timer::get_time_since_start(ctx);
        self.bar.set_value(1.0);
    }

    pub fn restore(&mut self,duration:f64,x: f32, y: f32, w: f32, h: f32){
        self.bar.restore(x,y,w,h);
        self.duration = duration;
    }

    pub fn update(&mut self,ctx: &Context) {
        if !self.is_disable() {
            let time_passed = timer::duration_to_f64(timer::get_time_since_start(ctx)) - timer::duration_to_f64(self.started);
            let value = 1.0 - time_passed / self.duration;
            self.set_value(value as f32);
        }
    }

    pub fn draw(&self,ctx:&mut Context) ->GameResult<()> {
        if !self.is_disable(){
            self.bar.draw(ctx)?;
        }
        Ok(())
    }
}