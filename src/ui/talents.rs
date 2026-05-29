use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::components::{
    ActivePanel, ScreenFixed, TalentConnector, TalentHeaderText, TalentInfoText, TalentNodeButton,
    TalentNodeLabel, TalentPanelPiece, TalentResetButton, TalentResetLabel, UiState,
};
use crate::data::{GameDatabase, PlayerProfile, TalentNode};

use super::cursor_offset;

const TALENT_NODE_SIZE: f32 = 34.0;

pub(super) fn spawn_talent_panel(commands: &mut Commands, talents: &[TalentNode]) {
    let center = Vec3::new(-150.0, 90.0, 44.0);
    spawn_talent_rect(
        commands,
        center,
        Vec2::new(724.0, 540.0),
        Color::srgba(0.02, 0.02, 0.02, 0.99),
    );
    spawn_talent_rect(
        commands,
        center + Vec3::new(0.0, 0.0, 1.0),
        Vec2::new(712.0, 528.0),
        Color::srgba(0.17, 0.16, 0.15, 0.97),
    );
    spawn_talent_rect(
        commands,
        center + Vec3::new(0.0, -22.0, 2.0),
        Vec2::new(696.0, 462.0),
        Color::srgba(0.07, 0.08, 0.10, 0.97),
    );
    spawn_talent_rect(
        commands,
        center + Vec3::new(0.0, 238.0, 3.0),
        Vec2::new(710.0, 40.0),
        Color::srgba(0.56, 0.10, 0.07, 0.98),
    );
    spawn_talent_rect(
        commands,
        center + Vec3::new(0.0, 215.0, 4.0),
        Vec2::new(710.0, 4.0),
        Color::srgba(0.98, 0.56, 0.12, 0.92),
    );
    spawn_talent_rect(
        commands,
        Vec3::new(8.0, 78.0, 47.0),
        Vec2::new(3.0, 430.0),
        Color::srgba(0.32, 0.30, 0.27, 0.9),
    );

    spawn_talent_label(
        commands,
        "TALENTS",
        Vec3::new(-490.0, 252.0, 48.0),
        22.0,
        Color::srgb(1.0, 0.72, 0.20),
    );
    spawn_talent_dynamic(
        commands,
        TalentHeaderText,
        Vec3::new(70.0, 250.0, 48.0),
        14.0,
        Color::srgb(1.0, 0.86, 0.45),
    );
    spawn_talent_dynamic(
        commands,
        TalentInfoText,
        Vec3::new(24.0, 150.0, 48.0),
        13.0,
        Color::srgb(0.90, 0.88, 0.80),
    );
    spawn_talent_reset_button(commands, Vec3::new(110.0, -120.0, 48.0));

    for (index, node) in talents.iter().enumerate() {
        if let Some(req) = node.requires {
            spawn_talent_connector(commands, index, talents[req].position, node.position);
        }
    }
    for (index, node) in talents.iter().enumerate() {
        spawn_talent_node(commands, index, node);
    }
}

fn spawn_talent_rect(commands: &mut Commands, offset: Vec3, size: Vec2, color: Color) {
    commands.spawn((
        Sprite::from_color(color, size),
        Transform::from_translation(offset),
        Visibility::Hidden,
        ScreenFixed { offset },
        TalentPanelPiece,
    ));
}

fn spawn_talent_label(
    commands: &mut Commands,
    label: &'static str,
    offset: Vec3,
    font_size: f32,
    color: Color,
) {
    commands.spawn((
        Text2d::new(label),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(color),
        TextLayout::new_with_justify(Justify::Left),
        Anchor::TOP_LEFT,
        Transform::from_translation(offset),
        Visibility::Hidden,
        ScreenFixed { offset },
        TalentPanelPiece,
    ));
}

