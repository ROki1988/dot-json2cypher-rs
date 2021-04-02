use anyhow::{anyhow, Result};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

use std::fmt::Write;

#[derive(Debug, Deserialize)]
struct Attributes(HashMap<String, Value>);

impl Attributes {
    fn to_string_vec(&self) -> Vec<String> {
        self.0
            .iter()
            .filter_map(|(key, value)| match value {
                Value::Bool(_) | Value::Number(_) | Value::String(_) => {
                    Some(format!("{}: {}", key, value))
                }
                _ => None,
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct Graph {
    name: String,
    objects: Vec<Object>,
    edges: Vec<Edge>,
}

impl Graph {
    pub(crate) fn to_cypher_string(&self) -> Result<String> {
        let mut buff = String::new();

        self.objects
            .iter()
            .try_for_each(|x| writeln!(buff, "{}", x.to_cypher_string()))?;

        let object_map: HashMap<u32, &Object> = self.objects.iter().map(|x| (x._gvid, x)).collect();

        for edge in self.edges.as_slice() {
            let tail = object_map
                .get(&edge.tail)
                .ok_or_else(|| anyhow!("invalid node id: {}", edge.tail))?;

            let head = object_map
                .get(&edge.head)
                .ok_or_else(|| anyhow!("invalid node id: {}", edge.head))?;

            writeln!(
                buff,
                r#"CREATE ({}) -[ :EDGE {{ {} }} ]-> ({})"#,
                tail.name, edge.attributes.to_string_vec().join(","),head.name
            )?;
        }

        Ok(buff)
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct Object {
    _gvid: u32,
    name: String,
    label: String,
    #[serde(flatten)]
    attributes: Attributes,
}

impl Object {
    fn to_cypher_string(&self) -> String {
        let mut l = self.attributes.to_string_vec();
        let v = format!("label: {}", Value::String(self.label.to_string()));
        l.push(v);

        format!(
            r#"CREATE ({}:Object {{ {} }}) "#,
            self.name,
            l.join(", ")
        )
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct Edge {
    _gvid: u32,
    tail: u32,
    head: u32,

    #[serde(flatten)]
    attributes: Attributes,
}
