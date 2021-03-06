use convert_case::{Case, Casing};
use handlebars::handlebars_helper;
use handlebars::Handlebars;

fn rust_type_helper(spanner_type: String) -> String {
    let should_wrap_array = spanner_type.starts_with("ARRAY");
    let spanner_type = if should_wrap_array {
        spanner_type.replace("ARRAY<", "").replace(">", "")
    } else {
        spanner_type
    };

    let v = if spanner_type == "BOOL" {
        "bool"
    } else if spanner_type == "DATE" {
        "chrono::NaiveDate"
    } else if spanner_type == "TIMESTAMP" {
        "chrono::DateTime<chrono::Utc>"
    } else if spanner_type == "FLOAT64" {
        "f64"
    } else if spanner_type == "NUMERIC" {
        "google_cloud_spanner::value::SpannerNumeric"
    } else if spanner_type.starts_with("BYTES") {
        "Vec<u8>"
    } else if spanner_type == "INT64" {
        "i64"
    } else {
        "String"
    };
    if should_wrap_array {
        return format!("Vec<{}>", v.to_string());
    }
    return v.to_string();
}

fn rust_arg_type_helper(v: String) -> String {
    return v.replace("String", "&str");
}

fn rust_caller_type_helper(v: String) -> String {
    return v.replace("<", "::<");
}

fn snake_helper(v: String) -> String {
    return v.to_case(Case::Snake);
}

fn upper_snake_helper(v: String) -> String {
    return v.to_case(Case::UpperSnake);
}

handlebars_helper!(rust_type: |v: String | rust_type_helper(v));
handlebars_helper!(rust_arg_type: |v: String | rust_arg_type_helper(v));
handlebars_helper!(rust_caller_type: |v: String | rust_caller_type_helper(v));
handlebars_helper!(snake: |v: String | snake_helper(v));
handlebars_helper!(upper_snake: |v: String | upper_snake_helper(v));

pub fn register(handlebars: &mut Handlebars) {
    handlebars.register_helper("rust_type", Box::new(rust_type));
    handlebars.register_helper("rust_arg_type", Box::new(rust_arg_type));
    handlebars.register_helper("rust_caller_type", Box::new(rust_caller_type));
    handlebars.register_helper("snake", Box::new(snake));
    handlebars.register_helper("upper_snake", Box::new(upper_snake));
}
