mod linear_search;

use linear_search::linear_search;

fn main() {
    let nums = vec![1,2,3,4,5];
    let target = 3;
    let result = linear_search(&nums, target);


    if result == -1{
        println!("Number {} not found in: {:?}", target, nums);
    } else {
        println!("Number {} found in: {}", target, result);
    }

}
