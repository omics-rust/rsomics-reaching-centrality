//! Value-exact compatibility against networkx 3.6.1
//! `global_reaching_centrality` / `local_reaching_centrality`
//! (weight=None, normalized=True).
//!
//! Golden values were produced once with networkx 3.6.1 and are hardcoded as
//! 17-significant-figure constants / committed edge lists — no Python or
//! subprocess runs at test time.

use rsomics_reaching_centrality::{
    global_reaching_centrality_from_edge_list, local_reaching_centrality_from_edge_list,
};

const EPS: f64 = 1e-12;

fn local_sorted(text: &str, directed: bool) -> Vec<(String, f64)> {
    let mut v = local_reaching_centrality_from_edge_list(text, directed).unwrap();
    v.sort_by(|a, b| a.0.cmp(&b.0));
    v
}

fn assert_close(got: f64, want: f64, label: &str) {
    let ulps = got.to_bits().abs_diff(want.to_bits());
    assert!(
        (got - want).abs() <= EPS && ulps <= 1,
        "{label}: got {got:.17e} want {want:.17e} (|Δ|={:.3e}, ulps={ulps})",
        (got - want).abs()
    );
}

fn assert_local(got: &[(String, f64)], want: &[(&str, f64)], label: &str) {
    assert_eq!(got.len(), want.len(), "{label}: node count mismatch");
    for (i, ((gn, gv), (wn, wv))) in got.iter().zip(want.iter()).enumerate() {
        assert_eq!(gn, wn, "{label}[{i}]: label mismatch");
        assert_close(*gv, *wv, &format!("{label}[{gn}]"));
    }
}

// ── directed hand graphs ─────────────────────────────────────────────

const PATH4: &str = "a b\nb c\nc d\n";
const STAROUT5: &str = "s a\ns b\ns c\ns d\n";
const DAG6: &str = "a b\na c\nb d\nc d\nd e\nd f\n";
const CYCLE4: &str = "a b\nb c\nc d\nd a\n";

#[test]
fn path4_directed() {
    assert_close(
        global_reaching_centrality_from_edge_list(PATH4, true).unwrap(),
        0.6666666666666666,
        "path4 global",
    );
    assert_local(
        &local_sorted(PATH4, true),
        &[
            ("a", 1.0),
            ("b", 0.6666666666666666),
            ("c", 0.3333333333333333),
            ("d", 0.0),
        ],
        "path4 local",
    );
}

#[test]
fn starout5_directed() {
    assert_close(
        global_reaching_centrality_from_edge_list(STAROUT5, true).unwrap(),
        1.0,
        "starout5 global",
    );
    assert_local(
        &local_sorted(STAROUT5, true),
        &[("a", 0.0), ("b", 0.0), ("c", 0.0), ("d", 0.0), ("s", 1.0)],
        "starout5 local",
    );
}

#[test]
fn dag6_directed() {
    assert_close(
        global_reaching_centrality_from_edge_list(DAG6, true).unwrap(),
        0.6799999999999999,
        "dag6 global",
    );
    assert_local(
        &local_sorted(DAG6, true),
        &[
            ("a", 1.0),
            ("b", 0.6),
            ("c", 0.6),
            ("d", 0.4),
            ("e", 0.0),
            ("f", 0.0),
        ],
        "dag6 local",
    );
}

#[test]
fn cycle4_directed() {
    assert_close(
        global_reaching_centrality_from_edge_list(CYCLE4, true).unwrap(),
        0.0,
        "cycle4 global",
    );
    assert_local(
        &local_sorted(CYCLE4, true),
        &[("a", 1.0), ("b", 1.0), ("c", 1.0), ("d", 1.0)],
        "cycle4 local",
    );
}

// ── karate club as a real DiGraph (both-way edges) ───────────────────

const KARATE_DIRECTED: &str = include_str!("golden/karate_directed.txt");

#[test]
fn karate_directed() {
    // Strongly connected → every node reaches all others → all local = 1, global = 0.
    assert_close(
        global_reaching_centrality_from_edge_list(KARATE_DIRECTED, true).unwrap(),
        0.0,
        "karate_directed global",
    );
    let got = local_sorted(KARATE_DIRECTED, true);
    assert_eq!(got.len(), 34);
    for (n, v) in &got {
        assert_close(*v, 1.0, &format!("karate_directed[{n}]"));
    }
}

// ── gnp_random_graph(n, p, seed, directed=True) ──────────────────────

const GNP20: &str = include_str!("golden/gnp_n20_seed7.txt");
const GNP30: &str = include_str!("golden/gnp_n30_seed42.txt");

