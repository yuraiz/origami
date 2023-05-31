use super::position_flags::PositionFlags;
use std::cell::Cell;

use gtk::{glib, gsk};
use gtk::{graphene, prelude::*};

use crate::traits::Lerp;

#[derive(Debug, Clone)]
pub(crate) struct ChildWrapper {
    widget: gtk::Widget,
    pub(crate) aspect_ratio: f32,
    pub(crate) layout_frame: Cell<(f32, f32, f32, f32)>,
    pub(crate) position_flags: Cell<PositionFlags>,
}

impl ChildWrapper {
    pub fn new(widget: gtk::Widget) -> Self {
        let (_min, natural) = widget.preferred_size();

        let aspect_ratio = natural.width().max(1) as f32 / natural.height().max(1) as f32;

        Self {
            widget,
            aspect_ratio,
            layout_frame: Cell::default(),
            position_flags: Default::default(),
        }
    }

    pub fn from_result<T>(res: Result<glib::Object, T>) -> Option<Self> {
        let widget = res.ok()?.downcast::<gtk::Widget>().ok()?;
        if widget.is_visible() {
            Some(Self::new(widget))
        } else {
            None
        }
    }

    pub fn allocate(&self) {
        let (shift_x, shift_y, width, height) = self.layout_frame.get();
        let transform = gsk::Transform::new().translate(&graphene::Point::new(shift_x, shift_y));

        self.widget
            .allocate(width as i32, height as i32, -1, Some(transform))
    }
}

impl Lerp<f32> for &ChildWrapper {
    type Output = ChildWrapper;

    fn lerp(self, other: &ChildWrapper, t: f32) -> ChildWrapper {
        let start_frame = self.layout_frame.get();
        let end_frame = other.layout_frame.get();

        let frame = start_frame.lerp(end_frame, t);

        let res = if t < 0.5 { self.clone() } else { other.clone() };

        res.layout_frame.set(frame);
        res
    }
}

impl Lerp<f32> for &[ChildWrapper] {
    type Output = Vec<ChildWrapper>;

    fn lerp(self, other: Self, t: f32) -> Self::Output {
        self.iter()
            .zip(other.iter())
            .map(|(s, e)| s.lerp(e, t))
            .collect()
    }
}
