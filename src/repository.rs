use crate::model::{Column, Index, Table};
use google_cloud_spanner::client::Client;
use google_cloud_spanner::reader::AsyncIterator;
use google_cloud_spanner::statement::Statement;

pub struct TableRepository {
    client: Client,
    json: bool,
}

impl TableRepository {
    pub fn new(client: Client, json: bool) -> Self {
        Self { client, json }
    }

    pub async fn read_all(&self) -> anyhow::Result<Vec<Table>> {
        let stmt = Statement::new("SELECT TABLE_NAME, PARENT_TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_SCHEMA = '' ORDER BY TABLE_NAME");
        let mut tx = self.client.single().await?;
        let mut itr = tx.query(stmt).await?;

        let mut table_names: Vec<(String, Option<String>)> = vec![];
        while let Some(row) = itr.next().await? {
            table_names.push((
                row.column_by_name("TABLE_NAME")?,
                row.column_by_name("PARENT_TABLE_NAME")?,
            ));
        }

        let mut tables: Vec<Table> = vec![];
        while let Some(table_name) = table_names.pop() {
            let columns = self.read_columns(&table_name.0).await?;
            let indexes = self.read_indexes(&table_name.0).await?;
            let table = Table::new(table_name.0, table_name.1, columns, indexes, self.json);
            tables.push(table)
        }
        log::info!("{} tables found", tables.len());
        Ok(tables)
    }

    async fn read_columns(&self, table_name: &str) -> anyhow::Result<Vec<Column>> {
        let mut stmt = Statement::new(
            "
            SELECT
                c.COLUMN_NAME, c.ORDINAL_POSITION, c.IS_NULLABLE = 'YES' AS IS_NULLABLE, c.SPANNER_TYPE,
                EXISTS (
                    SELECT 1 FROM INFORMATION_SCHEMA.COLUMN_OPTIONS oc
                    WHERE oc.OPTION_NAME = 'allow_commit_timestamp'
                    AND oc.TABLE_NAME = c.TABLE_NAME
                    AND oc.COLUMN_NAME = c.COLUMN_NAME
                ) ALLOW_COMMIT_TIMESTAMP,
                EXISTS (
                    SELECT 1 FROM INFORMATION_SCHEMA.INDEX_COLUMNS ic
                    WHERE ic.TABLE_SCHEMA = ''
                    AND ic.TABLE_NAME = c.TABLE_NAME
                    AND ic.COLUMN_NAME = c.COLUMN_NAME
                    AND ic.INDEX_NAME = 'PRIMARY_KEY'
                ) IS_PRIMARY_KEY,
                IS_GENERATED = 'ALWAYS' AS IS_GENERATED
            FROM
                INFORMATION_SCHEMA.COLUMNS c
            WHERE
                c.TABLE_SCHEMA = ''
            AND
                c.TABLE_NAME = @table
            ORDER BY \
                c.ORDINAL_POSITION",
        );
        stmt.add_param("table", &table_name);
        let mut columns: Vec<Column> = vec![];
        let mut tx = self.client.single().await?;
        let mut itr = tx.query(stmt).await?;
        while let Some(row) = itr.next().await? {
            let column = Column::new(
                row.column_by_name("COLUMN_NAME")?,
                row.column_by_name("ORDINAL_POSITION")?,
                row.column_by_name("SPANNER_TYPE")?,
                row.column_by_name("IS_NULLABLE")?,
                row.column_by_name("IS_PRIMARY_KEY")?,
                row.column_by_name("IS_GENERATED")?,
                row.column_by_name("ALLOW_COMMIT_TIMESTAMP")?,
            );
            columns.push(column)
        }
        Ok(columns)
    }

    async fn read_indexes(&self, table_name: &str) -> anyhow::Result<Vec<Index>> {
        let mut stmt = Statement::new(
            "\
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
        ",
        );
        stmt.add_param("table", &table_name);
        let mut index_names: Vec<(String, bool)> = vec![];
        let mut tx = self.client.single().await?;
        let mut itr = tx.query(stmt).await?;
        while let Some(row) = itr.next().await? {
            index_names.push((
                row.column_by_name("INDEX_NAME")?,
                row.column_by_name("IS_UNIQUE")?,
            ))
        }

        let mut indexes: Vec<Index> = vec![];
        while let Some(index_name) = index_names.pop() {
            let mut stmt = Statement::new(
                "\
                SELECT \
                    ORDINAL_POSITION, COLUMN_NAME \
                FROM \
                    INFORMATION_SCHEMA.INDEX_COLUMNS \
                WHERE \
                    TABLE_SCHEMA = '' \
                AND \
                    INDEX_NAME = @index AND TABLE_NAME = @table \
                ORDER BY ORDINAL_POSITION
            ",
            );
            stmt.add_param("table", &table_name);
            stmt.add_param("index", &index_name.0);

            let mut index = Index {
                index_name: index_name.0,
                unique: index_name.1,
                columns: vec![],
            };
            let mut tx = self.client.single().await?;
            let mut itr = tx.query(stmt).await?;
            while let Some(row) = itr.next().await? {
                let column = (
                    row.column_by_name::<String>("COLUMN_NAME")?,
                    row.column_by_name::<i64>("ORDINAL_POSITION")?,
                );
                index.columns.push(column);
            }
            indexes.push(index);
        }
        Ok(indexes)
    }
}
