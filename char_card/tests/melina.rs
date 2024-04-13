use char_card::Character;

#[test]
fn test_parse_melina() {
    let bytes = include_bytes!("data/melina.png");
    let char = Character::from_png(&bytes[..]);
    assert!(char.is_ok());
    let char = unsafe { char.unwrap_unchecked() };
    assert!(char.name() == "Melina");
}
