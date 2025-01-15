mod engine;
mod game;
fn main() {
    let env = engine::janet_handler::controller::init();
    engine::janet_handler::controller::do_string(&env, "(print `hello, world!`)");
    engine::janet_handler::controller::deinit();
}
