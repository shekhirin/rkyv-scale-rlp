mod block;
mod message;
mod numbers;
mod wrappers;

use rkyv::de::deserializers::SharedDeserializeMap;
use rkyv_scale_rlp::{
    compress, decompress, rkyv_deserialize, rkyv_serialize, rlp_deserialize, rlp_serialize,
    scale_deserialize, scale_serialize, BLOCK, MESSAGE,
};

fn main() {
    compare("block", BLOCK.clone());
    println!();
    compare("message", MESSAGE.clone());
}

fn compare<T>(name: &str, target: T)
where
    T: Clone
        + parity_scale_codec::Encode
        + parity_scale_codec::Decode
        + rkyv::Archive
        + rkyv::Serialize<rkyv::ser::serializers::AllocSerializer<256>>
        + fastrlp::Encodable
        + fastrlp::Decodable,
    T::Archived: rkyv::Deserialize<T, SharedDeserializeMap>,
{
    let rlp_encoded = rlp_serialize(target.clone());
    let scale_encoded = scale_serialize(target.clone());
    let rkyv_encoded = rkyv_serialize(target.clone());

    let rlp_encoded_len = rlp_encoded.len();
    let scale_encoded_len = scale_encoded.len();
    let rkyv_encoded_len = rkyv_encoded.len();

    println!("{name}/rlp: {} bytes", rlp_encoded_len);
    println!("{name}/scale: {} bytes", scale_encoded_len);
    println!("{name}/rkyv: {} bytes", rkyv_encoded_len);

    rlp_deserialize::<T>(rlp_encoded.clone());
    scale_deserialize::<T>(scale_encoded.clone());
    rkyv_deserialize::<T>(rkyv_encoded.clone());

    println!();

    let rlp_encoded_compressed = compress(rlp_encoded);
    let scale_encoded_compressed = compress(scale_encoded);
    let rkyv_encoded_compressed = compress(rkyv_encoded);

    println!(
        "{name}/compress/rlp: {} bytes",
        rlp_encoded_compressed.len()
    );
    println!(
        "{name}/compress/scale: {} bytes",
        scale_encoded_compressed.len()
    );
    println!(
        "{name}/compress/rkyv: {} bytes",
        rkyv_encoded_compressed.len()
    );

    rlp_deserialize::<T>(decompress(rlp_encoded_compressed, rlp_encoded_len));
    scale_deserialize::<T>(decompress(scale_encoded_compressed, scale_encoded_len));
    rkyv_deserialize::<T>(decompress(rkyv_encoded_compressed, rkyv_encoded_len));
}
