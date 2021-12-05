use google_cloud_spanner::row::{Error, Struct, TryFromStruct};
use google_cloud_spanner::statement::{Kinds, ToKind, ToStruct, Types};
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Column {
    pub column_name: String,
    pub ordinal_position: i64,
    pub spanner_type: String,
    pub nullable: bool,
    pub primary_key: bool,
    pub generated: bool,
    pub allow_commit_timestamp: bool,
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
