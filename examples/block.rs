extern crate ggez;
extern crate fallingball;
extern crate rand;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::timer;

use rand::{thread_rng, Rng};

use fallingball::block::Block;
use fallingball::timer::Timer;

#[derive(Debug)]
struct StateBlock {
    state_block: Block,
    index:usize,
}

const BLOCK_COUNT :usize = 8;
const BLOCK_NUM: usize = BLOCK_COUNT*BLOCK_COUNT;
const BLOCK_ALIVE: f64 = 2.0;

struct MainState {
    timer_tick:Timer,
    rng : rand::ThreadRng,
    block_list: Vec<StateBlock>,
    block_index: [bool;BLOCK_NUM],
    delta_length: f32,
    left: usize,
    score: usize,
    font: graphics::Font,
    lost_count:usize,
    is_start:bool,
}

impl MainState {
     fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let window_size = graphics::get_size(_ctx);
        let font = graphics::Font::new(_ctx, "/DejaVuSerif.ttf", 12)?;
        let mut s = MainState{
            font: font,
            timer_tick: Timer::new(5f64),
            rng:thread_rng(),
            block_list: vec![],
            block_index:[false;BLOCK_NUM],
            delta_length: window_size.1 as f32/(BLOCK_COUNT+8) as f32,
            left:5,
            score:0,
            lost_count:0,
            is_start:false,
        };
        s.restore_timer();
        Ok(s)
    }

    fn restore_timer(&mut self){
        let count = self.block_index.iter().filter(|&&x| x).count();
        let count = 1.0*count.min(5) as f64;
        let duration = self.rng.gen_range(0.1+count,1.0 + count);
        self.timer_tick.restore(duration);
    }

    fn get_left_point(&self,ctx:&Context)->f32{
        let window_size = graphics::get_size(ctx);
        (window_size.0 as f32- BLOCK_COUNT as f32 * self.delta_length) as f32 /2f32
    }

    fn random_block(&mut self,ctx:&Context){
        let count = self.block_index.iter().filter(|&&x| !x).count();
        if count > 0{
            let mut index = self.rng.gen_range(0usize,count);
            let mut count = 0;
            for (i,v) in self.block_index.iter_mut().enumerate(){
                if *v{
                    if count == index{
                        index = i;
                        *v = true;
                        break;
                    }
                    count +=1;
                }
            }
            let pos = (
                    (index%BLOCK_COUNT) as f32*self.delta_length+ self.get_left_point(ctx),
                    (index/BLOCK_COUNT +2 ) as f32* self.delta_length,
                );
            if let Some(block_item) = self.block_list.iter_mut().find(|b| b.state_block.is_stopped()){
                block_item.state_block.restore(BLOCK_ALIVE,pos,self.delta_length);
                block_item.state_block.start(ctx);
                block_item.index = index;
                return;
            }
            let mut block_item = StateBlock{
                index: index,
                state_block:Block::new(BLOCK_ALIVE,pos,self.delta_length),
            };
            block_item.state_block.start(ctx);
            self.block_list.push(block_item);
        }
    }

}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.timer_tick.update(ctx);
        self.block_list.iter_mut().for_each(|b| b.state_block.update(ctx));
        for b in self.block_list.iter_mut(){
            if b.state_block.get_event(){
                self.block_index[b.index] = false;
                self.lost_count +=1;
                if self.left >0 {self.left -=1;}
            }
        }
        if self.left == 0{
            self.left = 5;
        }
        if self.timer_tick.get_event(){
            self.random_block(ctx);
            self.restore_timer();
            self.timer_tick.start(ctx);
        }
        timer::yield_now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let window_size = graphics::get_size(ctx);
        graphics::clear(ctx);
        graphics::set_background_color(ctx,graphics::Color::new(1f32,1f32,1f32,1f32));
        //draw the rim
        let mut rect = graphics::Rect::new(
            self.get_left_point(ctx)-1.0 ,
            2.0*self.delta_length-1.0,
            self.delta_length*BLOCK_COUNT as f32 +2.0,
            self.delta_length*BLOCK_COUNT as f32 +2.0
            );
        graphics::rectangle(ctx,graphics::DrawMode::Line(1.0),rect)?;
        //draw the left life
        graphics::set_color(ctx,graphics::Color::from_rgb(0,205,205))?;
        rect.y = 0.1*self.delta_length;rect.w = self.delta_length*0.6;rect.h=self.delta_length*0.6;
        for i in 0..self.left{
            rect.x = i as f32* self.delta_length*0.7 + 0.1*self.delta_length ;
            graphics::rectangle(ctx,graphics::DrawMode::Fill,rect)?;
        }
        //draw the score
        let s = format!("Score: {}", self.score);
        let mut dest_point = graphics::Point2::new(
            (window_size.0 as usize - self.font.get_width(&s)) as f32 - 0.1*self.delta_length,
            0.1*self.delta_length);
        let text = graphics::Text::new(ctx, s.as_str(), &self.font)?;
        graphics::set_color(ctx,graphics::Color::from_rgb(110,123,139))?;
        graphics::draw(ctx, &text, dest_point, 0.0)?;

        let s = format!("Lost block:{}",self.lost_count);
        dest_point.x = 3.6 * self.delta_length;
        let text = graphics::Text::new(ctx, s.as_str(), &self.font)?;
        graphics::draw(ctx, &text, dest_point, 0.0)?;
        if !self.is_start{
            let s = "PAUSE(click to start)";
            dest_point.x =  ((window_size.0 as usize- self.font.get_width(&s))/2 ) as f32;
            let text = graphics::Text::new(ctx, &s, &self.font)?;
            graphics::set_color(ctx,graphics::Color::from_rgb(255,0,0))?;
            graphics::draw(ctx, &text, dest_point, 0.0)?;
        }
        //draw the block
        for b in self.block_list.iter(){
            b.state_block.draw(ctx)?;
        }
        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: event::MouseButton, x: i32, y: i32) {
        use graphics::Point2;
        match button {
            event::MouseButton::Left => {
                if !self.is_start{
                    self.is_start = true;
                    self.timer_tick.start(_ctx);
                }
                let point = Point2::new(x as f32,y as f32);
                for b in self.block_list.iter_mut().filter(|b| b.state_block.is_contains(point)){
                    b.state_block.stop();
                    self.block_index[b.index] = false;
                    self.score += 1;
                };              
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