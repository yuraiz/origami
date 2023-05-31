use super::*;

use adw::prelude::*;
use gtk::glib;
use gtk::subclass::prelude::*;

mod imp {
    use super::*;
    use std::{
        cell::{Cell, RefCell},
        collections::BTreeMap,
    };

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = default::Group)]
    pub struct Group {
        #[property(get, set = Self::set_spacing)]
        pub(super) spacing: Cell<i32>,

        pub(super) widths: RefCell<BTreeMap<i32, i32>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Group {
        const NAME: &'static str = "OriGroup";
        type Type = default::Group;
        type ParentType = gtk::Widget;
    }

    impl ObjectImpl for Group {
        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            self.derived_property(id, pspec)
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }
    }

    impl WidgetImpl for Group {
        fn measure(&self, orientation: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
            if for_size >= 1000000 {
                return (-1, -1, -1, -1);
            }

            let widget = self.obj();

            if widget.observe_children().n_items() <= 1 {
                return if let Some(child) = widget.first_child() {
                    child.measure(orientation, for_size)
                } else {
                    (0, 0, -1, -1)
                };
            }

            let (min, size) = if orientation == gtk::Orientation::Vertical {
                let height = {
                    let layout = shared::layout(
                        widget.as_ref().upcast_ref(),
                        for_size,
                        widget.spacing() as f32,
                    );

                    let animated_layout = { layout };

                    animated_layout
                        .iter()
                        .map(|c| {
                            let lf = c.layout_frame.get();
                            (lf.1 + lf.3).ceil() as i32
                        })
                        .max()
                        .unwrap()
                };

                self.widths.borrow_mut().insert(height, for_size);

                (height, height)
            } else {
                let size = if for_size == -1 {
                    270
                } else {
                    *self.widths.borrow_mut().get(&for_size).unwrap_or(&-1)
                };

                if size == -1 {
                    (-1, -1)
                } else {
                    (64, size.max(64))
                }
            };

            (min, size, -1, -1)
        }

        // fn measure(&self, orientation: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
        //     if for_size >= 1000000 {
        //         return (-1, -1, -1, -1);
        //     }

        //     let widget = self.obj();

        //     if widget.observe_children().n_items() <= 1 {
        //         return if let Some(child) = widget.first_child() {
        //             child.measure(orientation, for_size)
        //         } else {
        //             (0, 0, -1, -1)
        //         };
        //     }

        //     let (min, size) = if orientation == gtk::Orientation::Vertical {
        //         let height = shared::measure_height(
        //             widget.as_ref().upcast_ref(),
        //             for_size,
        //             widget.spacing() as f32,
        //         );
        //         self.widths.borrow_mut().insert(height, for_size);

        //         (height, height)
        //     } else {
        //         let size = if for_size == -1 {
        //             270
        //         } else {
        //             *self.widths.borrow_mut().get(&for_size).unwrap_or(&1)
        //         };

        //         if size == -1 {
        //             (-1, -1)
        //         } else {
        //             (64, size.max(64))
        //         }
        //     };

        //     (min, size, -1, -1)
        // }

        fn request_mode(&self) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::HeightForWidth
        }

        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            let widget = self.obj();

            if widget.observe_children().n_items() <= 1 {
                if let Some(child) = widget.first_child() {
                    child.allocate(width, height, baseline, None);
                }
            } else {
                let layout =
                    shared::layout(widget.as_ref().upcast_ref(), width, widget.spacing() as f32);

                let last_frame = layout.last().unwrap().layout_frame.get();

                let layout_height = (last_frame.1 + last_frame.3) as f32;

                // Fit layout into smaller height if gtk didn't allocate required size
                if layout_height > height as f32 {
                    let scale = height as f32 / layout_height;

                    layout.iter().for_each(|child| {
                        let (x, y, w, h) = child.layout_frame.get();
                        let y = y * scale;
                        let h = h * scale;
                        child.layout_frame.set((x, y, w, h))
                    });
                }

                layout.iter().for_each(|c| c.allocate());
            }
        }
    }

    impl Group {
        fn set_spacing(&self, spacing: i32) {
            self.spacing.set(spacing);
            self.obj().queue_allocate();
        }
    }
}

glib::wrapper! {
    pub struct Group(ObjectSubclass<imp::Group>)
        @extends gtk::Widget;
}

impl Group {}
