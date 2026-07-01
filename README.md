# rsomics-reaching-centrality

Global and local **reaching centrality** (Mones–Vicsek–Vicsek hierarchy measure)
for directed and undirected graphs. A value-exact Rust port of NetworkX's
`nx.global_reaching_centrality` and `nx.local_reaching_centrality`
(`weight=None, normalized=True`), computed over an integer-interned adjacency
with a reused per-source BFS instead of NetworkX's per-node Python shortest-path
dicts.

One cohesive tool: the same operation (reaching centrality) at two scopes,
selected with `--scope`.

## Install

```
cargo install rsomics-reaching-centrality
```

## Usage

Input is an edge list on stdin (or a file), one `u v` per line meaning the edge
`u -> v`. `#` comments and blank lines are ignored; self-loops are dropped and
parallel edges collapse. Node labels are arbitrary strings. The graph is
**directed by default** (reaching centrality is inherently directional); pass
`--undirected` to read each edge both ways.

```
# Global reaching centrality (single float)
printf 'a b\nb c\nc d\n' | rsomics-reaching-centrality --scope global
# 0.6666666666666666

# Local reaching centrality per node (label-sorted, node<TAB>value)
printf 'a b\nb c\nc d\n' | rsomics-reaching-centrality --scope local
# a	1
# b	0.6666666666666666
# c	0.3333333333333333
# d	0

# From a file, undirected, JSON envelope
rsomics-reaching-centrality --scope local --undirected --json edges.txt
```

Common flags (`-t/--threads`, `-q/--quiet`, `--json`) are shared across all
rsomics-* tools.

## What it computes

For `weight=None, normalized=True` (the NetworkX defaults):

- **Directed** — `local(v) = |{w : w reachable from v, w ≠ v}| / (n − 1)`, the
  fraction of the graph reachable from `v`.
- **Undirected** — NetworkX's average-edge-weight branch: each shortest path of
  edge-length `d` contributes `1/d`, so
  `local(v) = ( Σ_{w reachable, w ≠ v} 1/d(v,w) ) / (n − 1)`.
- **Global** — `Σ_v (C_max − C_local(v)) / (n − 1)` where `C_max = max_v C_local(v)`,
  accumulated in node-insertion order to match NetworkX bit-for-bit.

## Origin

This crate is an independent Rust reimplementation of NetworkX's
`networkx.algorithms.centrality.reaching` module, based on:

- The published method: Mones, Enys, Lilla Vicsek, and Tamás Vicsek,
  "Hierarchy Measure for Complex Networks," *PLoS ONE* 7.3 (2012): e33799,
  https://doi.org/10.1371/journal.pone.0033799
- The NetworkX 3.6.1 source (BSD-3-Clause), which is a permissively licensed
  pure-Python upstream and may be read and cited.
- Black-box value comparison against NetworkX 3.6.1: goldens are generated once
  from NetworkX and committed as hardcoded constants / edge lists; no Python is
  invoked at test time.

License: MIT OR Apache-2.0.

Upstream credit: NetworkX (https://networkx.org/, BSD-3-Clause).
