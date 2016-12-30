extern crate sdl2;

use sdl2::EventPump;

pub struct Events {
    pump: EventPump,

    pub quit: bool,
    pub key_escape: bool,
}

impl Events {
    pub fn new(pump: EventPump) -> Events {
        Events {
            pump: pump,

            quit: false,
            key_escape: false,
        }
    }
}
