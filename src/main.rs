extern crate ggez;
extern crate fallingball;

fn main(){
    let mut c = ggez::conf::Conf::new();

    c.window_mode.width = 400;
    let ctx = &mut ggez::Context::load_from_conf("falling ball", "ggez", c).unwrap();
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }
    let state = &mut fallingball::gamestate::GameState::new(ctx).unwrap();
    ggez::event::run(ctx, state).unwrap();
}