#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::vec;
    use fmlib::envs::lending::{
        lending::*,
        mapper::*,
        memoryless::*,
    };

    #[test]
    fn build_markov_chain_tiny() {
        let lending = Lending {
            group_count: 2,
            credit_score: 1,
            group_population: vec![2, 2],
            payback_prob: HashMap::from([(0, 0.6)]),
            init_credit: vec![vec![2], vec![2]],
            policy: HashMap::from(
                [
                    ((0, 0), 0.5),
                    ((1, 0), 0.4),
                ]
            ),
        };

        let mut visitor = LendingMarkovChainMapper::new(lending);
        let (mc, label_map) = visitor.map();
        assert_eq!(mc.m.len(), 11);
        for (k, v) in label_map {
            println!("{:?} <- {:?}", v, k);
        }

        for (i, row) in mc.m.iter().enumerate() {
            println!("{i}: {:?}", row);
        }

    }

    #[test]
    fn build_markov_chain() {
        let lending = Lending {
            group_count: 2,
            credit_score: 2,
            group_population: vec![1, 1],
            payback_prob: HashMap::from([(0, 0.4), (1, 0.7)]),
            init_credit: vec![vec![0, 1], vec![1, 0]],
            policy: HashMap::from(
                [
                    ((0, 0), 0.5),
                    ((0, 1), 0.5),
                    ((1, 0), 0.4),
                    ((1, 1), 0.6),
                ]
            )
        };

        let mut visitor = LMCMapper::new(lending);
        let (mc, _) = visitor.map();
        assert_eq!(mc.m.len(), 44);
    }

    #[test]
    fn build_markov_chain_memless() {
        let payback_prob = (0..10).map(
            |x| (x, ((x + 5) % 10) as f64 / 10.0)
        ).collect::<HashMap<_, _>>();
        
        let mut policy = HashMap::<(i32, i32), f64>::new();
        (0..10).for_each(|i| {
            policy.insert((0, i), (i + 1) as f64 / 10.0);
        });
        (0..10).for_each(|i| {
            policy.insert((1, i), ((i + 8) % 10) as f64 / 10.0);
        });

        let init_credit = vec![
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            vec![5, 5, 1, 1, 1, 1, 1, 2, 2, 1]
        ];

        let lending = Lending {
            group_count: 2,
            credit_score: 10,
            group_population: vec![20, 20],
            payback_prob,
            init_credit,
            policy,
        };
        
        let mc = Lmmc::new(lending);
        for (i, s) in mc.take(100).enumerate() {
            println!("{}: {:?}", i, s);
        }
    }
}