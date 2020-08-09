// convert strings to pig latin. the first consonant of each word is moved to the end of the word and 'ay' is added,
// so first becomes 'irst-fay'. words that start with a vowel have 'hay' added to the end insted. so 'apple' becomes
// 'apple-hay'.

#[derive(Debug)]
pub struct PigLatin {
    pub word: String,
}

impl PigLatin {
    pub fn pig_latin_converter(&self) -> String {
        let vowels: &str = "aeiou";
        let word: &str = &self.word;
        let mut res = String::new();
        for i in word.chars() {
            if vowels.contains(i) || vowels.to_uppercase().contains(i) {
                res = format!("{}-hay", word);
            } else {
                res = format!("{}-{}ay", word.replace(i, ""), i);
            }
            break;
        }
        res
    }
}
