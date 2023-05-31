use gtk::glib;
use gtk::prelude::*;

use super::*;

pub(super) fn layout(widget: &gtk::Widget, width: i32, spacing: f32) -> Vec<ChildWrapper> {
    let children: Vec<_> = widget
        .observe_children()
        .iter::<glib::Object>()
        .filter_map(ChildWrapper::from_result)
        .collect();

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

    let layout_function = layout_function(children.len(), force_calc);

    let width = width as f32;

    layout_function(
        &children,
        &proportions,
        average_aspect_ratio,
        width,
        spacing,
    );

    children
}

pub(super) fn measure_height(widget: &gtk::Widget, width: i32, spacing: f32) -> i32 {
    let children: Vec<_> = widget
        .observe_children()
        .iter::<glib::Object>()
        .filter_map(ChildWrapper::from_result)
        .collect();

    if children.len() <= 1 {
        if let Some(child) = widget.first_child() {
            child.measure(gtk::Orientation::Vertical, width).1
        } else {
            -1
        }
    } else {
        let layout = layout(widget, width, spacing);
        let lf = layout.last().unwrap().layout_frame.get();
        (lf.1 + lf.3) as i32
    }
}
