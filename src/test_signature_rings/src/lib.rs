use std::{cell::RefCell, collections::BTreeSet};

// use ring_compat::signature::{
//     ed25519::{Signature, SigningKey, VerifyingKey},
//     Signer, Verifier
// };

// ERROR 
use ring;


const PUBKEY_LEN: usize = 32;
const PRVKEY_LEN: usize = 32;

pub struct Keypair<S, V> {
    pub private_key: S,
    pub public_key: V,
}

pub struct User<S, V> {
    pub name: String,
    pub keypair: Keypair<S, V>,
}

type Users<S, V> = BTreeSet<User<S, V>>;


thread_local! {
    static USERS: RefCell<Users<[u8; PRVKEY_LEN], [u8; PUBKEY_LEN]>> = RefCell::default();
}

#[ic_cdk_macros::update]
fn pem_upload() -> bool {
    

    return true;
}

#[ic_cdk_macros::query]
fn verify_sign() -> bool {
    return true;
} 

#[ic_cdk_macros::update]
fn new() -> () {
    //let rng = rand::SystemRandom::new();
}

