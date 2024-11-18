use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{error::Error, fs::File, io::Read};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub _id: String,
    pub actual_price: String,
    pub average_rating: String,
    pub brand: String,
    pub category: String,
    pub crawled_at: String,
    pub description: String,
    pub discount: String,
    pub images: Vec<String>,
    pub out_of_stock: bool,
    pub pid: String,
    pub product_details: Vec<std::collections::HashMap<String, String>>,
    pub seller: String,
    pub selling_price: String,
    pub sub_category: String,
    pub title: String,
    pub url: String,
}

pub fn parse_json_product(json_str: &str) -> Result<Vec<Product>, Box<dyn Error>> {
    let v: Value = serde_json::from_str(json_str)?;
    let products: Vec<Product> = serde_json::from_value(v)?;
    Ok(products)
}

pub fn read_json_file(file_path: &str) -> Result<Vec<Product>, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut json_str = String::new();
    file.read_to_string(&mut json_str)?;
    let products = parse_json_product(&json_str)?;
    Ok(products)
}
