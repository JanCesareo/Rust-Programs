impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        fn get_str(s: String) -> Vec<char> {
            let mut stack = Vec::new();
            s.chars().for_each(|c| match c {
                '#' => {
                    stack.pop();
                }
                _ => stack.push(c),
            });
            stack
        }
        get_str(s) == get_str(t)
    }
        
}
