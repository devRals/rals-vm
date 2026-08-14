use rals_assembler::*;

#[test]
fn parses_header_section() {
    let src = "section .header\n    arch: 32\n";
    let prog = parse(src).unwrap();
    assert_eq!(prog.sections.len(), 1);
    assert_eq!(prog.sections[0].name, "header");
}

#[test]
fn parses_label_and_instruction() {
    let src = "section .text\n_start:\n    ldi r0, 0x5\n";
    let prog = parse(src).unwrap();
    let items = &prog.sections[0].items;
    assert!(matches!(items[0], ast::Item::Label(ref s) if s == "_start"));
    match &items[1] {
        ast::Item::Instruction(i) => {
            assert_eq!(i.mnemonic, "ldi");
            assert_eq!(i.operands.len(), 2);
        }
        _ => panic!("expected instruction"),
    }
}

#[test]
fn full_example() {
    let src = std::fs::read_to_string("./example.rasm").unwrap();
    let prog = parse(&src).unwrap();
    let text_sec = &prog.sections[1];
    assert_eq!(prog.sections.len(), 2);
    assert_eq!(text_sec.items.len(), 7)
}
