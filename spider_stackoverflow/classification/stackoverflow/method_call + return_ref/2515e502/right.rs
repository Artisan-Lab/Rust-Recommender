fn valid_braces(s: &str) -> bool {
    let s1 = s.chars();
    let mut vec = Vec::new();
    for x in s1 {
        let i = x.to_string();
        if i == "{" || i == "[" || i == "(" {
            vec.push(i);
        } else {
            if vec.len() == 0 {
                return false;
            }
            let curr = vec.pop().unwrap();
            //if curr == "{" { /* ... */ }
            //vec.remove(vec.len() - 1); // this line is source of the problem
            if curr == "{" {
                if i != "}" {
                    return false;
                }
            }
            if curr == "(" {
                if i != ")" {
                    return false;
                }
            }
            if curr == "[" {
                if i != "]" {
                    return false;
                }
            }
        }
    }
    if vec.len() == 0 {
        return false;
    } else {
        return true;
    };
}