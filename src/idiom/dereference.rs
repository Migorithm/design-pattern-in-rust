
pub fn three_vowels(word: &str) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true
                }
            }
            _ => vowel_count = 0
        }
    }
    false
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_three_vowels(){
        assert_eq!(three_vowels("Ferris"),false);

        //Here, even though we pass &String, it works thanks to defer coercion.
        assert_eq!(three_vowels(&"Curious".to_string()),true);

        let sentence_string =
        "Once upon a time, there was a friendly curious crab named Ferris".to_string();

        for word in sentence_string.split(' '){
            if three_vowels(word){
                println!("{} has three consecutive vowels!",word);
            }
        }
    }
}