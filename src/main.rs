use std::fmt::Debug;

use cranelift_codegen::ir::instructions::{Opcode, ResolvedConstraint, ValueTypeSet};
use cranelift_codegen::ir::types::*;
use itertools::Itertools;

#[derive(Debug)]
struct OpcodeSignature {
    pub opcode: Opcode,
    pub ctrl_type: Type,
    pub args: Vec<Type>,
    pub rets: Vec<Type>,
}

fn multi_cartesian_product<T: Clone + Debug>(v: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut ret = Vec::new();
    match v {
        [] => {}
        [head] => {
            for i in head {
                ret.push(vec![i.clone()]);
            }
        }
        [head, tail @ ..] => {
            let tail_products = multi_cartesian_product(tail);

            for first_item in head.into_iter() {
                for product in tail_products.iter() {
                    let mut a = product.clone();
                    a.insert(0, first_item.clone());
                    ret.push(a);
                }
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let product: Vec<Vec<u32>> = multi_cartesian_product(&[]);
        assert_eq!(product, Vec::<Vec<u32>>::new());
    }

    #[test]
    fn test() {
        let data = &[vec![1, 2], vec![3, 4]];
        let product = multi_cartesian_product(data);
        assert_eq!(
            product,
            vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4]]
        )
    }
}

fn main() {
    // let opcode = Opcode::Call;
    // let opcode = Opcode::Fma;
    // let opcode = Opcode::Fcmp;
    // let opcode = Opcode::Iadd;
    // let opcode = Opcode::Bmask;
    // let opcode = Opcode::AtomicStore;
    let mut cnt = 0;
    for opcode in Opcode::all() {
        if opcode != &Opcode::AtomicStore {
            continue;
        }
        dbg!(opcode);

        if [
            Opcode::Jump,
            Opcode::Brz,
            Opcode::Brnz,
            Opcode::Brif,
            Opcode::Brff,
            Opcode::BrTable,
            Opcode::Debugtrap,
            Opcode::Trap,
            Opcode::ResumableTrap,
            Opcode::Trapif,
            Opcode::Trapff,
            Opcode::Return,
            Opcode::Call,
            Opcode::CallIndirect,
            // TODO: Review this
            Opcode::Vsplit,
            Opcode::SwidenLow,
            Opcode::SwidenHigh,
            Opcode::UwidenLow,
            Opcode::UwidenHigh,
            Opcode::ExtractVector,
        ]
        .contains(opcode)
        {
            continue;
        }

        let constraints = opcode.constraints();
        dbg!(constraints.num_fixed_value_arguments());
        // dbg!(constraints.value_argument_constraint(0, I8));

        let ctrl_typeset = constraints.ctrl_typeset().map_or_else(
            || ResolvedConstraint::Bound(INVALID),
            ResolvedConstraint::Free,
        );
        let ctrl_types = ctrl_typeset.types_iter();
        dbg!(ctrl_typeset.types_iter().collect::<Vec<_>>());
        // return;

        for ctrl_type in ctrl_types {
            dbg!(ctrl_type);

            let rets: Vec<_> = (0..constraints.num_fixed_results())
                .map(|n| constraints.result_type(n, ctrl_type))
                .collect();
            dbg!(&rets);

            let args_types = (0..constraints.num_fixed_value_arguments())
                .map(|n| constraints.value_argument_constraint(n, ctrl_type))
                .map(|rc| rc.types_iter().collect::<Vec<_>>())
                .collect::<Vec<_>>();
            dbg!(&args_types);

            let prod = multi_cartesian_product(&args_types[..]);
            dbg!(&prod);

            cnt += prod.len();

            // dbg!(opcode);
            // dbg!(prod);

            // dbg!(ctrl_type);
            // dbg!(rets);
            // dbg!(args_types);
        }

        // let sigs = ctrl_types.flat_map(|ctrl_type| {
        //     let args_types = (0..constraints.num_fixed_value_arguments())
        //         .map(|n| constraints.value_argument_constraint(n, ctrl_type))
        //         .map(|rc| rc.types_iter().collect::<Vec<_>>());
        //     // .multi_cartesian_product();

        //     dbg!(args_types.collect::<Vec<_>>());

        //     unimplemented!()
        //     // args.map(move |args| {
        //     //     let rets = (0..constraints.num_fixed_results())
        //     //         .map(|n| constraints.result_type(n, ctrl_type))
        //     //         .collect();
        //     //     OpcodeSignature {
        //     //         opcode,
        //     //         ctrl_type,
        //     //         rets,
        //     //         args,
        //     //     }
        //     // })
        // });

        // dbg!(sigs.collect::<Vec<_>>());

        // .map(|n| constraints.result_type(n, ctrl_types))
    }
    dbg!(cnt);
}
