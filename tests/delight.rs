#[cfg(test)]
mod tests {
use std::collections::HashSet;
// TODO: add branch instructions. How to model?
use egg::*;
    define_language! {
        enum Bits {
            Num(i64),
            "&" = And([Id; 2]),
            "!" = Not(Id),
            "|" = Or([Id;2]),
            "+" = Add([Id;2]),
            "-" = Mul([Id;2]),
            "*" = Sub([Id;2]),
            "/" = Div([Id;2]),
            "==" = BEq([Id;2]),
            "!=" = BNEq([Id;2]),
            "<" = BLt([Id;2]),
            "<=" = BLe([Id;2]),
            ">" = BGt([Id;2]),
            ">=" = BGe([Id;2]),
            "shl" = Shl([Id;2]),
            "shr" = Shr([Id;2]),
            // does not work.
            // "+" = Addi((Id, i64)),
            Symbol(Symbol),
        }
    }
}

impl Bits {
    fn num(&self) -> Option<i64> {
        match self {
            Bits::Num(n) => Some(*n),
            _ => None,
        }
    }
}

type EGraph = egg::EGraph<Bits, BitsAnalysis>;

#[derive(Default)]
struct BitsAnalysis;



struct Data {
    free: HashSet<Id>,
    constant: Option<Lambda>,
}

impl Analysis<Bits> for BitsAnalysis {
    type Data = Data;
    fn merge(&self, to: &mut Data, from: Data) -> bool {
        let before_len = to.free.len();
        // to.free.extend(from.free);
        to.free.retain(|i| from.free.contains(i));
        let did_change = before_len != to.free.len();
        if to.constant.is_none() && from.constant.is_some() {
            to.constant = from.constant;
            true
        } else {
            did_change
        }
    }

    fn make(egraph: &EGraph, enode: &Bits) -> Data {
        let f = |i: &Id| egraph[*i].data.free.iter().cloned();
        let mut free = HashSet::default();
        match enode {
            Bits::Var(v) => {
                free.insert(*v);
            }
            Bits::Let([v, a, b]) => {
                free.extend(f(b));
                free.remove(v);
                free.extend(f(a));
            }
            Bits::Bits([v, a]) | Bits::Fix([v, a]) => {
                free.extend(f(a));
                free.remove(v);
            }
            _ => enode.for_each(|c| free.extend(&egraph[c].data.free)),
        }
        let constant = eval(egraph, enode);
        Data { constant, free }
    }

    fn modify(egraph: &mut EGraph, id: Id) {
        if let Some(c) = egraph[id].data.constant.clone() {
            let const_id = egraph.add(c);
            egraph.union(id, const_id);
        }
    }
}

