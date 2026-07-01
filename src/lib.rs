pub mod io;
pub mod reaching;

use rsomics_common::Result;

pub use io::Graph;

/// Build a graph from edge-list text (`nx.read_edgelist` semantics; see
/// [`io::read_edges`]) and return the graph's global reaching centrality.
pub fn global_reaching_centrality_from_edge_list(text: &str, directed: bool) -> Result<f64> {
    let g = io::read_edges(text.as_bytes(), directed)?;
    Ok(reaching::global(&g))
}

/// Build a graph from edge-list text and return `(label, local_reaching_centrality)`
/// for every node, in interned first-seen order.
pub fn local_reaching_centrality_from_edge_list(
    text: &str,
    directed: bool,
) -> Result<Vec<(String, f64)>> {
    let g = io::read_edges(text.as_bytes(), directed)?;
    let vals = reaching::local_all(&g);
    Ok(g.labels.into_iter().zip(vals).collect())
}
