use std::{fs, path::Path};

use serde::Deserialize;
use thiserror::Error;

const DEFAULT_TARGET: &str = ".env";

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VariableType {
    String,
    Int,
    Float,
    Bool,
    Port,
    Url,
    Email,
    Enum,
    Regex,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Variable {
    #[serde(rename = "type")]
    pub variable_type: VariableType,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub options: Vec<String>,
    #[serde(default)]
    pub pattern: Option<String>,
    #[serde(default)]
    pub default: Option<serde_yaml::Value>,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub secret: bool,
    #[serde(default)]
    pub example: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

impl Variable {
    pub fn target(&self) -> &str {
        self.target.as_deref().unwrap_or(DEFAULT_TARGET)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Contract {
    pub version: u32,
    #[serde(default)]
    pub targets: Vec<String>,
    pub variables: std::collections::BTreeMap<String, Variable>,
}

#[derive(Debug, Error)]
pub enum ContractError {
    #[error("could not read contract {path}: {source}")]
    Read {
        path: String,
        source: std::io::Error,
    },
    #[error("could not parse contract: {0}")]
    Parse(#[from] serde_yaml::Error),
}

impl Contract {
    pub fn from_str(contents: &str) -> Result<Self, ContractError> {
        Ok(serde_yaml::from_str(contents)?)
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, ContractError> {
        let path = path.as_ref();
        let contents = fs::read_to_string(path).map_err(|source| ContractError::Read {
            path: path.display().to_string(),
            source,
        })?;

        Self::from_str(&contents)
    }

    pub fn target_files(&self) -> Vec<String> {
        let mut targets = self.targets.clone();
        for variable in self.variables.values() {
            let target = variable.target().to_owned();
            if !targets.contains(&target) {
                targets.push(target);
            }
        }
        targets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_contract_and_applies_default_target() {
        let contract = Contract::from_str(
            r#"
version: 1
targets:
  - .env
  - db.env
variables:
  APP_ENV:
    type: enum
    options: [development, production]
    default: development
    required: true
  DATABASE_URL:
    target: db.env
    type: url
    required: true
    secret: true
"#,
        )
        .expect("valid contract");

        assert_eq!(contract.version, 1);
        assert_eq!(contract.target_files(), vec![".env", "db.env"]);
        assert_eq!(contract.variables["APP_ENV"].target(), ".env");
        assert_eq!(contract.variables["DATABASE_URL"].target(), "db.env");
        assert!(contract.variables["DATABASE_URL"].secret);
    }

    #[test]
    fn rejects_unknown_variable_fields() {
        let result = Contract::from_str(
            r#"
version: 1
variables:
  PORT:
    type: port
    requred: true
"#,
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_unknown_variable_types() {
        let result = Contract::from_str(
            r#"
version: 1
variables:
  VALUE:
    type: unsupported
"#,
        );

        assert!(result.is_err());
    }
}
