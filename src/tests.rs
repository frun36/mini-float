#[cfg(test)]
use super::*;

#[test]
fn f32_from_f8_test() {
    let f = f8::from_byte(0b01111111);
    assert_eq!(31.0_f32, f32::from(f));

    let f = f8::from_byte(0b11111111);
    assert_eq!(-31.0_f32, f32::from(f));

    let f = f8::from_byte(0b00111111);
    assert_eq!(1.9375_f32, f32::from(f));

    let f = f8::from_byte(0b10111111);
    assert_eq!(-1.9375_f32, f32::from(f));

    let f = f8::from_byte(0b00000000);
    assert_eq!(0_f32, f32::from(f));

    let f = f8::from_byte(0b10000000);
    assert_eq!(0_f32, f32::from(f));
}

#[test]
fn f8_from_f32_test() {
    let f = f8::from_f32(5.25);
    assert_eq!(5.25_f32, f32::from(f));

    let f = f8::from_f32(-5.25);
    assert_eq!(-5.25_f32, f32::from(f));

    let f = f8::from_f32(-534.25);
    assert_eq!(-0.0, f32::from(f));

    let f = f8::from_f32(534.25);
    assert_eq!(-0.0, f32::from(f));

    let f = f8::from_f32(31.);
    assert_eq!(31_f32, f32::from(f));

    let f = f8::from_f32(-31.);
    assert_eq!(-31_f32, f32::from(f));

    let f = f8::from_f32(0.8125);
    assert_eq!(0.8125_f32, f32::from(f));

    let f = f8::from_f32(-0.8125);
    assert_eq!(-0.8125_f32, f32::from(f));

    let f = f8::from_f32(0.);
    assert_eq!(0_f32, f32::from(f));

    let f = f8::from_f32(-0.);
    assert_eq!(-0_f32, f32::from(f));
}
