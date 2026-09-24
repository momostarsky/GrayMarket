use account::amount::{notional, Amount, Money, Price, Quantity, Rounding};

fn main() {
    println!("Hello, GrayDB!");

    let qty = Quantity::from_decimal(rust_decimal::dec!(777)).unwrap();
    let price = Price::from_decimal(rust_decimal::dec!(10.0015)).unwrap();
    let amount: Money = notional(price, qty, Rounding::MidpointAwayFromZero).unwrap();
    println!("notional = {}", amount.format_fixed(4));
}