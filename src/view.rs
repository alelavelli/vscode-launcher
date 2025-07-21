use crate::enums::ProgrammingLanguage;
use crate::model::Configuration;
use tabled::settings::object::Columns;
use tabled::settings::{Alignment, Style};
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct ConfigurationInfo {
    name: String,
    language: ProgrammingLanguage,
}

impl From<Configuration> for ConfigurationInfo {
    fn from(value: Configuration) -> Self {
        Self {
            name: value.name,
            language: value.programming_language,
        }
    }
}

pub fn print_configurations_info(configurations: Vec<Configuration>) {
    let mut table = Table::new(
        configurations
            .into_iter()
            .map(|elem| elem.into())
            .collect::<Vec<ConfigurationInfo>>(),
    );

    table.with(Style::modern());
    table.modify(Columns::first(), Alignment::left());
    println!("{}", table);
}
