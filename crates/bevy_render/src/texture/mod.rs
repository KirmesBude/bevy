mod fallback_image;
#[cfg(feature = "hdr")]
mod hdr_texture_loader;
#[allow(clippy::module_inception)]
mod image;
mod image_texture_loader;
mod texture_cache;

pub use self::image::*;
#[cfg(feature = "hdr")]
pub use hdr_texture_loader::*;

pub use fallback_image::*;
pub use image_texture_loader::*;
pub use texture_cache::*;

use crate::{
    render_asset::{PrepareAssetLabel, RenderAssetPlugin},
    renderer::RenderDevice,
    RenderApp, RenderStage,
};
use bevy_app::{App, Plugin};
use bevy_asset::{AddAsset, Assets};

// TODO: replace Texture names with Image names?
/// Adds the [`Image`] as an asset and makes sure that they are extracted and prepared for the GPU.
pub struct ImagePlugin;

impl Plugin for ImagePlugin {
    fn build(&self, app: &mut App) {
        #[cfg(any(
            feature = "png",
            feature = "dds",
            feature = "tga",
            feature = "jpeg",
            feature = "bmp",
            feature = "basis-universal",
            feature = "ktx2",
        ))]
        {
            app.init_asset_loader::<ImageTextureLoader>();
        }

        #[cfg(feature = "hdr")]
        {
            app.init_asset_loader::<HdrTextureLoader>();
        }

        app.add_plugin(RenderAssetPlugin::<Image>::with_prepare_asset_label(
            PrepareAssetLabel::PreAssetPrepare,
        ))
        .add_asset::<Image>();
        app.world
            .resource_mut::<Assets<Image>>()
            .set_untracked(DEFAULT_IMAGE_HANDLE, Image::default());

        let default_sampler = app
            .world
            .get_resource_or_insert_with(ImageSettings::default)
            .default_sampler
            .clone();
        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            let default_sampler = {
                let device = render_app.world.resource::<RenderDevice>();
                device.create_sampler(&default_sampler)
            };
            render_app
                .insert_resource(DefaultImageSampler(default_sampler))
                .init_resource::<TextureCache>()
                .init_resource::<FallbackImage>()
                .add_system_to_stage(RenderStage::Cleanup, update_texture_cache_system);
        }
    }
}
