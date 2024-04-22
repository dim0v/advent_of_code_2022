use std::cmp::max;
use std::iter;

use ahash::AHashMap;

use crate::Stage;

pub fn solve(stage: Stage, input: &str) -> String {
    let g = read_graph(input);

    search(&g, "AA").to_string()
}

fn search(g: &Graph, start: &str) -> i64 {
    fn search_impl(g: &Graph, v: usize, visited: u64, mut time_remaining: i64) -> i64 {
        if time_remaining <= 0 {
            return 0;
        }
        let base = g.flow_rates[v] * time_remaining;
        time_remaining -= 1;

        let mut best = 0;

        for i in 0..g.vertex_count() {
            if (visited & (1 << i)) != 0 {
                continue;
            }
            best = max(
                best,
                search_impl(g, i, visited | (1 << i), time_remaining - g.get_len(v, i)),
            )
        }

        best + base
    }

    let mut visited = 0;
    for r in g.flow_rates.iter().rev() {
        visited <<= 1;
        visited |= (*r <= 0) as u64;
    }
    search_impl(g, g.index_map[start], visited, 30)
}

struct Graph<'a> {
    index_map: AHashMap<&'a str, usize>,
    flow_rates: Vec<i64>,

    lengths: Vec<i64>,
}

impl<'a> Graph<'a> {
    fn new(flow_rates_src: AHashMap<&'a str, i64>, edges: AHashMap<&str, Vec<&str>>) -> Graph<'a> {
        let n = flow_rates_src.len();
        let mut index_map = AHashMap::with_capacity(n);
        let mut flow_rates = Vec::with_capacity(n);

        for (vert, rate) in flow_rates_src {
            index_map.insert(vert, flow_rates.len());
            flow_rates.push(rate);
        }

        let lengths = Vec::from_iter(iter::repeat(i64::MAX).take(n * n));

        let mut result = Graph {
            index_map,
            flow_rates,
            lengths,
        };

        for (src, destinations) in edges {
            let src = result.index_map[src];
            for dst in destinations {
                let dst = result.index_map[dst];
                result.set_len(src, dst, 1);
            }
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    let current = result.get_len(i, j);
                    let updated = result
                        .get_len(i, k)
                        .checked_add(result.get_len(k, j))
                        .unwrap_or(current);
                    if current > updated {
                        result.set_len(i, j, updated)
                    }
                }
            }
        }

        result
    }

    fn vertex_count(&self) -> usize {
        self.flow_rates.len()
    }

    fn get_len(&self, from: usize, to: usize) -> i64 {
        self.lengths[from * self.vertex_count() + to]
    }

    fn set_len(&mut self, from: usize, to: usize, new_len: i64) {
        let i = from * self.vertex_count() + to;
        self.lengths[i] = new_len
    }
}

fn read_graph(input: &str) -> Graph {
    let mut flow_rates = AHashMap::new();
    let mut edges = AHashMap::new();

    for line in input.lines() {
        let mut split = line.split(';');
        let (vert, tunnels) = (split.next().unwrap(), split.next().unwrap());

        let vert_id = &vert[6..8];
        let flow: i64 = vert[23..].parse().unwrap();

        let base_tunnels_idx = if tunnels.len() % 2 == 1 { 23 } else { 24 };
        let tunnels = tunnels[base_tunnels_idx..].split(", ").collect();

        flow_rates.insert(vert_id, flow);
        edges.insert(vert_id, tunnels);
    }

    Graph::new(flow_rates, edges)
}
