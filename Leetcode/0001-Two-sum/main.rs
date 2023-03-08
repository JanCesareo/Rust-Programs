impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut resultV = vec![0,0];
        
        for (pos, element) in nums.iter().enumerate() {
            // add each element with each other elements in the vector and compare 
            // if it matches with target, skip the loop if same index element encounters 
            for (pos1, element1) in nums.iter().enumerate() {
                if pos == pos1 {
                    continue;
                } else if element+ element1 == target {
                    resultV[0]= pos as i32;
                    resultV[1]= pos1 as i32;
                }

            }

        }
        return resultV;

    }
}
