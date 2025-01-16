mod engine;
mod game;
fn main() {
    let env = engine::janet_handler::controller::init();
    let hello = "(defn sq [n] (* n n))";
    engine::janet_handler::controller::do_string(&env, hello);
    let _ = engine::janet_handler::controller::read_script(&env, "scripts/loader.janet")
        .expect("File not found");
    let fun =
        engine::janet_handler::types::function::Function::get_method(&env, "square-area", "test")
            .expect("Upsie");
    let input = [1.0];
    let res = fun.eval(&input).expect("Upsie2");
    engine::janet_handler::controller::deinit();
}
