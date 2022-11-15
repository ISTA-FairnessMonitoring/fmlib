#[cfg(test)]
mod tests {
    use fmlib::util;

    #[test]
    fn test_admission_memoryless() {
        let mc = util::markov_chain_admission_small();

        for v in mc.take(100) {
            println!("{:?}", v);
        }
    }
}