//#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate bellman;
extern crate pairing;
extern crate rand;
use bellman::{groth16::{VerifyingKey}};
use pairing::{PrimeField};
use std::{fs, env, io::{Error}};

fn main() -> Result<(), Error> {
    use pairing::bls12_381::{Bls12, Fr};
    use rand::thread_rng;
    use bellman::groth16::{
        prepare_verifying_key, verify_proof, Proof,
    };

    let args: Vec<_> = env::args().collect();
    let vk_path: &str = &args[1].to_owned()[..];
    let proof_path: &str = &args[2].to_owned()[..];
    
    let rng = &mut thread_rng();
    
    // Read verification key
    let mut vk_f = fs::File::open(vk_path)?;
    let vk = VerifyingKey::<Bls12>::read(&mut vk_f).unwrap();
    // Prepare the verification key (for proof verification)
    let pvk = prepare_verifying_key(&vk);

    // Read Proof
    let mut proof_f = fs::File::open(proof_path)?;

    let proof = Proof::<Bls12>::read(&mut proof_f).unwrap();
        
    // assert!(verify_proof(
    //     &pvk,
    //     &proof,
    //     &[Fr::from_str("35").unwrap()]
    // ).unwrap());

    let verified = verify_proof(
            &pvk,
            &proof,
            &[Fr::from_str("35").unwrap()]
        ).unwrap();

    if verified {
        println!("Verification PASS");
    }
    else {
        println!("Verification Fail");
    }

    Ok(())
}