#[cfg(test)]
mod tests {
    use std::{collections::HashMap, vec};
    use fmlib::envs::mc::*;

    #[test]
    fn markov_chain_2_state() {
        let mc = MarkovChain::new(
            vec![
                vec![(1, 1.0)],
                vec![(0, 1.0)],
            ],
            vec![(0, 1.0)]
        );

        let path: Vec<i32> = mc.take(3).collect();
        assert_eq!(path[0], 0);
        assert_eq!(path[1], 1);
        assert_eq!(path[2], 0);
    }

    #[test]
    fn markov_chain_multi_state() {
        let mc = MarkovChain::new(
            vec![
                vec![(1, 0.5), (2, 0.5)],
                vec![(2, 1.0)],
                vec![(0, 1.0)],
            ],
            vec![(0, 1.0)]
        );

        let mut h = HashMap::<i32, i32>::new();
        for v in mc.take(1000) {
            match h.get(&v) {
                None => h.insert(v, 1),
                Some(x) => h.insert(v, x+1)
            };
        }

        assert!((h[&0]-400).abs() < 50);
        assert!((h[&1]-200).abs() < 50);
        assert!((h[&2]-400).abs() < 50);
    }
}
