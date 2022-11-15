// Definition of the MC for the admission problem

#[derive(Clone)]
pub struct Admission {
    pub score: i32,
    pub threshold: i32,
    pub label_threshold: i32,
    pub cost: Vec<i32>,
    pub change_prob: Vec<f64>,
}
