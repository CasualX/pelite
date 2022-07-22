use pelite::pattern as pat;

#[allow(dead_code)]
const MY_PATTERN: &[pat::Atom] = pat!("55 8BEC 5D E9$ 55 8BEC 56 8B35*{'}");

#[test]
fn main() {
    let pat = pat!("abcdef");
    assert_eq!(pat, {
        use pat::Atom::*;
        &[Save(0), Byte(0xab), Byte(0xcd), Byte(0xef)]
    });
}
