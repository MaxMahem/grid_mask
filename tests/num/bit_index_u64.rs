use grid_mask::num::BitIndexU64;

#[test]
fn test_all_values() {
    let values: Vec<BitIndexU64> = BitIndexU64::all_values().collect();

    assert_eq!(values.len(), 64);
    values.iter().enumerate().for_each(|(i, val)| assert_eq!(val.get(), i as u8));
}

#[test]
fn test_first_set_in() {
    assert_eq!(BitIndexU64::from_first_set(0), None);
    assert_eq!(BitIndexU64::from_first_set(1), Some(BitIndexU64::new(0).unwrap()));
    assert_eq!(BitIndexU64::from_first_set(2), Some(BitIndexU64::new(1).unwrap()));
    assert_eq!(BitIndexU64::from_first_set(0x8000_0000_0000_0000), Some(BitIndexU64::new(63).unwrap()));
}

#[test]
fn test_iter_set_bits() {
    // 0 has no set bits
    let iter = BitIndexU64::iter_set_bits(0);
    assert_eq!(iter.count(), 0);

    // 1 has bit 0 set
    let values: Vec<u8> = BitIndexU64::iter_set_bits(1).map(|b| b.get()).collect();
    assert_eq!(values, vec![0]);

    // 6 has bits 1, 2 set
    let values: Vec<u8> = BitIndexU64::iter_set_bits(6).map(|b| b.get()).collect();
    assert_eq!(values, vec![1, 2]);

    // Max value
    let iter = BitIndexU64::iter_set_bits(u64::MAX);
    assert_eq!(iter.count(), 64);
}

#[test]
fn test_iter_set_bits_fused() {
    let mut iter = BitIndexU64::iter_set_bits(1);
    assert!(iter.next().is_some());
    assert!(iter.next().is_none());
    assert!(iter.next().is_none());
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_set_bits_patterns() {
    // Alternating bits 101010... -> 0xAA... (Indices 1, 3, 5, ... 63)
    let val = 0xAA_AA_AA_AA_AA_AA_AA_AA_u64;
    let indexes: Vec<u8> = BitIndexU64::iter_set_bits(val).map(|b| b.get()).collect();
    let expected: Vec<u8> = (1..64).step_by(2).collect();
    assert_eq!(indexes, expected);

    // Alternating bits 010101... -> 0x55... (Indices 0, 2, 4, ... 62)
    let val = 0x55_55_55_55_55_55_55_55_u64;
    let indexes: Vec<u8> = BitIndexU64::iter_set_bits(val).map(|b| b.get()).collect();
    let expected: Vec<u8> = (0..64).step_by(2).collect();
    assert_eq!(indexes, expected);
}

#[test]
fn test_double_ended() {
    let val = 0b1011; // 11: bits 0, 1, 3
    let mut iter = BitIndexU64::iter_set_bits(val);

    assert_eq!(iter.next().map(|b| b.get()), Some(0));
    assert_eq!(iter.next_back().map(|b| b.get()), Some(3));
    assert_eq!(iter.next().map(|b| b.get()), Some(1));
    assert_eq!(iter.next().map(|b| b.get()), None);
    assert_eq!(iter.next_back().map(|b| b.get()), None);

    // Empty
    let mut iter = BitIndexU64::iter_set_bits(0);
    assert_eq!(iter.next_back(), None);

    // Full
    let mut iter = BitIndexU64::iter_set_bits(u64::MAX);
    assert_eq!(iter.next_back().map(|b| b.get()), Some(63));
    assert_eq!(iter.next().map(|b| b.get()), Some(0));
}

#[test]
fn test_exact_size() {
    let mut iter = BitIndexU64::iter_set_bits(0b1101); // 3 bits set
    assert_eq!(iter.len(), 3);
    assert_eq!(iter.size_hint(), (3, Some(3)));

    iter.next();
    assert_eq!(iter.len(), 2);
    assert_eq!(iter.size_hint(), (2, Some(2)));

    iter.next_back();
    assert_eq!(iter.len(), 1);

    iter.next();
    assert_eq!(iter.len(), 0);
    assert_eq!(iter.size_hint(), (0, Some(0)));
}
