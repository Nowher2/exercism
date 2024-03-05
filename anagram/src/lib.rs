use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_cased_word = word.to_lowercase();
    let mut word_chars: Vec<char> = lower_cased_word.chars().collect();
    word_chars.sort();

    possible_anagrams
        .iter()
        .filter(|candidate| {
            is_anagram(&lower_cased_word, &word_chars, candidate)
        }).copied().collect::<HashSet<&str>>()
}

pub fn is_anagram(word: &str, word_chars: &Vec<char>, candidate: &str) -> bool {
    if word.len() != candidate.len() {
        return false;
    }
    let lower_cased_candidate = candidate.to_lowercase();
    if word == lower_cased_candidate {
        return false;
    }
    let mut candidate_chars: Vec<char> = lower_cased_candidate.chars().collect();
    candidate_chars.sort();
    candidate_chars == *word_chars
}
