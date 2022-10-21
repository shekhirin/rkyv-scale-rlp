## Info

Block model:
```rust
Block {
    header: BlockHeader {
        parent_hash: "0x1cfa99ca7cbe89fc887932ca33523b008bd5211fb420b0616326db2af228af18",
        ommers_hash: "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
        beneficiary: "0x388C818CA8B9251b393131C08a736A67ccB19297",
        state_root: "0x253d9d9e77340d7008bbd5fd53f42f62ce000ffe094adf19709234991856ca6a",
        transactions_root: "0x055b9b67790fd3a4708028e5d0ada786b84a8bb0a0b49f427e81d9338abae941",
        receipts_root: "0xa89b734245e913a1423e99264171c2c5df50dfb4b79173c1061fd99dca2c534f",
        logs_bloom: "0x106050058002030080062002a089d09e005000444102000011010140100160100000115181040004000200088000110022001600084802003a00100002202044800064400092084028409008080221e004100209006800000100800880120000000110006202001803009418200009c00207006208000e401802441140080080000103200000100c094801d400010000400248d5c150408820502040001048300f0501e0080d600220074080005900001060010a0050048005d10281300110500318010600480000055000040808126104286002500a081809240106001020200078202020002480240812200050094015e44081800420408020180000000030",
        difficulty: "58750003716598352816469",
        number: 15702558,
        gas_limit: 30000000,
        gas_used: 27578524,
        timestamp: 1665222299,
        extra_data: Default::default(),
        mix_hash: "0xeee4bdd8efcf6a26a1857835b4980c56e17d735ef5bbf265123253043b3b4f95",
        nonce: Default::default(),
        base_fee_per_gas: "8164745781",
    }
}
```

Message model:
```rust
EIP1559MessageWithSignature {
    message: EIP1559Message {
        chain_id: 1,
        nonce: 171,
        max_priority_fee_per_gas: "1000000000",
        max_fee_per_gas: "23000000000",
        gas_limit: 38177,
        action: "0x6b175474e89094c44da98b954eedeac495271d0f",
        value: U256::ZERO,
        input: hex::decode("a9059cbb00000000000000000000000070c16d2db6b00683b29602cbab72ce0dcbc243c40000000000000000000000000000000000000000000000006f05b59d3b200000").unwrap().into(),
        access_list: vec![],
    },
    signature: MessageSignature {
        odd_y_parity: false,
        r: "0x3b44abe6990c6aba17cb26216e9a5a204f47264a360c7c7dea24ac7963a2cf60",
        s: "0x5dadeef25494252799a57477d3191768523bea2e7e8841ad3cd891698e325ed2",
    },
}
```

zstd compression level in benchmarks: -6

## Size benchmarks

```
block/rlp: 541 bytes
block/scale: 526 bytes
block/rkyv: 584 bytes

block/compress/rlp: 551 bytes
block/compress/scale: 536 bytes
block/compress/rkyv: 560 bytes

message/rlp: 186 bytes
message/scale: 174 bytes
message/rkyv: 304 bytes

message/compress/rlp: 175 bytes
message/compress/scale: 162 bytes
message/compress/rkyv: 199 bytes
```

## Performance benchmarks

```
serialize/block/rlp                     time:   [514.48 ns 517.17 ns 520.09 ns]
serialize/block/scale                   time:   [526.93 ns 530.21 ns 534.93 ns]
serialize/block/rkyv                    time:   [91.884 ns 92.149 ns 92.477 ns]

serialize/block/compress/rlp            time:   [1.7680 µs 1.7755 µs 1.7831 µs]
serialize/block/compress/scale          time:   [1.2117 µs 1.2166 µs 1.2219 µs]
serialize/block/compress/rkyv           time:   [1.3812 µs 1.3902 µs 1.4001 µs]

serialize/message/rlp                   time:   [395.03 ns 396.14 ns 397.35 ns]
serialize/message/scale                 time:   [306.52 ns 307.47 ns 308.47 ns]
serialize/message/rkyv                  time:   [135.30 ns 135.70 ns 136.15 ns]

serialize/message/compress/rlp          time:   [1.4276 µs 1.4324 µs 1.4378 µs]
serialize/message/compress/scale        time:   [1.3281 µs 1.3298 µs 1.3318 µs]
serialize/message/compress/rkyv         time:   [1.2023 µs 1.2147 µs 1.2274 µs]

deserialize/block/rlp                   time:   [808.42 ns 817.14 ns 828.63 ns]
deserialize/block/scale                 time:   [636.65 ns 639.43 ns 643.31 ns]
deserialize/block/rkyv                  time:   [283.73 ns 285.28 ns 286.91 ns]

deserialize/block/compress/rlp          time:   [727.36 ns 728.28 ns 729.29 ns]
deserialize/block/compress/scale        time:   [757.12 ns 758.69 ns 760.23 ns]
deserialize/block/compress/rkyv         time:   [483.64 ns 484.52 ns 485.49 ns]

deserialize/message/rlp                 time:   [290.52 ns 291.15 ns 291.85 ns]
deserialize/message/scale               time:   [313.97 ns 315.55 ns 317.53 ns]
deserialize/message/rkyv                time:   [134.44 ns 135.10 ns 135.84 ns]

deserialize/message/compress/rlp        time:   [423.77 ns 424.61 ns 425.57 ns]
deserialize/message/compress/scale      time:   [455.65 ns 456.89 ns 458.39 ns]
deserialize/message/compress/rkyv       time:   [312.97 ns 314.15 ns 315.39 ns]
```