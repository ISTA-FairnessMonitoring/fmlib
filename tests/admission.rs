#[cfg(test)]
mod tests {
  use fmlib::util;

  #[test]
  fn test_admission_small() {
    let mc = util::markov_chain_admission_small();

    for v in mc.take(10) {
      println!("{:?}", v);
    }
  }

    #[test]
  fn test_admission_large() {
    let mc = util::markov_chain_admission_large();

    for v in mc.take(10) {
      println!("{:?}", v);
    }
  }
}