use sdl2::rect::Rect as SdlRect;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rectangle {
    /// Generates an SDL-compatible Rect equivalent to `self`.
    /// Panics if it could not be created, for example if a
    /// coordinate of a corner overflows an `i32`.
    pub fn to_sdl(self) -> Option<SdlRect> {
        // Reject negative width and height
        assert!(self.w >= 0.0 && self.h >= 0.0);

        SdlRect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
            .unwrap()
    }

    /// Return a (perhaps moved) rectangle which is contained by a `parent`
    /// rectangle. If it can indeed be moved to fit, return `Some(result)`;
    /// otherwise, return `None`.
    pub fn move_inside(self, parent: Rectangle) -> Option<Rectangle> {
        // It must be smaller than the parent rectangle to fit in it.
        if self.w > parent.w || self.h > parent.h {
            return None;
        }

        Some(Rectangle {
            w: self.w,
            h: self.h,
            x: if self.x < parent.x { parent.x }
            else if self.x + self.w >= parent.x + parent.w { parent.x + parent.w - self.w }
            else { self.x },
            y: if self.y < parent.y { parent.y }
            else if self.y + self.h >= parent.y + parent.h { parent.y + parent.h - self.h }
            else { self.y }
        })
    }

    pub fn contains(&self, rect: Rectangle) -> bool {
        let minx = rect.x;
        let maxx = minx + rect.w;
        let miny = rect.y;
        let maxy = miny + rect.h;

        minx >= self.x && minx <= self.x + self.w &&
        maxx >= self.x && maxx <= self.x + self.w &&
        miny >= self.y && miny <= self.y + self.h &&
        maxy >= self.y && maxy <= self.y + self.h
    }

    pub fn overlaps(&self, other: Rectangle) -> bool {
        self.x < other.x + other.w &&
        self.x + self.w > other.x &&
        self.y < other.y + other.h &&
        self.y + self.h > other.y
    }
}
