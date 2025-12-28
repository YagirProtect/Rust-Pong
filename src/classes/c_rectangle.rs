use vek::Vec2;
use crate::classes::c_canvas::Canvas;
use crate::classes::c_color::Color;
use crate::classes::c_rect_collider::RectCollider;
use crate::classes::t_collision::Intersection;
use crate::classes::t_drawable::Drawable;

pub struct Rectangle{

    is_can_draw: bool,

    width: u32,
    height: u32,

    x: f32,
    y: f32,

    color: u32,

    collider: RectCollider,
}


impl Rectangle{
    pub fn X(&self) -> f32 {
        self.x
    }
    pub fn Y(&self) -> f32 {
        self.y
    }

    pub fn HalfH(&self) -> f32 {
        self.height as f32/2.0
    }

    pub fn HalfW(&self) -> f32 {
        self.width as f32/2.0
    }
}

impl Drawable for Rectangle {
    fn is_can_draw(&self) -> bool {
        self.is_can_draw
    }

    fn draw(&self, buffer: &mut [u32], canvas: &Canvas) {
        let sw = canvas.Width() as i32;
        let sh = canvas.Height() as i32;
        if sw <= 0 || sh <= 0 { return; }

        let expected = (sw as usize) * (sh as usize);
        if buffer.len() < expected { return; }

        // левый верх из центра
        let half_w = (self.width as i32) / 2;
        let half_h = (self.height as i32) / 2;

        let left   = self.x as i32 - half_w;
        let top    = self.y as i32 - half_h;
        let right  = left + self.width as i32;
        let bottom = top + self.height as i32;

        // клиппинг
        let x0 = left.max(0) as usize;
        let y0 = top.max(0) as usize;
        let x1 = right.min(sw).max(0) as usize;
        let y1 = bottom.min(sh).max(0) as usize;

        if x0 >= x1 || y0 >= y1 { return; }

        let stride = sw as usize;
        let col = self.color;

        for yy in y0..y1 {
            let row = yy * stride;
            buffer[row + x0 .. row + x1].fill(col);
        }
    }
}

impl Rectangle {
    pub fn new(width: u32, height: u32, x: u32, y: u32, color: Color) -> Rectangle {

        let c = Vec2::new(x as f32, x as f32);
        let half = Vec2::new(width as f32 * 0.5, height as f32 * 0.5);

        Self{
            width,
            height,
            x: x as f32,
            y: y as f32,
            color: color.to_8bit(),
            collider: RectCollider::new(c - half, c + half),
            is_can_draw: true,
        }
    }

    pub fn change_rect_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;

        let c = Vec2::new(x, y);
        let half = Vec2::new(self.width as f32 * 0.5, self.height as f32 * 0.5);

        self.collider.set_bounds(c - half, c + half);
    }

    pub fn move_rect_pos(&mut self, dx: f32, dy: f32) {
        self.change_rect_pos(self.x + dx, self.y + dy);
    }

    pub fn get_intersections(&mut self) -> &mut dyn Intersection {
        &mut self.collider
    }
}