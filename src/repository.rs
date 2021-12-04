use crate::model::{Table, Column, Index};
use google_cloud_spanner::client::Client;
use google_cloud_spanner::statement::Statement;
use google_cloud_spanner::reader::AsyncIterator;
use std::intrinsics::variant_count;

pub struct TableRepository {
    client: Client
}

impl TableRepository {
    pub async fn read_all(&self) -> anyhow::Result<Vec<Table>> {
        let stmt = Statement::new("SELECT TABLE_NAME, PARENT_TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_SCHEMA = '' ORDER BY TABLE_NAME");
        let mut itr = self.client.single().await?.query(stmt).await?;

        let mut table_names : Vec<(String,String)>= vec![];
        while let Some(row) = itr.next().await? {
            table_names.push((row.column_by_name("TABLE_NAME")?, row.column_by_name("PARENT_TABLE_NAME")?));
        }

        let mut tables : Vec<Table>= vec![];
        while let Some(table_name ) = table_names.pop() {
            let table = Table {
                columns: self.read_columns(&table_name.0),
                indexes: self.read_indexes(&table_name.0),
                table_name: table_name.0,
                parent_table_name: table_name.1,
            };
            tables.push(table)
        }
        Ok(table)
    }

    async fn read_columns(&self, table_name: &str) -> anyhow::Result<Vec<Column>> {
        let mut stmt = Statement::new("\
            SELECT \
                c.COLUMN_NAME, c.ORDINAL_POSITION, c.IS_NULLABLE, c.SPANNER_TYPE, \
                EXISTS ( \
                    SELECT 1 FROM INFORMATION_SCHEMA.INDEX_COLUMNS ic \
                    WHERE ic.TABLE_SCHEMA = '' and ic.TABLE_NAME = c.TABLE_NAME \
                    AND ic.COLUMN_NAME = c.COLUMN_NAME \
                    AND ic.INDEX_NAME = 'PRIMARY_KEY' \
                ) IS_PRIMARY_KEY, \
                IS_GENERATED = 'ALWAYS' AS IS_GENERATED \
            FROM \
                INFORMATION_SCHEMA.COLUMNS c \
            WHERE \
                c.TABLE_SCHEMA = '' \
            AND \
                c.TABLE_NAME = @table \
            ORDER BY \
                c.ORDINAL_POSITION");
        stmt.add_param("table",&table_name.0);
        let mut columns: Vec<Column>= vec![];
        let mut itr = self.client.single().await?.query(stmt).await?;
        while let Some(row) = itr.next().await? {
            let column = Column {
                column_name: row.column_by_name("COLUMN_NAME")?,
                ordinal_position: row.column_by_name("ORDINAL_POSITION")?,
                spanner_type: row.column_by_name("SPANNER_TYPE")?,
                nullable: row.column_by_name("IS_NULLABLE")?,
                primary_key: row.column_by_name("IS_PRIMARY_KEY")?,
                generated: row.column_by_name("IS_GENERATED")?,
            };
            columns.push(column)
        }
        Ok(columns)
    }

    async fn read_indexes(&self, table_name: &str) -> anyhow::Result<Vec<Index>> {
        let mut stmt = Statement::new("\
            SELECT \
                INDEX_NAME, IS_UNIQUE  \
		    FROM  \
		        INFORMATION_SCHEMA.INDEXES  \
		    WHERE \
		        TABLE_SCHEMA = ''  \
		    AND \
		        INDEX_NAME != 'PRIMARY_KEY'  \
            AND \
                TABLE_NAME = @table \
            AND \
                SPANNER_IS_MANAGED = FALSE\
        ");
        stmt.add_param("table",&table_name.0);
        let mut index_names: Vec<(String,bool)>= vec![];
        let mut itr = self.client.single().await?.query(stmt).await?;
        while let Some(row) = itr.next().await? {
            index_names.push((row.column_by_name("INDEX_NAME")? , row.column_by_name("IS_UNIQUE")?))
        }

        let mut indexes : Vec<Index>= vec![];
        while let Some(index_name ) = index_names.pop() {
            let mut stmt = Statement::new("\
                SELECT \
                    ORDINAL_POSITION, COLUMN_NAME \
                FROM \
                    INFORMATION_SCHEMA.INDEX_COLUMNS \
                WHERE \
                    TABLE_SCHEMA = '' \
                AND \
                    INDEX_NAME = @index AND TABLE_NAME = @table \
                ORDER BY ORDINAL_POSITION
            ");
            stmt.add_param("table", &table_name);
            stmt.add_param("index", &index_name.0);

            let mut index = Index {
                index_name: index_name.0,
                unique: index_name.1,
                columns: vec![],
            };
            let mut itr = self.client.single().await?.query(stmt).await?;
            while let Some(row) = itr.next().await? {
                let column = (row.column_by_name::<String>("COLUMN_NAME")?, row.column_by_name::<i64>("ORDINAL_POSITION")?);
                index.columns.push(column);
            }
            indexes.push(index);
        }
        Ok(indexes)
    }
}