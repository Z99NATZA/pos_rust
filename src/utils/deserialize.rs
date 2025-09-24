use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

pub fn deserialize_string_to_decimal<'de, D>(
    deserializer: D,
) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    let opt_str = Option::<String>::deserialize(deserializer)?;

    if let Some(s) = opt_str {
        // ลบ , ออก และกรองแค่เลข/จุด/ลบ
        let cleaned: String = s
            .replace(',', "")
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
            .collect();

        if cleaned.trim().is_empty() {
            return Ok(Decimal::ZERO);
        }

        // ✅ ไม่ต้อง .into()
        Ok(Decimal::from_str(&cleaned).unwrap_or(Decimal::ZERO))
    } else {
        Ok(Decimal::ZERO)
    }
}
