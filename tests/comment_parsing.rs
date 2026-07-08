//! `nx.parse_edgelist` treats `#` as a comment anywhere on a line: text from the
//! first `#` is dropped before tokenising. An inline `#` must not leak into a node
//! label, and a line left with fewer than two tokens is skipped.

use rsomics_reaching_centrality::{
    global_reaching_centrality_from_edge_list, local_reaching_centrality_from_edge_list,
};

fn sorted_labels(text: &str, directed: bool) -> Vec<String> {
    let mut labels: Vec<String> = local_reaching_centrality_from_edge_list(text, directed)
        .unwrap()
        .into_iter()
        .map(|(label, _)| label)
        .collect();
    labels.sort();
    labels
}

#[test]
fn inline_comment_matches_comment_free_graph() {
    // "1 2#c" -> edge (1,2), not a node "2#c"; "0 #x" -> single token -> skipped.
    let commented = "0 1\n1 2#c\n2 3\n0 #x\n";
    let clean = "0 1\n1 2\n2 3\n";

    assert_eq!(sorted_labels(commented, true), sorted_labels(clean, true));
    assert_eq!(
        sorted_labels(commented, true),
        vec!["0", "1", "2", "3"],
        "inline '#' must not create a spurious node"
    );

    for directed in [true, false] {
        assert_eq!(
            global_reaching_centrality_from_edge_list(commented, directed).unwrap(),
            global_reaching_centrality_from_edge_list(clean, directed).unwrap(),
            "global reaching centrality diverges with an inline comment (directed={directed})"
        );
    }
}

#[test]
fn hash_stripped_mid_token() {
    // A single "u v#comment" line is one edge between exactly two nodes.
    let g = local_reaching_centrality_from_edge_list("1 2#note\n", true).unwrap();
    let mut labels: Vec<&str> = g.iter().map(|(l, _)| l.as_str()).collect();
    labels.sort();
    assert_eq!(labels, vec!["1", "2"]);
}
