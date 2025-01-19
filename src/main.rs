use engine::janet_handler::{
    api::cfun_draw,
    types::janetenum::{JanetEnum, JanetItem},
};

mod engine;
mod game;
fn main() {
    let env = engine::janet_handler::controller::Environment::init();
    let hello = "(defn sq [n] (* n n))";
    env.do_string(hello);
    env.register("add", cfun_draw, "n times n", Some("set"));
    let _ = env
        .read_script("scripts/loader.janet")
        .expect("File not found");
    let fun = engine::janet_handler::types::function::Function::get_method(
        &env,
        "triangle-area",
        Some("test"),
    )
    .expect("Upsie");

    let hello = "(print (set/add 3))";
    env.do_string(hello);
    let input: Vec<Box<dyn JanetItem>> = vec![Box::new(3.0), Box::new(4.5)];
    let res = fun.eval::<i32>(&input).expect("Upsie2");
    if let JanetEnum::_Float(n) = res {
        println!("{}", n)
    }
    engine::janet_handler::controller::Environment::deinit();
}
