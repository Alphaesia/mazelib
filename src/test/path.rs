use crate::path::Path;

#[test]
fn test_path_is_simple() {
    assert!(Path::from_vec(vec![1]).is_simple());

    assert!(Path::from_vec(vec![1, 2, 3]).is_simple());

    assert_eq!(Path::from_vec(vec![1, 2, 1]).is_simple(), false);
}

#[test]
fn test_path_simplification() {
    let mut path = Path::from_vec(vec![1]);
    path.make_simple();
    assert!(path.is_simple());

    let mut path = Path::from_vec(vec![1, 2]);
    path.make_simple();
    assert!(path.is_simple());

    let mut path = Path::from_vec(vec![1, 2, 1]);
    path.make_simple();
    assert!(path.is_simple());

    let mut path = Path::from_vec(vec![1, 2, 1, 2]);
    path.make_simple();
    assert!(path.is_simple());

    let mut path = Path::from_vec(vec![0_0, 0_1, 0_2, 1_2, 1_1, 2_1, 2_0, 3_0, 3_1, 3_2, 3_3, 3_4, 2_4, 3_4, 3_3, 3_2, 2_2, 2_3, 1_3, 0_3, 0_4, 1_4, 1_5, 0_5, 0_6, 1_6, 2_6, 2_5, 3_5, 3_6, 4_6, 4_5, 5_5, 5_6, 6_6, 6_5, 7_5, 8_5, 9_5, 9_6, 9_7, 9_8, 8_8, 8_9, 9_9, 9_10, 10_10]);
    path.make_simple();
    assert!(path.is_simple());

    let mut path = Path::from_vec(vec![0_0, 1_0, 1_1, 2_1, 3_1, 3_0, 2_0, 3_0, 3_1, 2_1, 1_1, 1_0, 0_0, 1_0, 1_1, 0_1, 0_2, 0_3, 1_3, 1_2, 1_3, 0_3, 0_2, 0_1, 1_1, 1_0, 0_0, 1_0, 1_1, 0_1, 0_2, 0_3, 1_3, 1_2, 1_3, 2_3, 2_2, 3_2, 3_3]);
    path.make_simple();
    assert!(path.is_simple());
}