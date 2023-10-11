pub struct Solution;

impl Solution{
    pub fn two_sums(v: Vec<i32>, alvo: i32) -> Vec<i32>{
        use std::collections::HashMap;

        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut result: Vec<i32> = Vec::new();

        println!("{:?}, {}",v, alvo);

        for i in 0..v.len(){

            if map.contains_key(&(alvo - v[i])){
                result.push(map[&(alvo -v[i])]);
                result.push(i as i32);

            }
            println!("result={:?}",result);

            println!("{:?}",map);
            map.insert(v[i], i as i32);
        }

        vec![]
    }
}