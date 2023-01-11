use serde::{Deserialize, Serialize};

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
    pub fn new(
        column_name: String,
        ordinal_position: i64,
        spanner_type: String,
        nullable: bool,
        primary_key: bool,
        generated: bool,
        allow_commit_timestamp: bool,
    ) -> Self {
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
pub struct PrimaryKey {
    pub uppers: Vec<Column>,
    pub column: Column,
    pub last: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Table {
    pub table_name: String,
    pub parent_table_name: Option<String>,
    pub columns: Vec<Column>,
    pub indexes: Vec<Index>,
    pub primary_keys: Vec<PrimaryKey>,
    pub composite_key: bool,
    pub json: bool,
    pub default: bool,
}

impl Table {
    pub fn new(
        table_name: String,
        parent_table_name: Option<String>,
        columns: Vec<Column>,
        indexes: Vec<Index>,
        json: bool,
        default: bool,
    ) -> Self {
        let mut primary_keys = vec![];
        for c in columns.iter() {
            if c.primary_key {
                primary_keys.push(c.clone())
            }
        }
        let mut primary_keys_with_rest = vec![];
        for c in primary_keys.iter() {
            let mut uppers = Vec::with_capacity(primary_keys.len() - 1);
            for r in primary_keys.iter() {
                // include self
                if c.ordinal_position >= r.ordinal_position {
                    uppers.push(r.clone())
                }
            }
            primary_keys_with_rest.push(PrimaryKey {
                uppers: uppers,
                column: c.clone(),
                last: false,
            })
        }
        primary_keys_with_rest[primary_keys.len() - 1].last = true;

        Self {
            table_name,
            parent_table_name,
            columns,
            indexes,
            composite_key: primary_keys.len() > 1,
            primary_keys: primary_keys_with_rest,
            json,
            default,
        }
    }
}
