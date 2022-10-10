mod models;

use models::*;

use std::str::FromStr;

fn main() {
    let block = Block {
        header: BlockHeader {
            parent_hash: H256(ethereum_types::H256::from_str("0x1cfa99ca7cbe89fc887932ca33523b008bd5211fb420b0616326db2af228af18").unwrap()),
            ommers_hash: H256(ethereum_types::H256::from_str("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347").unwrap()),
            beneficiary: H160(ethereum_types::H160::from_str("0x388C818CA8B9251b393131C08a736A67ccB19297").unwrap()),
            state_root: H256(ethereum_types::H256::from_str("0x253d9d9e77340d7008bbd5fd53f42f62ce000ffe094adf19709234991856ca6a").unwrap()),
            transactions_root: H256(ethereum_types::H256::from_str("0x055b9b67790fd3a4708028e5d0ada786b84a8bb0a0b49f427e81d9338abae941").unwrap()),
            receipts_root: H256(ethereum_types::H256::from_str("0xa89b734245e913a1423e99264171c2c5df50dfb4b79173c1061fd99dca2c534f").unwrap()),
            logs_bloom: Bloom(ethereum_types::Bloom::from_str("0x106050058002030080062002a089d09e005000444102000011010140100160100000115181040004000200088000110022001600084802003a00100002202044800064400092084028409008080221e004100209006800000100800880120000000110006202001803009418200009c00207006208000e401802441140080080000103200000100c094801d400010000400248d5c150408820502040001048300f0501e0080d600220074080005900001060010a0050048005d10281300110500318010600480000055000040808126104286002500a081809240106001020200078202020002480240812200050094015e44081800420408020180000000030").unwrap()),
            difficulty: U256(ethereum_types::U256::from_str("58750003716598352816469").unwrap()),
            number: 15702558,
            gas_limit: 30000000,
            gas_used: 27578524,
            timestamp: 1665222299,
            mix_hash: H256(ethereum_types::H256::from_str("0xeee4bdd8efcf6a26a1857835b4980c56e17d735ef5bbf265123253043b3b4f95").unwrap()),
            nonce: H64(ethereum_types::H64::default()),
            base_fee_per_gas: U256(ethereum_types::U256::from_str("8164745781").unwrap()),
        }
    };

    let rlp_encoded = fastrlp::encode_fixed_size(&block);
    let scale_encoded = parity_scale_codec::Encode::encode(&block);
    let rkyv_encoded = rkyv::to_bytes::<_, 256>(&block).unwrap();

    println!("rlp: {}", rlp_encoded.len());
    println!("scale: {}", scale_encoded.len());
    println!("rkyv: {}", rkyv_encoded.len());

    println!();

    let rlp_encoded_compressed = compress(fastrlp::encode_fixed_size(&block));
    let scale_encoded_compressed = compress(parity_scale_codec::Encode::encode(&block));
    let rkyv_encoded_compressed = compress(rkyv::to_bytes::<_, 256>(&block).unwrap());

    println!("rlp compressed: {}", rlp_encoded_compressed.len());
    println!("scale compressed: {}", scale_encoded_compressed.len());
    println!("rkyv compressed: {}", rkyv_encoded_compressed.len());
}

fn compress(source: impl AsRef<[u8]>) -> Vec<u8> {
    zstd::bulk::compress(source.as_ref(), zstd::compression_level_range().max().unwrap()).unwrap()
}