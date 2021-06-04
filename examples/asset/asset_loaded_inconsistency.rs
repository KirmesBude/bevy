use std::collections::hash_map::Entry;

use bevy::{asset::LoadState, prelude::*, utils::HashMap, window::WindowId};

/// This example illustrates various ways to load assets
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_system_to_stage(
            CoreStage::PostUpdate,
            inconsistency.system())
        .run();
}

fn inconsistency(
    windows: Res<Windows>,
    asset_server: Res<AssetServer>,
    textures: Res<Assets<Texture>>,
    mut map: Local<HashMap<WindowId, Handle<Texture>>>,
) {
    let window= windows.get_primary().unwrap();
    match map.entry(window.id()) {
        Entry::Vacant(v) => {
            /* Load the texture */
            v.insert(asset_server.load("android-res/mipmap-mdpi/ic_launcher.png"));
        }
        Entry::Occupied(o) => {
            /* Poll load state */
            let handle = o.get();
            match asset_server.get_load_state(handle) {
                LoadState::Loaded => {
                    let texture = textures.get(handle).unwrap();
                    info!("{:?}", texture);
                    o.remove();
                }
                LoadState::Failed => {
                    o.remove();
                }
                _ => {}
            }
        }
    }
}
