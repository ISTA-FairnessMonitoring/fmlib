#[cfg(test)]
mod tests {
  use fmlib::util;

  #[test]
  fn test_lending_small() {
    let mc = util::markov_chain_lending_small();

    for v in mc.take(10) {
      println!("{:?}", v);
    }
  }

  #[test]
  fn test_lending_large() {
    let mc = util::markov_chain_lending_large();

    for v in mc.take(10) {
      println!("{:?}", v);
    }
  }
}