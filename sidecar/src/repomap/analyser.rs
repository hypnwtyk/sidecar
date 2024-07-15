use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashMap, HashSet};

use super::tag::TagIndex;
pub struct TagAnalyzer {
    graph: DiGraph<String, f64>,
    node_indices: HashMap<String, NodeIndex>,
}

impl TagAnalyzer {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_indices: HashMap::new(),
        }
    }
    pub fn build_graph(&mut self, tag_index: &mut TagIndex, mentioned_idents: &HashSet<String>) {
        // Iterate through all common tags in the tag index
        for ident in &tag_index.common_tags {
            // Calculate the multiplier for this identifier based on whether it's mentioned or starts with an underscore
            let mul = self.get_multiplier(ident, mentioned_idents);

            // Get the number of references for this identifier
            let num_refs = tag_index.references[ident].len() as f64;

            // Scale the number of references using square root to dampen the effect of very high reference counts
            let scaled_refs = num_refs.sqrt();

            // For each file that references this identifier
            for referencer in &tag_index.references[ident] {
                // For each file that defines this identifier
                for definer in &tag_index.defines[ident] {
                    // Get or create a node in the graph for the referencing file
                    let referencer_idx = self.get_or_create_node(referencer.to_str().unwrap());

                    // Get or create a node in the graph for the defining file
                    let definer_idx = self.get_or_create_node(definer.to_str().unwrap());

                    // Add an edge from the referencer to the definer
                    // The edge weight is the product of:
                    // - the multiplier (based on whether the ident is mentioned or starts with '_')
                    // - the scaled reference count (square root of the number of references)
                    self.graph
                        .add_edge(referencer_idx, definer_idx, mul * scaled_refs);
                }
            }
        }
    }

    fn get_or_create_node(&mut self, name: &str) -> NodeIndex {
        *self
            .node_indices
            .entry(name.to_string())
            .or_insert_with(|| self.graph.add_node(name.to_string()))
    }

    fn get_multiplier(&self, tag: &str, mentioned_idents: &HashSet<String>) -> f64 {
        if mentioned_idents.contains(tag) {
            10.0
        } else if tag.starts_with('_') {
            0.1
        } else {
            1.0
        }
    }
}
