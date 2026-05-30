use bevy::prelude::*;

use crate::components::{
    ActivePanel, TalentConnector, TalentHeaderText, TalentInfoText, TalentNodeButton,
    TalentNodeLabel, TalentPanelPiece, TalentResetButton, TalentResetLabel, UiState,
};
use crate::data::{GameDatabase, PlayerProfile, TalentNode};

use super::theme::{
    UiColors, UiFontSize, UiPanelSpec, WORKSPACE_PANEL_SIZE, action_button_color,
    action_button_node, bounded_lines, spawn_panel_window, spawn_text_label, truncate_chars,
};

const TALENT_CANVAS_WIDTH: f32 = 470.0;
const TALENT_CANVAS_HEIGHT: f32 = 390.0;
const TALENT_CANVAS_MIN_X: f32 = -485.0;
const TALENT_CANVAS_MAX_Y: f32 = 270.0;
const TALENT_NODE_SIZE: Vec2 = Vec2::new(70.0, 52.0);
const TALENT_CONNECTOR_THICKNESS: f32 = 4.0;

pub(super) fn spawn_talent_panel(commands: &mut Commands, talents: &[TalentNode]) {
    spawn_panel_window(
        commands,
        TalentPanelPiece,
        UiPanelSpec {
            left: 12.0,
            top: 17.0,
            size: WORKSPACE_PANEL_SIZE,
            title: "TALENTS",
            body_color: UiColors::frame_body_cool(),
            visibility: Visibility::Hidden,
            z_index: 12,
        },
        |body| {
            body.spawn((Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Row,
                column_gap: px(10),
                ..default()
            },))
                .with_children(|layout| {
                    layout
                        .spawn((
                            Node {
                                width: px(492),
                                height: percent(100),
                                padding: UiRect::all(px(8)),
                                flex_direction: FlexDirection::Column,
                                row_gap: px(6),
                                overflow: Overflow::clip_y(),
                                ..default()
                            },
                            BackgroundColor(UiColors::section()),
                        ))
                        .with_children(|tree| {
                            spawn_text_label(
                                tree,
                                "Class Tree",
                                UiFontSize::SECTION_TITLE,
                                UiColors::text_section(),
                            );
                            tree.spawn((
                                Node {
                                    width: px(TALENT_CANVAS_WIDTH),
                                    height: px(TALENT_CANVAS_HEIGHT),
                                    position_type: PositionType::Relative,
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0.055, 0.06, 0.072, 0.88)),
                            ))
                            .with_children(|canvas| {
                                for (index, node) in talents.iter().enumerate() {
                                    if let Some(required_index) = node.requires {
                                        spawn_talent_connector(
                                            canvas,
                                            index,
                                            talents[required_index].position,
                                            node.position,
                                        );
                                    }
                                }
                                for (index, node) in talents.iter().enumerate() {
                                    spawn_talent_node(canvas, index, node);
                                }
                            });
                        });

                    layout
                        .spawn((Node {
                            flex_grow: 1.0,
                            height: percent(100),
                            flex_direction: FlexDirection::Column,
                            row_gap: px(8),
                            ..default()
                        },))
                        .with_children(|details| {
                            details
                                .spawn((
                                    Node {
                                        width: percent(100),
                                        padding: UiRect::all(px(8)),
                                        flex_direction: FlexDirection::Column,
                                        row_gap: px(6),
                                        ..default()
                                    },
                                    BackgroundColor(UiColors::section()),
                                ))
                                .with_children(|summary| {
                                    spawn_text_label(
                                        summary,
                                        "Progress",
                                        UiFontSize::SECTION_TITLE,
                                        UiColors::text_section(),
                                    );
                                    summary.spawn((
                                        Text::new(""),
                                        TextFont {
                                            font_size: UiFontSize::BODY,
                                            ..default()
                                        },
                                        TextColor(UiColors::text_primary()),
                                        TextLayout::new_with_justify(Justify::Left),
                                        TalentHeaderText,
                                        Label,
                                    ));
                                });

                            details
                                .spawn((
                                    Node {
                                        width: percent(100),
                                        flex_grow: 1.0,
                                        min_height: px(0),
                                        padding: UiRect::all(px(8)),
                                        flex_direction: FlexDirection::Column,
                                        row_gap: px(6),
                                        overflow: Overflow::clip_y(),
                                        ..default()
                                    },
                                    BackgroundColor(UiColors::section()),
                                ))
                                .with_children(|info| {
                                    spawn_text_label(
                                        info,
                                        "Selected Talent",
                                        UiFontSize::SECTION_TITLE,
                                        UiColors::text_section(),
                                    );
                                    info.spawn((
                                        Text::new(""),
                                        TextFont {
                                            font_size: UiFontSize::BODY,
                                            ..default()
                                        },
                                        TextColor(UiColors::text_primary()),
                                        TextLayout::new_with_justify(Justify::Left),
                                        Node {
                                            width: percent(100),
                                            ..default()
                                        },
                                        TalentInfoText,
                                        Label,
                                    ));
                                });

                            details
                                .spawn((
                                    Button,
                                    action_button_node(Vec2::new(150.0, 34.0)),
                                    BorderColor::all(UiColors::accent()),
                                    BackgroundColor(action_button_color(false, false)),
                                    TalentResetButton,
                                ))
                                .with_children(|button| {
                                    button.spawn((
                                        Text::new("RESET ALL"),
                                        TextFont {
                                            font_size: UiFontSize::BUTTON,
                                            ..default()
                                        },
                                        TextColor(UiColors::text_section()),
                                        TalentResetLabel,
                                        Label,
                                    ));
                                });
                        });
                });
        },
    );
}

