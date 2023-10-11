pub fn linear_search(nums: &Vec<i32>, target:i32)->i32{
    for (index, &val) in nums.iter().enumerate(){
        if val == target{
            return index as i32;
        }
    }
    return -1
}