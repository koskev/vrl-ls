use std::collections::HashMap;

use lsp_types::{CompletionItem, CompletionItemKind, CompletionList, Documentation};
use serde::{Deserialize, Serialize};

use crate::completion::Completion;
const STDLIB_DEFINITIONS: &'static str = include_str!("stdlib.json");

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Argument {
    pub name: String,
    pub description: String,
    pub required: bool,
    #[serde(rename = "type")]
    pub vecor_type: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ReturnType {
    #[serde(rename = "type")]
    pub vecor_type: Option<Vec<String>>,
    pub rules: Option<Vec<String>>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Example {
    pub title: String,
    pub source: String,
    #[serde(rename = "return")]
    pub ret_val: serde_json::Value,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct StdFunction {
    pub anchor: String,
    pub name: String,
    pub category: String,
    pub description: String,

    pub arguments: Vec<Argument>,
    #[serde(rename = "return")]
    pub return_type: ReturnType,
    pub examples: Vec<Example>,
    pub deprecated: bool,
    pub pure: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StdFunctions {
    #[serde(flatten)]
    pub functions: HashMap<String, StdFunction>,
}

impl StdFunctions {
    pub fn generate() -> Self {
        serde_json::from_str(STDLIB_DEFINITIONS).unwrap()
    }
}

impl Default for StdFunctions {
    fn default() -> Self {
        Self::generate()
    }
}

#[derive(Debug, Default)]
pub struct StdCompletion {
    pub functions: StdFunctions,
}

impl StdCompletion {
    pub fn new() -> Self {
        Self {
            functions: StdFunctions::generate(),
        }
    }
}

impl Completion for StdCompletion {
    fn complete(
        &self,
        _location: lsp_types::Position,
        _filename: &str,
    ) -> lsp_types::CompletionList {
        let mut items: Vec<CompletionItem> = self
            .functions
            .functions
            .values()
            .map(|func| CompletionItem {
                label: func.name.clone(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some(format!(
                    "{}({})\n",
                    func.name.clone(),
                    func.arguments
                        .iter()
                        .map(|a| a.name.clone())
                        .collect::<Vec<String>>()
                        .join(", "),
                )),
                documentation: Some(Documentation::String(format!(
                    "{}",
                    func.description.clone()
                ))),
                deprecated: Some(func.deprecated),

                ..Default::default()
            })
            .collect();
        items.sort_by(|a, b| a.label.cmp(&b.label));

        CompletionList {
            items,
            ..Default::default()
        }
    }
}
