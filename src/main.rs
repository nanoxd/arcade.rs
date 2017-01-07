extern crate sdl2;

mod phi;
mod views;

fn main() {
    ::phi::spawn("Arcade Shooter", |phi| {
        Box::new(::views::ShipView::new(phi))
    });
}
