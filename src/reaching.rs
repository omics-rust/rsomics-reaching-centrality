use crate::io::Graph;

// Reaching centrality follows Mones, Vicsek & Vicsek, PLoS ONE 7.3 (2012), e33799,
// matching networkx's weight=None, normalized=True defaults. For a directed graph nx
// returns the plain reachable fraction |reachable \ {v}| / (n - 1). For an undirected
// graph nx falls into the average-edge-weight branch, where every shortest path of edge
// length d contributes 1/d, so local(v) = (Σ_{w reachable, w≠v} 1/d(v,w)) / (n - 1).
//
// networkx accumulates these float sums with CPython's built-in sum(), which since
// 3.12 uses Neumaier compensated summation. To stay value-exact we mirror that here
// (Neumaier) — a naive left-fold drifts by a few ULP on the undirected and global sums.

/// Neumaier compensated accumulator, matching CPython 3.12's float `sum()`.
#[derive(Clone, Copy)]
struct Neumaier {
    sum: f64,
    comp: f64,
}

impl Neumaier {
    fn new() -> Self {
        Neumaier {
            sum: 0.0,
            comp: 0.0,
        }
    }

    fn add(&mut self, x: f64) {
        let t = self.sum + x;
        if self.sum.abs() >= x.abs() {
            self.comp += (self.sum - t) + x;
        } else {
            self.comp += (x - t) + self.sum;
        }
        self.sum = t;
    }

    fn total(self) -> f64 {
        self.sum + self.comp
    }
}

/// Reusable BFS scratch so per-source traversals allocate once for the whole run.
struct Bfs {
    visited: Vec<u32>,
    stamp: u32,
    frontier: Vec<u32>,
    next: Vec<u32>,
}

impl Bfs {
    fn new(n: usize) -> Self {
        Bfs {
            visited: vec![0; n],
            stamp: 0,
            frontier: Vec::with_capacity(n),
            next: Vec::with_capacity(n),
        }
    }

    /// Count of nodes reachable from `src` excluding `src` itself.
    fn reachable_count(&mut self, g: &Graph, src: u32) -> usize {
        self.stamp += 1;
        let s = self.stamp;
        self.visited[src as usize] = s;
        self.frontier.clear();
        self.frontier.push(src);
        let mut count = 0usize;
        while !self.frontier.is_empty() {
            self.next.clear();
            for &u in &self.frontier {
                for &w in &g.succ[u as usize] {
                    if self.visited[w as usize] != s {
                        self.visited[w as usize] = s;
                        count += 1;
                        self.next.push(w);
                    }
                }
            }
            std::mem::swap(&mut self.frontier, &mut self.next);
        }
        count
    }

    /// `Σ_{w reachable from src, w≠src} 1 / dist(src, w)`, accumulating each node's
    /// `1/d` in BFS discovery order (matching networkx's `paths.values()` order).
    fn inverse_distance_sum(&mut self, g: &Graph, src: u32) -> f64 {
        self.stamp += 1;
        let s = self.stamp;
        self.visited[src as usize] = s;
        self.frontier.clear();
        self.frontier.push(src);
        let mut acc = Neumaier::new();
        let mut dist = 0u32;
        while !self.frontier.is_empty() {
            dist += 1;
            let inv = 1.0 / f64::from(dist);
            self.next.clear();
            for &u in &self.frontier {
                for &w in &g.succ[u as usize] {
                    if self.visited[w as usize] != s {
                        self.visited[w as usize] = s;
                        acc.add(inv);
                        self.next.push(w);
                    }
                }
            }
            std::mem::swap(&mut self.frontier, &mut self.next);
        }
        acc.total()
    }
}

fn local_from(g: &Graph, bfs: &mut Bfs, src: u32) -> f64 {
    let denom = (g.n() - 1) as f64;
    if g.directed {
        bfs.reachable_count(g, src) as f64 / denom
    } else {
        bfs.inverse_distance_sum(g, src) / denom
    }
}

/// Local reaching centrality of every node, in interned-index (first-seen) order.
pub fn local_all(g: &Graph) -> Vec<f64> {
    let mut bfs = Bfs::new(g.n());
    (0..g.n() as u32)
        .map(|v| local_from(g, &mut bfs, v))
        .collect()
}

/// Local reaching centrality of a single node index.
pub fn local_one(g: &Graph, v: u32) -> f64 {
    local_from(g, &mut Bfs::new(g.n()), v)
}

/// Global reaching centrality: `Σ_v (C_max − C_local(v)) / (n − 1)`.
pub fn global(g: &Graph) -> f64 {
    let lrc = local_all(g);
    let max = lrc.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mut acc = Neumaier::new();
    for &c in &lrc {
        acc.add(max - c);
    }
    acc.total() / (g.n() - 1) as f64
}
