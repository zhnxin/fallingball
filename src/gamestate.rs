use ggez::{event,graphics,Context, GameResult,timer,audio};
use rand::{self,thread_rng, Rng};

use super::{block::Block,timer::Timer,ball::{Ball},bar};

const BALL_PERIOD:f64 = 1f64;
const BALL_MAX_TIME: f32 = 1.5f32;

const  BLOCK_COUNT: usize = 8;
const BLOCK_NUM: usize = BLOCK_COUNT*BLOCK_COUNT;
const BLOCK_ALIVE:f64 = 10f64;
const BLOCK_GENERATE: f64 = 5f64;

const RELOADING_TIME:f64 = 1f64;

#[derive(Debug)]
struct StateBlock {
    block: Block,
    index:usize,
}

#[derive(Debug)]
struct StateBall {
    ball: Ball,
    extra_live_timer:Timer,
}


impl StateBall{

    fn update(&mut self,ctx:&Context,time_delta:f32){
        self.ball.update(time_delta);
        self.extra_live_timer.update(ctx);
        self.extra_live_timer.get_event();
    }
}
#[derive(Debug)]
struct SouldEffects {
    energy_up:audio::Source,
    boom : audio::Source,
    shot : audio::Source,
    loss : audio::Source,
}

impl SouldEffects{
    fn new(ctx :&mut Context)->SouldEffects{
        SouldEffects{
            energy_up: audio::Source::new(ctx, "/energy_charge.ogg").unwrap(),
            boom: audio::Source::new(ctx, "/boom.ogg").unwrap(),
            shot: audio::Source::new(ctx, "/pew.ogg").unwrap(),
            loss: audio::Source::new(ctx, "/loss.ogg").unwrap(),
        }
    }

    fn energy_charge_reload(&mut self,ctx:&mut Context){
        self.energy_up = audio::Source::new(ctx, "/energy_charge.ogg").unwrap();
    }
}



#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum GameStatus {
    Ready = 0,
    Running = 1,
    GameOver = 2,
}

pub struct GameState {
    font:graphics::Font,
    ball_ready_timer: Timer,
    block_generate_time_ticker: Timer,
    power_record_bar: bar::TimerBar,
    rng : rand::ThreadRng,
    block_list: Vec<StateBlock>,
    block_index: [bool;BLOCK_NUM],
    ball_list: Vec<StateBall>,

    delta_length:f32,
    window_size: (f32,f32),

    status: GameStatus,
    score: usize,
    left:usize,

    sould_effects: SouldEffects,
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> GameResult<GameState> {
        let window_size = graphics::get_size(_ctx);
        let delta_length = window_size.1 as f32/(BLOCK_COUNT+8) as f32;
        let font = graphics::Font::new(_ctx, "/DejaVuSerif.ttf", 12)?;
        let mut s = GameState{
            ball_ready_timer: Timer::new(BALL_PERIOD),
            block_generate_time_ticker: Timer::new(BLOCK_ALIVE),
            power_record_bar: bar::TimerBar::new(
                RELOADING_TIME,
                window_size.0 as f32 - delta_length*0.5,
                window_size.1 as f32 - delta_length*3.2,
                delta_length*3.0,
                delta_length*0.4
                 ),
            rng:thread_rng(),
            block_list: vec![],
            block_index:[false;BLOCK_NUM],
            ball_list: vec![],

            font: font,
            delta_length: window_size.1 as f32/(BLOCK_COUNT+8) as f32,
            window_size: (window_size.0 as f32,window_size.1 as f32),

            left:5,
            score:0,
            status: GameStatus::Ready,

            sould_effects: SouldEffects::new(_ctx),
        };
        s.power_record_bar.set_direction(bar::BarDirection::Vertical);
        s.power_record_bar.set_increase(true);
        s.restore_timer();
        Ok(s)
    }

    fn restore_timer(&mut self){
        let count = self.block_index.iter().filter(|&&x| x).count();
        if count == 0{
            self.block_generate_time_ticker.restore(0.1);
        }else{
            let count = BLOCK_GENERATE*count.min(10) as f64;
            let duration = self.rng.gen_range(count,3.0 + count);
            self.block_generate_time_ticker.restore(duration);
        }
    }

    fn get_left_point(&self)->f32{
        (self.window_size.0 as f32- BLOCK_COUNT as f32 * self.delta_length) as f32 /2f32
    }

    fn get_ball_max_range(&self)->f32{
        let w = self.delta_length*BLOCK_COUNT as f32/2.0;
        let h = self.delta_length*(6+BLOCK_COUNT) as f32;
        (w*w+h*h).sqrt()
    }