#[test]
fn gnp20_seed7_directed() {
    assert_close(
        global_reaching_centrality_from_edge_list(GNP20, true).unwrap(),
        0.0,
        "gnp20 global",
    );
    let got = local_sorted(GNP20, true);
    assert_eq!(got.len(), 20);
    for (n, v) in &got {
        assert_close(*v, 1.0, &format!("gnp20[{n}]"));
    }
}

#[test]
fn gnp30_seed42_directed() {
    assert_close(
        global_reaching_centrality_from_edge_list(GNP30, true).unwrap(),
        0.06896551724137931,
        "gnp30 global",
    );
    assert_local(
        &local_sorted(GNP30, true),
        &[
            ("0", 1.0),
            ("1", 1.0),
            ("10", 1.0),
            ("11", 1.0),
            ("12", 1.0),
            ("13", 1.0),
            ("14", 1.0),
            ("15", 1.0),
            ("16", 1.0),
            ("17", 1.0),
            ("18", 1.0),
            ("19", 0.0),
            ("2", 1.0),
            ("20", 1.0),
            ("21", 1.0),
            ("22", 1.0),
            ("23", 1.0),
            ("24", 1.0),
            ("25", 1.0),
            ("26", 1.0),
            ("27", 0.0),
            ("28", 1.0),
            ("29", 1.0),
            ("3", 1.0),
            ("4", 1.0),
            ("5", 1.0),
            ("6", 1.0),
            ("7", 1.0),
            ("8", 1.0),
            ("9", 1.0),
        ],
        "gnp30 local",
    );
}

// ── undirected reads (nx average-edge-weight branch) ─────────────────

const KARATE_UNDIRECTED: &str = include_str!("golden/karate_undirected.txt");

#[test]
fn path4_undirected() {
    assert_close(
        global_reaching_centrality_from_edge_list(PATH4, false).unwrap(),
        0.14814814814814822,
        "path4_undirected global",
    );
    assert_local(
        &local_sorted(PATH4, false),
        &[
            ("a", 0.611111111111111),
            ("b", 0.8333333333333334),
            ("c", 0.8333333333333334),
            ("d", 0.611111111111111),
        ],
        "path4_undirected local",
    );
}

#[test]
fn karate_undirected() {
    assert_close(
        global_reaching_centrality_from_edge_list(KARATE_UNDIRECTED, false).unwrap(),
        0.21897765534129174,
        "karate_undirected global",
    );
    assert_local(
        &local_sorted(KARATE_UNDIRECTED, false),
        &[
            ("0", 0.7020202020202021),
            ("1", 0.5808080808080809),
            ("10", 0.4444444444444444),
            ("11", 0.4090909090909091),
            ("12", 0.42424242424242425),
            ("13", 0.5606060606060606),
            ("14", 0.4303030303030303),
            ("15", 0.4303030303030303),
            ("16", 0.33636363636363636),
            ("17", 0.4292929292929293),
            ("18", 0.4303030303030303),
            ("19", 0.5303030303030303),
            ("2", 0.6363636363636364),
            ("20", 0.4303030303030303),
            ("21", 0.4292929292929293),
            ("22", 0.4303030303030303),
            ("23", 0.48585858585858593),
            ("24", 0.4217171717171717),
            ("25", 0.4217171717171717),
            ("26", 0.42272727272727273),
            ("27", 0.5126262626262627),
            ("28", 0.4974747474747475),
            ("29", 0.46565656565656566),
            ("3", 0.5353535353535354),
            ("30", 0.5126262626262627),
            ("31", 0.5858585858585859),
            ("32", 0.6338383838383839),
            ("33", 0.7045454545454546),
            ("4", 0.4444444444444444),
            ("5", 0.45959595959595956),
            ("6", 0.45959595959595956),
            ("7", 0.4974747474747475),
            ("8", 0.5606060606060606),
            ("9", 0.47222222222222227),
        ],
        "karate_undirected local",
    );
}

// ── key-set / structural sanity ──────────────────────────────────────

#[test]
fn single_source_local_matches_batch() {
    let batch = local_reaching_centrality_from_edge_list(DAG6, true).unwrap();
    let g = rsomics_reaching_centrality::io::read_edges(DAG6.as_bytes(), true).unwrap();
    for (i, (_, want)) in batch.iter().enumerate() {
        let one = rsomics_reaching_centrality::reaching::local_one(&g, i as u32);
        assert_close(one, *want, "local_one vs local_all");
    }
}
