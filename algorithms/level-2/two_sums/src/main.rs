mod solution;

fn main() {
    let mut v = vec![1,2,3,4,5];
    let target = 3;
    solution::Solution::two_sums(v, target);

    assert_eq!(solution::Solution::two_sums(vec![34, 1, 12, 99], 100), vec![1,3]);

}
