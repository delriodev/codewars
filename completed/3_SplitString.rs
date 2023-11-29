fn main(){}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(solution(""), [] as [&str; 0]);
    }
}

fn solution(s: &str) -> Vec<String> {
    s.as_bytes().chunks(2).map(|sub| match sub.len() == 2 {
        true => std::str::from_utf8(sub).unwrap().to_string(),
        false => std::str::from_utf8(sub).unwrap().to_string() + "_"
    }).collect()
}

/*Other solutions

use itertools::Itertools;
fn solution(s: &str) -> Vec<String> {
    s.chars().chunks(2).into_iter().map(|c| c.pad_using(2, |_| '_').collect()).collect()
}
    Uses iter tools and the pad_using method. Apperently is slower

s.chars().collect::<Vec<_>>().chunks(2).map(|v| {
    if v.len() == 1 {
        format!("{}_", v[0])
    }
    else {
        v.into_iter().collect()
    }
}).collect()

    Uses string format 




*/