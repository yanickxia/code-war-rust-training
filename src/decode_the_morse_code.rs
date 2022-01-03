// Preloaded:
//
#![feature(split_ascii_whitespace)]

use std::collections::HashMap;

struct MorseDecoder {
    morse_code: HashMap<String, String>,
}

//
// MorseDecoder::new() populates the morse_code map, e.g. ".-" -> "A".


impl MorseDecoder {
    fn new() -> MorseDecoder {
        let mut md = MorseDecoder {
            morse_code: HashMap::new()
        };
        md.morse_code.insert(String::from(".-"), String::from("A"));
        md.morse_code.insert(String::from("-..."), String::from("B"));
        md.morse_code.insert(String::from("-.-."), String::from("C"));
        md.morse_code.insert(String::from("-.."), String::from("D"));
        md.morse_code.insert(String::from("."), String::from("E"));
        md.morse_code.insert(String::from("..-."), String::from("F"));
        md.morse_code.insert(String::from("--."), String::from("G"));
        md.morse_code.insert(String::from("...."), String::from("H"));
        md.morse_code.insert(String::from(".."), String::from("I"));
        md.morse_code.insert(String::from(".---"), String::from("J"));
        md.morse_code.insert(String::from("-.-"), String::from("K"));
        md.morse_code.insert(String::from(".-.."), String::from("L"));
        md.morse_code.insert(String::from("--"), String::from("M"));
        md.morse_code.insert(String::from("-."), String::from("N"));
        md.morse_code.insert(String::from("---"), String::from("O"));
        md.morse_code.insert(String::from(".--."), String::from("P"));
        md.morse_code.insert(String::from("--.-"), String::from("Q"));
        md.morse_code.insert(String::from(".-."), String::from("R"));
        md.morse_code.insert(String::from("..."), String::from("S"));
        md.morse_code.insert(String::from("-"), String::from("T"));
        md.morse_code.insert(String::from("..-"), String::from("U"));
        md.morse_code.insert(String::from("...-"), String::from("V"));
        md.morse_code.insert(String::from(".--"), String::from("W"));
        md.morse_code.insert(String::from("-..-"), String::from("X"));
        md.morse_code.insert(String::from("-.--"), String::from("Y"));
        md.morse_code.insert(String::from("--.."), String::from("Z"));
        md
    }


    fn decode_morse(&self, encoded: &str) -> String {
        encoded
            .trim()
            .split("   ")
            .map(|x| x.split(' ')
                .filter_map(|y| { self.morse_code.get(y) })
                .cloned()
                .collect())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hey_jude() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse(".... . -.--   .--- ..- -.. ."), "HEY JUDE");
    }
}
