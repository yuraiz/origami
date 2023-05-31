use super::*;

use adw::prelude::*;
use gtk::glib;
use gtk::subclass::prelude::*;

mod imp {
    use gtk::glib::clone;

    use crate::traits::Lerp;

    use super::*;
    use std::{
        cell::{Cell, RefCell},
        collections::BTreeMap,
    };

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = animated::AnimatedGroup)]
    pub struct AnimatedGroup {
        #[property(get, set = Self::set_spacing)]
        pub(super) spacing: Cell<i32>,

        #[property(get, set = Self::set_progress)]
        pub(super) progress: Cell<f32>,

        pub(super) animation: RefCell<adw::TimedAnimation>,

        pub(super) animate_from: RefCell<Vec<ChildWrapper>>,
        pub(super) last_state: RefCell<Vec<ChildWrapper>>,

        pub(super) prev_stats: RefCell<(i32, Vec<i32>)>,

        pub(super) widths: RefCell<BTreeMap<i32, i32>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AnimatedGroup {
        const NAME: &'static str = "OriAnimatedGroup";
        type Type = animated::AnimatedGroup;
        type Interfaces = (gtk::Buildable,);
        type ParentType = gtk::Widget;
    }

    impl ObjectImpl for AnimatedGroup {
        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            self.derived_property(id, pspec)
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }

        fn constructed(&self) {
            self.parent_constructed();

            let widget = self.obj();

            self.progress.set(1.0);

            let target =
                adw::CallbackAnimationTarget::new(clone!(@weak widget => move |progress| {
                    widget.set_progress(progress as f32);
                }));

            let animation = adw::TimedAnimation::builder()
                .widget(widget.as_ref())
                .value_to(1.0)
                .duration(200)
                .build();

            animation.set_target(&target.upcast::<adw::AnimationTarget>());

            self.animation.replace(animation);
        }
    }

    impl WidgetImpl for AnimatedGroup {
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
                    let progress = self.progress.get();

                    let layout = shared::layout(
                        widget.as_ref().upcast_ref(),
                        for_size,
                        widget.spacing() as f32,
                    );

                    let animated_layout = {
                        let animate_from = self.animate_from.borrow();

                        if animate_from.len() == 0 || progress == 1.0 {
                            layout
                        } else {
                            animate_from.lerp(&layout, progress)
                        }
                    };

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

        fn request_mode(&self) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::HeightForWidth
        }

        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            let widget = self.obj();

            if widget.observe_children().n_items() <= 1 {
                if let Some(child) = widget.first_child() {
                    child.allocate(width, height, baseline, None);
                }
                return;
            }

            let progress = self.progress.get();

            let layout =
                shared::layout(widget.as_ref().upcast_ref(), width, widget.spacing() as f32);

            let stats = items_per_row(&layout);

            let animated_layout = {
                let animate_from = self.animate_from.borrow();

                if animate_from.len() == 0 || progress == 1.0 {
                    layout
                } else {
                    animate_from.lerp(&layout, progress)
                }
            };

            let layout_height = animated_layout
                .iter()
                .map(|c| {
                    let lf = c.layout_frame.get();
                    (lf.1 + lf.3).ceil() as i32
                })
                .max()
                .unwrap();

            // Fit layout into smaller height if gtk didn't allocate required size
            if layout_height > height {
                let scale = height as f32 / layout_height as f32;

                animated_layout.iter().for_each(|child| {
                    let (x, y, w, h) = child.layout_frame.get();
                    let y = y * scale;
                    let h = h * scale;
                    child.layout_frame.set((x, y, w, h))
                });
            }

            if self.prev_stats.borrow().1 != stats {
                let animation_start_candidate = if self.last_state.borrow().len() == 0 {
                    self.last_state.replace(animated_layout.clone());
                    animated_layout
                } else {
                    self.last_state.replace(animated_layout)
                };

                animation_start_candidate.iter().for_each(|c| c.allocate());

                let stats = self.prev_stats.replace((width, stats)).1;

                self.animate_from.replace(animation_start_candidate);

                if !stats.is_empty() {
                    self.progress.set(0.0);
                    self.animation.borrow().play();
                }
            } else {
                animated_layout.iter().for_each(|c| c.allocate());
                self.last_state.replace(animated_layout);
            }
        }
    }

    impl BuildableImpl for AnimatedGroup {}

    impl AnimatedGroup {
        fn set_spacing(&self, spacing: i32) {
            self.spacing.set(spacing);
            self.obj().queue_allocate();
        }

        fn set_progress(&self, progress: f32) {
            self.progress.set(progress);
            self.obj().queue_resize();
        }
    }
}

glib::wrapper! {
    pub struct AnimatedGroup(ObjectSubclass<imp::AnimatedGroup>)
        @extends gtk::Widget, @implements gtk::Buildable;
}

impl AnimatedGroup {
    pub fn remove_chldren(&self) {
        while let Some(child) = self.first_child() {
            child.unparent();
        }
    }

    pub fn append(&self, child: &impl IsA<gtk::Widget>) {
        child.insert_before(self, gtk::Widget::NONE);
    }
}

fn items_per_row(layout: &[ChildWrapper]) -> Vec<i32> {
    let mut wrap_pos = 0;
    let mut pos = 0;
    let mut result = vec![];
    for child in layout {
        if child.position_flags.get().contains(PositionFlags::RIGHT) {
            pos += 1;
            result.push(pos - wrap_pos);
            wrap_pos = pos;
        } else {
            pos += 1;
        }
    }
    result
}
