/*
 *   Copyright (c) 2023 
 *   All rights reserved.
 */

// TODO :: THIS IS UGLY CLEAN THIS UP


use std::collections::hash_set::HashSet;
use std::cell::RefCell;
use std::ops::{Add,Sub,Mul,Div};
use std::cmp::Eq;
use std::hash::{Hash,Hasher};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ValueOp {
    ADD, SUB, DIV, MUL, POW, NONE
}

impl ValueOp {
    pub const fn sign(&self) -> char {
        match *self {
            Self::ADD => '+',
            Self::SUB => '-',
            Self::DIV => '/',
            Self::MUL => '*',
            Self::POW => '^',
            _ => ' '
        }
    }
}

impl Default for ValueOp {
    fn default() -> Self { ValueOp::NONE }

}

#[derive(Debug, Clone, Default)]
pub struct BackPropInfo {
    result_grad: f64,
    op_from: ValueOp,
}

#[derive(Debug, Clone, Default)]
pub struct ValueOperation {
    operator: ValueOp,
    loperand: RefCell<Value>, // from the POV of 'self'
    roperand: RefCell<Value>,
    output: Option<RefCell<Value>>
}

#[derive(Debug, Clone)]
pub struct Value {
    data: f64,
    grad: f64,
    prev: Option<(RefCell<Self>, RefCell<Self>)>,
    op: ValueOperation
}

impl ValueOperation {

    pub fn new(operator: ValueOp, lhs: RefCell<Value>, rhs: RefCell<Value>) -> Self {
        Self {
            operator,
            loperand: lhs,
            roperand: rhs,
            output: None
        }
    }

    pub fn new_calc(operator: ValueOp, lhs: RefCell<Value>, rhs: RefCell<Value>) -> Self {
        let n = Self::new(operator, lhs, rhs);
        return n.calc_output()
    }

    pub fn calc_output(&self) -> ValueOperation {
        let lhs: f64 = self.loperand.data;
        let rhs: f64 = self.roperand.data;

        let r = match self.operator {
            ValueOp::ADD => lhs + rhs,
            ValueOp::SUB => lhs - rhs,
            ValueOp::DIV => lhs / rhs,
            ValueOp::MUL => lhs * rhs,
            ValueOp::POW => lhs.pow(2),
            _ => 0.0
        };

        let rv = Value::new(r, &[lhs, rhs], None);
        Self {
            output: RefCell::new(rv),
            ..self
        }
    }

    pub fn operands(&self) -> (RefCell<Value>, RefCell<Value>) {
        (self.loperand, self.roperand)
    }

    pub fn backwards(&mut self) {
        let (lhs, rhs) = self.operands();
        let out = self.output;
        match self.operator {
            ValueOp::ADD => {
                lhs.grad += out.grad;
                rhs.grad += out.grad;
            },
            ValueOp::SUB => {
                lhs.grad -= out.grad;
                rhs.grad -= out.grad;
            },
            ValueOp::DIV => {
                // TODO :: Implement
                // lhs.grad *= rhs.grad
            },
            ValueOp::MUL => {
                lhs.grad += rhs.data * out.grad;
                rhs.grad += lhs.data * out.grad;
            },
            ValueOp::POW => lhs.pow(2),
            _ => 0.0
        }


    }

}

fn build_topo(root: &Value) -> Vec<RefCell<Value>> {

    let topo: Vec<RefCell<Value>> = Vec::new();
    let mut visited: HashSet<RefCell<Value>> = HashSet::new();
    _build_topo(root, topo, visited);
    return topo;

    fn _build_topo(root: &Value, mut topo: Vec<RefCell<Value>>, mut visited: HashSet<RefCell<Value>>) {
        if !visited.contains(root) {
            visited.insert(root);
            for child in root.prev {
                _build_topo(child, topo, visited);
            }
            topo.insert(root);
        }
    }
}


impl Default for Value {
    fn default() -> Self {
        Self::new(0.0, None)
    }
}


impl Value {

    pub fn new(n: f64, operation: Option<ValueOperation>) -> Self  {
        Self {
            data: n,
            grad: 0.0,
            prev: operation.unwrap_or_default().operands(),
            op: operation
        }
    }

    pub fn with_op(n: f64, operation: ValueOperation) -> Self {
        Self::new(n, operation)
    }

    pub fn add(&self, other: &mut Self) -> Self {
        let op = ValueOperation::new_calc(ValueOp::ADD, self, other);
        let n = op.output.data;
        Value::new(n, op)
    }

    pub fn sub(&self, other: &mut Self) -> Self {
        let op = ValueOperation::new_calc(ValueOp::SUB, self, other);
        let n = op.output.data;
        Value::new(n, op)
    }

    pub fn div(&self, other: &mut Self) -> Self {
        let op = ValueOperation::new_calc(ValueOp::DIV, self, other);
        let n = op.output.data;
        Value::new(n, op)
    }

    pub fn mul(&self, other: &mut Self) -> Self {
        let op = ValueOperation::new_calc(ValueOp::MUL, self, other);
        let n = op.output.data;
        Value::new(n, op)
    }

    pub fn backwards(&mut self) {
        let topo = build_topo(self);

        self.grad = 1.0;
        for v in topo.iter().reverse() {
            v.op.backwards();
        }

    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
            && self.grad == other.grad
            && self.op == other.op
    }
}
impl Eq for Value {}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let add = (self.data as u64) + (self.grad as u64);
        let mul = (self.data as u64) * (self.grad as u64);
        add.hash(state);
        mul.hash(state);
    }
}

// pub fn backwards_prop(v: &mut Value) -> () {
//     let other = v.;
//     match v.op {
//
// }

//
// impl Add for Value {
//     type Output = Self;
//     fn add(self, other: &Self) -> Self {
//         let mut children = HashSet::new();
//         children.insert(RefCell::new(self));
//         children.insert(RefCell::new(other));
//         let mut res: Value = Value::new(self.data + other.data, children, ValueOp::ADD);
//         res.backward = || {
//             self.grad += res.grad;
//             other.grad += res.grad;
//         };
//         return res;
//
//     }
// }


const NOOP: fn() -> () = || {};
