use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut set: HashSet<&str> = HashSet::new();
    for s in possible_anagrams {
        if word.len() != s.len() {
            continue;
        }
        let lower_cased_word = word.to_lowercase();
        let lower_cased_candidate = s.to_lowercase();
        if lower_cased_word.eq(&lower_cased_candidate) {
            continue;
        }
        let mut word_chars: Vec<char> = lower_cased_word.chars().collect();
        word_chars.sort();
        let mut candidate_chars: Vec<char> = lower_cased_candidate.chars().collect();
        candidate_chars.sort();
        let mut is_anagram = true;
        for i in 0..word_chars.len() {
            if word_chars[i] != candidate_chars[i] {
                is_anagram = false;
                break;
            }
        }
        if is_anagram {
            set.insert(s);
        }
    }
    set
}
