use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::components::ScreenFixed;

pub(super) const STANDARD_PANEL_SIZE: Vec2 = Vec2::new(368.0, 502.0);
pub(super) const WIDE_PANEL_SIZE: Vec2 = Vec2::new(724.0, 540.0);
pub(super) const ACTION_BUTTON_SIZE: Vec2 = Vec2::new(112.0, 34.0);

pub(super) struct UiColors;

impl UiColors {
    pub(super) fn frame_shadow() -> Color {
        Color::srgba(0.025, 0.025, 0.025, 0.98)
    }

    pub(super) fn frame_shell() -> Color {
        Color::srgba(0.18, 0.17, 0.16, 0.97)
    }

    pub(super) fn frame_body() -> Color {
        Color::srgba(0.09, 0.085, 0.08, 0.97)
    }

    pub(super) fn frame_body_cool() -> Color {
        Color::srgba(0.075, 0.08, 0.095, 0.97)
    }

    pub(super) fn header() -> Color {
        Color::srgba(0.56, 0.10, 0.07, 0.98)
    }

    pub(super) fn accent() -> Color {
        Color::srgba(0.98, 0.56, 0.12, 0.92)
    }

    pub(super) fn divider() -> Color {
        Color::srgba(0.70, 0.50, 0.24, 0.62)
    }

    pub(super) fn section() -> Color {
        Color::srgba(0.13, 0.115, 0.10, 0.96)
    }

    pub(super) fn text_header() -> Color {
        Color::srgb(1.0, 0.72, 0.20)
    }

    pub(super) fn text_section() -> Color {
        Color::srgb(0.96, 0.70, 0.32)
    }

    pub(super) fn text_primary() -> Color {
        Color::srgb(0.92, 0.89, 0.80)
    }

    pub(super) fn text_muted() -> Color {
        Color::srgb(0.78, 0.72, 0.62)
    }

    pub(super) fn text_dark() -> Color {
        Color::srgb(0.14, 0.08, 0.035)
    }
}

pub(super) struct UiFontSize;

impl UiFontSize {
    pub(super) const WINDOW_TITLE: f32 = 22.0;
    pub(super) const BODY_SMALL: f32 = 10.5;
    pub(super) const BUTTON: f32 = 12.0;
}

pub(super) fn spawn_wide_panel_chrome<C: Component>(
    commands: &mut Commands,
    marker: impl Fn() -> C + Copy,
    center: Vec3,
    title: &'static str,
    body_color: Color,
    visibility: Visibility,
) {
    spawn_panel_rect(
        commands,
        marker(),
        center,
        WIDE_PANEL_SIZE,
        UiColors::frame_shadow(),
        visibility,
    );
    spawn_panel_rect(
        commands,
        marker(),
        center + Vec3::new(0.0, 0.0, 1.0),
        Vec2::new(712.0, 528.0),
        UiColors::frame_shell(),
        visibility,
    );
    spawn_panel_rect(
        commands,
        marker(),
        center + Vec3::new(0.0, -20.0, 2.0),
        Vec2::new(696.0, 466.0),
        body_color,
        visibility,
    );
    spawn_panel_rect(
        commands,
        marker(),
        center + Vec3::new(0.0, 238.0, 3.0),
        Vec2::new(710.0, 40.0),
        UiColors::header(),
        visibility,
    );
    spawn_panel_rect(
        commands,
        marker(),
        center + Vec3::new(0.0, 215.0, 4.0),
        Vec2::new(710.0, 4.0),
        UiColors::accent(),
        visibility,
    );
    spawn_panel_label(
        commands,
        marker(),
        title,
        Vec3::new(
            center.x - WIDE_PANEL_SIZE.x * 0.5 + 24.0,
            center.y + 252.0,
            center.z + 6.0,
        ),
        UiFontSize::WINDOW_TITLE,
        UiColors::text_header(),
        visibility,
        Justify::Left,
        Anchor::TOP_LEFT,
    );
}

pub(super) fn spawn_panel_rect<C: Component>(
    commands: &mut Commands,
    marker: C,
    offset: Vec3,
    size: Vec2,
    color: Color,
    visibility: Visibility,
) {
    commands.spawn((
        Sprite::from_color(color, size),
        Transform::from_translation(offset),
        visibility,
        ScreenFixed { offset },
        marker,
    ));
}

pub(super) fn spawn_panel_label<C: Component>(
    commands: &mut Commands,
    marker: C,
    label: &'static str,
    offset: Vec3,
    font_size: f32,
    color: Color,
    visibility: Visibility,
    justify: Justify,
    anchor: Anchor,
) {
    commands.spawn((
        Text2d::new(label),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(color),
        TextLayout::new_with_justify(justify),
        anchor,
        Transform::from_translation(offset),
        visibility,
        ScreenFixed { offset },
        marker,
    ));
}

pub(super) fn action_button_color(enabled: bool, hovered: bool) -> Color {
    if enabled && hovered {
        Color::srgba(0.92, 0.50, 0.08, 0.98)
    } else if enabled {
        Color::srgba(0.56, 0.18, 0.05, 0.98)
    } else if hovered {
        Color::srgba(0.34, 0.19, 0.12, 0.98)
    } else {
        Color::srgba(0.18, 0.13, 0.10, 0.98)
    }
}

pub(super) fn navigation_button_color(active: bool, hovered: bool) -> Color {
    if active {
        Color::srgba(0.92, 0.50, 0.08, 0.98)
    } else if hovered {
        Color::srgba(0.58, 0.20, 0.06, 0.98)
    } else {
        Color::srgba(0.30, 0.11, 0.04, 0.98)
    }
}

pub(super) fn truncate_chars(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }

    let keep = max_chars.saturating_sub(3);
    let mut truncated = text.chars().take(keep).collect::<String>();
    truncated.push_str("...");
    truncated
}

pub(super) fn bounded_lines(
    raw_lines: impl IntoIterator<Item = String>,
    max_chars: usize,
    max_lines: usize,
) -> String {
    if max_lines == 0 {
        return String::new();
    }

    let mut lines = Vec::new();
    let mut truncated = false;

    'outer: for raw_line in raw_lines {
        for line in wrap_line(&raw_line, max_chars) {
            if lines.len() == max_lines {
                truncated = true;
                break 'outer;
            }
            lines.push(line);
        }
    }

    if truncated {
        if let Some(last_line) = lines.last_mut() {
            let keep = max_chars.saturating_sub(3);
            while last_line.chars().count() > keep {
                last_line.pop();
            }
            last_line.push_str("...");
        }
    }

    lines.join("\n")
}

pub(super) fn wrap_line(line: &str, max_chars: usize) -> Vec<String> {
    if line.is_empty() || line.chars().count() <= max_chars {
        return vec![line.to_string()];
    }

    let mut wrapped = Vec::new();
    let mut current_line = String::new();

    for word in line.split_whitespace() {
        let word_length = word.chars().count();
        if current_line.is_empty() {
            current_line = word.chars().take(max_chars).collect();
        } else if current_line.chars().count() + 1 + word_length <= max_chars {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            wrapped.push(current_line);
            current_line = word.chars().take(max_chars).collect();
        }
    }

    if !current_line.is_empty() {
        wrapped.push(current_line);
    }
    wrapped
}
