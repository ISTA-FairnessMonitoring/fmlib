#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use fmlib::envs::{
        lending::lending::Lending,
        dyn_lending::markov_chain::DynamicLendingMarkovChain
    };

    #[test]
    fn test_dyn_lending() {
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

        let mc = DynamicLendingMarkovChain::new(lending);
        for (i, s) in mc.take(100).enumerate() {
            println!("{}: {:?}", i, s.0);
        }
    }
}