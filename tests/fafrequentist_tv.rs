#[cfg(test)]
mod tests {
    use fmlib::monitors::fafrequentist::tv::*;

    #[test]
    fn test_tv_frequency() {
        let v = Tv::Frequency(0,
            Frequency::<i32> {
                a: 1, a_prime: 2, count: 0, estimate: 0.0, delta: 0.05
            }
        );

        let result = v.eval(&vec![]);
        assert_eq!(result.delta, 0.05);
    }    
}