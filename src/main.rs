mod header;
mod impls;
mod message;
mod wrapper;

use rkyv_scale_rlp::{compress, rkyv, rlp, scale, BLOCK, MESSAGE};

fn main() {
    println!("===BLOCK===");
    compare(BLOCK.clone());

    print!("\n\n");

    println!("===MESSAGE===");
    compare(MESSAGE.clone());
}

fn compare(
    target: impl Clone
        + parity_scale_codec::Encode
        + rkyv::Serialize<rkyv::ser::serializers::AllocSerializer<256>>
        + fastrlp::Encodable,
) {
    let rlp_encoded = rlp(target.clone());
    let scale_encoded = scale(target.clone());
    let rkyv_encoded = rkyv(target.clone());

    println!("rlp: {}", rlp_encoded.len());
    println!("scale (compact): {}", scale_encoded.len());
    println!("rkyv: {}", rkyv_encoded.len());

    println!();

    let rlp_encoded_compressed = compress(rlp_encoded);
    let scale_encoded_compressed = compress(scale_encoded);
    let rkyv_encoded_compressed = compress(rkyv_encoded);

    println!("rlp compressed: {}", rlp_encoded_compressed.len());
    println!(
        "scale (compact) compressed: {}",
        scale_encoded_compressed.len()
    );
    println!("rkyv compressed: {}", rkyv_encoded_compressed.len());
}
