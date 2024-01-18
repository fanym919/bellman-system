#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate bellman;
extern crate pairing;
extern crate rand;

// For randomness (during paramgen and proof generation)
use self::rand::{thread_rng, Rng};

// Bring in some tools for using pairing-friendly curves
use self::pairing::{
    Engine,
    Field,
    PrimeField
};

// We're going to use the BLS12-381 pairing-friendly elliptic curve.
use self::pairing::bls12_381::{
    Bls12,
    Fr
};

// We'll use these interfaces to construct our circuit.
use self::bellman::{
    Circuit,
    ConstraintSystem,
    SynthesisError
};

// We're going to use the Groth16 proving system.
use self::bellman::groth16::{
    Proof,
    generate_random_parameters,
    prepare_verifying_key,
    create_random_proof,
    verify_proof,
};

// proving that I know x such that x^3 + x + 5 == 35
// Generalized: x^3 + x + 5 == out
pub struct CubeDemo<E: Engine> {
    pub x: Option<E::Fr>,
    pub tmp_1: Option<E::Fr>,
    pub y: Option<E::Fr>,
    pub tmp_2: Option<E::Fr>,
    pub out: Option<E::Fr>,
}

impl <E: Engine> Circuit<E> for CubeDemo<E> {
    fn synthesize<CS: ConstraintSystem<E>>(
        self, 
        cs: &mut CS
    ) -> Result<(), SynthesisError>
    {
        // Flattened into quadratic equations (x^3 + x + 5 == 35): 
        // x * x = tmp_1
        // tmp_1 * x = y
        // y + x = tmp_2
        // tmp_2 + 5 = out
        // Resulting R1CS with w = [one, x, tmp_1, y, tmp_2, out]
        
        // Allocate the first private "auxiliary" variable
        let x = cs.alloc(|| "x", || {
            self.x.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // Allocate: x * x = tmp_1
        let tmp_1 = cs.alloc(|| "tmp_1", || {
            self.tmp_1.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Allocate: tmp_1 * x = y
        let y = cs.alloc(|| "y", || {
            self.y.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Allocate: y + x = tmp_2
        let tmp_2 = cs.alloc(|| "tmp_2", || {
            self.tmp_2.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Allocating the public "primary" output uses alloc_input
        let out = cs.alloc_input(|| "out", || {
            self.out.ok_or(SynthesisError::AssignmentMissing)
        })?;  



        // Enforce: x * x = tmp_1
        cs.enforce(
            || "tmp_1",
            |lc| lc + x,
            |lc| lc + x,
            |lc| lc + tmp_1
        );
        
        // Enforce: tmp_1 * x = y
        cs.enforce(
            || "y",
            |lc| lc + tmp_1,
            |lc| lc + x,
            |lc| lc + y
        );

        // Enforce: y + x = tmp_2
        cs.enforce(
            || "tmp_2",
            |lc| lc + y + x,
            |lc| lc + CS::one(),
            |lc| lc + tmp_2
        );
          
        // tmp_2 + 5 = out
        // => (tmp_2 + 5) * 1 = out
        cs.enforce(
            || "out",
            |lc| lc + tmp_2 + (E::Fr::from_str("5").unwrap(), CS::one()),
            |lc| lc + CS::one(),
            |lc| lc + out
        );
        // lc is an inner product of all variables with some vector of coefficients
        // bunch of variables added together with some coefficients
        
        // usually if mult by 1 can do more efficiently
        // x2 * x = out - x - 5
        
        // mult quadratic constraints 
        // 
        
        Ok(())
    }
}
