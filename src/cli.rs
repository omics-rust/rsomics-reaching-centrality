use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, ValueEnum};
use serde::Serialize;

use rsomics_common::{run, CommonFlags, Result, RsomicsError, ToolMeta};

use rsomics_reaching_centrality::{io, reaching};

pub const META: ToolMeta = ToolMeta {
    name: env!("CARGO_PKG_NAME"),
    version: env!("CARGO_PKG_VERSION"),
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Scope {
    /// Single global reaching centrality of the whole graph.
    Global,
    /// Per-node local reaching centrality.
    Local,
}

/// Reaching centrality (Mones-Vicsek-Vicsek 2012), value-exact with networkx.
///
/// Reads an edge list (`u v` per line, `u -> v`) from FILE or stdin (`-`);
/// `#` comments and blank lines are ignored, self-loops dropped, parallel
/// edges collapsed. The graph is directed by default (reaching centrality is
/// inherently directional); pass `--undirected` for an undirected read.
///
/// `--scope global` prints one float. `--scope local` prints `node<TAB>value`
/// per node in label-sorted order. `--json` emits the rsomics-common envelope
/// wrapping a structured result.
#[derive(Parser, Debug)]
#[command(name = "rsomics-reaching-centrality", version, about, long_about = None)]
pub struct Cli {
    /// Which reaching centrality to compute.
    #[arg(long = "scope", value_enum, default_value_t = Scope::Global)]
    pub scope: Scope,

    /// Treat the edge list as undirected (adds the reverse of each edge).
    #[arg(
        long = "undirected",
        conflicts_with = "directed",
        default_value_t = false
    )]
    pub undirected: bool,

    /// Treat the edge list as directed (the default; explicit override of `--undirected`).
    #[arg(long = "directed", default_value_t = false)]
    pub directed: bool,

    /// Edge list file (`-` or omitted reads stdin).
    #[arg(value_name = "EDGELIST")]
    pub edgelist: Option<PathBuf>,

    #[command(flatten)]
    pub common: CommonFlags,
}

#[derive(Serialize)]
#[serde(untagged)]
enum Out {
    Global {
        scope: &'static str,
        directed: bool,
        global_reaching_centrality: f64,
    },
    Local {
        scope: &'static str,
        directed: bool,
        nodes: Vec<String>,
        local_reaching_centrality: Vec<f64>,
    },
}

impl Cli {
    pub fn run(self) -> ExitCode {
        let common = self.common.clone();
        run(&common, META, || self.execute(&common))
    }

    fn execute(self, common: &CommonFlags) -> Result<Out> {
        let directed = !self.undirected;
        let g = io::read_edgelist(self.edgelist.as_deref(), directed)?;

        match self.scope {
            Scope::Global => {
                let grc = reaching::global(&g);
                if !common.json {
                    let mut w = BufWriter::new(std::io::stdout().lock());
                    writeln!(w, "{grc}").map_err(RsomicsError::Io)?;
                    w.flush().map_err(RsomicsError::Io)?;
                }
                Ok(Out::Global {
                    scope: "global",
                    directed,
                    global_reaching_centrality: grc,
                })
            }
            Scope::Local => {
                let vals = reaching::local_all(&g);
                let mut rows: Vec<(String, f64)> = g.labels.iter().cloned().zip(vals).collect();
                rows.sort_by(|a, b| a.0.cmp(&b.0));

                if !common.json {
                    let mut w = BufWriter::new(std::io::stdout().lock());
                    for (label, val) in &rows {
                        writeln!(w, "{label}\t{val}").map_err(RsomicsError::Io)?;
                    }
                    w.flush().map_err(RsomicsError::Io)?;
                }
                let (nodes, local): (Vec<String>, Vec<f64>) = rows.into_iter().unzip();
                Ok(Out::Local {
                    scope: "local",
                    directed,
                    nodes,
                    local_reaching_centrality: local,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    #[test]
    fn cli_debug_assert() {
        super::Cli::command().debug_assert();
    }
}
