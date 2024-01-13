use plonky2::{iop::target::BoolTarget, plonk::circuit_builder::CircuitBuilder, hash::hash_types::RichField};
use plonky2_field::extension::Extendable;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};

pub fn array_to_bits(bytes: &[u8]) -> Vec<bool> {
    let len = bytes.len();
    let mut ret = Vec::new();
    for i in 0..len {
        for j in 0..8 {
            let b = (bytes[i] >> (7 - j)) & 1;
            ret.push(b == 1);
        }
    }
    ret
}

pub struct MultiplexerTarget {
    pub selector: BoolTarget,
    pub input0: [BoolTarget; 256],
    pub input1: [BoolTarget; 256],
    pub output0: [BoolTarget; 256],
    pub output1: [BoolTarget; 256],
}

pub fn make_multiplexer<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>
) -> MultiplexerTarget {
    let selector = builder.add_virtual_bool_target_safe();
    let mut input0 = Vec::new();
    let mut input1 = Vec::new();
    let mut output0_vec = Vec::new();
    let mut output1_vec = Vec::new();
    //builder.register_public_input(selector.target);
    let nots = builder.not(selector);
    for i in 0..256 {
        let i0 = builder.add_virtual_bool_target_safe();
        let i1 = builder.add_virtual_bool_target_safe();
        /*builder.register_public_input(i0.target);
        builder.register_public_input(i1.target);*/
        let nots_and_a = builder.and(nots, i0);
        let s_and_b = builder.and(selector, i1);
        let out0 = builder.or(nots_and_a, s_and_b);
        let s_and_a = builder.and(selector, i0);
        let nots_and_b = builder.and(nots, i1);
        let out1 = builder.or(s_and_a, nots_and_b);
        output0_vec.push(out0);
        output1_vec.push(out1);
        input0.push(i0);
        input1.push(i1);
    }
    MultiplexerTarget { 
        selector: selector,
        input0: input0.try_into().unwrap(),
        input1: input1.try_into().unwrap(),
        output0: output0_vec.try_into().unwrap(),
        output1: output1_vec.try_into().unwrap(),
    }
}

pub fn fill_circuits<F: RichField + Extendable<D>, const D: usize>(
    pw: &mut PartialWitness<F>,
    input0: [u8; 32],
    input1: [u8; 32],
    selector: bool,
    targets: MultiplexerTarget,
) {
    let i0 = array_to_bits(&input0);
    let i1 = array_to_bits(&input1);
    let o0: Vec<bool>;
    let o1: Vec<bool>;
    if !selector {
        o0=i0.clone();
        o1=i1.clone();
    }
    else {
        o0=i1.clone();
        o1=i0.clone();
    }
    pw.set_bool_target(targets.selector, selector);
    for i in 0..256 {
        pw.set_bool_target(targets.input0[i], i0[i]);
        pw.set_bool_target(targets.input1[i], i1[i]);
        pw.set_bool_target(targets.output0[i], o0[i]);
        pw.set_bool_target(targets.output1[i], o1[i]);
    }
}

#[cfg(test)]
mod tests {
    use crate::dualmux;
    use plonky2::{plonk::{config::{PoseidonGoldilocksConfig, GenericConfig}, circuit_builder::CircuitBuilder, circuit_data::CircuitConfig}, iop::witness::PartialWitness};

    #[test]
    fn test_mux() {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_recursion_config());
        let targets = dualmux::make_multiplexer(&mut builder);
        let mut pw = PartialWitness::new();
        let input0: [u8; 32] = [0; 32];
        let input1: [u8; 32] = [1; 32];
        dualmux::fill_circuits::<F, D>(&mut pw, input0, input1, false, targets);
        println!(
            "Constructing proof with {} gates",
            builder.num_gates()
        );
        let data = builder.build::<C>();
        println!("Proving...");
        let proof = data.prove(pw).unwrap();
        println!("Done proving!");
        println!("Verifying proof...");
        data.verify(proof.clone()).expect("verify error");
        println!("Done verifying!");

    }
}