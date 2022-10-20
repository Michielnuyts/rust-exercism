use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let note_counts = count_words(note);
    let magazine_counts = count_words(magazine);

    for (word, note_count) in note_counts {
        let magazine_count = magazine_counts.get(&word).unwrap_or(&0);

        if *magazine_count < note_count {
            return false;
        }
    }

    true
}

fn count_words(words: &[&str]) -> HashMap<String, u16> {
    let mut word_counts = HashMap::<String, u16>::new();

    for &current in words {
        word_counts
            .entry(current.into())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    word_counts
}
