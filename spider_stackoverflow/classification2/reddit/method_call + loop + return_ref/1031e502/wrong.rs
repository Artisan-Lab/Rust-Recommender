fn test(letter: char, pairs: &mut Vec<(char, f64)>) -> &mut (char, f64) {
    for pair in pairs.iter_mut().rev() {
      if pair.0 == letter {
        return pair;
      }
    }
    pairs.push((letter, 0.0));
    &mut pairs[pairs.len() -1]
  }