use crate::envs::lending::lv::*;

pub trait Visitor {
    fn visit_env(&mut self, v: &EnvVertex);
    fn visit_sample(&mut self, v: &SampleVertex);
    fn visit_decision(&mut self, v: &DecisionVertex);
    fn visit_payback(&mut self, v: &PaybackVertex);
}
