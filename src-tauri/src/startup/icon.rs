use base64::Engine;
use std::path::Path;

#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{
    DestroyIcon, GetIconInfo, ICONINFO,
};
#[cfg(windows)]
use windows::Win32::Graphics::Gdi::{
    CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits, GetObjectW,
    BITMAP, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
};
#[cfg(windows)]
use windows::Win32::UI::Shell::ExtractIconExW;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;

#[cfg(windows)]
pub fn extract_icon_base64(exe_path: &str) -> Option<String> {
    use std::ffi::OsStr;

    let path = Path::new(exe_path);
    if !path.exists() {
        return None;
    }

    unsafe {
        let wide_path: Vec<u16> = OsStr::new(exe_path)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let mut large_icon = [windows::Win32::UI::WindowsAndMessaging::HICON::default()];
        let mut small_icon = [windows::Win32::UI::WindowsAndMessaging::HICON::default()];

        let count = ExtractIconExW(
            windows::core::PCWSTR(wide_path.as_ptr()),
            0,
            Some(large_icon.as_mut_ptr()),
            Some(small_icon.as_mut_ptr()),
            1,
        );

        if count == 0 {
            return None;
        }

        let icon = if !large_icon[0].is_invalid() {
            large_icon[0]
        } else if !small_icon[0].is_invalid() {
            small_icon[0]
        } else {
            return None;
        };

        let result = icon_to_base64_png(icon);

        // Cleanup
        if !large_icon[0].is_invalid() {
            let _ = DestroyIcon(large_icon[0]);
        }
        if !small_icon[0].is_invalid() {
            let _ = DestroyIcon(small_icon[0]);
        }

        result
    }
}

#[cfg(windows)]
unsafe fn icon_to_base64_png(icon: windows::Win32::UI::WindowsAndMessaging::HICON) -> Option<String> {
    let mut icon_info = ICONINFO::default();
    unsafe {
        if GetIconInfo(icon, &mut icon_info).is_err() {
            return None;
        }
    }

    // Get actual bitmap size
    let mut bmp = BITMAP::default();
    let bmp_handle = if !icon_info.hbmColor.is_invalid() {
        icon_info.hbmColor
    } else {
        icon_info.hbmMask
    };

    unsafe {
        GetObjectW(bmp_handle, std::mem::size_of::<BITMAP>() as i32, Some(&mut bmp as *mut _ as *mut _));
    }
    let width = bmp.bmWidth as u32;
    let height = bmp.bmHeight.unsigned_abs();

    let hdc = unsafe {
        CreateCompatibleDC(None)
    };
    if hdc.is_invalid() {
        unsafe {
            if !icon_info.hbmColor.is_invalid() { let _ = DeleteObject(icon_info.hbmColor); }
            if !icon_info.hbmMask.is_invalid() { let _ = DeleteObject(icon_info.hbmMask); }
        }
        return None;
    }

    let mut bmp_info = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width as i32,
            biHeight: -(height as i32),
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0,
            ..Default::default()
        },
        bmiColors: [Default::default()],
    };

    let mut pixels: Vec<u8> = vec![0u8; (width * height * 4) as usize];

    if !icon_info.hbmColor.is_invalid() {
        unsafe {
            GetDIBits(hdc, icon_info.hbmColor, 0, height, Some(pixels.as_mut_ptr() as *mut _), &mut bmp_info, DIB_RGB_COLORS);
        }
    }

    let has_alpha = pixels.chunks_exact(4).any(|c| c[3] != 0);
    if !has_alpha && !icon_info.hbmMask.is_invalid() {
        let mut mask_pixels: Vec<u8> = vec![0u8; (width * height * 4) as usize];
        unsafe {
            GetDIBits(hdc, icon_info.hbmMask, 0, height, Some(mask_pixels.as_mut_ptr() as *mut _), &mut bmp_info, DIB_RGB_COLORS);
        }
        for (pixel, mask) in pixels.chunks_exact_mut(4).zip(mask_pixels.chunks_exact(4)) {
            pixel[3] = if mask[0] == 0 { 255 } else { 0 };
        }
    }

    unsafe {
        let _ = DeleteDC(hdc);
        if !icon_info.hbmColor.is_invalid() { let _ = DeleteObject(icon_info.hbmColor); }
        if !icon_info.hbmMask.is_invalid() { let _ = DeleteObject(icon_info.hbmMask); }
    }

    // Convert BGRA to RGBA
    for chunk in pixels.chunks_exact_mut(4) {
        chunk.swap(0, 2);
    }

    let img = image::RgbaImage::from_raw(width, height, pixels)?;
    let mut png_data = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut png_data), image::ImageFormat::Png).ok()?;

    Some(format!("data:image/png;base64,{}", base64::engine::general_purpose::STANDARD.encode(&png_data)))
}

#[cfg(not(windows))]
pub fn extract_icon_base64(_exe_path: &str) -> Option<String> {
    None
}
