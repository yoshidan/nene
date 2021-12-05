use google_cloud_spanner::row::{Error, Struct, TryFromStruct};
use google_cloud_spanner::statement::{Kinds, ToKind, ToStruct, Types};
use serde::{Serialize,Deserialize};
use convert_case::{Casing, Case};

#[derive(Serialize, Deserialize)]
pub struct Column {
    pub column_name: String,
    pub snake_column_name: String,
    pub ordinal_position: i64,
    pub spanner_type: String,
    pub rust_type: String,
    pub nullable: bool,
    pub primary_key: bool,
    pub generated: bool,
    pub allow_commit_timestamp: bool,
}

impl Column {
    pub fn new(column_name: String, ordinal_position: i64, spanner_type: String, nullable: bool, primary_key: bool, generated: bool, allow_commit_timestamp: bool) ->Self {
        Self {
            rust_type: Self::rust_type(&spanner_type).to_string(),
            snake_column_name: column_name.to_case(Case::Snake),
            column_name,
            ordinal_position,
            spanner_type,
            nullable,
            primary_key,
            generated,
            allow_commit_timestamp,
        }
    }

    fn rust_type(spanner_type: &str) -> &'static str {
        if spanner_type == "BOOL" {
            return "bool"
        }else if spanner_type == "DATE" {
            return "chrono::NativeDate"
        }else if spanner_type == "TIMESTAMP" {
            return "chrono::DateTime<chrono::Utc>"
        }else if spanner_type == "FLOAT64" {
            return "f64"
        }else if spanner_type == "NUMERIC" {
            return "rust_decimal::Decimal"
        }else if spanner_type.starts_with("BYTES") {
            return "Vec<u8>"
        }else if spanner_type == "INT64" {
            return "i64"
        }
        return "string"
    }
}

#[derive(Serialize, Deserialize)]
pub struct Index {
    pub index_name: String,
    pub unique: bool,
    pub columns: Vec<(String, i64)>,
}

#[derive(Serialize, Deserialize)]
pub struct Table {
    pub table_name: String,
    pub parent_table_name: Option<String>,
    pub columns: Vec<Column>,
    pub indexes: Vec<Index>,
    pub snake_table_name: String,
}


impl Table {
    pub fn new(table_name: String, parent_table_name: Option<String>, columns: Vec<Column>, indexes: Vec<Index>) ->Self {
        Self {
            snake_table_name: table_name.to_case(Case::Snake),
            table_name,
            parent_table_name,
            columns,
            indexes
        }
    }


}
