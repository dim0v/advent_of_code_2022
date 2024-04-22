use std::cmp::{max, min};
use std::iter;

use ahash::AHashMap;

use crate::Stage;

pub fn solve(stage: Stage, input: &str) -> String {
    let g = read_graph(input);
    let (time, split) = match stage {
        Stage::Easy => (30, false),
        Stage::Hard => (26, true),
    };

    search(g, time, split).to_string()
}

fn search(g: Graph, time: i64, split: bool) -> i64 {
    fn search_impl<F>(
        g: &Graph,
        v: usize,
        visited: u64,
        mut time_remaining: i64,
        cache: &mut Vec<Option<i64>>,
        cache_index_provider: &F,
    ) -> i64
    where
        F: Fn(usize, u64, i64) -> usize,
    {
        if time_remaining <= 0 {
            return 0;
        }

        if let Some(cached) = cache[cache_index_provider(v, visited, time_remaining - 1)] {
            return cached;
        }

        let base = g.flow_rates[v] * time_remaining;
        time_remaining -= 1;

        let mut best = 0;

        for i in 0..g.non_zero_flow_cnt {
            if (visited & (1 << i)) != 0 {
                continue;
            }
            best = max(
                best,
                search_impl(
                    g,
                    i,
                    visited | (1 << i),
                    time_remaining - g.get_len(v, i),
                    cache,
                    cache_index_provider,
                ),
            )
        }

        let result = best + base;
        cache[cache_index_provider(v, visited, time_remaining)] = Some(result);

        result
    }

    let visited_max = 1u64 << g.non_zero_flow_cnt;
    let variants_count = match split {
        true => visited_max >> 1,
        false => 1,
    };

    let start = g.aa_idx;
    let mut cache = iter::repeat(None)
        .take((g.non_zero_flow_cnt + 1) * (visited_max * time as u64) as usize)
        .collect();

    let mut result = 0;

    let index_provider = |v, visited, time_remaining| {
        let visited = visited & (visited_max - 1);
        let v = min(v, g.non_zero_flow_cnt);
        let mut idx = v;
        idx = idx * visited_max as usize + visited as usize;
        idx = idx * time as usize + time_remaining as usize;
        
        idx
    };

    for mask in 0..variants_count {
        let total_a = search_impl(&g, start, mask, time, &mut cache, &index_provider);
        let total_b = search_impl(&g, start, !mask, time, &mut cache, &index_provider);

        result = max(result, total_a + total_b);
    }

    result
}

struct Graph {
    aa_idx: usize,
    flow_rates: Vec<i64>,

    lengths: Vec<i64>,
    non_zero_flow_cnt: usize,
}

impl Graph {
    fn new(mut flow_rates_src: Vec<(&str, i64)>, edges: Vec<(&str, Vec<&str>)>) -> Graph {
        let n = flow_rates_src.len();
        flow_rates_src.sort_by_key(|(_, flow)| -*flow);

        let mut index_map = AHashMap::with_capacity(n);
        let mut flow_rates = Vec::with_capacity(n);
        let mut non_zero_flow_cnt = 0;

        for (vert, rate) in flow_rates_src {
            index_map.insert(vert.to_string(), flow_rates.len());
            flow_rates.push(rate);
            non_zero_flow_cnt += (rate > 0) as usize;
        }

        let aa_idx = index_map["AA"];

        let lengths = Vec::from_iter(iter::repeat(i64::MAX).take(n * n));

        let mut result = Graph {
            aa_idx,
            flow_rates,
            lengths,
            non_zero_flow_cnt,
        };

        for (src, destinations) in edges {
            let src = index_map[src];
            for dst in destinations {
                let dst = index_map[dst];
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
    let mut flow_rates = Vec::new();
    let mut edges = Vec::new();

    for line in input.lines() {
        let mut split = line.split(';');
        let (vert, tunnels) = (split.next().unwrap(), split.next().unwrap());

        let vert_id = &vert[6..8];
        let flow: i64 = vert[23..].parse().unwrap();

        let base_tunnels_idx = if tunnels.len() % 2 == 1 { 23 } else { 24 };
        let tunnels = tunnels[base_tunnels_idx..].split(", ").collect();

        flow_rates.push((vert_id, flow));
        edges.push((vert_id, tunnels));
    }

    Graph::new(flow_rates, edges)
}
