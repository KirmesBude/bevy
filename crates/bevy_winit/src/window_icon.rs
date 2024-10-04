//! Components to customize winit window icon

use bevy_app::{App, Last, Plugin};
use bevy_asset::Handle;
use bevy_ecs::{
    component::Component, observer::Trigger, reflect::ReflectComponent, world::OnRemove,
};
use bevy_reflect::{std_traits::ReflectDefault, Reflect};
use bevy_render::texture::Image;

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
pub struct WindowIcon(pub Handle<Image>);

fn update_icons() {}

fn on_remove_window_icon(_trigger: Trigger<OnRemove, WindowIcon>) {}
