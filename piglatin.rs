/*
Convert strings to pig latin. The first consonant of each word is moved
to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
Keep in mind the details about UTF-8 encoding!
*/

use regex::Regex;

fn main() {
    let result = pig_latin("tupple");
    println!("{}", result);
}

fn pig_latin(str: &str) -> String {
    let mut pig = String::from(str);
    let reg = Regex::new(r"[aeiou]").unwrap();

    for char in pig.chars() {
        let fst = format!("{}", char);
        if reg.is_match(&fst) {
            pig.push_str("-hay");
        } else {
            pig = pig.replace(&fst, "");
            let mods = vec!["-", &fst, "ay"];
            for e in mods {
                pig.push_str(e);
            }
        }
        break;
    }
    pig
}
