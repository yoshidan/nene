use crate::model::Table;
use crate::repository::TableRepository;
use handlebars::Handlebars;
use std::alloc::handle_alloc_error;
use std::fs::File;
use std::io::Write;
use std::{fs, io};

pub struct Config {
    file_prefix: String,
    file_suffix: String,
    output_dir: String,
    input_dir: String,
}

pub struct TableGenerator {
    repository: TableRepository,
}

impl TableGenerator {
    pub fn new(repository: TableRepository) -> Self {
        Self { repository }
    }

    pub fn generate(&self, config: Config) -> anyhow::Result<()> {
        let tables = self.repository.read_all().await?;

        let mut templates = fs::read_dir(".")?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        let mut handlebars = Handlebars::new();

        templates.iter().for_each(|e| {
            let template_string = fs::read_to_string(e)?;
            tables.iter().for_each(|table| {
                let rendered = handlebars.render_template::<Table>(&template_string, table)?;
                let file_path = format!(
                    "{}/{}{}{}.rs",
                    config.output_dir, config.file_prefix, table.table_name, config.file_suffix
                );
                let mut file = File::create(file_path)?;
                write!(file, "{}", rendered)?;
                file.flush()?;
            });
        });
        Ok(())
    }
}