    fn get_ball_max_vel(&self) ->f32{
        self.get_ball_max_range()/BALL_MAX_TIME
    }

    fn game_over(&mut self){
        self.block_generate_time_ticker.stop();
        self.ball_ready_timer.stop();
    }

    fn game_restart(&mut self,ctx:&Context){
            self.status = GameStatus::Running;
            self.score = 0;
            self.left = 5;
            self.restore_timer();
            self.block_generate_time_ticker.start(ctx);
    }

    fn random_block(&mut self,ctx:&Context){
        let count = self.block_index.iter().filter(|&&x| !x).count();
        if count > 0{
            let mut index = self.rng.gen_range(0usize,count);
            let mut count = 0;
            for (i,v) in self.block_index.iter().enumerate(){
                if *v{
                    if count == index{
                        index = i;
                        break;
                    }
                    count +=1;
                }
            }
            self.block_index[index] = true;
            let pos = (
                    (index%BLOCK_COUNT) as f32*self.delta_length+ self.get_left_point(),
                    (index/BLOCK_COUNT +2 ) as f32* self.delta_length,
                );
            if let Some(block_item) = self.block_list.iter_mut().find(|b| b.block.is_stopped()){
                block_item.block.restore(BLOCK_ALIVE,pos,self.delta_length);
                block_item.block.start(ctx);
                block_item.index = index;
                return;
            }
            let mut block_item = StateBlock{
                index: index,
                block:Block::new(BLOCK_ALIVE,pos,self.delta_length),
            };
            block_item.block.start(ctx);
            self.block_list.push(block_item);
        }
    }

    fn throw_ball(&mut self,(x,y):(f32,f32),ctx:&Context){
        let point = (x - self.window_size.0/2.0,y -self.window_size.1);
        let point_len = (point.0*point.0+point.1*point.1).sqrt();
        let max_vel = self.get_ball_max_vel();
        let power = self.power_record_bar.get_value();
        let power = Ball::get_vel_alpha(power,BALL_MAX_TIME);
        let radius = self.delta_length*0.2;
        let b_pos = (self.window_size.0/2.0,self.window_size.1);
        let vel = (point.0/point_len*max_vel*power,point.1/point_len*max_vel*power,power);
        if let Some(b) = self.ball_list.iter_mut().find(|b| !b.ball.is_avtive()){
            b.ball.restore(radius,b_pos,vel);
            self.ball_ready_timer.start(ctx);
            return;
        }
        let b = StateBall{
            ball:Ball::new(radius,b_pos,vel),
            extra_live_timer: Timer::new(0.5),
            };
        self.ball_list.push(b);
        self.ball_ready_timer.start(ctx);
    }

    fn update_running(&mut self,ctx:&mut Context) -> GameResult<()>{
        self.block_generate_time_ticker.update(ctx);
        self.ball_ready_timer.update(ctx);
        if self.ball_ready_timer.get_event(){
            //ready sould
        }
        self.power_record_bar.update(ctx);
        if self.power_record_bar.get_event(){
            self.sould_effects.energy_up.stop();
            self.sould_effects.energy_charge_reload(ctx);
        }

        if self.block_generate_time_ticker.get_event(){
            self.random_block(ctx);
            self.restore_timer();
            self.block_generate_time_ticker.start(ctx);
        }

        let delta_time =  ((timer::get_delta(ctx)).subsec_millis() as f32)/1.0e3; 
        self.ball_list.iter_mut().for_each(|b| b.update(ctx,delta_time));
        self.block_list.iter_mut().for_each(|b| b.block.update(ctx));
        //update block status
        for b in self.block_list.iter_mut(){
            if b.block.get_event(){
                self.block_index[b.index] = false;
                if self.left == 0{
                    self.status = GameStatus::GameOver;
                    break;
                }
                self.left -=1;
            }
        }
        if self.status == GameStatus::GameOver{
            self.game_over();
        }

        //if ball fall fown on ground
        let mut sould_hit :u8= 0;
        for b in self.ball_list.iter_mut().filter(|b| b.ball.is_avtive() && b.ball.is_on_ground()){
            b.ball.disable();
            sould_hit = sould_hit.max(1);
            let mut is_hit = false;
            for bk in self.block_list.iter_mut().filter(|bk| {
                !bk.block.is_stopped() && bk.block.is_hit_cricle(b.ball.get_pos(),b.ball.get_radius())
            }){
                is_hit = true;
                sould_hit = 2;
                bk.block.stop();
                self.block_index[bk.index] = false;
                self.score += 1;
            }
            if !is_hit{
                b.extra_live_timer.start(ctx);
            }
        }

        match sould_hit{
            1 => {self.sould_effects.loss.play()?;},
            2 => {self.sould_effects.boom.play()?;},
            _=>{},
        }
        Ok(())
    }

