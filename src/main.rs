use account::amount::{notional, Money, Price, Quantity, Rounding};
fn main() {
    println!("Hello, world!");

    let symbol_code = "09018";
    let nums = Quantity::from_decimal(rust_decimal::dec!(777)); // Changed from let numbs = Quantity::from(1000u32);
    let buy_price = Price::from_decimal(rust_decimal::dec!(9.0025));
    let amtstock = Money::from_decimal(rust_decimal::dec!(10000)); // 100.05 元
    println!("Stock: {}", amtstock.unwrap().units());
    println!("Stock: {}", amtstock.unwrap().to_string());
    println!("Stock: {}", amtstock.unwrap().to_decimal().unwrap());
    println!("symbol_code Of Stock: {}", symbol_code);
    println!("Num Of Stock: {}", nums.unwrap().to_string());
    if let Some(price) = buy_price {
        println!("Buy Price: {}", price.to_decimal().unwrap());
    }

    let sell_price = Price::from_str("10.00153");
    if let Some(price) = sell_price {
        println!("Sell Price: {}", price.to_decimal().unwrap());
    }
    let buy_amount: Money = notional(buy_price.unwrap(), nums.unwrap(), Rounding::MidpointAwayFromZero).unwrap();
    println!("Buy Amount: {}", buy_amount);

    let sell_amount: Money = notional(sell_price.unwrap(), nums.unwrap(), Rounding::MidpointAwayFromZero).unwrap();
    println!("Sell Amount: {}", sell_amount);

    let mid_price = Price::from_whole(9);

    println!("Mid Price: {}", mid_price.unwrap().to_decimal().unwrap());
    println!("Mid Price: {}", mid_price.unwrap().format_fixed(6));


}