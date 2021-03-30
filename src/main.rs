use std::io;
use std::io::Read;

use crate::graph::Graph;
use anyhow::Result;
use serde_json;

mod graph;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let graph: Graph = serde_json::from_str(input.as_str())?;

    println!("{}", graph.to_cypher_string()?);

    Ok(())
}
