use sdes::{p_10_to_8, p_x};
use rand::Rng;
use crate::sdes::s_boxes;
use crate::sdes::permute_plaintext;
use crate::sdes::sp_comb;
use crate::sdes::split_expand;
use crate::sdes::keys;
use crate::sdes::xor;
use crate::sdes::sdes;

mod sdes;

fn main() {
    let mut rng = rand::thread_rng();
    let key = rng.gen_range(1..513);
    let plaintext: u8 = 131;
    println!("{}", sdes(key, plaintext));
}
