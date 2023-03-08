impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for c in s.chars() {
            match c {
                ')' => match stack.pop() {
                    Some(c) => {
                        if c != '(' {
                            return false;
                        }
                    }
                    None => return false,
                },
                ']' => match stack.pop() {
                    Some(c) => {
                        if c != '[' {
                            return false;
                        }
                    }
                    None => return false,
                },
                '}' => match stack.pop() {
                    Some(c) => {
                        if c != '{' {
                            return false;
                        }
                    }
                    None => return false,
                },
                _ => stack.push(c),
            }
        }
        stack.is_empty()
        
    }
}
