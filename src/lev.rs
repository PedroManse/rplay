fn min3<T: Ord>(a: T, b: T, c: T) -> T {
    std::cmp::min(std::cmp::min(a, b), c)
}

// Using the Wagner-Fischer algorithm to compute the Levenshtein distance of two strings
pub fn lev(s0: &str, s1: &str) -> usize {
    if s0.is_empty() {
        return s1.chars().count();
    } else if s1.is_empty() {
        return s0.chars().count();
    }

    let s1_chars: Vec<char> = s1.chars().collect();

    // get lengths of string 1
    let len_1 = s1_chars.len();
    let mut row: Vec<usize> = (0..=len_1).collect();
    let mut d0 = 0;

    for (i, s0_char) in s0.chars().enumerate() {
        let mut e = i + 1;

        for j in 0..len_1 {
            let c: usize = (s0_char != s1_chars[j]) as usize;
            d0 = min3(row[j + 1] + 1, e + 1, row[j] + c);

            row[j] = e;
            e = d0;
        }

        row[len_1] = d0;
    }

    row[len_1]
}
