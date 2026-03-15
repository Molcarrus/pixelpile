use raylib::prelude::*;

pub trait RecExt {
    fn intersect(&self, other: Rectangle) -> Rectangle;
    fn pad(&self, padding: f32) -> Rectangle;
    fn pad_ex(&self, left: f32, top: f32, right: f32, bottom: f32) -> Rectangle;
    fn center_point(&self) -> (f32, f32);
    fn center_in_area(&self, area: Rectangle) -> Rectangle;
    fn extend_top(&mut self, amount: f32) -> Rectangle;
    fn take_right(&self, amount: f32) -> Rectangle;
    fn delete_top(&mut self, amount: f32);
    fn cut_top(&mut self, amount: f32) -> Rectangle;
    fn cut_bottom(&mut self, amount: f32) -> Rectangle;
    fn cut_left(&mut self, amount: f32) -> Rectangle;
    fn cut_right(&mut self, amount: f32) -> Rectangle;
}

impl RecExt for Rectangle {
    fn intersect(&self, other: Rectangle) -> Rectangle {
        let x1 = self.x.max(other.x);
        let y1 = self.y.max(other.y);
        let mut x2 = (self.x + self.width).min(other.x + other.width);
        let mut y2 = (self.y + self.height).min(other.y + other.height);
        if x2 < x1 { x2 = x1; }
        if y2 < y1 { y2 = y1; }
        Rectangle { x: x1, y: y1, width: x2 - x1, height: y2 - y1 }
    }

    fn pad(&self, padding: f32) -> Rectangle {
        Rectangle {
            x: self.x + padding,
            y: self.y + padding,
            width: self.width - padding * 2.0,
            height: self.height - padding * 2.0,
        }
    }

    fn pad_ex(&self, left: f32, top: f32, right: f32, bottom: f32) -> Rectangle {
        Rectangle {
            x: self.x + left,
            y: self.y + top,
            width: self.width - right * 2.0,
            height: self.height - bottom * 2.0,
        }
    }

    fn center_point(&self) -> (f32, f32) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    fn center_in_area(&self, area: Rectangle) -> Rectangle {
        let x = area.x + area.width / 2.0 - self.width / 2.0;
        let y = area.y + area.height / 2.0 - self.height / 2.0;
        Rectangle { x, y, width: self.width, height: self.height }
    }

    fn extend_top(&mut self, amount: f32) -> Rectangle {
        let res = Rectangle {
            x: self.x,
            y: self.y - amount,
            width: self.width,
            height: amount,
        };
        self.y -= amount;
        self.height += amount;
        res
    }

    fn take_right(&self, amount: f32) -> Rectangle {
        Rectangle {
            x: self.x + self.width - amount,
            y: self.y,
            width: amount,
            height: self.height,
        }
    }

    fn delete_top(&mut self, amount: f32) {
        self.y += amount;
        self.height -= amount;
    }

    fn cut_top(&mut self, amount: f32) -> Rectangle {
        let res = Rectangle {
            x: self.x,
            y: self.y,
            width: self.width,
            height: amount,
        };
        self.y += amount;
        self.height -= amount;
        res
    }

    fn cut_bottom(&mut self, amount: f32) -> Rectangle {
        let res = Rectangle {
            x: self.x,
            y: self.y + self.height - amount,
            width: self.width,
            height: amount,
        };
        self.height -= amount;
        res
    }

    fn cut_left(&mut self, amount: f32) -> Rectangle {
        let res = Rectangle {
            x: self.x,
            y: self.y,
            width: amount,
            height: self.height,
        };
        self.x += amount;
        self.width -= amount;
        res
    }

    fn cut_right(&mut self, amount: f32) -> Rectangle {
        let res = Rectangle {
            x: self.x + self.width - amount,
            y: self.y,
            width: amount,
            height: self.height,
        };
        self.width -= amount;
        res
    }
}