fn spawn_talent_connector(
    parent: &mut ChildSpawnerCommands,
    node_index: usize,
    from: Vec2,
    to: Vec2,
) {
    let from = talent_canvas_position(from);
    let to = talent_canvas_position(to);
    let elbow_y = (from.y + to.y) * 0.5;

    spawn_talent_connector_segment(parent, node_index, from, Vec2::new(from.x, elbow_y));
    spawn_talent_connector_segment(
        parent,
        node_index,
        Vec2::new(from.x, elbow_y),
        Vec2::new(to.x, elbow_y),
    );
    spawn_talent_connector_segment(parent, node_index, Vec2::new(to.x, elbow_y), to);
}

fn spawn_talent_connector_segment(
    parent: &mut ChildSpawnerCommands,
    node_index: usize,
    from: Vec2,
    to: Vec2,
) {
    let thickness = TALENT_CONNECTOR_THICKNESS;
    let left = from.x.min(to.x) - thickness * 0.5;
    let top = from.y.min(to.y) - thickness * 0.5;
    let width = (from.x - to.x).abs().max(thickness);
    let height = (from.y - to.y).abs().max(thickness);

    parent.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: px(left),
            top: px(top),
            width: px(width),
            height: px(height),
            ..default()
        },
        BackgroundColor(Color::srgba(0.22, 0.22, 0.22, 0.92)),
        TalentConnector { node: node_index },
    ));
}

fn spawn_talent_node(parent: &mut ChildSpawnerCommands, index: usize, node: &TalentNode) {
    let center = talent_canvas_position(node.position);
    parent
        .spawn((
            Button,
            Node {
                position_type: PositionType::Absolute,
                left: px(center.x - TALENT_NODE_SIZE.x * 0.5),
                top: px(center.y - TALENT_NODE_SIZE.y * 0.5),
                width: px(TALENT_NODE_SIZE.x),
                height: px(TALENT_NODE_SIZE.y),
                padding: UiRect::all(px(4)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(px(2)),
                ..default()
            },
            BorderColor::all(Color::srgba(0.36, 0.31, 0.22, 0.78)),
            BackgroundColor(Color::srgb(0.20, 0.19, 0.18)),
            TalentNodeButton { index },
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(""),
                TextFont {
                    font_size: 8.5,
                    ..default()
                },
                TextColor(Color::srgb(0.85, 0.83, 0.76)),
                TextLayout::new_with_justify(Justify::Center),
                Node {
                    width: percent(100),
                    ..default()
                },
                TalentNodeLabel { index },
                Label,
            ));
        });
}

