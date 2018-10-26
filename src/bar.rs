use ggez::graphics::{self, DrawMode};
use ggez::{Context, GameResult};

use super::timer::Timer;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum BarDirection {
    Vertical = 0,
    Horizontal = 1,
}

#[derive(Debug)]
pub struct Bar {
    rim_rect:graphics::Rect,
    value_rect:graphics::Rect,
    value:f32,
    color: graphics::Color,
    direction : BarDirection,
}

impl Bar {
    pub fn new(x: f32, y: f32, length: f32, width: f32) -> Bar {
        Bar{
            rim_rect:graphics::Rect::new(x,y,length,width),
            value_rect:graphics::Rect::new(x+1f32,y+1f32,length-2f32,width-2f32),
            value:1.0,
            color:graphics::Color::new(0.0,1.0,0.0,1.0),
            direction: BarDirection::Horizontal,
        }
    }

    fn update_graphic(&mut self){
        match self.direction {
            BarDirection::Horizontal => {
                self.value_rect.w = (self.rim_rect.w-1.0)*self.value;
                self.value_rect.h = self.rim_rect.h - 2.0;
                },
            BarDirection::Vertical => {
                self.value_rect.h = (self.rim_rect.h-2.0)*self.value;
                self.value_rect.y = self.rim_rect.y+1.0+(self.rim_rect.h-2.0)*(1.0 - self.value);
                self.value_rect.w = self.rim_rect.w - 2.0;
            },
        }
    }

    pub fn set_size(&mut self,length: f32, width: f32){
        if self.direction == BarDirection::Horizontal{
            self.rim_rect.w = length;
            self.rim_rect.h = width;
        }else{
            self.rim_rect.w = width;
            self.rim_rect.h = length;
        }
        self.update_graphic();
    }

    pub fn restore(&mut self,x: f32, y: f32, length: f32, width: f32){
        self.value = 1.0;
        self.rim_rect.x = x;
        self.rim_rect.y = y;
        self.set_size(length,width);
    }

    pub fn set_direction(&mut self,direction:BarDirection){
        if self.direction != direction{
            let (w,h) = (self.rim_rect.w,self.rim_rect.h);
            self.rim_rect.h = w;
            self.rim_rect.w = h;
            self.direction = direction;
            self.update_graphic();
        }
    }

    pub fn set_color(&mut self,color: graphics::Color){
        self.color = color;
    }

    pub fn set_value(&mut self,value:f32){
        if value >= 0f32 && value <=1f32{
            self.value = value;
            self.update_graphic();
        }
    }

    pub fn get_value(&self) -> f32{
        self.value
    }

    pub fn draw(&self,ctx:&mut Context) ->GameResult<()>{
        graphics::set_color(ctx, graphics::Color::new(0.74218,0.74218,0.74218,1.0))?;
        graphics::rectangle(ctx,DrawMode::Fill,self.rim_rect)?;
        if self.value > 0f32{
            graphics::set_color(ctx, self.color)?;
            graphics::rectangle(ctx,DrawMode::Fill,self.value_rect)?;
        }
        Ok(())
    }

}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum TimeBarMode {
    IncreaseAutoDisappear = 0,
    DecreaseAutoDisappear = 1,
    Increase = 2,
    Decrease = 3,
}

#[derive(Debug)]
pub struct TimerBar {
    bar : Bar,
    time_tick: Timer,
    mode: TimeBarMode,
}

impl TimerBar{
    pub fn new(duration:f64,x: f32, y: f32, w: f32, h: f32)-> TimerBar {
        TimerBar{
            bar: Bar::new(x,y,w,h),
            time_tick:Timer::new(duration),
            mode: TimeBarMode::DecreaseAutoDisappear,
        }
    }

    fn update_value(&mut self){
        let value = self.get_value();
        match self.mode {
            TimeBarMode::Increase|TimeBarMode::IncreaseAutoDisappear=> {
                self.bar.set_value(value);
            },
            TimeBarMode::Decrease|TimeBarMode::DecreaseAutoDisappear=>{
                self.bar.set_value(1.0-value);
            }
        }
    }

    pub fn set_mode(&mut self,mode: TimeBarMode){
        self.update_value();
        self.mode = mode;
    }

    pub fn set_direction(&mut self,direction: BarDirection){
        self.bar.set_direction(direction);
    }

    pub fn set_color(&mut self,color:graphics::Color){
        self.bar.set_color(color);
    }

    pub fn get_value(&self)->f32{
        self.time_tick.get_value()
    }

    pub fn get_event(&mut self) ->bool{
        self.time_tick.get_event()
    }

    pub fn pause(&mut self){
        self.time_tick.pause();
    }

    pub fn is_paused(&self) ->bool{
        self.time_tick.is_paused()
    }


    pub fn is_stopped(&self) ->bool{
        self.time_tick.is_stopped()
    }

    pub fn start(&mut self,ctx:&Context){
        self.time_tick.start(ctx);
        self.update_value();
    }

    pub fn restore(&mut self,duration:f64,x: f32, y: f32, w: f32, h: f32){
        self.bar.restore(x,y,w,h);
        self.time_tick.restore(duration);
    }

    pub fn update(&mut self,ctx: &Context) {
        if !self.is_paused() && !self.is_stopped() {
            self.time_tick.update(ctx);
            self.update_value();
        }
    }

    pub fn draw(&self,ctx:&mut Context) ->GameResult<()> {
        match self.mode {
            TimeBarMode::Decrease|TimeBarMode::Increase => {
                self.bar.draw(ctx)?;
            },
            _ => {
                if !self.is_stopped(){self.bar.draw(ctx)?;}
            },
        }
        Ok(())
    }
}