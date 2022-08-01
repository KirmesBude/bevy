pub use bevy_rendertype::image::*;
pub use bevy_rendertype::image_texture_conversion::*;

use crate::{
    render_asset::{PrepareAssetError, RenderAsset},
    render_resource::{Sampler, Texture, TextureView},
    renderer::{RenderDevice, RenderQueue},
};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::system::{lifetimeless::SRes, SystemParamItem};
use bevy_math::Vec2;
use wgpu::{
    ImageCopyTexture, ImageDataLayout, Origin3d, TextureFormat,
    TextureViewDescriptor,
};

/// A rendering resource for the default image sampler which is set during renderer
/// initialization.
///
/// The [`ImageSettings`] resource can be set during app initialization to change the default
/// image sampler.
#[derive(Debug, Clone, Deref, DerefMut)]
pub struct DefaultImageSampler(pub(crate) Sampler);

/// The GPU-representation of an [`Image`].
/// Consists of the [`Texture`], its [`TextureView`] and the corresponding [`Sampler`], and the texture's size.
#[derive(Debug, Clone)]
pub struct GpuImage {
    pub texture: Texture,
    pub texture_view: TextureView,
    pub texture_format: TextureFormat,
    pub sampler: Sampler,
    pub size: Vec2,
}

impl RenderAsset for Image {
    type ExtractedAsset = Image;
    type PreparedAsset = GpuImage;
    type Param = (
        SRes<RenderDevice>,
        SRes<RenderQueue>,
        SRes<DefaultImageSampler>,
    );

    /// Clones the Image.
    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    /// Converts the extracted image into a [`GpuImage`].
    fn prepare_asset(
        image: Self::ExtractedAsset,
        (render_device, render_queue, default_sampler): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let texture = if image.texture_descriptor.mip_level_count > 1 || image.is_compressed() {
            render_device.create_texture_with_data(
                render_queue,
                &image.texture_descriptor,
                &image.data,
            )
        } else {
            let texture = render_device.create_texture(&image.texture_descriptor);
            let format_size = image.texture_descriptor.format.pixel_size();
            render_queue.write_texture(
                ImageCopyTexture {
                    texture: &texture,
                    mip_level: 0,
                    origin: Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &image.data,
                ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(
                        std::num::NonZeroU32::new(
                            image.texture_descriptor.size.width * format_size as u32,
                        )
                        .unwrap(),
                    ),
                    rows_per_image: if image.texture_descriptor.size.depth_or_array_layers > 1 {
                        std::num::NonZeroU32::new(image.texture_descriptor.size.height)
                    } else {
                        None
                    },
                },
                image.texture_descriptor.size,
            );
            texture
        };

        let texture_view = texture.create_view(
            image
                .texture_view_descriptor
                .or_else(|| Some(TextureViewDescriptor::default()))
                .as_ref()
                .unwrap(),
        );
        let size = Vec2::new(
            image.texture_descriptor.size.width as f32,
            image.texture_descriptor.size.height as f32,
        );
        let sampler = match image.sampler_descriptor {
            ImageSampler::Default => (***default_sampler).clone(),
            ImageSampler::Descriptor(descriptor) => render_device.create_sampler(&descriptor),
        };

        Ok(GpuImage {
            texture,
            texture_view,
            texture_format: image.texture_descriptor.format,
            sampler,
            size,
        })
    }
}
