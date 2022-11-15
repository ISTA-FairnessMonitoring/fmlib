#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use fmlib::monitors::frequentist::tv::*;
    
    #[test]
    fn test_tv_frequency() {
        let mut v: Tv<i32> = Tv::Frequency(0, Frequency::<i32> { a: 1, a_prime: 2, count: 0 });
        let update = v.update(-1, 1.0);
        assert_eq!(update, Some(1.0));
    }

    #[test]
    fn test_tv_sum() {
        let mut v: Tv<i32> = Tv::Sum(
            0, Binary { left: 1, right: 2, q_left: VecDeque::new(), q_right: VecDeque::new() }
        );
        
        let _ = v.update(1, 1.0);
        let update = v.update(2, 1.0);
        assert_eq!(update, Some(2.0));
    }

    #[test]
    fn test_tv_sum_unary() {
        let mut v: Tv<i32> = Tv::SumUnary(0, -1, 5.0);
        
        let update = v.update(1, 1.0);
        assert_eq!(update, Some(6.0));
    }

    #[test]
    fn test_tv_sub() {
        let mut v: Tv<i32> = Tv::Subtract(
            0, Binary { left: 1, right: 2, q_left: VecDeque::new(), q_right: VecDeque::new() }
        );
        
        let _ = v.update(1, 1.0);
        let update = v.update(2, 2.0);
        println!("{:?}", update);
        assert_eq!(update, Some(-1.0));
    }

    #[test]
    fn test_tv_sub_unary() {
        let mut v: Tv<i32> = Tv::SubtractUnary(0, -1, 5.0);
        
        let update = v.update(1, 1.0);
        assert_eq!(update, Some(-4.0));
    }
}