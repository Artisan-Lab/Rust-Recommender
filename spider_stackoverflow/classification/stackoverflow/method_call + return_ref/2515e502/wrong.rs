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
            let curr = vec.last().unwrap();

            vec.remove(vec.len() - 1); // this line is source of the problem
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
}// curr本身搞了个最后一个位置的引用，&str,(当然不改变语义的情况下)可以用clone()解决