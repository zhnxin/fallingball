extern crate ggez;
extern crate rand;
pub mod ball;
pub mod bar;
pub mod timer;
pub mod block;
pub mod gamestate;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
