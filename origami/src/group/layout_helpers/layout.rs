use super::child_wrapper::ChildWrapper;
use super::position_flags::PositionFlags;

const MIN_WIDTH: f32 = 70.0;

pub(crate) fn layout_function(
    child_count: usize,
    force_calc: bool,
) -> fn(
    children: &[ChildWrapper],
    proportions: &str,
    average_aspect_ratio: f32,
    width: f32,
    spacing: f32,
) {
    if force_calc {
        layout_fallback
    } else {
        match child_count {
            2 => layout_two_children,
            3 => layout_three_children,
            4 => layout_four_children,
            _ => layout_fallback,
        }
    }
}

fn layout_two_children(
    children: &[ChildWrapper],
    proportions: &str,
    average_aspect_ratio: f32,
    width: f32,
    spacing: f32,
) {
    let [child_0, child_1]: &[_; 2] = children.try_into().unwrap();

    let aspect_ratio_0 = child_0.aspect_ratio;
    let aspect_ratio_1 = child_1.aspect_ratio;

    let (layout_frame_0, position_flags_0, layout_frame_1, position_flags_1);
    if proportions == "ww"
        && average_aspect_ratio > 1.4
        && child_0.aspect_ratio - child_1.aspect_ratio < 0.2
    {
        let width = width;
        let height = (width / aspect_ratio_0).min(width / aspect_ratio_1);

        layout_frame_0 = (0.0, 0.0, width, height);
        position_flags_0 = PositionFlags::TOP | PositionFlags::FULL_WIDTH;

        layout_frame_1 = (0.0, height + spacing, width, height);
        position_flags_1 = PositionFlags::BOTTOM | PositionFlags::FULL_WIDTH;
    } else if matches!(proportions, "ww" | "qq") {
        let width = (width - spacing) * 0.5;
        let height = (width / aspect_ratio_0).min(width / aspect_ratio_1);

        layout_frame_0 = (0.0, 0.0, width, height);
        position_flags_0 = PositionFlags::LEFT | PositionFlags::FULL_HEIGHT;
        layout_frame_1 = (width + spacing, 0.0, width, height);
        position_flags_1 = PositionFlags::RIGHT | PositionFlags::FULL_HEIGHT;
    } else {
        let first_width = (width - spacing)
            / child_1.aspect_ratio
            / (1.0 / aspect_ratio_0 + 1.0 / aspect_ratio_1);
        let second_width = width - first_width - spacing;

        let height = (first_width / aspect_ratio_0).min(second_width / aspect_ratio_1);

        layout_frame_0 = (0.0, 0.0, first_width, height);
        position_flags_0 = PositionFlags::LEFT | PositionFlags::FULL_HEIGHT;
        layout_frame_1 = (first_width + spacing, 0.0, second_width, height);
        position_flags_1 = PositionFlags::RIGHT | PositionFlags::FULL_HEIGHT;
    };

    child_0.layout_frame.set(layout_frame_0);
    child_0.position_flags.set(position_flags_0);
    child_1.layout_frame.set(layout_frame_1);
    child_1.position_flags.set(position_flags_1);
}

fn layout_three_children(
    children: &[ChildWrapper],
    proportions: &str,
    average_aspect_ratio: f32,
    width: f32,
    spacing: f32,
) {
    let [child_0, child_1, child_2]: &[_; 3] = children.try_into().unwrap();

    let aspect_ratio_0 = child_0.aspect_ratio;
    let aspect_ratio_1 = child_1.aspect_ratio;
    let aspect_ratio_2 = child_2.aspect_ratio;

    let height = width / average_aspect_ratio;

    let (
        layout_frame_0,
        position_flags_0,
        layout_frame_1,
        position_flags_1,
        layout_frame_2,
        position_flags_2,
    );

    if proportions.starts_with('n') {
        let first_height = height;

        let third_height = ((height - spacing) * 0.5)
            .min((aspect_ratio_1 * (width - spacing) / (aspect_ratio_2 + aspect_ratio_0)).round());
        let second_height = height - third_height - spacing;

        let right_width = ((width - spacing) * 0.5)
            .min(
                (third_height * aspect_ratio_2)
                    .min(second_height * aspect_ratio_1)
                    .round(),
            )
            .max(MIN_WIDTH);

        let left_width = (first_height * aspect_ratio_0)
            .min(width - spacing - right_width)
            .round();

        layout_frame_0 = (0.0, 0.0, left_width, first_height);
        position_flags_0 = PositionFlags::LEFT | PositionFlags::FULL_HEIGHT;

        layout_frame_1 = (left_width + spacing, 0.0, right_width, second_height);
        position_flags_1 = PositionFlags::TOP_RIGHT;

        layout_frame_2 = (
            left_width + spacing,
            second_height + spacing,
            right_width,
            third_height,
        );
        position_flags_2 = PositionFlags::BOTTOM_RIGHT;
    } else {
        let width = width;
        let first_height = (width / aspect_ratio_0)
            .min((height - spacing) * 0.66)
            .floor();

        let half_width = (width - spacing) * 0.5;

        let second_height = (height - first_height - spacing).min(
            (half_width / aspect_ratio_1)
                .min(half_width / aspect_ratio_2)
                .round(),
        );

        layout_frame_0 = (0.0, 0.0, width, first_height);
        position_flags_0 = PositionFlags::TOP | PositionFlags::FULL_WIDTH;

        layout_frame_1 = (0.0, first_height + spacing, half_width, second_height);
        position_flags_1 = PositionFlags::BOTTOM_LEFT;

        layout_frame_2 = (
            half_width + spacing,
            first_height + spacing,
            half_width,
            second_height,
        );

        position_flags_2 = PositionFlags::BOTTOM_RIGHT;
    };

    child_0.layout_frame.set(layout_frame_0);
    child_0.position_flags.set(position_flags_0);
    child_1.layout_frame.set(layout_frame_1);
    child_1.position_flags.set(position_flags_1);
    child_2.layout_frame.set(layout_frame_2);
    child_2.position_flags.set(position_flags_2);
}

