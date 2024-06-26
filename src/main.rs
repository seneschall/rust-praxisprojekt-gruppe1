use num::Zero;
use vers_vecs::{BitVec, RsVec};

fn main() {
    // Create a BitVec and fill it with some bits
    let mut bitvec = BitVec::new();
    bitvec.append(true);
    bitvec.append(false);
    bitvec.append(true);
    bitvec.append(true);
    bitvec.append(false);
    bitvec.append(true);
    bitvec.append(true);
    bitvec.append(false);
    bitvec.append(true);
    bitvec.append(true);
    bitvec.append(false);

    // Convert BitVec to RsVec
    let rsvec: RsVec = RsVec::from_bit_vec(bitvec.clone());

    // Convert RsVec back to BitVec
    let converted_bitvec = rsvec_to_bitvec(&rsvec);

    // Print the converted BitVec to verify
    for bit in converted_bitvec.iter() {
        println!("{}", bit);
    }
}

// Efficiently convert RsVec back to BitVec
fn rsvec_to_bitvec(rs_vec: &RsVec) -> BitVec {
    let mut bit_vec = BitVec::with_capacity(rs_vec.len());

    for val in rs_vec.iter() {
        if val.is_zero() {
            bit_vec.append(false);
        } else {
            bit_vec.append(true);
        }
    }

    bit_vec
}
