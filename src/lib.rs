use image::ImageFormat;
use js_sys::{Boolean, Number, Uint8Array};
use std::panic;
use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub fn resize(
    width: Number,
    height: Number,
    exact: Boolean,
    bytes: Uint8Array,
    extension: Option<String>,
    filter_type: FilterType,
) -> Result<Vec<u8>, String> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    // let fmt: ImageFormat;
    let mut reader = image::io::Reader::new(std::io::Cursor::new(bytes.to_vec()));
    if let Some(e) = extension {
        if let Some(f) = ImageFormat::from_extension(&e) {
            // fmt = f;
            reader.set_format(f);
        } else {
            return Err("format for specified extension `{}` not available".to_string());
        }
    } else {
        match reader.with_guessed_format() {
            Ok(r) => {
                reader = r;
                // fmt = r.format();
            }
            Err(e) => return Err(e.to_string()),
        }
    }
    match reader.decode() {
        Ok(i) => {
            if exact.value_of() {
                i.resize_exact(
                    width.value_of() as u32,
                    height.value_of() as u32,
                    match_filter_type(filter_type),
                );
            } else {
                i.resize(
                    width.value_of() as u32,
                    height.value_of() as u32,
                    match_filter_type(filter_type),
                );
            };
            Ok(i.into_bytes())
        }
        Err(e) => {
            let e = e.to_string();
            log!("reader.decode() error: {}", e);
            Err(e)
        }
    }
}

#[wasm_bindgen]
pub enum FilterType {
    Nearest,
    Triangle,
    CatmullRom,
    Gaussian,
    Lanczos3,
}

#[wasm_bindgen]
pub fn get_filter_type(f: &str) -> Result<FilterType, String> {
    match f {
        "Nearest" => Ok(FilterType::Nearest),
        "Triangle" => Ok(FilterType::Triangle),
        "CatmullRom" => Ok(FilterType::CatmullRom),
        "Gaussian" => Ok(FilterType::Gaussian),
        "Lanczos3" => Ok(FilterType::Lanczos3),
        _ => Err("unavailable filter specified".to_string())
    }
}

pub fn match_filter_type(f: FilterType) -> image::imageops::FilterType {
    match f {
        FilterType::Nearest => image::imageops::FilterType::Nearest,
        FilterType::Triangle => image::imageops::FilterType::Triangle,
        FilterType::CatmullRom => image::imageops::FilterType::CatmullRom,
        FilterType::Gaussian => image::imageops::FilterType::Gaussian,
        FilterType::Lanczos3 => image::imageops::FilterType::Lanczos3,
    }
}
