extern crate toml;

#[derive(Deserialize)]
pub struct Configuration {
    pub window: Window,
    pub gameplay: Gameplay,
    pub graphics: Graphics,
    pub camera: Camera,
    pub event_loop: EventLoop,
    pub control: Control,
    pub physics: Physics,
    pub audio: Audio,
}

#[derive(Deserialize)]
pub struct Control {
    pub mouse_sensibility: f64,
}
#[derive(Deserialize)]
pub struct Window {
    pub samples: u8,
    pub fullscreen: bool,
    pub vsync: bool,
    pub dimensions: [u32; 2],
}
#[derive(Deserialize)]
pub struct Gameplay {
    pub gravity: f64,
    pub ball_radius: f64,
    pub damping: f64,
    pub impulse: f64,
    pub reset: bool,
}
#[derive(Deserialize)]
pub struct Graphics {
    pub ball_color: [f32; 4],
    pub wall_color: [f32; 4],
    pub background_color: [f32; 4],
    pub cursor_color: [f32; 4],
    pub cursor_inner_radius: f32,
    pub cursor_outer_radius: f32,
    pub cursor_thickness: f32,
    pub effect_timer: f64,
    pub effect_color: [f32; 4],
    pub effect_thickness: f32,
}
#[derive(Deserialize)]
pub struct Camera {
    pub zoom: f64,
}
#[derive(Deserialize)]
pub struct EventLoop {
    pub max_fps: u32,
}
#[derive(Deserialize)]
pub struct Physics {
    pub unit: f64,
}
#[derive(Deserialize)]
pub struct Audio {
    pub jump_volume: f32,
    pub wall_volume: f32,

    pub wall_max_intensity: f64,
    pub wall_min_intensity: f64,
}

lazy_static! {
    pub static ref CFG: Configuration = toml::from_str(include_str!("../config.toml")).unwrap();
}