fn talent_canvas_position(position: Vec2) -> Vec2 {
    Vec2::new(
        position.x - TALENT_CANVAS_MIN_X,
        TALENT_CANVAS_MAX_Y - position.y,
    )
}

pub(crate) fn handle_talent_panel(
    mut ui_state: ResMut<UiState>,
    database: Res<GameDatabase>,
    mut profile: ResMut<PlayerProfile>,
    mouse: Res<ButtonInput<MouseButton>>,
    node_query: Query<(&TalentNodeButton, &Interaction), With<Button>>,
    reset_query: Query<(&TalentResetButton, &Interaction), With<Button>>,
) {
    if ui_state.active_panel != ActivePanel::Talents {
        ui_state.hovered_talent = None;
        return;
    }
    profile.ensure_talent_slots(&database);

    let hovered = node_query.iter().find_map(|(button, interaction)| {
        matches!(*interaction, Interaction::Hovered | Interaction::Pressed).then_some(button.index)
    });
    ui_state.hovered_talent = hovered;

    if let Some(index) = hovered {
        if mouse.just_pressed(MouseButton::Left) {
            profile.allocate_talent(&database, index);
        } else if mouse.just_pressed(MouseButton::Right) {
            profile.deallocate_talent(&database, index);
        }
        return;
    }

    if mouse.just_pressed(MouseButton::Left)
        && reset_query.iter().any(|(_button, interaction)| {
            matches!(*interaction, Interaction::Hovered | Interaction::Pressed)
        })
    {
        profile.reset_talents();
    }
}

