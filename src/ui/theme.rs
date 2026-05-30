use bevy::prelude::*;

pub(super) const STANDARD_PANEL_SIZE: Vec2 = Vec2::new(368.0, 502.0);
pub(super) const WORKSPACE_PANEL_SIZE: Vec2 = Vec2::new(762.0, 502.0);
pub(super) const ACTION_BUTTON_SIZE: Vec2 = Vec2::new(112.0, 34.0);
pub(super) const GRID_GAP: f32 = 6.0;

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
    pub(super) const SECTION_TITLE: f32 = 14.0;
    pub(super) const BODY: f32 = 12.0;
    pub(super) const BODY_SMALL: f32 = 10.5;
    pub(super) const BUTTON: f32 = 12.0;
}

pub(super) struct UiPanelSpec {
    pub(super) left: f32,
    pub(super) top: f32,
    pub(super) size: Vec2,
    pub(super) title: &'static str,
    pub(super) body_color: Color,
    pub(super) visibility: Visibility,
    pub(super) z_index: i32,
}

pub(super) fn spawn_panel_window<C: Component>(
    commands: &mut Commands,
    marker: C,
    spec: UiPanelSpec,
    spawn_body: impl FnOnce(&mut ChildSpawnerCommands),
) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(spec.left),
                top: px(spec.top),
                width: px(spec.size.x),
                height: px(spec.size.y),
                padding: UiRect::all(px(6)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(UiColors::frame_shadow()),
            spec.visibility,
            ZIndex(spec.z_index),
            marker,
        ))
        .with_children(|panel| {
            panel
                .spawn((
                    Node {
                        width: percent(100),
                        height: percent(100),
                        padding: UiRect::all(px(6)),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(8),
                        ..default()
                    },
                    BackgroundColor(UiColors::frame_shell()),
                ))
                .with_children(|shell| {
                    shell
                        .spawn((
                            Node {
                                width: percent(100),
                                height: px(40),
                                padding: UiRect::axes(px(18), px(0)),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(UiColors::header()),
                        ))
                        .with_children(|header| {
                            spawn_text_label(
                                header,
                                spec.title,
                                UiFontSize::WINDOW_TITLE,
                                UiColors::text_header(),
                            );
                        });

                    shell.spawn((
                        Node {
                            width: percent(100),
                            height: px(4),
                            ..default()
                        },
                        BackgroundColor(UiColors::accent()),
                    ));

                    shell
                        .spawn((
                            Node {
                                width: percent(100),
                                flex_grow: 1.0,
                                min_height: px(0),
                                padding: UiRect::all(px(10)),
                                flex_direction: FlexDirection::Column,
                                row_gap: px(8),
                                overflow: Overflow::clip_y(),
                                ..default()
                            },
                            BackgroundColor(spec.body_color),
                        ))
                        .with_children(spawn_body);
                });
        });
}

pub(super) fn spawn_panel_section(
    parent: &mut ChildSpawnerCommands,
    title: &'static str,
    flex_grow: f32,
    spawn_content: impl FnOnce(&mut ChildSpawnerCommands),
) {
    parent
        .spawn((
            Node {
                width: percent(100),
                flex_grow,
                min_height: px(0),
                padding: UiRect::all(px(8)),
                flex_direction: FlexDirection::Column,
                row_gap: px(6),
                overflow: Overflow::clip_y(),
                ..default()
            },
            BackgroundColor(UiColors::section()),
        ))
        .with_children(|section| {
            if !title.is_empty() {
                spawn_text_label(
                    section,
                    title,
                    UiFontSize::SECTION_TITLE,
                    UiColors::text_section(),
                );
            }
            spawn_content(section);
        });
}

pub(super) fn spawn_text_label(
    parent: &mut ChildSpawnerCommands,
    label: impl Into<String>,
    font_size: f32,
    color: Color,
) {
    parent.spawn((
        Text::new(label.into()),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(color),
        TextLayout::new_with_justify(Justify::Left),
        Label,
    ));
}

pub(super) fn action_button_node(size: Vec2) -> Node {
    Node {
        width: px(size.x),
        height: px(size.y),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        border: UiRect::all(px(2)),
        ..default()
    }
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