    fn draw_game_ready(&mut self,ctx:&mut Context) -> GameResult<()>{
        let s = "click to start";
        let dest_point = graphics::Point2::new(
            self.window_size.0 /2.0 - self.font.get_width(&s) as f32 /2.0,
            self.window_size.1/2.0 -self.delta_length);
        let text = graphics::Text::new(ctx, &s, &self.font)?;
        graphics::set_color(ctx,graphics::Color::from_rgb(0,0,0))?;
        graphics::draw(ctx, &text, dest_point, 0.0)?;
        Ok(())
    }

    fn draw_game_over(&mut self,ctx:&mut Context) -> GameResult<()>{
        let s = format!("YOU GOT:{}",self.score);
        let mut dest_point = graphics::Point2::new(
            self.window_size.0 /2.0 - self.font.get_width(&s) as f32 /2.0,
            self.window_size.1/2.0 -self.delta_length);
        let text = graphics::Text::new(ctx, &s, &self.font)?;
        graphics::set_color(ctx,graphics::Color::from_rgb(0,205,102))?;
        graphics::draw(ctx, &text, dest_point, 0.0)?;

        let s = "click to restart";
        dest_point.x = self.window_size.0 /2.0 - self.font.get_width(&s) as f32 /2.0;
        dest_point.y = self.window_size.1/2.0 +self.delta_length;
        let text = graphics::Text::new(ctx, &s, &self.font)?;
        graphics::set_color(ctx,graphics::Color::from_rgb(0,0,0))?;
        graphics::draw(ctx, &text, dest_point, 0.0)?;
        Ok(())
    }

    fn draw_game_running(&mut self,ctx:&mut Context) -> GameResult<()>{

        //draw the rim
        let mut rect = graphics::Rect::new(
            self.get_left_point()-1.0 ,
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
        let dest_point = graphics::Point2::new(
            self.window_size.0  - self.font.get_width(&s) as f32 - self.delta_length,
            0.2*self.delta_length);
        let text = graphics::Text::new(ctx, s.as_str(), &self.font)?;
        graphics::set_color(ctx,graphics::Color::from_rgb(110,123,139))?;
        graphics::draw(ctx, &text, dest_point, 0.0)?;



        //draw power bar
        self.power_record_bar.draw(ctx)?;

        //draw the flag ball
        let ball_pos = graphics::Point2::new(self.window_size.0/2.0,self.window_size.1);
        graphics::set_color(ctx,graphics::Color::from_rgb(112,128,144))?;
        if self.ball_ready_timer.is_stopped(){
            graphics::circle(ctx,graphics::DrawMode::Fill,ball_pos,self.delta_length*0.3,1.0)?;
        }else{
            graphics::circle(ctx,graphics::DrawMode::Line(1.0),ball_pos,self.delta_length*0.3,1.0)?;
        }

        //draw block
        for b in self.block_list.iter(){
            b.block.draw(ctx)?;
        }
        //draw the flying ball
        for b in self.ball_list.iter().filter(|b| {
            (b.ball.is_avtive()|| !b.extra_live_timer.is_stopped())&&b.ball.get_pos().y > self.delta_length
            }){
            b.ball.draw(ctx)?;
        }

        Ok(())
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.update_running(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_background_color(ctx,graphics::Color::new(1f32,1f32,1f32,1f32));
        match self.status {
            GameStatus::Ready => {self.draw_game_ready(ctx)?;},
            GameStatus::Running => {self.draw_game_running(ctx)?;},
            GameStatus::GameOver =>{self.draw_game_over(ctx)?;},
        }        
        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: event::MouseButton, _x: i32, _y: i32) {
        if self.status == GameStatus::Running && button == event::MouseButton::Left{
            self.power_record_bar.start(_ctx);
            
            self.sould_effects.energy_up.play().unwrap();
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: event::MouseButton, x: i32, y: i32) {        
        
        self.power_record_bar.update(_ctx);
        self.power_record_bar.pause();
        if self.status == GameStatus::Running && button == event::MouseButton::Left && self.ball_ready_timer.is_stopped(){
            //sould effetc
            self.sould_effects.shot.play().unwrap();
            if self.sould_effects.energy_up.playing(){
                self.sould_effects.energy_up.stop();
                self.sould_effects.energy_charge_reload(_ctx);
            }
            self.throw_ball((x as f32,y as f32),_ctx);
            
        }

        if self.status != GameStatus::Running && button == event::MouseButton::Left{
            self.game_restart(_ctx);
        }
    }
}