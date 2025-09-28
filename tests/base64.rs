use rand::RngCore;
use base64::{engine::general_purpose, Engine as _};

#[test]
#[ignore]
fn generate_secret_base64() {
    // สุ่ม 64 ไบต์ (512-bit)
    let mut bytes = [0u8; 64];
    rand::rng().fill_bytes(&mut bytes);

    // เข้ารหัสเป็น Base64
    let secret_b64 = general_purpose::STANDARD.encode(&bytes);

    // log ออก console
    println!("BASE_64={}", secret_b64);

    // แค่ assert เพื่อไม่ให้ test ตก
    assert!(!secret_b64.is_empty());
}
