use google_cloud_spanner::row::{Error, Struct, TryFromStruct};
use google_cloud_spanner::statement::{Kinds, ToKind, ToStruct, Types};
use serde::{Serialize,Deserialize};
use convert_case::{Casing, Case};

#[derive(Serialize, Deserialize, Clone)]
pub struct Column {
    pub column_name: String,
    pub ordinal_position: i64,
    pub spanner_type: String,
    pub nullable: bool,
    pub primary_key: bool,
    pub generated: bool,
    pub allow_commit_timestamp: bool,
}

impl Column {
    pub fn new(column_name: String, ordinal_position: i64, spanner_type: String, nullable: bool, primary_key: bool, generated: bool, allow_commit_timestamp: bool) ->Self {
        Self {
            column_name,
            ordinal_position,
            spanner_type,
            nullable,
            primary_key,
            generated,
            allow_commit_timestamp,
        }
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
}

impl Table {
    pub fn new(table_name: String, parent_table_name: Option<String>, columns: Vec<Column>, indexes: Vec<Index>) ->Self {
        Self {
            table_name,
            parent_table_name,
            columns,
            indexes,
        }
    }
}
