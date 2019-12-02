use piston_window::{*, types::{Color, Rectangle, Radius}};
use chrono::{Date, Utc, Datelike, Local};
use super::{
    object::Object,
    moon_calendar_date::MoonCalendarDate,
};
use super::super::util::moon_age;
use crate::util::{
    moon_age::MoonAge,
    moons_description,
};
use std::thread;
use std::time::Duration;

pub struct Scene {
    width: u32,
    height: u32,
    moon_obj: Object,
    state_changed: bool,
    current_date: MoonCalendarDate,
    moon_name: String,
    moon_age: String,
    moon_description: String,
}

impl Scene {
    pub fn new(width: u32, height: u32, moon_obj: Object) -> Scene {
        Scene {
            width,
            height,
            moon_obj,
            current_date: MoonCalendarDate::new(),
            state_changed: true,
            moon_name: String::new(),
            moon_age: String::new(),
            moon_description: String::new(),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.moon_name
    }

    pub fn set_name(&mut self, moon_name: String) {
        self.moon_name = moon_name;
        self.state_changed = true;
    }

    pub fn get_age(&self) -> &String {
        &self.moon_age
    }

    pub fn set_age(&mut self, moon_age: String) {
        self.moon_age = moon_age;
        self.state_changed = true;
    }

    pub fn get_description(&self) -> &String {
        &self.moon_description
    }

    pub fn set_description(&mut self, moon_description: String) {
        self.moon_description = moon_description;
        self.state_changed = true;
    }

    pub fn state_changed(&self) -> bool {
        self.state_changed
    }
}

pub fn draw(ctx: &Context, g2d: &mut G2d, scene: &mut Scene) {
    scene.moon_obj.draw(ctx, g2d);
    draw_eclipse(ctx, g2d, scene);
    scene.state_changed = false;
}

pub fn draw_text(
    text: &String,
    location: (f64, f64),
    ctx: &Context,
    g2d: &mut G2d,
    font: &mut Glyphs,
) {
    Text::new_color([1.0, 1.0, 1.0, 1.0], 10)
        .draw(
            text.as_str(),
            font,
            &ctx.draw_state,
            ctx.transform.trans(location.0, location.1),
            g2d,
        ).unwrap();
}

pub fn update(delta: f64) {}

pub fn key_pressed(scene: &mut Scene, key: Key) {
    match key {
        Key::Left => scene.current_date.set_day(scene.current_date.day - 1),
        Key::Right => scene.current_date.set_day(scene.current_date.day + 1),
        Key::Up => scene.current_date.set_month(scene.current_date.month + 1),
        Key::Down => scene.current_date.set_month(scene.current_date.month - 1),
        _ => {}
    }
    scene.state_changed = true;
}

fn draw_eclipse(ctx: &Context, g2d: &mut G2d, scene: &mut Scene) {
    let height = scene.height as f64;
    let (age, x) = MoonAge::new(|option| option)
        .get_options(
            height,
            scene.current_date.year,
            scene.current_date.month,
            scene.current_date.day,
        );

    let name = moons_description::get_moon_name(scene.current_date.month);
    scene.set_name(name.to_string());
    scene.set_age(format!(
        "Date: {}/{}/{} Age: {}, {}",
        scene.current_date.year,
        scene.current_date.month,
        scene.current_date.day,
        age,
        x
    ));
    scene.set_description(format!("Description"));
    let radian = if age <= 15.0 { -1.6 } else { 1.6 };
    circle_arc(
        [0.0, 0.0, 0.0, 0.98],
        x,
        -radian,
        radian,
        [
            0.0,
            -x,
            height,
            height + x * 2.0
        ],
        ctx.transform,
        g2d,
    );
}
