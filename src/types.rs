use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Protocol {
    #[serde(rename = "version")]
    pub version: Version,

    #[serde(rename = "domains")]
    pub domains: Vec<Domain>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Domain {
    #[serde(rename = "domain")]
    pub domain: String,

    #[serde(rename = "experimental")]
    pub experimental: Option<bool>,

    #[serde(rename = "dependencies")]
    pub dependencies: Option<Vec<String>>,

    #[serde(rename = "types")]
    pub types: Option<Vec<TypeElement>>,

    #[serde(rename = "commands")]
    pub commands: Vec<Command>,

    #[serde(rename = "events")]
    pub events: Option<Vec<Event>>,

    #[serde(rename = "description")]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "experimental")]
    pub experimental: Option<bool>,

    #[serde(rename = "parameters")]
    pub parameters: Option<Vec<Parameter>>,

    #[serde(rename = "returns")]
    pub returns: Option<Vec<Parameter>>,

    #[serde(rename = "deprecated")]
    pub deprecated: Option<bool>,

    #[serde(rename = "redirect")]
    pub redirect: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parameter {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "optional")]
    pub optional: Option<bool>,

    #[serde(rename = "$ref")]
    pub parameter_ref: Option<String>,

    #[serde(rename = "type")]
    pub parameter_type: Option<TypeEnum>,

    #[serde(rename = "items")]
    pub items: Option<Items>,

    #[serde(rename = "enum")]
    pub parameter_enum: Option<Vec<String>>,

    #[serde(rename = "experimental")]
    pub experimental: Option<bool>,

    #[serde(rename = "deprecated")]
    pub deprecated: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Items {
    #[serde(rename = "type")]
    pub items_type: Option<TypeEnum>,

    #[serde(rename = "$ref")]
    pub items_ref: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "parameters")]
    pub parameters: Option<Vec<Parameter>>,

    #[serde(rename = "experimental")]
    pub experimental: Option<bool>,

    #[serde(rename = "deprecated")]
    pub deprecated: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TypeElement {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "type")]
    pub type_type: TypeEnum,

    #[serde(rename = "enum")]
    pub type_enum: Option<Vec<String>>,

    #[serde(rename = "properties")]
    pub properties: Option<Vec<Parameter>>,

    #[serde(rename = "experimental")]
    pub experimental: Option<bool>,

    #[serde(rename = "items")]
    pub items: Option<Items>,

    #[serde(rename = "deprecated")]
    pub deprecated: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    #[serde(rename = "major")]
    pub major: String,

    #[serde(rename = "minor")]
    pub minor: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TypeEnum {
    #[serde(rename = "any")]
    Any,

    #[serde(rename = "array")]
    Array,

    #[serde(rename = "boolean")]
    Boolean,

    #[serde(rename = "integer")]
    Integer,

    #[serde(rename = "number")]
    Number,

    #[serde(rename = "object")]
    Object,

    #[serde(rename = "string")]
    String,
}
