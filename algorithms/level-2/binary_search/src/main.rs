mod binary_search;

use binary_search::binary_search;

fn main() {
    let nums = vec![1,3,2,4,5];
    let target = 3;
    let result = binary_search(&nums, target);

    if result == -1{
        println!("Number {} not found in: {:?}", target, nums);
    } else {
        println!("Number {} found in: {}", target, result);
    }

}
