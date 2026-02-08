use grid_mask::GridSize;

#[test]
fn test_const_new() {
    const S1: GridSize = GridSize::const_new::<1, 1>();
    assert_eq!(S1.width.get(), 1);
    assert_eq!(S1.height.get(), 1);

    const S2: GridSize = GridSize::const_new::<8, 8>();
    assert_eq!(S2.width.get(), 8);
    assert_eq!(S2.height.get(), 8);

    const S3: GridSize = GridSize::const_new::<1, 8>();
    assert_eq!(S3.width.get(), 1);
    assert_eq!(S3.height.get(), 8);
}
