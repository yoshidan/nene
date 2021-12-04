use google_cloud_spanner::statement::{ToStruct, Types, Kinds, ToKind};
use google_cloud_spanner::row::{TryFromStruct, Error, Struct};

pub struct Column {
    pub column_name: String,
    pub ordinal_position: i64,
    pub spanner_type: String,
    pub nullable: bool,
    pub primary_key: bool,
    pub generated: bool
}

pub struct Index {
    pub index_name: String,
    pub unique: bool,
    pub columns: Vec<(String,i64)>,
}

pub struct Table {
    pub table_name: String,
    pub parent_table_name: String,
    pub columns: Vec<Column>,
    pub indexes: Vec<Index>,
}
