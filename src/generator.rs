use crate::model::Table;
use crate::repository::TableRepository;
use handlebars::Handlebars;
use std::alloc::handle_alloc_error;
use std::fs::File;
use std::io::Write;
use std::{fs, io};
use anyhow::Context;
use std::collections::HashMap;
use convert_case::{Case, Casing};

pub struct Config {
    pub file_prefix: String,
    pub file_suffix: String,
    pub output_dir: String,
    pub input_dir: String,
}

pub struct TableGenerator {
    repository: TableRepository,
}

impl TableGenerator {
    pub fn new(repository: TableRepository) -> Self {
        Self { repository }
    }

    pub async fn generate(&self, config: Config) -> anyhow::Result<()> {

        let mut templates = fs::read_dir(config.input_dir)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        let mut handlebars = Handlebars::new();

        let tables = self.repository.read_all().await?;
        for e in templates.iter() {
            let template_string = fs::read_to_string(e)?;
            for table in tables.iter() {
                let rendered = handlebars.render_template::<Table>(&template_string, &table)?;
                let file_path = format!(
                    "{}/{}{}{}.rs",
                    config.output_dir, config.file_prefix,  table.table_name.to_case(Case::Snake), config.file_suffix
                );
                let mut file = File::create(file_path)?;
                write!(file, "{}", rendered)?;
                file.flush()?;
            }
        }
        Ok(())
    }
}
