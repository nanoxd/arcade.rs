use phi::{Phi, View, ViewAction};
use phi::data::Rectangle;
use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureQuery};
use sdl2_image::LoadTexture;
use std::path::Path;

// Constants

/// Pixels traveled by the player's ship every second, when it is moving.
const PLAYER_SPEED: f64 = 180.0;

// Data types

struct Ship {
    rect: Rectangle,
    tex: Texture,
}

// View Definitions

pub struct ShipView {
    player: Ship,
}

impl ShipView {
    pub fn new(phi: &mut Phi) -> ShipView {
        //? Load the texture from the filesystem.
        //? If it cannot be found, then there is no point in continuing: panic!
        let tex = phi.renderer.load_texture(Path::new("assets/spaceship.png")).unwrap();

        //? Destructure some properties of the texture, notably width and
        //? height, which we will use for the ship's bounding box.
        let TextureQuery { width, height, .. } = tex.query();

        ShipView {
            player: Ship {
                rect: Rectangle {
                    x: 64.0,
                    y: 64.0,
                    w: width as f64,
                    h: height as f64,
                },
                tex: tex,
            }
        }
    }
}

impl View for ShipView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        // Move the ship
        let diagonal =
            (phi.events.key_up ^ phi.events.key_down) &&
            (phi.events.key_left ^ phi.events.key_right);

        let moved =
            if diagonal { 1.0 / 2.0f64.sqrt() }
            else { 1.0 } * PLAYER_SPEED * elapsed;

        let dx = match (phi.events.key_left, phi.events.key_right) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        let dy = match (phi.events.key_up, phi.events.key_down) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };


        self.player.rect.x += dx;
        self.player.rect.y += dy;

        let movable_region = Rectangle {
            x: 0.0,
            y: 0.0,
            w: phi.output_size().0 * 0.70,
            h: phi.output_size().1,
        };

        self.player.rect = self.player.rect.move_inside(movable_region).unwrap();

        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        phi.renderer.set_draw_color(Color::RGB(200, 200, 50));
        phi.renderer.fill_rect(self.player.rect.to_sdl().unwrap());

        ViewAction::None
    }
}
