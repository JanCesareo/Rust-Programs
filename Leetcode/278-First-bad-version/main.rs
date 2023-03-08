// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut l:i32 = 1;
        let mut r:i32 = n;
        let mut res:i32 = 1;

        while l <= r {
            let mid = (((l as i128) + (r as i128))/2) as i32;
            if self.isBadVersion(mid) == true {
                res = mid;
                r = mid - 1;
                println!("true");
            } else {
                l = mid + 1;
                println!("false");
            }
        }
        res
		
    }
}
