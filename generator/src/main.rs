#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate bellman;
extern crate pairing;
extern crate rand;
use std::{fs, env, io::{Error}};

mod cube;

fn main() -> Result<(), Error> {
    use pairing::bls12_381::{Bls12};
    use rand::thread_rng;
    use bellman::groth16::{
        generate_random_parameters
    };

    let args: Vec<_> = env::args().collect();
    let pk_path: &str = &args[1].to_owned()[..];
    let vk_path: &str = &args[2].to_owned()[..];

    println!("Prove that I know x such that x^3 + x + 5 == 35.");
    
    let rng = &mut thread_rng();
    
    println!("Creating parameters...");
    
    // Create parameters for our circuit
    let params = {
        let c = cube::CubeDemo::<Bls12> {
            x: None
        };

        generate_random_parameters(c, rng).unwrap()
    };

    // Write proving key
    let mut v = vec![];
    params.write(&mut v)?;
    fs::write(pk_path, &v[..])?;
    
    // Write verification key
    let mut h = vec![];
    params.vk.write(&mut h)?;
    fs::write(vk_path, &h[..])?;

    Ok(())
}