extern crate piston_window;
extern crate chrono;

mod util;
mod ui;

use util::*;
use ui::*;
use piston_window::*;
use std::path::Path;
use crate::ui::object::Object;
use crate::ui::moon_panel::Scene;

fn main() {
    let title = "Moon Calendar v0.0.1";
    let size: [u32; 2] = [600, 350];
    let mut piston_window: PistonWindow = WindowSettings::new(
        title,
        size,
    )
        .resizable(false) // no effect
        .exit_on_esc(true)
        .build()
        .unwrap();

    let moon_picture: G2dTexture = Texture::from_path(
        &mut piston_window.create_texture_context(),
        Path::new("assets/images/moon.png"),
        Flip::None,
        &TextureSettings::new(),
    ).unwrap();

    let object = Object::new(
        moon_picture,
        [0.0, 0.0, size[1] as f64, size[1] as f64],
        [0.0, 0.0, 0.0, 1.0],
    );

    let mut font = piston_window.load_font(Path::new("assets/styles/josefinsans-regular.ttf"))
                                .unwrap();
    let mut scene = Scene::new(size[0], size[1], object);

    while let Some(event) = piston_window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            moon_panel::key_pressed(&mut scene, key);
        }

        piston_window.draw_2d(&event, |c, g, d| {
            if scene.state_changed() {
                clear([0.0, 0.0, 0.0, 1.0], g);
                moon_panel::draw(&c, g, &mut scene);

                // We don't have access to gfx_device_gl::Device via piston_window.
                // To solve this issue you need to add dependencies to Cargo.toml
                // or draw text in this function (dirty-hack).
                moon_panel::draw_text(
                    &scene.get_name(),
                    (355.0, 15.0),
                    &c,
                    g,
                    &mut font,
                );
                moon_panel::draw_text(
                    &scene.get_age(),
                    (355.0, 30.0),
                    &c,
                    g,
                    &mut font,
                );
                moon_panel::draw_text(
                    &scene.get_description(),
                    (355.0, 45.0),
                    &c,
                    g,
                    &mut font,
                );
                font.factory.encoder.flush(d);
            }
        });

        event.update(|arg| {
            moon_panel::update(arg.dt);
        });
    }
}
