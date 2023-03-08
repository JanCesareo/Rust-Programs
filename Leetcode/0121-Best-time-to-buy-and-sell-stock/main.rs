impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        
        prices.iter().fold((0, None), |(mut ret, mut lowest), p| {
            
            let lowest = lowest.map_or(Some(p), |l| {
                ret = ret.max(p - l);
                Some(if p > l {l}else{p})
            });
                
            (ret, lowest)
        }).0
    }

}
      
      
// The following code is an alternative way of solving the problem using maps instead of vectors.
/*
    pub fn max_profit(prices: Vec<i32>) -> i32 {
         prices
            .into_iter()
            .fold((0, i32::MIN), |(a, b), x| {
                let b = (a - x).max(b);
                (a.max(b + x), b)
            })
            .0
    }
*/ 
