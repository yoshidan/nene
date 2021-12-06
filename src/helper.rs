use handlebars::Handlebars;
use handlebars::handlebars_helper;
use convert_case::{Casing, Case};
use crate::model::Column;

fn rust_type_helper(spanner_type: String) -> String {
    let v = if spanner_type == "BOOL" {
        "bool"
    } else if spanner_type == "DATE" {
        "chrono::NaiveDate"
    } else if spanner_type == "TIMESTAMP" {
        "chrono::DateTime<chrono::Utc>"
    } else if spanner_type == "FLOAT64" {
        "f64"
    } else if spanner_type == "NUMERIC" {
        "rust_decimal::Decimal"
    } else if spanner_type.starts_with("BYTES") {
        "Vec<u8>"
    } else if spanner_type == "INT64" {
        "i64"
    } else {
        "String"
    };
    return v.to_string();
}

fn rust_caller_type_helper(v: String) -> String {
    return v.replace("<", "::<");
}

fn snake_helper(v: String) -> String {
    return v.to_case(Case::Snake)
}

fn upper_snake_helper(v: String) -> String {
    return v.to_case(Case::UpperSnake)
}

fn primary_keys(columns: Vec<Column>) -> Vec<Column> {
    let mut primary_keys = vec![];
    for c in columns.iter() {
        if c.primary_key {
            primary_keys.push(c.clone())
        }
    }
    return primary_keys;
}

fn composite(pk: Vec<Column>) -> bool{
    return pk.len() > 1;
}

handlebars_helper!(rust_type: |v: String | rust_type_helper(v));
handlebars_helper!(rust_caller_type: |v: String | rust_caller_type_helper(v));
handlebars_helper!(snake: |v: String | snake_helper(v));
handlebars_helper!(upper_snake: |v: String | upper_snake_helper(v));

pub fn register(handlebars: &mut Handlebars) {
    handlebars.register_helper("rust_type", Box::new(rust_type));
    handlebars.register_helper("rust_caller_type", Box::new(rust_caller_type));
    handlebars.register_helper("snake", Box::new(snake));
    handlebars.register_helper("upper_snake", Box::new(upper_snake));
}