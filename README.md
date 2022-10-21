## Size benchmarks

```
block/rlp: 541 bytes
block/scale: 526 bytes
block/rkyv: 584 bytes

block/compress/rlp: 551 bytes
block/compress/scale: 536 bytes
block/compress/rkyv: 538 bytes

message/rlp: 186 bytes
message/scale: 174 bytes
message/rkyv: 304 bytes

message/compress/rlp: 170 bytes
message/compress/scale: 157 bytes
message/compress/rkyv: 181 bytes
```

## Performance benchmarks

```
serialize/block/rlp                 time:   [511.10 ns 512.67 ns 514.49 ns]
serialize/block/scale               time:   [526.45 ns 530.52 ns 534.61 ns]
serialize/block/rkyv                time:   [92.152 ns 92.801 ns 93.841 ns]

serialize/block/compress/rlp        time:   [7.0616 µs 7.0860 µs 7.1156 µs]
serialize/block/compress/scale      time:   [7.0744 µs 7.0956 µs 7.1205 µs]
serialize/block/compress/rkyv       time:   [6.7159 µs 6.7485 µs 6.7891 µs]

serialize/message/rlp               time:   [395.56 ns 396.29 ns 397.01 ns]
serialize/message/scale             time:   [307.05 ns 307.83 ns 308.79 ns]
serialize/message/rkyv              time:   [135.57 ns 135.87 ns 136.24 ns]

serialize/message/compress/rlp      time:   [5.5982 µs 5.6273 µs 5.6767 µs]
serialize/message/compress/scale    time:   [1.7767 µs 1.7829 µs 1.7897 µs]
serialize/message/compress/rkyv     time:   [5.4836 µs 5.4995 µs 5.5180 µs]

deserialize/block/rlp               time:   [795.08 ns 796.05 ns 797.26 ns]
deserialize/block/scale             time:   [625.50 ns 626.60 ns 627.90 ns]
deserialize/block/rkyv              time:   [278.14 ns 278.44 ns 278.81 ns]

deserialize/block/compress/rlp      time:   [726.36 ns 727.07 ns 727.82 ns]
deserialize/block/compress/scale    time:   [752.74 ns 753.85 ns 755.03 ns]
deserialize/block/compress/rkyv     time:   [509.37 ns 510.19 ns 511.25 ns]

deserialize/message/rlp             time:   [288.23 ns 288.77 ns 289.48 ns]
deserialize/message/scale           time:   [309.85 ns 310.30 ns 310.82 ns]
deserialize/message/rkyv            time:   [132.77 ns 133.40 ns 134.05 ns]

deserialize/message/compress/rlp    time:   [422.69 ns 423.78 ns 425.00 ns]
deserialize/message/compress/scale  time:   [457.10 ns 457.91 ns 458.83 ns]
deserialize/message/compress/rkyv   time:   [322.77 ns 324.09 ns 325.45 ns]
```