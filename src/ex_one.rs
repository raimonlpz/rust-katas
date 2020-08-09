// given a list of integers, use a vector and return the mean, median and mode of the list.
use std::collections::HashMap;

pub struct Integers {
    pub list: Vec<i32>,
}

impl Integers {
    pub fn calc_mean(&self) -> f32 {
        let mut sum: i32 = 0;
        for i in &self.list {
            sum += i;
        }
        let res: f32 = sum as f32 / self.list.len() as f32;
        res
    }
    pub fn calc_median(&mut self) -> i32 {
        self.list.sort();
        //println!("{:?}", self.list);
        let res: i32 = self.list[self.list.len() / 2];
        res
    }
    pub fn calc_mode(&self) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut res: i32 = 0;
        let mut key: i32 = 0;
        for i in &self.list {
            let count = map.entry(*i).or_insert(0);
            *count += 1;
        }
        for (i, x) in map.iter() {
            //println!("{}: {}", i, x);
            if x > &res {
                res = *x;
                key = *i;
            }
        }
        key
    }
}
