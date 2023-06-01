use std::{iter, array};
use std::iter::{zip, Enumerate};
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::BoolTarget;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2_field::extension::Extendable;
use plonky2::plonk::circuit_data::{
    CircuitConfig, CommonCircuitData, VerifierCircuitTarget, VerifierOnlyCircuitData,
};
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig, Hasher, PoseidonGoldilocksConfig};

struct MultiplexerTarget {
    selector: BoolTarget,
    input0: [BoolTarget; 256],
    input1: [BoolTarget; 256],
    output0: [BoolTarget; 256],
    output1: [BoolTarget; 256],
}

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

fn make_multiplexer<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>
) -> MultiplexerTarget {
    let selector = builder.add_virtual_bool_target_safe();
    let input0 = [builder.add_virtual_bool_target_safe(); 256];
    let input1 = [builder.add_virtual_bool_target_safe(); 256];
    let mut output0_vec = Vec::new();
    let mut output1_vec = Vec::new();
    builder.register_public_input(selector.target);
    for (a,b) in zip(input0, input1) {
        builder.register_public_input(a.target);
        builder.register_public_input(b.target);
        let nots = builder.not(selector);
        let nots_and_a = builder.and(nots, a);
        let s_and_b = builder.and(selector, b);
        let out0 = builder.or(nots_and_a, s_and_b);
        builder.register_public_input(out0.target);
        output0_vec.push(out0);
        let s_and_a = builder.and(selector, a);
        let nots_and_b = builder.and(nots, b);
        let out1 = builder.or(s_and_a, nots_and_b);
        builder.register_public_input(out1.target);
        output1_vec.push(out1);
    }
    MultiplexerTarget { 
        selector: selector,
        input0: input0,
        input1: input1,
        output0: output0_vec.try_into().unwrap(),
        output1: output1_vec.try_into().unwrap(),
    }
}

fn fill_circuits<F: RichField + Extendable<D>, const D: usize>(
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
    if selector {
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

fn main()  {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;
    let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::wide_ecc_config());
    let targets = make_multiplexer(&mut builder);
    let mut pw = PartialWitness::new();
    let input0: [u8; 32] = [0; 32];
    let input1: [u8; 32] = [1; 32];
    fill_circuits::<F, D>(&mut pw, input0, input1, false, targets);
    println!(
        "Constructing inner proof with {} gates",
        builder.num_gates()
    );
}
