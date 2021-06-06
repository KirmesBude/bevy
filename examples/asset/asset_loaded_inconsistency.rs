use std::collections::hash_map::Entry;

use bevy::{asset::LoadState, prelude::*, utils::HashMap, window::WindowId};

/// This example illustrates various ways to load assets
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_system_to_stage(
            CoreStage::PostUpdate,
            inconsistency_map.system())
        .add_system_to_stage(
            CoreStage::PostUpdate,
            inconsistency_local.system()
        )
        .run();
}

fn inconsistency_map(
    input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>,
    textures: Res<Assets<Texture>>,
    mut map: Local<HashMap<WindowId, Handle<Texture>>>,
) {
    let window= windows.get_primary().unwrap();
    match map.entry(window.id()) {
        Entry::Vacant(v) => {
            if input.just_pressed(KeyCode::I) {
                /* Load the texture */
                v.insert(asset_server.load("android-res/mipmap-mdpi/ic_launcher.png"));
            }
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

fn inconsistency_local(
    input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    textures: Res<Assets<Texture>>,
    mut local_handle: Local<Option<Handle<Texture>>>,
) {
    match &*local_handle {
        None => {
            if input.just_pressed(KeyCode::W) {
                /* Load the texture */
                *local_handle = Some(asset_server.load("android-res/mipmap-mdpi/ic_launcher.png"));
            }
        }
        Some(handle) => {
            /* Poll load state */
            match asset_server.get_load_state(handle) {
                LoadState::Loaded => {
                    let texture = textures.get(handle).unwrap();
                    info!("{:?}", texture);
                    
                    *local_handle = None;
                }
                LoadState::Failed => {
                    *local_handle = None;
                }
                _ => {}
            }
        }
    }
}