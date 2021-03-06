use std::time;
use ggez::timer;
use ggez::Context;

#[derive(Debug)]
pub struct Timer {
    duration : f64,
    started : time::Duration,
    paused: f64,
    event_flag: (bool,bool),
    value: f32,
}

impl Timer {
    pub fn new(duration: f64) -> Timer {
        Timer{
            duration:duration,
            started: time::Duration::new(0,0),
            paused: 0f64,
            event_flag:(true,true),
            value:0f32,
        }
    }

    pub fn get_value(&self) ->f32{
        self.value
    }

    pub fn get_event(&mut self) ->bool{
        if self.event_flag.0 && ! self.event_flag.1{
            self.event_flag.1 = true;
            return true;
        }
        return false;
    }

    pub fn pause(&mut self){
        self.event_flag = (false,true);
    }

    pub fn pause_triggle(&mut self){
        self.event_flag = (false,!self.event_flag.1);
    }

    pub fn is_paused(&self) ->bool{
        !self.event_flag.0 && self.event_flag.1
    }

    pub fn stop(&mut self){
        self.event_flag = (true,true);
    }

    pub fn is_stopped(&self) ->bool{
        self.event_flag.0 && self.event_flag.1
    }

    pub fn on_start(&self) ->bool{
        !(self.event_flag.1 || self.event_flag.0)
    }

    pub fn start(&mut self,ctx:&Context){
        self.value = 0f32;
        self.event_flag = (false,false);
        self.started = timer::get_time_since_start(ctx);
        self.paused = 0f64;
    }

    pub fn restore(&mut self,duration:f64){
        self.value = 0f32;
        self.duration = duration;
    }

    pub fn update(&mut self,ctx: &Context) {
        if self.on_start() {
            let time_passed = timer::duration_to_f64(timer::get_time_since_start(ctx)) - timer::duration_to_f64(self.started) - self.paused;
            self.value = (time_passed / self.duration)as f32;
            self.event_flag.0 = self.value > 1f32;
            return;
        }
        if self.is_paused(){
            self.paused = timer::duration_to_f64(timer::get_time_since_start(ctx)) - timer::duration_to_f64(self.started)*self.value as f64;
            return;
        }
    }
}