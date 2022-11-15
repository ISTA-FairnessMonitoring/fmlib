use std::rc::Rc;
use crate::envs::lending::visitor::Visitor;

pub type Env = Vec<Vec<i32>>;

pub type Sample = (i32, i32);

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Decision {
    Reject = 0,
    Accept,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Payback {
    Fail = 0,
    Success,
}

pub type Lv = LendingVertex;

pub type LendingVertex = (
    Option<Env>,
    Option<Sample>,
    Option<Decision>,
    Option<Payback>
);

pub trait LendingAbstractVertex {
    fn accept(&self, v: &mut dyn Visitor);
    fn to_vertex(&self) -> Rc<Lv>;
}

#[derive(Clone)]
pub struct EnvVertex {
    pub env: Env,
    vertex: Rc<Lv>,
}

impl EnvVertex {
    pub fn new(env: Env) -> Self {
        Self {
            env: env.clone(),
            vertex: Rc::from((Some(env.clone()), None, None, None))
        }
    }
}

impl LendingAbstractVertex for EnvVertex {
    fn accept(&self, v: &mut dyn Visitor) {
        v.visit_env(self)
    }
    
    fn to_vertex(&self) -> Rc<Lv> {
        self.vertex.clone()
    }
}

#[derive(Clone)]
pub struct SampleVertex {
    pub env: Env,
    pub sample: Sample,
    vertex: Rc<Lv>
}

impl SampleVertex {
    pub fn new(env: Env, sample: Sample) -> Self {
        Self {
            env: env.clone(),
            sample,
            vertex: Rc::from((Some(env.clone()), Some(sample), None, None))
        }
    }
}

impl<'a> LendingAbstractVertex for SampleVertex {
    fn accept(&self, v: &mut dyn Visitor) { v.visit_sample(self) }
    
    fn to_vertex(&self) -> Rc<Lv> {
        self.vertex.clone()
    }
}

#[derive(Clone)]
pub struct DecisionVertex {
    pub env: Env,
    pub sample: Sample,
    pub decision: Decision,
    vertex: Rc<Lv>
}

impl DecisionVertex {
    pub fn new(env: Env, sample: Sample, status: Decision) -> Self {
        Self {
            env: env.clone(),
            sample,
            decision: status,
            vertex: Rc::from((Some(env.clone()), Some(sample), Some(status), None))
        }
    }
}

impl LendingAbstractVertex for DecisionVertex {
    fn accept(&self, v: &mut dyn Visitor) { v.visit_decision(self) }

    fn to_vertex(&self) -> Rc<Lv> {
        self.vertex.clone()
    }
}

#[derive(Clone)]
pub struct PaybackVertex {
    pub env: Env,
    pub sample: Sample,
    pub _decision: Decision,
    pub payback: Payback,
    vertex: Rc<Lv>
}

impl PaybackVertex {
    pub fn new(env: Env, sample: Sample, decision: Decision, payback: Payback) -> Self {
        Self {
            env: env.clone(),
            sample,
            _decision: decision,
            payback,
            vertex: Rc::from((Some(env.clone()), Some(sample), Some(decision), Some(payback)))
        }
    }
}

impl LendingAbstractVertex for PaybackVertex {
    fn accept(&self, v: &mut dyn Visitor) { v.visit_payback(self) }

    fn to_vertex(&self) -> Rc<Lv> {
        self.vertex.clone()
    }
}
