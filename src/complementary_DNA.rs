fn DNA_strand(dna: &str) -> String {
    let mut rs = String::new();
    for x in dna.as_bytes() {
        let xc = *x as char;
        match xc {
            'A' => rs.push('T'),
            'T' => rs.push('A'),
            'G' => rs.push('C'),
            'C' => rs.push('G'),
            _ => rs.push(' ')
        }
    }

    rs
}


// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn returns_expected() {
    assert_eq!(DNA_strand("AAAA"), "TTTT");
    assert_eq!(DNA_strand("ATTGC"), "TAACG");
    assert_eq!(DNA_strand("GTAT"), "CATA");
}