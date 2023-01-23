//#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate bellman;
extern crate pairing;
extern crate rand;
use bellman::{groth16::{Parameters}};
use pairing::{PrimeField};
use std::{fs, env, io::{Error}};

mod cube;

fn main() -> Result<(), Error> {
    use pairing::bls12_381::{Bls12, Fr};
    use rand::thread_rng;
    use bellman::groth16::{
        create_random_proof
    };

    let args: Vec<_> = env::args().collect();
    let pk_path: &str = &args[1].to_owned()[..];
    let proof_path: &str = &args[2].to_owned()[..];
    
    let rng = &mut thread_rng();
    
    // Read proving key
    let mut pk_f = fs::File::open(pk_path)?;
    let pk = Parameters::<Bls12>::read(&mut pk_f, true).unwrap();

    println!("Creating proofs...");
    
    // Create an instance of circuit
    let c = cube::CubeDemo::<Bls12> {
        x: Fr::from_str("3")
    };
    
    // Create a groth16 proof with our parameters.
    let proof = create_random_proof(c, &pk, rng).unwrap();
    
    // Write proof
    let mut w = vec![];
    proof.write(&mut w)?;
    fs::write(proof_path, &w[..])?;

    Ok(())
}