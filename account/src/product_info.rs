pub struct ProductInfo {
    pub product_id: String,
    pub product_name: String,
    pub product_description: String,
    pub product_price: rust_decimal::Decimal,
    pub product_currency: String,
    pub product_category: String,
    pub product_image_url: String,
}