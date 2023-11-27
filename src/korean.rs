/// Check whether a letter is a vowel.
pub fn is_consonant(c: &char) -> bool {
    match c {
        '\u{3131}'..='\u{314e}' => true,
        _ => false
    }
}

/// Check whether a letter is a syllable.
pub fn is_syllable(c: &char) -> bool {
    match c {
        '\u{ac00}'..='\u{d7a3}' => true,
        _ => false
    }
}

///  Check whether a letter is a Korean letter.
pub fn is_korean(c: &char) -> bool {
    return is_consonant(c) || is_syllable(c);
}

/// Check whether two Korean letters are similar.
pub fn is_similar(c1: &char, c2: &char) -> bool {
    if c1 == c2 {
        return true;
    }
    if is_consonant(c1) || is_consonant(c2) {
        let cc1 = get_consonant(c1);
        let cc2 = get_consonant(c2);
        return cc1 == cc2;
        // return get_consonant(c1) == get_consonant(c2);
    }

    return omit_final(c1) == omit_final(c2);
}

/// Check whether a Korean letter has a final consonant.
pub fn has_final(c: &char) -> bool {
    if !is_syllable(c) {
        return false;
    }

    let code = (c.clone() as u32) - 0xac00;

    return (code % 28) > 0;
}

/// Remove the final consonant from a Korean letter.
fn omit_final(c: &char) -> char {
    if !has_final(c) {
        return c.clone();
    }

    let code = ((c.clone() as u32) - 0xac00) / 28 * 28 + 0xac00;

    return std::char::from_u32(code).unwrap();
}

/// Get the first consonant of a Korean letter.
fn get_consonant(c: &char) -> char {
    let consonant_syllables = [
        ('ㄱ', '가'),
        ('ㄲ', '까'),
        ('ㄴ', '나'),
        ('ㄷ', '다'),
        ('ㄸ', '따'),
        ('ㄹ', '라'),
        ('ㅁ', '마'),
        ('ㅂ', '바'),
        ('ㅃ', '빠'),
        ('ㅅ', '사'),
        ('ㅆ', '싸'),
        ('ㅇ', '아'),
        ('ㅈ', '자'),
        ('ㅉ', '짜'),
        ('ㅊ', '차'),
        ('ㅋ', '카'),
        ('ㅌ', '타'),
        ('ㅍ', '파'),
        ('ㅎ', '하'),
    ];

    if !is_syllable(c) {
        return c.clone();
    }

    let without_final = (((c.clone() as u32) - 0xac00) / 588) * 588 + 0xac00;
    let syllable = std::char::from_u32(without_final).unwrap();

    for (_, (con, syl)) in consonant_syllables.iter().enumerate() {
        if *syl == syllable {
            return *con;
        }
    }

    return '\u{0000}';
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_korean() {
        assert!(is_korean(&'강'));
        assert!(is_korean(&'핳'));
        assert!(is_korean(&'ㅅ'));
        assert!(!is_korean(&'a'));
        assert!(!is_korean(&'*'));
        assert!(!is_korean(&'🍗'));
    }

    #[test]
    fn test_similar_when_both_are_the_same() {
        assert!(is_similar(&'강', &'강'));
        assert!(is_similar(&'나', &'나'));
        assert!(is_similar(&'나', &'나'));
    }

    #[test]
    fn test_similar_when_partial_matches() {
        assert!(is_similar(&'ㄱ', &'강'));
        assert!(!is_similar(&'ㄲ', &'강'));
        assert!(is_similar(&'가', &'강'));
        assert!(!is_similar(&'거', &'강'));
    }
}