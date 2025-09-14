use std::fs;
use std::path::Path;

pub fn create_dir() {
    let path = Path::new("images/products");

    // ถ้ายังไม่มีโฟลเดอร์ สร้าง 'images/products'
    if !path.exists() {
        if let Err(e) = fs::create_dir_all(&path) {
            eprintln!("สร้างโฟลเดอร์ไม่สำเร็จ 'images/products': {}", e);
        }
    }
}
