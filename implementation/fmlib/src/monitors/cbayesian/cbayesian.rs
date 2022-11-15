use crate::monitors::bayesian::bayesian::Bayesian;

pub struct CBayesian<T: Clone> {
    pub exp_monitor: Bayesian<T>,
    pub exp2_monitor: Bayesian<T>,
    pub delta: f64,
}

impl<T: Clone + PartialEq> CBayesian<T> {
    pub fn init(&mut self, sigma: T) {
        self.exp_monitor.init(sigma.clone());
        self.exp2_monitor.init(sigma.clone());
    }

    pub fn next(&mut self, sigma: T) -> (f64, f64) {
        let e = self.exp_monitor.next(sigma.clone()).to_number();
        let e2 = self.exp2_monitor.next(sigma.clone()).to_number();
        
        
        
        let s = e2 - e*e;
        let error = (s / self.delta).sqrt();
        // println!("-- e: {}, e2: {}, s: {}, error:  {}", e, e2,s,error);
        (e - error, e + error)
    }
}