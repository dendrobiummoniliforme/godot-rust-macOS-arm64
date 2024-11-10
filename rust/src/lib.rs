// https://godot-rust.github.io/book/intro/hello-world.html#rust-entry-point
// There are multiple things going on here:
//   Place the prelude module from the godot crate into scope. This module contains the most common symbols in the gdext API.
//   Define a struct called MyExtension. This is just a type tag without data or methods, you can name it however you like.
//   Implement the ExtensionLibrary trait for our type, and mark it with the #[gdextension] attribute.
// The last point declares the actual GDExtension entry point, and the proc-macro attribute takes care of the low-level details.
use godot::prelude::*;
use godot::classes::Sprite2D;
use godot::classes::ISprite2D;

// Rust is the name of our package in Cargo.toml.
// This could be named anything, but you must update references else-where
// see ../README.md.
struct Rust;

#[gdextension]
unsafe impl ExtensionLibrary for Rust {}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,
    angular_speed: f64,
    base: Base<Sprite2D>
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello World!");

        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        // GDScript Code

        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);
    }
}