fn spawn_talent_dynamic<C: Component>(
    commands: &mut Commands,
    marker: C,
    offset: Vec3,
    font_size: f32,
    color: Color,
) {
    commands.spawn((
        Text2d::new(""),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(color),
        TextLayout::new_with_justify(Justify::Left),
        Anchor::TOP_LEFT,
        Transform::from_translation(offset),
        Visibility::Hidden,
        ScreenFixed { offset },
        TalentPanelPiece,
        marker,
    ));
}

fn spawn_talent_connector(commands: &mut Commands, node_index: usize, from: Vec2, to: Vec2) {
    let mid = (from + to) * 0.5;
    let delta = to - from;
    let length = delta.length().max(1.0);
    let angle = delta.y.atan2(delta.x);
    let offset = Vec3::new(mid.x, mid.y, 45.0);
    commands.spawn((
        Sprite::from_color(Color::srgba(0.28, 0.26, 0.24, 0.9), Vec2::new(length, 4.0)),
        Transform::from_translation(offset).with_rotation(Quat::from_rotation_z(angle)),
        Visibility::Hidden,
        ScreenFixed { offset },
        TalentPanelPiece,
        TalentConnector { node: node_index },
    ));
}

fn spawn_talent_node(commands: &mut Commands, index: usize, node: &TalentNode) {
    let offset = Vec3::new(node.position.x, node.position.y, 46.0);
    commands.spawn((
        Sprite::from_color(Color::srgb(0.20, 0.19, 0.18), Vec2::splat(TALENT_NODE_SIZE)),
        Transform::from_translation(offset),
        Visibility::Hidden,
        ScreenFixed { offset },
        TalentPanelPiece,
        TalentNodeButton {
            index,
            size: Vec2::splat(TALENT_NODE_SIZE),
        },
    ));

    let label_offset = Vec3::new(node.position.x, node.position.y - 20.0, 48.0);
    commands.spawn((
        Text2d::new(""),
        TextFont {
            font_size: 8.5,
            ..default()
        },
        TextColor(Color::srgb(0.85, 0.83, 0.76)),
        TextLayout::new_with_justify(Justify::Center),
        Anchor::TOP_CENTER,
        Transform::from_translation(label_offset),
        Visibility::Hidden,
        ScreenFixed {
            offset: label_offset,
        },
        TalentPanelPiece,
        TalentNodeLabel { index },
    ));
}

fn spawn_talent_reset_button(commands: &mut Commands, offset: Vec3) {
    let size = Vec2::new(150.0, 32.0);
    commands.spawn((
        Sprite::from_color(Color::srgba(0.30, 0.11, 0.04, 0.98), size),
        Transform::from_translation(offset),
        Visibility::Hidden,
        ScreenFixed { offset },
        TalentPanelPiece,
        TalentResetButton { size },
    ));

    let text_offset = offset + Vec3::new(0.0, 6.0, 1.0);
    commands.spawn((
        Text2d::new("RESET ALL"),
        TextFont {
            font_size: 13.0,
            ..default()
        },
        TextColor(Color::srgb(0.96, 0.70, 0.32)),
        TextLayout::new_with_justify(Justify::Center),
        Anchor::CENTER,
        Transform::from_translation(text_offset),
        Visibility::Hidden,
        ScreenFixed {
            offset: text_offset,
        },
        TalentPanelPiece,
        TalentResetLabel,
    ));
}

