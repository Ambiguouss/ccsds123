pub struct CentralDifferenceCache<const N:usize> {
    values: [i32;N],
    write_pos: usize,
    len: usize,
}

impl<const N:usize> CentralDifferenceCache<N>{
    pub fn new() -> Self {
        Self { values: [0; N], write_pos: 0, len: 0 }
    }

    pub fn push(&mut self, d: i32) {
        self.values[self.write_pos] = d;
        self.write_pos = (self.write_pos + 1) % N;
        self.len = (self.len + 1).min(N);
    }

    pub fn get(&self, back: usize) -> i32 {
        if back >= self.len {
            panic!("index out of bounds in cache!");
        }
        let idx = (self.write_pos + N - 1 - back) % N;
        self.values[idx]
    }
    
    pub fn recent(&self) -> impl Iterator<Item = i32> + '_ {
        (0..self.len).map(move |back| self.get(back))
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cache_is_empty() {
        let cache: CentralDifferenceCache<3> = CentralDifferenceCache::new();
        assert_eq!(cache.recent().collect::<Vec<_>>(), Vec::<i32>::new());
    }

    #[test]
    fn push_one_then_get_most_recent() {
        let mut cache: CentralDifferenceCache<3> = CentralDifferenceCache::new();
        cache.push(10);
        assert_eq!(cache.get(0), 10);
    }

    #[test]
    fn recent_returns_most_recent_first() {
        let mut cache: CentralDifferenceCache<3> = CentralDifferenceCache::new();
        cache.push(1);
        cache.push(2);
        cache.push(3);
        assert_eq!(cache.recent().collect::<Vec<_>>(), vec![3, 2, 1]);
    }

    #[test]
    fn cache_below_capacity_only_returns_pushed_values() {
        let mut cache: CentralDifferenceCache<5> = CentralDifferenceCache::new();
        cache.push(1);
        cache.push(2);
        // only 2 pushed, capacity is 5 — should only return 2 values, not 5
        assert_eq!(cache.recent().collect::<Vec<_>>(), vec![2, 1]);
    }

    #[test]
    fn cache_evicts_oldest_when_full() {
        let mut cache: CentralDifferenceCache<3> = CentralDifferenceCache::new();
        cache.push(1);
        cache.push(2);
        cache.push(3);
        cache.push(4); // should evict 1
        assert_eq!(cache.recent().collect::<Vec<_>>(), vec![4, 3, 2]);
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn get_beyond_len_panics() {
        let mut cache: CentralDifferenceCache<3> = CentralDifferenceCache::new();
        cache.push(1);
        cache.get(1); // only 1 valid entry (index 0), asking for index 1 should panic
    }

    #[test]
    fn wraparound_indexing_is_correct_across_multiple_evictions() {
        // stress the modular arithmetic specifically — push more than 2x capacity
        let mut cache: CentralDifferenceCache<3> = CentralDifferenceCache::new();
        for v in 1..=7 {
            cache.push(v);
        }
        // should hold the 3 most recent: 7, 6, 5
        assert_eq!(cache.recent().collect::<Vec<_>>(), vec![7, 6, 5]);
    }
}