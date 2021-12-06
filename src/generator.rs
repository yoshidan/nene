use crate::helper::register;
use crate::model::Table;
use crate::repository::TableRepository;
use anyhow::Context;
use convert_case::{Case, Casing};
use handlebars::Handlebars;
use std::alloc::handle_alloc_error;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::{fs, io};
use std::path::PathBuf;

const DEFAULT_MODEL_TEMPLATE: &str = include_str!("default/multi/${table_name}.tmpl");
const DEFAULT_MOD_TEMPLATE: &str = include_str!("default/single/mod.tmpl");

pub struct TableGenerator {
    repository: TableRepository,
}

impl TableGenerator {
    pub fn new(repository: TableRepository) -> Self {
        Self { repository }
    }

    pub async fn generate_default(&self, output_dir: &str) -> anyhow::Result<()> {
        let mut handlebars = Handlebars::new();
        register(&mut handlebars);
        let tables = self.repository.read_all().await?;

        self.generate_multi(&handlebars, &tables, DEFAULT_MODEL_TEMPLATE, "${table_name}", output_dir).await;
        self.generate_single(&handlebars, &tables, DEFAULT_MODEL_TEMPLATE, "mod", output_dir).await;
        Ok(())
    }

    pub async fn generate(&self, input_dir:&str, output_dir:&str) -> anyhow::Result<()> {
        let mut handlebars = Handlebars::new();
        register(&mut handlebars);

        let mut multi_templates = fs::read_dir(format!("{}/multi", input_dir))?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        let mut single_templates = fs::read_dir(format!("{}/single", input_dir))?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        let tables = self.repository.read_all().await?;

        for mut e in multi_templates.into_iter() {
            let template_string = fs::read_to_string(e.clone())?;
            e.set_extension("");
            let file_name = e.file_name().unwrap().to_str().unwrap().to_string();
            self.generate_multi(&handlebars, &tables, &template_string, &file_name, &output_dir).await?
        }

        for mut e in single_templates.into_iter() {
            let template_string = fs::read_to_string(e.clone())?;
            e.set_extension("");
            let file_name = e.file_name().unwrap().to_str().unwrap().to_string();
            self.generate_single(&handlebars, &tables, &template_string, &file_name, &output_dir).await?
        }
        Ok(())
    }

    async fn generate_single(&self, handlebars: &Handlebars<'_>, tables: &Vec<Table>, template_string: &str, file_name:&str, output_dir: &str) -> anyhow::Result<()> {
        let rendered = handlebars.render_template::<Vec<Table>>(template_string, tables)?;
        let file_path = format!("{}/{}.rs", output_dir, file_name);
        let mut file = File::create(file_path)?;
        write!(file, "{}", rendered)?;
        Ok(file.flush()?)
    }

    async fn generate_multi(&self, handlebars: &Handlebars<'_>, tables: &Vec<Table>, template_string: &str, file_name: &str, output_dir: &str) -> anyhow::Result<()> {
        for table in tables.iter() {
            let rendered = handlebars.render_template::<Table>(&template_string, &table)?;
            let file_path = format!(
                "{}/{}.rs",
                output_dir,
                file_name.replace("${table_name}", &table.table_name.to_case(Case::Snake))
            );
            let mut file = File::create(file_path)?;
            write!(file, "{}", rendered)?;
            file.flush()?;
        }
        Ok(())
    }
}
