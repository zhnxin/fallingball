use ggez::graphics::{self, DrawMode, Point2};
use ggez::{Context, GameResult};

const GRAVITY: f32 = -1.5f32;
const BALL_HIGHT_MAX : f32 = 1f32;
const BALL_HIGHT_INIT: f32 = 0.5f32;
pub const BALL_VELOCITY_MAX : f32 = 1f32;

#[derive(Debug)]
pub struct Ball {
    radius: f32,
    pos: (Point2,f32),
    velocity:(f32,f32,f32),
    alive:bool,
}

impl Ball {
    pub fn new(radius:f32,pos:(f32,f32),velocity:(f32,f32,f32)) ->Ball{
        Ball{
            radius:radius,
            pos:(Point2::new(pos.0,pos.1),BALL_HIGHT_INIT),
            velocity:velocity,
            alive:true,
        }
    }
    pub fn set_radius(&mut self,radius:f32){
        self.radius = radius;
    }
    pub fn restore(&mut self,radius:f32,pos:(f32,f32),velocity:(f32,f32,f32)) {
        self.alive = true;
        self.radius = radius;
        (self.pos.0).x = pos.0;
        (self.pos.0).y = pos.1;
        self.pos.1 = BALL_HIGHT_INIT;
        self.velocity = velocity;
    }

    pub fn get_pos(&self)->Point2{
        self.pos.0
    }
    pub fn get_radius(&self) ->f32{
        self.radius
    }

    pub fn is_avtive(&self)->bool{
        self.alive
    }

    pub fn is_on_ground(&self) ->bool{
        self.pos.1 <= 0f32
    }

    fn get_draw_radius(&self) -> f32 {
        (1.0+1.5*self.pos.1/BALL_HIGHT_MAX)*self.radius
    }

    fn get_draw_color(&self) -> graphics::Color{
        graphics::Color::new(0f32,0f32,0f32,1f32 - 0.1*self.pos.1/BALL_HIGHT_MAX)
    }

    pub fn disable(&mut self) {
        self.alive = false;
    }

    pub fn set_direction(&mut self,direction: f32){
        let vel = ((self.velocity.0).powi(2) + (self.velocity.1).powi(2)).sqrt();
        self.velocity.0 = vel*direction.sin();
        self.velocity.1 = vel*direction.cos();
    }

    pub fn set_direction_vec(&mut self,vectory:(f32,f32)){
        let vel = ((self.velocity.0).powi(2) + (self.velocity.1).powi(2)).sqrt();
        let vec_len = ((vectory.0).powi(2) + (vectory.1).powi(2)).sqrt();
        self.velocity.0 = vel * vectory.0 / vec_len;
        self.velocity.1 = vel* vectory.1 / vec_len;
    }


    pub fn update(&mut self,time_delta_persent:f32){
        if self.alive && self.pos.1 > 0f32{
            (self.pos.0).x += self.velocity.0 * time_delta_persent;
            (self.pos.0).y += self.velocity.1 * time_delta_persent;
            self.velocity.2 += GRAVITY * time_delta_persent;
            self.pos.1 += self.velocity.2 * time_delta_persent;
        }
    }

    pub fn draw(&self,ctx:&mut Context) ->GameResult<()>{
        graphics::set_color(ctx, self.get_draw_color())?;
            graphics::circle(
                ctx,
                DrawMode::Fill,
                self.pos.0,
                self.get_draw_radius(),
                0.2,
            )?;
        Ok(())
    }

}