pub(crate) fn sync_talent_panel(
    database: Res<GameDatabase>,
    profile: Res<PlayerProfile>,
    ui_state: Res<UiState>,
    mut piece_query: Query<&mut Visibility, With<TalentPanelPiece>>,
    mut node_query: Query<
        (
            &TalentNodeButton,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (
            With<Button>,
            Without<TalentConnector>,
            Without<TalentResetButton>,
        ),
    >,
    mut connector_query: Query<
        (&TalentConnector, &mut BackgroundColor),
        (Without<TalentNodeButton>, Without<TalentResetButton>),
    >,
    mut node_label_query: Query<
        (&TalentNodeLabel, &mut Text, &mut TextColor),
        (
            Without<TalentHeaderText>,
            Without<TalentInfoText>,
            Without<TalentResetLabel>,
        ),
    >,
    mut header_query: Query<
        &mut Text,
        (
            With<TalentHeaderText>,
            Without<TalentNodeLabel>,
            Without<TalentInfoText>,
            Without<TalentResetLabel>,
        ),
    >,
    mut info_query: Query<
        &mut Text,
        (
            With<TalentInfoText>,
            Without<TalentNodeLabel>,
            Without<TalentHeaderText>,
            Without<TalentResetLabel>,
        ),
    >,
    mut reset_button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            With<TalentResetButton>,
            With<Button>,
            Without<TalentNodeButton>,
            Without<TalentConnector>,
        ),
    >,
    mut reset_label_query: Query<
        &mut TextColor,
        (
            With<TalentResetLabel>,
            Without<TalentNodeLabel>,
            Without<TalentHeaderText>,
            Without<TalentInfoText>,
        ),
    >,
) {
    let is_visible = ui_state.active_panel == ActivePanel::Talents;
    for mut visibility in &mut piece_query {
        *visibility = if is_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    if !is_visible {
        return;
    }

    let tree = profile.talent_tree(&database);

    for (button, interaction, mut background, mut border) in &mut node_query {
        let index = button.index;
        let Some(node) = tree.get(index) else {
            continue;
        };
        let points = profile.talent_points_in(index);
        let hovered = matches!(*interaction, Interaction::Hovered | Interaction::Pressed);
        background.0 = talent_node_color(
            points,
            node.max_points,
            profile.can_allocate_talent(&database, index),
            hovered,
        );
        border.set_all(if hovered {
            UiColors::accent()
        } else if points > 0 {
            Color::srgba(0.94, 0.66, 0.22, 0.9)
        } else {
            Color::srgba(0.36, 0.31, 0.22, 0.78)
        });
    }

    for (connector, mut background) in &mut connector_query {
        background.0 = if profile.talent_points_in(connector.node) > 0 {
            Color::srgba(0.96, 0.58, 0.12, 0.95)
        } else {
            Color::srgba(0.22, 0.22, 0.22, 0.92)
        };
    }

    for (label, mut text, mut text_color) in &mut node_label_query {
        let index = label.index;
        if let Some(node) = tree.get(index) {
            let points = profile.talent_points_in(index);
            text.0 = format!(
                "{}\n{}/{}",
                truncate_chars(node.name, 13),
                points,
                node.max_points
            );
            text_color.0 = if points > 0 {
                UiColors::text_dark()
            } else {
                Color::srgb(0.85, 0.83, 0.76)
            };
        }
    }

    for mut text in &mut header_query {
        text.0 = format!(
            "Available {}\nSpent {}/{}",
            profile.available_talent_points(),
            profile.spent_talent_points(),
            profile.total_talent_points(),
        );
    }

    for mut text in &mut info_query {
        let info = talent_info_text(&profile, &database, ui_state.hovered_talent);
        text.0 = bounded_lines(info.lines().map(|line| line.to_string()), 30, 14);
    }

    let reset_enabled = profile.spent_talent_points() > 0;
    for (interaction, mut background) in &mut reset_button_query {
        let hovered = matches!(*interaction, Interaction::Hovered | Interaction::Pressed);
        background.0 = action_button_color(reset_enabled, hovered);
    }

    for mut text_color in &mut reset_label_query {
        text_color.0 = if reset_enabled {
            UiColors::text_section()
        } else {
            Color::srgba(0.64, 0.58, 0.48, 0.78)
        };
    }
}

fn talent_node_color(points: u8, max_points: u8, can_allocate: bool, hovered: bool) -> Color {
    if points >= max_points && hovered {
        Color::srgba(0.98, 0.64, 0.16, 0.98)
    } else if points >= max_points {
        Color::srgb(0.96, 0.58, 0.12)
    } else if points > 0 && hovered {
        Color::srgba(0.92, 0.72, 0.28, 0.98)
    } else if points > 0 {
        Color::srgb(0.82, 0.66, 0.24)
    } else if can_allocate && hovered {
        Color::srgba(0.42, 0.60, 0.66, 0.98)
    } else if can_allocate {
        Color::srgb(0.28, 0.46, 0.56)
    } else if hovered {
        Color::srgba(0.27, 0.25, 0.23, 0.98)
    } else {
        Color::srgb(0.20, 0.19, 0.18)
    }
}

fn talent_info_text(
    profile: &PlayerProfile,
    database: &GameDatabase,
    hovered: Option<usize>,
) -> String {
    let tree = profile.talent_tree(database);
    let Some(index) = hovered.filter(|index| *index < tree.len()) else {
        return "No talent selected.\n\nRanks can be reassigned freely at any time.\n\nUnlocked nodes brighten as their requirements are met."
            .to_string();
    };

    let node = &tree[index];
    let points = profile.talent_points_in(index);
    let mut lines = vec![
        node.name.to_string(),
        format!("Rank {}/{}", points, node.max_points),
    ];
    lines.push(String::new());
    lines.extend(wrap_talent_line(node.flavor));
    lines.push(String::new());
    lines.push(format!("Per rank: {}", node.grant.effect_line(1)));
    if points > 0 {
        lines.push(format!("Current:  {}", node.grant.effect_line(points)));
    }
    if let Some(req) = node.requires {
        let met = profile.talent_points_in(req) > 0;
        lines.push(String::new());
        lines.push(format!(
            "Requires: {}{}",
            tree[req].name,
            if met { " [met]" } else { " [locked]" }
        ));
    }
    lines.join("\n")
}

fn wrap_talent_line(line: &str) -> Vec<String> {
    const WRAP: usize = 28;
    let mut out = Vec::new();
    let mut current = String::new();
    for word in line.split_whitespace() {
        if current.is_empty() {
            current = word.to_string();
        } else if current.len() + 1 + word.len() <= WRAP {
            current.push(' ');
            current.push_str(word);
        } else {
            out.push(std::mem::take(&mut current));
            current = word.to_string();
        }
    }
    if !current.is_empty() {
        out.push(current);
    }
    out
}
