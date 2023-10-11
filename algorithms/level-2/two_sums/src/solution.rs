pub struct Solution{

}

impl Solution{
    pub fn two_sums(v: Vec<i32>, target:i32)->Vec<i32>{
        for (i,e1) in v.iter().enumerate(){
            for(j, e2)in v.iter().skip(i+1).enumerate(){
                if e1 + e2 ==target{
                    println!("{:?}", vec![i, j + 1 + i]); 
                    return vec![i as i32, j as i32+ 1+ i as i32];
                }
            }
        }
        vec![]
    }
}