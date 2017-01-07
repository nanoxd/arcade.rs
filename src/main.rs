extern crate sdl2;

mod phi;
mod views;

fn main() {
    ::phi::spawn("Arcade Shooter", |_| {
        Box::new(::views::ViewA)
    });
}
