use std::iter;
use std::iter::zip;
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

fn make_multiplexer<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>
) -> MultiplexerTarget {
    let selector = builder.add_virtual_bool_target_safe();
    let input0 = [builder.add_virtual_bool_target_safe(); 256];
    let input1 = [builder.add_virtual_bool_target_safe(); 256];
    let mut output0_vec = Vec::new();
    let mut output1_vec = Vec::new();
    for (a,b) in zip(input0, input1) {
        let not_s = builder.not(selector);
        let sa = builder.and(a, selector);
        let sandnotb = builder.and(not_s, b);
        let sa_or_sandnotb = builder.or(sa, sandnotb);
        output0_vec.push(sa_or_sandnotb);
        let sandnota = builder.and(a, not_s);
        let sb = builder.and(selector, b);
        let a_and_not_s_or_sandb = builder.or(sandnota, sb);
        output1_vec.push(a_and_not_s_or_sandb);
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
    output0: [u8; 32],
    output1: [u8; 32],
    selector: bool,
    targets: MultiplexerTarget,
) {

}

fn main()  {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;
    let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::wide_ecc_config());
    let targets = make_multiplexer(&mut builder);
    //let mut pw = PartialWitness::new();
}
