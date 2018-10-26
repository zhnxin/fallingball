use ggez::graphics::{self, DrawMode,Point2};
use ggez::{Context, GameResult};

use super::bar;
use super::timer::Timer;

#[derive(Debug)]
pub struct Block {
    pos: (f32,f32),
    width:f32,
    rect: graphics::Rect,
    time_bar: bar::Bar,
    timer_tick : Timer,
}

impl Block {
    pub fn new(alive_time:f64,pos:(f32,f32),width:f32)-> Block{
        let block_delta = width*0.1;
        Block{
            pos:pos,
            width:width,
            rect:graphics::Rect::new(pos.0+block_delta,pos.1+2.0*block_delta,8.0*block_delta,8.0*block_delta),
            time_bar: bar::Bar::new(pos.0,pos.1,width,block_delta),
            timer_tick: Timer::new(alive_time),
        }
    }

    pub fn restore(&mut self,alive_time:f64,pos:(f32,f32),width:f32){
        let block_delta = width*0.1;
        self.pos = pos;
        self.width =width;
        self.rect.x = pos.0 + block_delta;
        self.rect.y = pos.1 + 2.0*block_delta;
        self.rect.h = 8.0*block_delta;
        self.rect.w = 8.0*block_delta;
        self.timer_tick.restore(alive_time);
        self.time_bar.restore(pos.0,pos.1,width,block_delta);
        self.update_graphic();
    }

    pub fn is_hit_cricle(&self,point: Point2,radius:f32) -> bool {
        let pos = (self.pos.0 + self.width/2.0,self.pos.1+self.width * 0.6);
        (point.x-pos.0)*(point.x-pos.0) + (point.y - pos.1)*(point.y-pos.1) <= 0.48*self.width+radius
    }

    pub fn is_contains(&self,point: Point2)->bool{
        self.rect.contains(point)
    }

    pub fn is_stopped(&self)->bool{
        self.timer_tick.is_stopped()
    }

    pub fn get_event(&mut self)->bool{
        self.timer_tick.get_event()
    }

    fn update_graphic(&mut self) {
        let value = self.timer_tick.get_value();
        self.time_bar.set_value(1.0 - value);
    }

    pub fn update(&mut self,ctx:&Context){
        if self.timer_tick.is_to_updated(){
            self.timer_tick.update(ctx);
            self.update_graphic();
        }
    }

    pub fn stop(&mut self){
        self.timer_tick.stop();
    }
    pub fn start(&mut self,ctx:&Context){
        self.timer_tick.start(ctx);
    }

    pub fn draw(&self,ctx:&mut Context) ->GameResult<()>{
        if !self.timer_tick.is_stopped(){
            self.time_bar.draw(ctx)?;
            graphics::set_color(ctx,graphics::Color::from_rgb(54,100,139))?;
            graphics::rectangle(ctx,DrawMode::Fill,self.rect)?;
        }
        Ok(())
    }
}