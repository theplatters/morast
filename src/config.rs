use macroquad::window::Conf;

pub fn window_config() -> Conf {
    Conf {
        window_title: "Morast".to_owned(),
        window_width: 1200,
        window_height: 1000,
        ..Default::default()
    }
}
