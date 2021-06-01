use std::path::PathBuf;

use bevy::{
    prelude::*,
    window::{WindowIcon, WindowIconBytes},
};

/// This example illustrates how to customize the default window settings
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: 500.,
            height: 300.,
            vsync: true,
            icon_path: Some(PathBuf::from("android-res/mipmap-mdpi/ic_launcher.png")),
            ..Default::default()
        })
        .insert_resource(IconResource::default())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(change_title.system())
        .add_system(toggle_cursor.system())
        .add_system(toggle_icon.system())
        .run();
}

#[derive(Debug, Clone, Default)]
struct IconResource {
    handle: Handle<Texture>,
}

fn setup(asset_server: Res<AssetServer>, mut icon_resource: ResMut<IconResource>) {
    let icon = asset_server.load("android-res/mipmap-mdpi/ic_launcher.png");
    icon_resource.handle = icon;
}

/// This system will then change the title during execution
fn change_title(time: Res<Time>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_title(format!(
        "Seconds since startup: {}",
        time.seconds_since_startup().round()
    ));
}

/// This system toggles the cursor's visibility when the space bar is pressed
fn toggle_cursor(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    if input.just_pressed(KeyCode::Space) {
        window.set_cursor_lock_mode(!window.cursor_locked());
        window.set_cursor_visibility(!window.cursor_visible());
    }
}

fn toggle_icon(
    input: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
    icon_resource: Res<IconResource>,
    textures: Res<Assets<Texture>>,
) {
    let window = windows.get_primary_mut().unwrap();
    if input.just_pressed(KeyCode::I) {
        match window.icon() {
            None => {
                if let Some(texture) = textures.get(&icon_resource.handle) {
                    let window_icon = WindowIcon::from(WindowIconBytes {
                        bytes: texture.data.clone(),
                        width: texture.size.width,
                        height: texture.size.height,
                    });

                    window.set_icon(window_icon);
                }
            }
            _ => window.clear_icon(),
        }
    }
}
