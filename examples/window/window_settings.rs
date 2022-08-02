//! Illustrates how to change window settings and shows how to affect
//! the mouse pointer in various ways.

use bevy::{
    prelude::*,
    render::texture::{CompressedImageFormats, ImageType},
    window::PresentMode,
};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: 500.,
            height: 300.,
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(change_title)
        .add_system(toggle_cursor)
        .add_system(cycle_cursor_icon)
        .add_system(cycle_window_icon)
        .run();
}

/// This system will then change the title during execution
fn change_title(time: Res<Time>, mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    window.set_title(format!(
        "Seconds since startup: {}",
        time.seconds_since_startup().round()
    ));
}

/// This system toggles the cursor's visibility when the space bar is pressed
fn toggle_cursor(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    if input.just_pressed(KeyCode::Space) {
        window.set_cursor_lock_mode(!window.cursor_locked());
        window.set_cursor_visibility(!window.cursor_visible());
    }
}

/// This system cycles the cursor's icon through a small set of icons when clicking
fn cycle_cursor_icon(
    input: Res<Input<MouseButton>>,
    mut windows: ResMut<Windows>,
    mut index: Local<usize>,
) {
    const ICONS: &[CursorIcon] = &[
        CursorIcon::Default,
        CursorIcon::Hand,
        CursorIcon::Wait,
        CursorIcon::Text,
        CursorIcon::Copy,
    ];
    let window = windows.primary_mut();
    if input.just_pressed(MouseButton::Left) {
        *index = (*index + 1) % ICONS.len();
        window.set_cursor_icon(ICONS[*index]);
    } else if input.just_pressed(MouseButton::Right) {
        *index = if *index == 0 {
            ICONS.len() - 1
        } else {
            *index - 1
        };
        window.set_cursor_icon(ICONS[*index]);
    }
}

struct IconHandleBevy(Handle<Image>);
struct IconHandleWrench(Handle<Image>);

fn setup(
    asset_server: ResMut<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut windows: ResMut<Windows>,
    mut commands: Commands,
) {
    let image_bevy_bytes = include_bytes!("../../assets/branding/icon.png");
    let image_bevy = Image::from_buffer(
        image_bevy_bytes,
        ImageType::MimeType("image/png"),
        CompressedImageFormats::default(),
        true,
    )
    .unwrap();
    let icon_handle_bevy = images.add(image_bevy);

    let icon_handle_wrench = asset_server.load("textures/Game Icons/wrench.png");

    commands.insert_resource(IconHandleBevy(icon_handle_bevy.clone()));
    commands.insert_resource(IconHandleWrench(icon_handle_wrench));

    let window = windows.get_primary_mut().unwrap();
    window.set_icon(Some(icon_handle_bevy));
}

fn cycle_window_icon(
    input: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
    icon_handle_bevy: Res<IconHandleBevy>,
    icon_handle_wrench: Res<IconHandleWrench>,
) {
    let icon_handle_bevy = &icon_handle_bevy.0;
    let icon_handle_wrench = &icon_handle_wrench.0;

    let window = windows.get_primary_mut().unwrap();

    if input.just_pressed(KeyCode::I) {
        if Some(icon_handle_bevy) == window.icon() {
            window.set_icon(Some(icon_handle_wrench.clone_weak()));
        } else {
            window.set_icon(Some(icon_handle_bevy.clone_weak()));
        }
    }
}
