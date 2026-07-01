use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use rsomics_common::{Result, RsomicsError};

/// Integer-interned adjacency. Labels carry first-seen order; `succ[u]` lists
/// the out-neighbours of `u`. For an undirected read the reverse edge is added
/// too, so `succ` doubles as the full adjacency.
pub struct Graph {
    pub labels: Vec<String>,
    pub succ: Vec<Vec<u32>>,
    pub directed: bool,
}

impl Graph {
    pub fn n(&self) -> usize {
        self.labels.len()
    }
}

/// Parse an edge list from a file (or stdin for `None`/`-`), `nx.read_edgelist`
/// style — see [`read_edges`].
pub fn read_edgelist(path: Option<&Path>, directed: bool) -> Result<Graph> {
    let reader: Box<dyn BufRead> = match path {
        None => Box::new(BufReader::new(std::io::stdin())),
        Some(p) if p == Path::new("-") => Box::new(BufReader::new(std::io::stdin())),
        Some(p) => Box::new(BufReader::new(File::open(p).map_err(|e| {
            RsomicsError::Io(std::io::Error::new(
                e.kind(),
                format!("{}: {e}", p.display()),
            ))
        })?)),
    };
    read_edges(reader, directed)
}

/// Parse an edge list from any reader: `#`/blank lines skipped, first two
/// whitespace tokens per line are `u v`, extras ignored, self-loops dropped,
/// parallel edges collapsed. `u v` means the edge `u -> v`; when `directed` is
/// false the reverse `v -> u` is added as well.
pub fn read_edges<R: BufRead>(reader: R, directed: bool) -> Result<Graph> {
    let mut labels: Vec<String> = Vec::new();
    let mut index: HashMap<String, u32> = HashMap::new();
    let mut raw: Vec<(u32, u32)> = Vec::new();

    for (lineno, line) in reader.lines().enumerate() {
        let line = line.map_err(RsomicsError::Io)?;
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let mut tok = t.split_ascii_whitespace();
        let u_str = tok.next().unwrap();
        let v_str = tok.next().ok_or_else(|| {
            RsomicsError::InvalidInput(format!(
                "line {}: expected two node labels, got one",
                lineno + 1
            ))
        })?;
        if u_str == v_str {
            continue;
        }
        let u = intern(&mut labels, &mut index, u_str);
        let v = intern(&mut labels, &mut index, v_str);
        raw.push((u, v));
    }

    let n = labels.len();
    let mut succ: Vec<Vec<u32>> = vec![Vec::new(); n];
    let mut seen: Vec<HashSet<u32>> = vec![HashSet::new(); n];
    for (u, v) in raw {
        if seen[u as usize].insert(v) {
            succ[u as usize].push(v);
        }
        if !directed && seen[v as usize].insert(u) {
            succ[v as usize].push(u);
        }
    }

    Ok(Graph {
        labels,
        succ,
        directed,
    })
}

fn intern(labels: &mut Vec<String>, index: &mut HashMap<String, u32>, s: &str) -> u32 {
    if let Some(&id) = index.get(s) {
        return id;
    }
    let id = labels.len() as u32;
    labels.push(s.to_owned());
    index.insert(s.to_owned(), id);
    id
}
