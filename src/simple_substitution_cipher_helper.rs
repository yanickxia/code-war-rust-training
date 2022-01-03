use std::collections::HashMap;

struct Cipher {
    encodeMapping: HashMap<u8, u8>,
    decodeMapping: HashMap<u8, u8>,
}

impl Cipher {
    fn new(map1: &str, map2: &str) -> Cipher {
        let mut c = Cipher {
            encodeMapping: HashMap::new(),
            decodeMapping: HashMap::new(),
        };

        for i in 0..map1.len() {
            let a = map1.as_bytes()[i];
            let b = map2.as_bytes()[i];

            c.encodeMapping.insert(a, b);
            c.decodeMapping.insert(b, a);
        }

        c
    }

    fn encode(&self, string: &str) -> String {
        let mut x = String::new();

        for as_byte in string.as_bytes() {
            let v = self.encodeMapping.get(as_byte);
            if v.is_some() {
                x.push(*v.unwrap() as char)
            } else {
                x.push(*as_byte as char)
            }
        }

        x
    }

    fn decode(&self, string: &str) -> String {
        let mut x = String::new();

        for as_byte in string.as_bytes() {
            let v = self.decodeMapping.get(as_byte);
            if v.is_some() {
                x.push(*v.unwrap() as char)
            } else {
                x.push(*as_byte as char)
            }
        }

        x
    }
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn examples() {
    let map1 = "abcdefghijklmnopqrstuvwxyz";
    let map2 = "etaoinshrdlucmfwypvbgkjqxz";

    let cipher = Cipher::new(map1, map2);

    assert_eq!(cipher.encode("abc"), "eta");
    assert_eq!(cipher.encode("xyz"), "qxz");
    assert_eq!(cipher.decode("eirfg"), "aeiou");
    assert_eq!(cipher.decode("erlang"), "aikcfu");
}