pub(crate) fn handle_talent_panel(
    mut ui_state: ResMut<UiState>,
    database: Res<GameDatabase>,
    mut profile: ResMut<PlayerProfile>,
    mouse: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window>,
    node_query: Query<(&TalentNodeButton, &ScreenFixed)>,
    reset_query: Query<(&TalentResetButton, &ScreenFixed)>,
) {
    if ui_state.active_panel != ActivePanel::Talents {
        ui_state.hovered_talent = None;
        return;
    }
    profile.ensure_talent_slots(&database);

    let Some(cursor) = cursor_offset(&window_query) else {
        ui_state.hovered_talent = None;
        return;
    };

    let hovered = node_query.iter().find_map(|(button, fixed)| {
        let half = button.size * 0.5;
        if (cursor.x - fixed.offset.x).abs() <= half.x
            && (cursor.y - fixed.offset.y).abs() <= half.y
        {
            Some(button.index)
        } else {
            None
        }
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

    if mouse.just_pressed(MouseButton::Left) {
        let on_reset = reset_query.iter().any(|(button, fixed)| {
            let half = button.size * 0.5;
            (cursor.x - fixed.offset.x).abs() <= half.x
                && (cursor.y - fixed.offset.y).abs() <= half.y
        });
        if on_reset {
            profile.reset_talents();
        }
    }
}

pub(crate) fn sync_talent_panel(
    database: Res<GameDatabase>,
    profile: Res<PlayerProfile>,
    ui_state: Res<UiState>,
    mut piece_query: Query<&mut Visibility, With<TalentPanelPiece>>,
    mut node_query: Query<(&TalentNodeButton, &mut Sprite), Without<TalentConnector>>,
    mut connector_query: Query<(&TalentConnector, &mut Sprite), Without<TalentNodeButton>>,
    mut node_label_query: Query<
        (&TalentNodeLabel, &mut Text2d),
        (Without<TalentHeaderText>, Without<TalentInfoText>),
    >,
    mut header_query: Query<
        &mut Text2d,
        (
            With<TalentHeaderText>,
            Without<TalentNodeLabel>,
            Without<TalentInfoText>,
        ),
    >,
    mut info_query: Query<
        &mut Text2d,
        (
            With<TalentInfoText>,
            Without<TalentNodeLabel>,
            Without<TalentHeaderText>,
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

    for (button, mut sprite) in &mut node_query {
        let index = button.index;
        let Some(node) = tree.get(index) else {
            continue;
        };
        let points = profile.talent_points_in(index);
        sprite.color = if points >= node.max_points {
            Color::srgb(0.96, 0.58, 0.12)
        } else if points > 0 {
            Color::srgb(0.82, 0.66, 0.24)
        } else if profile.can_allocate_talent(&database, index) {
            Color::srgb(0.28, 0.46, 0.56)
        } else {
            Color::srgb(0.20, 0.19, 0.18)
        };
    }

    for (connector, mut sprite) in &mut connector_query {
        sprite.color = if profile.talent_points_in(connector.node) > 0 {
            Color::srgba(0.96, 0.58, 0.12, 0.95)
        } else {
            Color::srgba(0.28, 0.26, 0.24, 0.9)
        };
    }

    for (label, mut text) in &mut node_label_query {
        let index = label.index;
        if let Some(node) = tree.get(index) {
            text.0 = format!(
                "{}\n{}/{}",
                node.name,
                profile.talent_points_in(index),
                node.max_points
            );
        }
    }

    for mut text in &mut header_query {
        text.0 = format!(
            "Points {}   Spent {}/{}",
            profile.available_talent_points(),
            profile.spent_talent_points(),
            profile.total_talent_points(),
        );
    }

    for mut text in &mut info_query {
        text.0 = talent_info_text(&profile, &database, ui_state.hovered_talent);
    }
}

fn talent_info_text(
    profile: &PlayerProfile,
    database: &GameDatabase,
    hovered: Option<usize>,
) -> String {
    let tree = profile.talent_tree(database);
    let Some(index) = hovered.filter(|index| *index < tree.len()) else {
        return "Hover a talent to\ninspect it.\n\nLeft Click  allocate\nRight Click  refund\n\nReassign freely,\nno cost, anytime."
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
            "Needs: {}{}",
            tree[req].name,
            if met { " [met]" } else { " [locked]" }
        ));
    }
    lines.join("\n")
}

fn wrap_talent_line(line: &str) -> Vec<String> {
    const WRAP: usize = 26;
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
