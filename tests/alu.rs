use rals_vm::cpu::alu::Alu32;

#[test]
fn test_alu_flags() {
    let result = Alu32::new().add(u32::MAX, 1);
    let flags = result.flags;

    assert!(flags.zero);
    assert!(!flags.overflow);
    assert!(flags.carry);
    assert!(!flags.sign);

    let result = Alu32::new().sub(0, 1);
    let flags = result.flags;

    assert!(!flags.zero);
    assert!(flags.sign);
    assert!(!flags.overflow);
    assert!(flags.carry);

    let result = Alu32::new().add(i32::MAX as u32, 1);
    let flags = result.flags;

    assert!(!flags.zero);
    assert!(flags.sign);
    assert!(flags.overflow);
    assert!(!flags.carry);

    let result = Alu32::new().shl(0x8000_0000, 1);
    let flags = result.flags;

    assert!(flags.overflow);
    assert!(flags.carry);
    assert!(flags.zero);
    assert!(!flags.sign);
}
