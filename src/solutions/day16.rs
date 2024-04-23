use std::cmp::max;
use std::iter;
use std::process::id;

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
    fn search_impl(
        g: &Graph,
        v: usize,
        visited: u64,
        mut time_remaining: i64,
        cache: &mut Cache,
    ) -> i64 {
        if time_remaining <= 0 {
            return 0;
        }

        if let Some(cached) = cache.get(v, visited, time_remaining) {
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
                ),
            )
        }

        let result = best + base;
        cache.set(v, visited, time_remaining + 1, result);

        result
    }

    let visited_max = 1u64 << g.non_zero_flow_cnt;
    let variants_count = match split {
        true => visited_max >> 1,
        false => 1,
    };

    let start = g.aa_idx;
    let mut cache = Cache::new(&g, time);

    let mut result = 0;

    for mask in 0..variants_count {
        let total_a = search_impl(&g, start, mask, time, &mut cache);
        let total_b = search_impl(&g, start, !mask, time, &mut cache);

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

struct Cache {
    data: Vec<i64>,
    max_mask: usize,
    max_time: usize,
    max_v_idx: usize,
}

impl Cache {
    fn new(g: &Graph, max_time: i64) -> Cache {
        let max_mask = 1usize << g.non_zero_flow_cnt;
        let max_v_idx = g.non_zero_flow_cnt + 1;
        let max_time = max_time as usize;

        Cache {
            data: iter::repeat(-1)
                .take(max_mask * max_time * (max_v_idx + 1))
                .collect(),
            max_mask,
            max_time,
            max_v_idx,
        }
    }

    fn get_idx(&self, src_idx: usize, dst_mask: u64, time_remaining: i64) -> usize {
        let time_remaining = time_remaining as usize - 1;
        let dst_mask = dst_mask as usize & (self.max_mask - 1);

        let mut idx = dst_mask;
        idx = idx * self.max_time + time_remaining;
        idx = idx * (self.max_v_idx + 1) + src_idx.clamp(0, self.max_v_idx);

        idx
    }

    fn get(&self, src_idx: usize, dst_mask: u64, time_remaining: i64) -> Option<i64> {
        let idx = self.get_idx(src_idx, dst_mask, time_remaining);
        if self.data[idx] < 0 {
            None
        } else {
            Some(self.data[idx])
        }
    }

    fn set(&mut self, src_idx: usize, dst_mask: u64, time_remaining: i64, value: i64) {
        let idx = self.get_idx(src_idx, dst_mask, time_remaining);
        self.data[idx] = value;
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
