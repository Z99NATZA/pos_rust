use rust_decimal::Decimal;
use std::str::FromStr;

pub fn string_to_decimal_2(s: String) -> Decimal {
    let mut d = Decimal::from_str(s.as_str()).unwrap_or(Decimal::new(0, 2));
    d.rescale(2);
    d
}
