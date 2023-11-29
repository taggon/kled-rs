mod korean;

/// Calculate the Levenshtein distance between two strings.
/// Reference: https://xlinux.nist.gov/dads/HTML/Levenshtein.html
pub fn distance(a: &str, b: &str, case_sensitive: bool) -> f32 {
    if a.is_empty() || b.is_empty() {
        return (a.chars().count() as f32).max(b.chars().count() as f32);
    }

    let a_str = if case_sensitive { a.to_string() } else { a.to_lowercase() };
    let b_str = if case_sensitive { b.to_string() } else { b.to_lowercase() };
    let a_len = a_str.chars().count();
    let mut column: Vec<f32> = vec![0.0; a_len + 1];

    for i in 1..a_len+1 {
        column[i] = i as f32;
    }

    for (b_idx, b_char) in b_str.chars().enumerate() {
        column[0] = (b_idx + 1) as f32;
        let mut lastdiag = b_idx as f32;

        for (a_idx, a_char) in a_str.chars().enumerate() {
            let mut kor_similarity: f32 = 0.0;

            if (a_char != b_char) && korean::is_korean(&a_char) && korean::is_korean(&b_char) {
                if korean::is_similar(&a_char, &b_char) {
                    kor_similarity = 0.01;
                }
            }

            let olddiag = column[a_idx + 1];
            let subtitution_delta: f32 = if kor_similarity > 0.0 || a_char == b_char {
                kor_similarity
            } else {
                1.0
            };

            column[a_idx + 1] = (column[a_idx + 1] + 1.0).min(column[a_idx] + 1.0).min(lastdiag + subtitution_delta);

            lastdiag = olddiag;
        }
    }

    column[a_len]
}

pub fn matches(needle: &str, haystack: &str, case_sensitive: bool) -> f32 {
    if needle.chars().count() > haystack.chars().count() {
        return 0.0;
    }

    let n_str = if case_sensitive { needle.to_string() } else { needle.to_lowercase() };
    let h_str = if case_sensitive { haystack.to_string() } else { haystack.to_lowercase() };

    let mut contained = true;
    let mut last_pos: usize = 0;

    for (_, n_char) in n_str.chars().enumerate() {
        if !korean::is_korean(&n_char) || korean::has_final(&n_char) {
            let pos = h_str.chars().skip(last_pos).position(|c| c == n_char);
            if pos == None {
                contained = false;
                break;
            } else {
                last_pos = pos.unwrap() + 1;
            }
        } else {
            let pos = h_str
                .chars()
                .skip(last_pos)
                .position(|c| 
                    (korean::is_consonant(&c) && c == n_char) ||
                    (!korean::is_consonant(&c) && korean::is_similar(&c, &n_char))
                );

            if pos == None {
                contained = false;
                break;
            } else {
                last_pos = pos.unwrap() + 1;
            }
        }
    }

    // If the needle is not contained in the haystack, return 0.0.
    if ! contained {
        return 0.0;
    }

    let dist = distance(n_str.as_str(), h_str.as_str(), false);
    let haystack_len = haystack.chars().count() as f32;

    (haystack_len - dist) / haystack_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_between_two_english_words() {
        assert_eq!(distance("kitten", "sitting", false), 3.0);
    }

    #[test]
    fn distance_when_one_string_is_empty() {
        assert_eq!(distance("", "foo", false), 3.0);
        assert_eq!(distance("hello", "", false), 5.0);
        assert_eq!(distance("", "한글", false), 2.0);
    }

    #[test]
    fn distance_when_korean_letters_exist() {
        assert_eq!(distance("A학급", "B학급", false), 1.0);
        assert_eq!(distance("Aㅎㄱ", "B학급", true), 1.02);
    }

    #[test]
    fn matches_when_needle_is_same_with_haystack() {
        assert_eq!(matches("foo", "foo", true), 1.0);

        // Case-insensitive
        assert_eq!(matches("foo", "FOo", false), 1.0);
        assert_eq!(matches("foo", "FOo", true), 0.0);

        // Korean
        assert_eq!(matches("홍길동", "홍길동", true), 1.0);
    }

    #[test]
    fn matches_when_needle_is_not_in_the_haystack() {
        assert_eq!(matches("foo", "bar", true), 0.0);
        assert_eq!(matches("foo", "bar", false), 0.0);
        assert_eq!(matches("dog", "digging", false), 0.0);

        // Needle is longer than haystack
        assert_eq!(matches("longer", "long", false), 0.0);

        // Korean
        assert_eq!(matches("홍길동", "박문수", false), 0.0);
        assert_eq!(matches("홍길동", "호길동", false), 0.0);
        assert_eq!(matches("홍가동", "홍길동", false), 0.0);
        assert_eq!(matches("홍가두", "홍길동", false), 0.0);
        assert_eq!(matches("고성", "군산", false), 0.0);
        assert_eq!(matches("우산", "ㅇ산", false), 0.0);
    }

    #[test]
    fn matches_shows_how_much_similar_two_strings_are() {
        assert!(matches("brb", "Be Right Back!", false) > 0.0);
        assert!(matches("br", "bring back", false) > 0.0);
        assert!(matches("brb", "bring back", false) > matches("brb", "Be Right Back", false));
        assert!(matches("강성", "서울시 강남구 삼성동", false) > 0.0);
        assert!(matches("강남", "서울시 강남구 삼성동", false) > 0.0);
        assert!(matches("ㄱㅅ", "서울시 강남구 삼성동", false) > 0.0);
        assert!(matches("강남", "서울시 강남구 삼성동", false) > matches("ㄱㄴ", "서울시 강남구 삼성동", false));
    }

    #[test]
    fn matches_when_korean_characters_partially_match() {
        assert!(matches("ㅎㄱ", "한글", false) > 0.0);
        assert!(matches("하그", "한글", false) > 0.0);
        assert!(matches("하ㄱ", "한글", false) > 0.0);
        assert!(matches("하그", "한글", false) == matches("ㅎㄱ", "한글", false));
        assert!(matches("하그", "한글", false) == matches("하ㄱ", "한글", false));
    }
}
