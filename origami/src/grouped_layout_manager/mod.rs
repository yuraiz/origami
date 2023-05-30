mod child_wrapper;
mod layout;
mod position_flags;

use child_wrapper::ChildWrapper;
use layout::layout_function;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

const MIN_WIDTH: f32 = 70.0;

mod imp {
    use super::*;
    use std::{
        cell::{Cell, RefCell},
        collections::BTreeMap,
    };

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::GroupedLayout)]
    pub struct GroupedLayout {
        #[property(get, set)]
        pub(super) spacing: Cell<i32>,

        pub(super) last_measurements: RefCell<Option<Vec<ChildWrapper>>>,

        pub(super) widths: RefCell<BTreeMap<i32, i32>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GroupedLayout {
        const NAME: &'static str = "OriGroupedLayout";
        type Type = super::GroupedLayout;
        type ParentType = gtk::LayoutManager;
    }

    impl ObjectImpl for GroupedLayout {
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

    impl LayoutManagerImpl for GroupedLayout {
        fn measure(
            &self,
            widget: &gtk::Widget,
            orientation: gtk::Orientation,
            for_size: i32,
        ) -> (i32, i32, i32, i32) {
            if widget.observe_children().n_items() <= 1 {
                return if let Some(child) = widget.first_child() {
                    child.measure(orientation, for_size)
                } else {
                    (0, 0, -1, -1)
                };
            }

            let (min, size) = if orientation == gtk::Orientation::Vertical {
                let (width, height) = self.measure_height(widget, for_size);
                self.widths.borrow_mut().insert(height, width);

                // dbg!(self.widths.borrow());

                (height, height)
            } else {
                let size = if for_size == -1 {
                    270
                } else {
                    *self.widths.borrow_mut().get(&for_size).unwrap_or_else(|| {
                        println!("miss: {for_size}");
                        &1
                    })
                };
                (64, size)
            };

            (min, size, -1, -1)
        }

        fn request_mode(&self, _widget: &gtk::Widget) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::HeightForWidth
        }

        fn allocate(&self, widget: &gtk::Widget, width: i32, height: i32, baseline: i32) {
            if widget.observe_children().n_items() <= 1 {
                if let Some(child) = widget.first_child() {
                    child.allocate(width, height, baseline, None);
                }
                return;
            }

            self.measure_height(widget, width);

            if let Some(children) = &*self.last_measurements.borrow() {
                children.iter().for_each(|child| child.allocate());
            }
        }
    }

    impl GroupedLayout {
        fn measure_height(&self, widget: &gtk::Widget, width: i32) -> (i32, i32) {
            let children: Vec<_> = widget
                .observe_children()
                .iter::<glib::Object>()
                .filter_map(ChildWrapper::from_result)
                .collect();

            if children.len() <= 1 {
                return if let Some(child) = widget.first_child() {
                    (child.measure(gtk::Orientation::Vertical, width).1, 0)
                } else {
                    (-1, -1)
                };
            }

            assert!(children.len() >= 2);

            let aspect_ratios = children.iter().map(|child| child.aspect_ratio);

            let proportions: String = aspect_ratios
                .clone()
                .map(|ar| {
                    if ar > 1.2 {
                        "w"
                    } else if ar < 0.8 {
                        "n"
                    } else {
                        "q"
                    }
                })
                .collect();

            let average_aspect_ratio = aspect_ratios.clone().sum::<f32>() / children.len() as f32;

            let force_calc = aspect_ratios.clone().any(|ar| ar > 2.0);

            let spacing = self.spacing.get() as f32;

            let layout_function = layout_function(children.len(), force_calc);

            let width = width as f32;

            layout_function(
                &children,
                &proportions,
                average_aspect_ratio,
                width,
                spacing,
            );

            let lf = children.last().unwrap().layout_frame.get();

            self.last_measurements.replace(Some(children));

            let width = (lf.0 + lf.2) as i32;
            let height = (lf.1 + lf.3) as i32;

            (width, height)
        }
    }
}

glib::wrapper! {
    pub struct GroupedLayout(ObjectSubclass<imp::GroupedLayout>)
        @extends gtk::Widget;
}
