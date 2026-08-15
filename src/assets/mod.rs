use arcdps::d3d11_device;
use include_img::include_img;
use log::error;
use once_cell::sync::Lazy;
use windows::Win32::Graphics::{
    Direct3D::D3D11_SRV_DIMENSION_TEXTURE2D,
    Direct3D11::{
        ID3D11Device, ID3D11ShaderResourceView, ID3D11Texture2D, D3D11_BIND_SHADER_RESOURCE,
        D3D11_SHADER_RESOURCE_VIEW_DESC, D3D11_SHADER_RESOURCE_VIEW_DESC_0, D3D11_SUBRESOURCE_DATA,
        D3D11_TEX2D_SRV, D3D11_TEXTURE2D_DESC, D3D11_USAGE, D3D11_USAGE_IMMUTABLE,
    },
    Dxgi::Common::{DXGI_FORMAT, DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_SAMPLE_DESC},
};

/// Icon type, backed by a shader resource view.
///
/// Ported locally from `arc_util::ui::texture`/`render` (rather than depended on) because
/// `arc_util`'s fork still targets an older `windows` crate version than `arcdps`'s fork, whose
/// `d3d11_device()` this relies on.
pub type Icon = ID3D11ShaderResourceView;

/// Creates a 2-dimensional texture with the given description and subresource data.
fn create_texture2d(
    device: &ID3D11Device,
    desc: &D3D11_TEXTURE2D_DESC,
    data: &D3D11_SUBRESOURCE_DATA,
) -> windows::core::Result<ID3D11Texture2D> {
    let mut id = None;
    unsafe { device.CreateTexture2D(desc, Some(data), Some(&mut id)) }?;
    Ok(id.expect("CreateTexture2D failed without error"))
}

/// Creates a 2-dimensional texture from in-memory data.
fn create_texture2d_from_mem(
    device: &ID3D11Device,
    data: &[u8],
    width: u32,
    height: u32,
    pitch: u32,
    format: DXGI_FORMAT,
    usage: D3D11_USAGE,
) -> windows::core::Result<ID3D11Texture2D> {
    let desc = D3D11_TEXTURE2D_DESC {
        Width: width,
        Height: height,
        MipLevels: 1,
        ArraySize: 1,
        Format: format,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        Usage: usage,
        BindFlags: D3D11_BIND_SHADER_RESOURCE.0 as _,
        ..Default::default()
    };
    let sub_resource = D3D11_SUBRESOURCE_DATA {
        pSysMem: data.as_ptr() as _,
        SysMemPitch: pitch,
        SysMemSlicePitch: 0,
    };
    create_texture2d(device, &desc, &sub_resource)
}

/// Creates a shader resource view for the given 2-dimensional texture.
fn create_texture2d_view(
    device: &ID3D11Device,
    texture: &ID3D11Texture2D,
    format: DXGI_FORMAT,
) -> windows::core::Result<ID3D11ShaderResourceView> {
    let mut id = None;
    let desc = D3D11_SHADER_RESOURCE_VIEW_DESC {
        Format: format,
        ViewDimension: D3D11_SRV_DIMENSION_TEXTURE2D,
        Anonymous: D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
            Texture2D: D3D11_TEX2D_SRV {
                MostDetailedMip: 0,
                MipLevels: 1,
            },
        },
    };
    unsafe { device.CreateShaderResourceView(texture, Some(&desc), Some(&mut id)) }?;
    Ok(id.expect("CreateShaderResourceView failed without error"))
}

fn init_icon(data: &'static [u8]) -> Option<Icon> {
    let device = d3d11_device()?;
    let format = DXGI_FORMAT_R8G8B8A8_UNORM;
    let texture =
        create_texture2d_from_mem(&device, data, 32, 32, 32 * 4, format, D3D11_USAGE_IMMUTABLE)
            .map_err(|err| error!("failed to create texture: {err}"))
            .ok()?;
    create_texture2d_view(&device, &texture, format)
        .map_err(|err| error!("failed to create texture view: {err}"))
        .ok()
}

pub static FOOD_ICON: Lazy<Option<Icon>> =
    Lazy::new(|| init_icon(&include_img!("./src/assets/food.png", rgba8)));

pub static UTIL_ICON: Lazy<Option<Icon>> =
    Lazy::new(|| init_icon(&include_img!("./src/assets/util.png", rgba8)));

pub static UNKNOWN_ICON: Lazy<Option<Icon>> =
    Lazy::new(|| init_icon(&include_img!("./src/assets/unknown.png", rgba8)));
