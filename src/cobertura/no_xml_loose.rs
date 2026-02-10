//! Types for the Cobertura v4 schema that are specifically defined to (de)serialize them to formats other than XML.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename = "coverage")]
pub struct Coverage {
    #[serde(alias = "line-rate")]
    pub line_rate: Option<f64>,
    #[serde(alias = "branch-rate")]
    pub branch_rate: Option<f64>,
    #[serde(alias = "lines-covered")]
    pub lines_covered: Option<u64>,
    #[serde(alias = "lines-valid")]
    pub lines_valid: Option<u64>,
    #[serde(alias = "branches-covered")]
    pub branches_covered: Option<u64>,
    #[serde(alias = "branches-valid")]
    pub branches_valid: Option<u64>,
    pub complexity: Option<f64>,
    pub version: String,
    pub timestamp: String,
    pub sources: Option<Sources>,
    pub packages: Packages,
}

impl Coverage {
    pub fn try_into_json(&self) -> Result<String, impl core::error::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn try_into_json_value(&self) -> Result<serde_json::Value, impl core::error::Error> {
        serde_json::to_value(self)
    }

    pub fn try_from_json(json: &str) -> Result<Self, impl core::error::Error> {
        serde_json::from_str(json)
    }
}

impl From<super::schema_loose::Coverage> for Coverage {
    fn from(value: super::schema_loose::Coverage) -> Self {
        Self {
            line_rate: value.line_rate,
            branch_rate: value.branch_rate,
            lines_covered: value.lines_covered,
            lines_valid: value.lines_valid,
            branches_covered: value.branches_covered,
            branches_valid: value.branches_valid,
            complexity: value.complexity,
            version: value.version,
            timestamp: value.timestamp,
            sources: value.sources.map(|s| s.into()),
            packages: value.packages.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Sources {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<String>,
}

impl From<super::schema_loose::Sources> for Sources {
    fn from(value: super::schema_loose::Sources) -> Self {
        Self {
            source: value.source,
        }
    }
}

impl std::ops::Deref for Sources {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.source
    }
}

impl std::ops::DerefMut for Sources {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.source
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Packages {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub package: Vec<Package>,
}

impl From<super::schema_loose::Packages> for Packages {
    fn from(value: super::schema_loose::Packages) -> Self {
        Self {
            package: value.package.into_iter().map(|p| p.into()).collect(),
        }
    }
}

impl std::ops::Deref for Packages {
    type Target = Vec<Package>;

    fn deref(&self) -> &Self::Target {
        &self.package
    }
}

impl std::ops::DerefMut for Packages {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.package
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    #[serde(alias = "line-rate")]
    pub line_rate: Option<f64>,
    #[serde(alias = "branch-rate")]
    pub branch_rate: Option<f64>,
    pub complexity: Option<f64>,
    pub classes: Classes,
}

impl From<super::schema_loose::Package> for Package {
    fn from(value: super::schema_loose::Package) -> Self {
        Self {
            name: value.name,
            line_rate: value.line_rate,
            branch_rate: value.branch_rate,
            complexity: value.complexity,
            classes: value.classes.into(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Classes {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub class: Vec<Class>,
}

impl From<super::schema_loose::Classes> for Classes {
    fn from(value: super::schema_loose::Classes) -> Self {
        Self {
            class: value.class.into_iter().map(|c| c.into()).collect(),
        }
    }
}

impl std::ops::Deref for Classes {
    type Target = Vec<Class>;

    fn deref(&self) -> &Self::Target {
        &self.class
    }
}

impl std::ops::DerefMut for Classes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.class
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Class {
    pub name: String,
    pub filename: PathBuf,
    #[serde(alias = "line-rate")]
    pub line_rate: Option<f64>,
    #[serde(alias = "branch-rate")]
    pub branch_rate: Option<f64>,
    pub complexity: Option<f64>,
    pub methods: Methods,
    pub lines: Lines,
}

impl From<super::schema_loose::Class> for Class {
    fn from(value: super::schema_loose::Class) -> Self {
        Self {
            name: value.name,
            filename: value.filename,
            line_rate: value.line_rate,
            branch_rate: value.branch_rate,
            complexity: value.complexity,
            methods: value.methods.into(),
            lines: value.lines.into(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Methods {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub method: Vec<Method>,
}

impl From<super::schema_loose::Methods> for Methods {
    fn from(value: super::schema_loose::Methods) -> Self {
        Self {
            method: value.method.into_iter().map(|m| m.into()).collect(),
        }
    }
}

impl std::ops::Deref for Methods {
    type Target = Vec<Method>;

    fn deref(&self) -> &Self::Target {
        &self.method
    }
}

impl std::ops::DerefMut for Methods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.method
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Method {
    pub name: String,
    pub signature: String,
    #[serde(alias = "line-rate")]
    pub line_rate: Option<f64>,
    #[serde(alias = "branch-rate")]
    pub branch_rate: Option<f64>,
    pub complexity: Option<f64>,
    pub lines: Lines,
}

impl From<super::schema_loose::Method> for Method {
    fn from(value: super::schema_loose::Method) -> Self {
        Self {
            name: value.name,
            signature: value.signature,
            line_rate: value.line_rate,
            branch_rate: value.branch_rate,
            complexity: value.complexity,
            lines: value.lines.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Lines {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub line: Vec<Line>,
}

impl From<super::schema_loose::Lines> for Lines {
    fn from(value: super::schema_loose::Lines) -> Self {
        Self {
            line: value.line.into_iter().map(|l| l.into()).collect(),
        }
    }
}

impl std::ops::Deref for Lines {
    type Target = Vec<Line>;

    fn deref(&self) -> &Self::Target {
        &self.line
    }
}

impl std::ops::DerefMut for Lines {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.line
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Line {
    pub number: u64,
    pub hits: Option<u64>,
    #[serde(default)]
    pub branch: bool,
    #[serde(alias = "condition-coverage", skip_serializing_if = "Option::is_none")]
    pub condition_coverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Conditions>,
}

impl From<super::schema_loose::Line> for Line {
    fn from(value: super::schema_loose::Line) -> Self {
        Self {
            number: value.number,
            hits: value.hits,
            branch: value.branch,
            condition_coverage: value.condition_coverage,
            conditions: value.conditions.map(|c| c.into()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Conditions {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub condition: Vec<Condition>,
}

impl From<super::schema_loose::Conditions> for Conditions {
    fn from(value: super::schema_loose::Conditions) -> Self {
        Self {
            condition: value.condition.into_iter().map(|c| c.into()).collect(),
        }
    }
}

impl std::ops::Deref for Conditions {
    type Target = Vec<Condition>;

    fn deref(&self) -> &Self::Target {
        &self.condition
    }
}

impl std::ops::DerefMut for Conditions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.condition
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub number: u64,
    #[serde(rename = "type")]
    pub condition_type: String,
    pub coverage: String,
}

impl From<super::schema_loose::Condition> for Condition {
    fn from(value: super::schema_loose::Condition) -> Self {
        Self {
            number: value.number,
            condition_type: value.condition_type,
            coverage: value.coverage,
        }
    }
}