fn layout_four_children(
    children: &[ChildWrapper],
    proportions: &str,
    average_aspect_ratio: f32,
    width: f32,
    spacing: f32,
) {
    let [child_0, child_1, child_2, child_3]: &[_; 4] = children.try_into().unwrap();

    let aspect_ratio_0 = child_0.aspect_ratio;
    let aspect_ratio_1 = child_1.aspect_ratio;
    let aspect_ratio_2 = child_2.aspect_ratio;
    let aspect_ratio_3 = child_3.aspect_ratio;

    let (
        layout_frame_0,
        position_flags_0,
        layout_frame_1,
        position_flags_1,
        layout_frame_2,
        position_flags_2,
        layout_frame_3,
        position_flags_3,
    );

    if proportions.starts_with('w') {
        let w = width;

        let h0 = w / aspect_ratio_0;

        layout_frame_0 = (0.0, 0.0, w, h0);
        position_flags_0 = PositionFlags::TOP | PositionFlags::FULL_WIDTH;

        let h = (width - 2.0 * spacing) / (aspect_ratio_1 + aspect_ratio_2 + aspect_ratio_3);
        let w0 = ((width - 2.0 * spacing) * 0.33).max(h * aspect_ratio_1);
        let w2 = ((width - 2.0 * spacing) * 0.33).max(h * aspect_ratio_3);
        let w1 = w - w0 - w2 - 2.0 * spacing;

        let (w1, w2) = if w1 < MIN_WIDTH {
            (MIN_WIDTH, w2 - MIN_WIDTH - w1)
        } else {
            (w1, w2)
        };

        layout_frame_1 = (0.0, h0 + spacing, w0, h);
        position_flags_1 = PositionFlags::BOTTOM_LEFT;

        layout_frame_2 = (w0 + spacing, h0 + spacing, w1, h);
        position_flags_2 = PositionFlags::BOTTOM;

        layout_frame_3 = (w0 + w1 + 2.0 * spacing, h0 + spacing, w2, h);
        position_flags_3 = PositionFlags::BOTTOM_RIGHT;
    } else {
        let height = width / average_aspect_ratio;

        let h: f32 = height;
        let left_width: f32 = f32::min(h * aspect_ratio_0, (width - spacing) * 0.6);
        layout_frame_0 = (0.0, 0.0, left_width, h);
        position_flags_0 = PositionFlags::LEFT | PositionFlags::FULL_HEIGHT;

        let w: f32 = (height - 2.0 * spacing)
            / (1.0 / aspect_ratio_1 + 1.0 / aspect_ratio_2 + 1.0 / aspect_ratio_3);

        let h0: f32 = w / aspect_ratio_1;
        let h1: f32 = w / aspect_ratio_2;
        let h2: f32 = w / aspect_ratio_3;

        let right_width = width - left_width - spacing;

        layout_frame_1 = (left_width + spacing, 0.0, right_width, h0);
        position_flags_1 = PositionFlags::TOP_RIGHT;

        layout_frame_2 = (left_width + spacing, h0 + spacing, right_width, h1);
        position_flags_2 = PositionFlags::RIGHT;

        layout_frame_3 = (
            left_width + spacing,
            h0 + h1 + 2.0 * spacing,
            right_width,
            h2,
        );
        position_flags_3 = PositionFlags::BOTTOM_RIGHT;
    };

    child_0.layout_frame.set(layout_frame_0);
    child_0.position_flags.set(position_flags_0);
    child_1.layout_frame.set(layout_frame_1);
    child_1.position_flags.set(position_flags_1);
    child_2.layout_frame.set(layout_frame_2);
    child_2.position_flags.set(position_flags_2);
    child_3.layout_frame.set(layout_frame_3);
    child_3.position_flags.set(position_flags_3);
}

fn layout_fallback(
    children: &[ChildWrapper],
    _proportions: &str,
    average_aspect_ratio: f32,
    width: f32,
    spacing: f32,
) {
    struct GroupedLayoutAttempt {
        line_counts: Vec<usize>,
        heights: Vec<f32>,
    }

    let cropped_ratios: Vec<_> = children
        .into_iter()
        .map(|c| {
            if average_aspect_ratio > 1.1 {
                c.aspect_ratio.max(1.0)
            } else {
                c.aspect_ratio.min(1.0)
            }
        })
        .collect();

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
        // Try attempts for different line counts
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

    let max_height = 300.0;
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
