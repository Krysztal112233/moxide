use clap::{builder::PossibleValue, ValueEnum};

#[derive(Debug, Clone)]
pub(crate) enum CreateType {
    Page,
    Bundle,
    Project,
}

impl ValueEnum for CreateType {
    fn value_variants<'a>() -> &'a [Self] {
        &[CreateType::Page, CreateType::Bundle, CreateType::Project]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            CreateType::Page => PossibleValue::new("page"),
            CreateType::Bundle => PossibleValue::new("bundle"),
            CreateType::Project => PossibleValue::new("project"),
        })
    }
}
