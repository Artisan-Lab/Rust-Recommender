use std::collections::HashSet;

fn enter_pattern(
    new_state: &mut usize,
    pat: &[u8],
    idx: usize,
    goto_fn: &mut Vec<&mut Vec<i32>>,
    output_fn: &mut Vec<HashSet<usize>>,
) {
    let len = pat.len();
    let mut j: usize = 0;
    let mut state: usize = 0;

    // Find the first leaf corresponding to a character in `pat`. From there is
    // where a new state (if needed) will be added.
    while goto_fn[state][pat[j] as usize] != 0 {
        state = goto_fn[state][pat[j] as usize] as usize;
        j += 1;
        if j == len {
            break;
        }
    }

    // At this point, `state` points to the leaf in the automaton. Create new
    // states from here on for the remaining characters in `pat` that weren't
    // already in the automaton.
    for p in j..len {
        *new_state += 1;
        goto_fn[state][pat[p] as usize] = *new_state as i32;
        state = *new_state;
        // Create the new state and append it to goto_fn. Also have to create
        // a new set object and add it to output_fn.
        let mut new_state_vec: Vec<i32> = vec![0; 0];
        goto_fn.push(&mut new_state_vec);
        output_fn.push(HashSet::new());
    }

    // Add the index of this pattern to the output_fn set for this state:
    output_fn[state].insert(idx);
}