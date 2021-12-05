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

        let mut handlebars = Handlebars::new();

        let mut multi_templates = fs::read_dir(format!("{}/multi",config.input_dir))?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        let mut single_templates = fs::read_dir(format!("{}/single",config.input_dir))?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        let tables = self.repository.read_all().await?;

        for e in multi_templates.iter() {
            let template_string = fs::read_to_string(e)?;
            for table in tables.iter() {
                let rendered = handlebars.render_template::<Table>(&template_string, &table)?;
                let file_path = format!(
                    "{}/{}.rs", config.output_dir, table.snake_table_name
                );
                let mut file = File::create(file_path)?;
                write!(file, "{}", rendered)?;
                file.flush()?;
            }
        }

        for mut e in single_templates.into_iter() {
            let template_string = fs::read_to_string(e.clone())?;
            let rendered = handlebars.render_template::<Vec<Table>>(&template_string, &tables)?;
            e.set_extension("");
            let file_name= e.file_name().unwrap().to_str().unwrap().to_string();
            let file_path = format!("{}/{}.rs", config.output_dir, file_name);
            let mut file = File::create(file_path)?;
            write!(file, "{}", rendered)?;
            file.flush()?;
        }
        Ok(())
    }
}
