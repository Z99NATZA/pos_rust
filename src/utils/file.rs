use std::path::Path;
use crate::app::{error::AppError, result::AppResult};

pub fn ensure_valid_ext(filename: &str) -> AppResult<String> {
    let ext = Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_ascii_lowercase())
        .ok_or_else(|| AppError::BadRequestCustom(
            format!("นามกุลไฟล์ไม่ถูกต้อง").into()
        ))?;
    
    Ok(ext)
}

pub fn validate_image_ext(ext: &str) -> AppResult<bool> {
    let allowed = ["jpg", "jpeg", "png", "webp"];

    if !allowed.contains(&ext) {
        return Err(AppError::BadRequestCustom(
            format!("ไฟล์ภาพ (jpg, jpeg, png, web) เท่านั้น")
        ));
    }

    Ok(true)
}
