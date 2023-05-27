use std::cell::Cell;

use gtk::glib::bitflags::bitflags;
use gtk::subclass::prelude::*;
use gtk::{glib, gsk};
use gtk::{graphene, prelude::*};
// use gtk::{graphene, gsk};

const MIN_WIDTH: f32 = 70.0;

bitflags! {
    struct PositionFlags: u32 {
        const NONE = 0;
        const INSIDE = 0b000010000;

        const TOP = 0b00000001;
        const LEFT = 0b00000010;
        const RIGHT = 0b00000100;
        const BOTTOM = 0b000001000;

        const TOP_LEFT = Self::TOP.bits | Self::LEFT.bits;
        const TOP_RIGHT = Self::TOP.bits | Self::RIGHT.bits;
        const BOTTOM_RIGHT = Self::BOTTOM.bits | Self::RIGHT.bits;
        const BOTTOM_LEFT = Self::BOTTOM.bits | Self::LEFT.bits;

        const FULL_WIDTH = Self::LEFT.bits | Self::RIGHT.bits;
        const FULL_HEIGHT = Self::TOP.bits | Self::BOTTOM.bits;

        const FULL = Self::FULL_WIDTH.bits | Self::FULL_HEIGHT.bits;
    }
}

impl Default for PositionFlags {
    fn default() -> Self {
        Self::NONE
    }
}

#[derive(Debug)]
struct ChildWrapper {
    widget: gtk::Widget,
    aspect_ratio: f32,
    layout_frame: Cell<(f32, f32, f32, f32)>,
    position_flags: Cell<PositionFlags>,
}

impl ChildWrapper {
    fn new(widget: gtk::Widget) -> Self {
        let (_min, natural) = widget.preferred_size();

        let aspect_ratio = natural.width().max(1) as f32 / natural.height().max(1) as f32;

        Self {
            widget,
            aspect_ratio,
            layout_frame: Cell::default(),
            position_flags: Default::default(),
        }
    }

    fn from_result<T>(res: Result<glib::Object, T>) -> Option<Self> {
        let widget = res.ok()?.downcast::<gtk::Widget>().ok()?;
        if widget.is_visible() {
            Some(Self::new(widget))
        } else {
            None
        }
    }

    fn allocate(&self) {
        let (shift_x, shift_y, width, height) = self.layout_frame.get();
        let transform = gsk::Transform::new().translate(&graphene::Point::new(shift_x, shift_y));

        self.widget
            .allocate(width as i32, height as i32, -1, Some(transform))
    }
}

