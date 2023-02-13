use stats_fn::*;

#[test]
fn mean_() {
    let data = vec![1, 2, 3];
    let m = mean(&data);
    assert_eq!(m, 2.0);
}

#[test]
fn median_odd() {
    let data = vec![1, 2, 3];
    let m = median(&data);
    assert_eq!(m, 2.0);
}

#[test]
fn median_even() {
    let data = vec![1, 2, 3, 4];
    let m = median(&data);
    assert_eq!(m, 2.5);
}

#[test]
fn mode_single() {
    let data = vec![1, 2, 3, 4, 1];
    let m = mode(&data);
    assert_eq!(m, vec![1]);
}

#[test]
fn mode_multiple() {
    let data = vec![1, 2, 3, 3, 4, 1];
    let m = mode(&data);
    assert_eq!(m, vec![1, 3]);
}

#[test]
fn mode_none() {
    let data = vec![1, 2, 3, 4];
    let m = mode(&data);
    assert_eq!(m, vec![]);
}
