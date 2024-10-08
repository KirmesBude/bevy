//! Components to customize winit window icon

use bevy_app::{App, Last, Plugin};
use bevy_asset::{Assets, Handle, RenderAssetUsages};
use bevy_ecs::{
    change_detection::DetectChanges,
    component::Component,
    entity::Entity,
    observer::Trigger,
    query::With,
    reflect::ReflectComponent,
    system::{Commands, Local, Query, Res},
    world::{OnRemove, Ref},
};
use bevy_image::Image;
use bevy_math::{URect, UVec2};
use bevy_reflect::{std_traits::ReflectDefault, Reflect};
use bevy_utils::{tracing::warn, HashSet};
use bevy_window::Window;
use wgpu_types::TextureFormat;
use winit::window::Icon;

use crate::state::PendingWindowIcon;

pub(crate) struct WindowIconPlugin;

impl Plugin for WindowIconPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WindowIcon>()
            .add_systems(Last, update_icons);

        app.observe(on_remove_window_icon);
    }
}

/// Insert into a window entity to set the icon for that window.
#[derive(Component, Debug, Default, Clone, Reflect, PartialEq, Eq)]
#[reflect(Component, Debug, Default, PartialEq)]
pub struct WindowIcon(pub Option<Handle<Image>>);

fn update_icons(
    mut commands: Commands,
    windows: Query<(Entity, Ref<WindowIcon>), With<Window>>,
    images: Res<Assets<Image>>,
    mut queue: Local<HashSet<Entity>>,
) {
    for (entity, icon) in windows.iter() {
        if !(queue.remove(&entity) || icon.is_changed()) {
            continue;
        }

        let icon = match icon.0.as_ref() {
            Some(handle) => {
                let Some(image) = images.get(handle) else {
                    warn!(
                        "Window icon image {handle:?} is not loaded yet and couldn't be used. Trying again next frame."
                    );
                    queue.insert(entity);
                    continue;
                };

                let width = image.texture_descriptor.size.width;
                let height = image.texture_descriptor.size.height;
                let rect = URect::from_corners(UVec2::ZERO, UVec2::new(width, height - 1));
                let mut rgba = Image::new_fill(
                    image.texture_descriptor.size,
                    image.texture_descriptor.dimension,
                    &[0, 0, 0, 0],
                    TextureFormat::Rgba8UnormSrgb,
                    RenderAssetUsages::MAIN_WORLD,
                );
                let rgba = match rgba
                    .rect_bytes_mut(rect, 1)
                    .unwrap()
                    .translate_from(image.rect_bytes(rect, 1).unwrap())
                {
                    Ok(_) => rgba.data,
                    Err(err) => {
                        warn!("Window icon image {handle:?} conversion failed: {err}");
                        continue;
                    }
                };

                let icon = match Icon::from_rgba(rgba, width, height) {
                    Ok(icon) => icon,
                    Err(err) => {
                        warn!("Window icon image {handle:?} is invalid: {err}");
                        continue;
                    }
                };

                Some(icon)
            }
            None => None,
        };

        commands.entity(entity).insert(PendingWindowIcon(icon));
    }
}

fn on_remove_window_icon(_trigger: Trigger<OnRemove, WindowIcon>) {}