mod imp {
    use super::*;
    use std::cell::{Cell, RefCell};

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::GroupedLayout)]
    pub struct GroupedLayout {
        #[property(get, set)]
        pub(super) spacing: Cell<i32>,

        pub(super) last_measurements: RefCell<Option<Vec<ChildWrapper>>>,
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
            let size = if orientation == gtk::Orientation::Vertical {
                let res = self.measure_for_max_size(widget, for_size, 400);
                res.1
            } else {
                if let Some(children) = &*self.last_measurements.borrow() {
                    let lf = children.last().unwrap().layout_frame.get();
                    (lf.0 + lf.2) as i32
                } else {
                    1
                }
            };

            (size, size, -1, -1)
        }

        fn request_mode(&self, _widget: &gtk::Widget) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::HeightForWidth
        }

        fn allocate(&self, widget: &gtk::Widget, width: i32, height: i32, baseline: i32) {
            // let children: Vec<_> = widget
            //     .observe_children()
            //     .iter::<glib::Object>()
            //     .filter_map(ChildWrapper::from_result)
            //     .collect();

            if widget.observe_children().n_items() <= 1 {
                if let Some(child) = widget.first_child() {
                    child.allocate(width, height, baseline, None);
                }
                return;
            }

            dbg!(self.measure_for_max_size(widget, width, 400));

            // let aspect_ratios = children.iter().map(|child| child.aspect_ratio);

            // let proportions: String = aspect_ratios
            //     .clone()
            //     .map(|ar| {
            //         if ar > 1.2 {
            //             "w"
            //         } else if ar < 0.8 {
            //             "n"
            //         } else {
            //             "q"
            //         }
            //     })
            //     .collect();

            // let average_aspect_ratio = aspect_ratios.clone().sum::<f32>() / children.len() as f32;

            // let force_calc = aspect_ratios.clone().any(|ar| ar > 2.0);

            // let spacing = self.spacing.get() as f32;

            // dbg!((force_calc, children.len()));

            // let layout_function = if force_calc {
            //     layout_fallback
            // } else {
            //     match children.len() {
            //         2 => layout_two_children,
            //         3 => layout_three_children,
            //         4 => layout_four_children,
            //         _ => layout_fallback,
            //     }
            // };

            // let width = width as f32;
            // let height = height as f32;

            // layout_function(
            //     &children,
            //     &proportions,
            //     average_aspect_ratio,
            //     width,
            //     height,
            //     spacing,
            // );

            // let lf = children.last().unwrap().layout_frame.get();

            // dbg!(width - (lf.0 + lf.2));
            // dbg!(height - (lf.1 + lf.3));

            if let Some(children) = &*self.last_measurements.borrow() {
                // dbg!(children.last().unwrap().layout_frame.get());
                children.iter().for_each(|child| child.allocate());
            }

            // children.into_iter().for_each(|child| child.allocate());
        }
    }

    impl GroupedLayout {
        fn measure_for_max_size(
            &self,
            widget: &gtk::Widget,
            width: i32,
            height: i32,
        ) -> (i32, i32) {
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

            // dbg!((force_calc, children.len()));

            let layout_function = if force_calc {
                layout_fallback
            } else {
                match children.len() {
                    2 => layout_two_children,
                    3 => layout_three_children,
                    4 => layout_four_children,
                    _ => layout_fallback,
                }
            };

            let width = width as f32;
            let height = height as f32;

            layout_function(
                &children,
                &proportions,
                average_aspect_ratio,
                width,
                height,
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

fn layout_two_children(
    children: &[ChildWrapper],
    proportions: &str,
    average_aspect_ratio: f32,
    width: f32,
    height: f32,
    spacing: f32,
) {
    let [first, second]: &[_; 2] = children.try_into().unwrap();

    let aspect_ratio = width / height;

    let ar1 = first.aspect_ratio;
    let ar2 = second.aspect_ratio;

    let (lf1, pf1, lf2, pf2);
    if proportions == "ww"
        && average_aspect_ratio > 1.4 * aspect_ratio
        && first.aspect_ratio - second.aspect_ratio < 0.2
    {
        let width = width;
        let height = (width / ar1).min(width / ar2).min((height - spacing) / 2.0);

        lf1 = (0.0, 0.0, width, height);
        pf1 = PositionFlags::TOP | PositionFlags::FULL_WIDTH;

        lf2 = (0.0, height + spacing, width, height);
        pf2 = PositionFlags::BOTTOM | PositionFlags::FULL_WIDTH;
    } else if matches!(proportions, "ww" | "qq") {
        let width = (width - spacing) * 0.5;
        let height = (width / first.aspect_ratio)
            .min(width / second.aspect_ratio)
            .min(height);

        lf1 = (0.0, 0.0, width, height);
        pf1 = PositionFlags::LEFT | PositionFlags::FULL_HEIGHT;
        lf2 = (width + spacing, 0.0, width, height);
        pf2 = PositionFlags::RIGHT | PositionFlags::FULL_HEIGHT;
    } else {
        let first_width = (width - spacing)
            / second.aspect_ratio
            / (1.0 / first.aspect_ratio + 1.0 / second.aspect_ratio);
        let second_width = width - first_width - spacing;

        let height = height
            .min(first_width / first.aspect_ratio)
            .min(second_width / second.aspect_ratio);

        lf1 = (0.0, 0.0, first_width, height);
        pf1 = PositionFlags::LEFT | PositionFlags::FULL_HEIGHT;
        lf2 = (first_width + spacing, 0.0, second_width, height);
        pf2 = PositionFlags::RIGHT | PositionFlags::FULL_HEIGHT;
    };

    first.layout_frame.set(lf1);
    first.position_flags.set(pf1);
    second.layout_frame.set(lf2);
    second.position_flags.set(pf2);
}

fn layout_three_children(
    children: &[ChildWrapper],
    proportions: &str,
    _average_aspect_ratio: f32,
    width: f32,
    height: f32,
    spacing: f32,
) {
    let [first, second, third]: &[_; 3] = children.try_into().unwrap();

    let f_ar = first.aspect_ratio;
    let s_ar = second.aspect_ratio;
    let t_ar = third.aspect_ratio;

    let (f_lf, f_pf, s_lf, s_pf, t_lf, t_pf);
    if proportions.starts_with('n') {
        let first_height = height;

        let third_height =
            ((height - spacing) * 0.5).min((s_ar * (width - spacing) / (t_ar + f_ar)).round());
        let second_height = height - third_height - spacing;

        let right_width = ((width - spacing) * 0.5)
            .min((third_height * t_ar).min(second_height * s_ar).round())
            .max(MIN_WIDTH);

        let left_width = (first_height * f_ar)
            .min(width - spacing - right_width)
            .round();

        f_lf = (0.0, 0.0, left_width, first_height);
        f_pf = PositionFlags::LEFT | PositionFlags::FULL_HEIGHT;

        s_lf = (left_width + spacing, 0.0, right_width, second_height);
        s_pf = PositionFlags::TOP_RIGHT;

        t_lf = (
            left_width + spacing,
            second_height + spacing,
            right_width,
            third_height,
        );
        t_pf = PositionFlags::BOTTOM_RIGHT;
    } else {
        let width = width;
        let first_height = (width / f_ar).min((height - spacing) * 0.66).floor();

        let half_width = (width - spacing) * 0.5;

        let second_height = (height - first_height - spacing)
            .min((half_width / s_ar).min(half_width / t_ar).round());

        f_lf = (0.0, 0.0, width, first_height);
        f_pf = PositionFlags::TOP | PositionFlags::FULL_WIDTH;

        s_lf = (0.0, first_height + spacing, half_width, second_height);
        s_pf = PositionFlags::BOTTOM_LEFT;

        t_lf = (
            half_width + spacing,
            first_height + spacing,
            half_width,
            second_height,
        );

        t_pf = PositionFlags::BOTTOM_RIGHT;
    };

    first.layout_frame.set(f_lf);
    first.position_flags.set(f_pf);
    second.layout_frame.set(s_lf);
    second.position_flags.set(s_pf);
    third.layout_frame.set(t_lf);
    third.position_flags.set(t_pf);
}

fn layout_four_children(
    children: &[ChildWrapper],
    proportions: &str,
    _average_aspect_ratio: f32,
    width: f32,
    height: f32,
    spacing: f32,
) {
    let [first, second, third, last]: &[_; 4] = children.try_into().unwrap();

    let f_ar = first.aspect_ratio;
    let s_ar = second.aspect_ratio;
    let t_ar = third.aspect_ratio;
    let l_ar = last.aspect_ratio;

    let (f_lf, f_pf, s_lf, s_pf, t_lf, t_pf, l_lf, l_pf);
    if proportions.starts_with('w') {
        let w = width;

        let h0 = (w / f_ar).min((height - spacing) * 0.66);

        f_lf = (0.0, 0.0, w, h0);
        f_pf = PositionFlags::TOP | PositionFlags::FULL_WIDTH;

        let h = (width - 2.0 * spacing) / (s_ar + t_ar + l_ar);
        let w0 = ((width - 2.0 * spacing) * 0.33).max(h * s_ar);
        let w2 = ((width - 2.0 * spacing) * 0.33).max(h * l_ar);
        let w1 = w - w0 - w2 - 2.0 * spacing;

        let (w1, w2) = if w1 < MIN_WIDTH {
            (MIN_WIDTH, w2 - MIN_WIDTH - w1)
        } else {
            (w1, w2)
        };

        let h = h.min(height - h0 - spacing);

        s_lf = (0.0, h0 + spacing, w0, h);
        s_pf = PositionFlags::BOTTOM_LEFT;

        t_lf = (w0 + spacing, h0 + spacing, w1, h);
        t_pf = PositionFlags::BOTTOM;

        l_lf = (w0 + w1 + 2.0 * spacing, h0 + spacing, w2, h);
        l_pf = PositionFlags::BOTTOM_RIGHT;
    } else {
        let h: f32 = height;
        let w0: f32 = f32::min(h * f_ar, (width - spacing) * 0.6);
        f_lf = (0.0, 0.0, w0, h);
        f_pf = PositionFlags::LEFT | PositionFlags::FULL_HEIGHT;

        let w: f32 = (height - 2.0 * spacing) / (1.0 / s_ar + 1.0 / t_ar + 1.0 / l_ar);
        let h0: f32 = w / s_ar;
        let h1: f32 = w / t_ar;
        let h2: f32 = w / l_ar;

        let w = w.min(width - w0 - spacing);
        s_lf = (w0 + spacing, 0.0, w, h0);
        s_pf = PositionFlags::TOP_RIGHT;

        t_lf = (w0 + spacing, h0 + spacing, w, h1);
        t_pf = PositionFlags::RIGHT;

        l_lf = (w0 + spacing, h0 + h1 + 2.0 * spacing, w, h2);
        l_pf = PositionFlags::BOTTOM_RIGHT;
    };

    first.layout_frame.set(f_lf);
    first.position_flags.set(f_pf);
    second.layout_frame.set(s_lf);
    second.position_flags.set(s_pf);
    third.layout_frame.set(t_lf);
    third.position_flags.set(t_pf);
    last.layout_frame.set(l_lf);
    last.position_flags.set(l_pf);
}

fn layout_fallback(
    children: &[ChildWrapper],
    _proportions: &str,
    average_aspect_ratio: f32,
    width: f32,
    height: f32,
    spacing: f32,
) {
    struct GroupedLayoutAttempt {
        line_counts: Vec<usize>,
        heights: Vec<f32>,
    }

    let mut cropped_ratios = vec![];
    for child in children {
        if average_aspect_ratio > 1.1 {
            cropped_ratios.push(child.aspect_ratio.max(1.0));
        } else {
            cropped_ratios.push(child.aspect_ratio.min(1.0));
        }
    }

    let multi_height = |ratios: &[f32]| {
        let ratio_sum: f32 = ratios.iter().sum();
        (width - (ratios.len() as f32 - 1.0) * spacing) / ratio_sum
    };

    let mut attempts = vec![];

    let mut add_attempt = |line_counts: Vec<usize>, heights: Vec<f32>| {
        attempts.push(GroupedLayoutAttempt {
            line_counts,
            heights,
        });
    };

    add_attempt(vec![children.len()], vec![multi_height(&cropped_ratios)]);

    {
        // Try attemts for different line counts
        let mut second_line;
        let mut third_line;
        let mut fourth_line;

        let len = cropped_ratios.len();

        for first_line in 1..len {
            second_line = len - first_line;
            if first_line > 3 || second_line > 3 {
                continue;
            }

            add_attempt(
                vec![first_line, len - first_line],
                vec![
                    multi_height(&cropped_ratios[..first_line]),
                    multi_height(&cropped_ratios[first_line..]),
                ],
            )
        }

        for first_line in 1..len - 1 {
            for second_line in 1..len - first_line {
                third_line = len - first_line - second_line;
                if first_line > 3
                    || second_line > (if average_aspect_ratio < 0.85 { 4 } else { 3 })
                    || third_line > 3
                {
                    continue;
                }
                add_attempt(
                    vec![first_line, second_line, third_line],
                    vec![
                        multi_height(&cropped_ratios[..first_line]),
                        multi_height(&cropped_ratios[first_line..len - third_line]),
                        multi_height(&cropped_ratios[first_line + second_line..]),
                    ],
                )
            }
        }

        if len > 2 {
            for first_line in 1..len - 2 {
                for second_line in 1..len - first_line {
                    for third_line in 1..len - first_line - second_line {
                        fourth_line = len - first_line - second_line - third_line;
                        if first_line > 3 || second_line > 3 || third_line > 3 || fourth_line > 3 {
                            continue;
                        }

                        add_attempt(
                            vec![first_line, second_line, third_line, fourth_line],
                            vec![
                                multi_height(&cropped_ratios[..first_line]),
                                multi_height(
                                    &cropped_ratios[first_line..len - third_line - fourth_line],
                                ),
                                multi_height(
                                    &cropped_ratios[first_line + second_line..len - fourth_line],
                                ),
                                multi_height(
                                    &cropped_ratios[first_line + second_line + third_line..],
                                ),
                            ],
                        )
                    }
                }
            }
        }
    }

    let max_height = height / 3.0 * 4.0;
    let mut optimal = None;
    let mut optimal_diff = f32::MAX;

    for attempt in attempts {
        let mut total_height = spacing * (attempt.heights.len() - 1) as f32;
        let mut min_line_height = f32::MAX;
        let mut max_line_height = 0.0;

        for height in &attempt.heights {
            total_height += height;
            min_line_height = height.min(min_line_height);
            max_line_height = height.max(max_line_height);
        }

        let mut diff = (total_height - max_height).abs();

        if attempt.line_counts.len() >= 2
            && ((attempt.line_counts[0] > attempt.line_counts[1])
                || (attempt.line_counts.len() > 2
                    && attempt.line_counts[1] > attempt.line_counts[2])
                || (attempt.line_counts.len() > 3
                    && attempt.line_counts[2] > attempt.line_counts[3]))
        {
            diff *= 1.5;
        }

        if min_line_height < MIN_WIDTH {
            diff *= 1.5;
        }

        if diff < optimal_diff {
            optimal = Some(attempt);
            optimal_diff = diff;
        }
    }

    let mut index = 0;
    let mut y = 0.0;

    let optimal = optimal.unwrap();

    for i in 0..optimal.line_counts.len() {
        let count = optimal.line_counts[i];
        let line_height = optimal.heights[i];

        let mut x = 0.0;

        let mut position_flags = PositionFlags::NONE;

        if i == 0 {
            position_flags |= PositionFlags::TOP;
        } else if i == optimal.line_counts.len() - 1 {
            position_flags |= PositionFlags::BOTTOM;
        }

        for k in 0..count {
            let mut inner_position_flags = position_flags;

            if k == 0 {
                inner_position_flags |= PositionFlags::LEFT;
            }
            if k == count - 1 {
                inner_position_flags |= PositionFlags::RIGHT;
            }

            if position_flags == PositionFlags::NONE {
                inner_position_flags |= PositionFlags::INSIDE;
            }

            let ratio = cropped_ratios[index];
            let width = ratio * line_height;

            children[index].layout_frame.set((x, y, width, line_height));
            children[index].position_flags.set(inner_position_flags);

            x += width + spacing;
            index += 1
        }

        y += line_height + spacing;
    }
}
