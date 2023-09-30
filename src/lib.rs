#![no_std]

#[allow(dead_code)]
const F32_EXP_LEN: u8 = 8;
const F32_EXP_BIAS: u8 = 127;
const F32_MAN_LEN: u8 = 23;

const F8_EXP_LEN: u8 = 3;
const F8_EXP_BIAS: u8 = 3;
const F8_MAN_LEN: u8 = 4;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct f8(u8);

impl f8 {
    #[inline]
    pub fn new_from_byte(byte: u8) -> Self {
        Self(byte)
    }

    #[inline]
    fn get_sgn(&self) -> u8 {
        (self.0 >> 7) & 1_u8
    }

    #[inline]
    fn get_exp(&self) -> u8 {
        (self.0 >> 4) & ((1_u8 << F8_EXP_LEN) - 1)
    }

    #[inline]
    fn get_man(&self) -> u8 {
        self.0 & ((1_u8 << F8_MAN_LEN) - 1)
    }

    pub fn as_byte(&self) -> u8 {
        self.0
    }

    pub fn as_f32(&self) -> f32 {
        f32::from(*self)
    }
}

union Conv32 {
    bits: u32,
    value: f32,
}

impl From<f8> for f32 {
    fn from(x: f8) -> Self {
        if x.0 == 0 {
            return 0.;
        }
        if x.0 == 128 {
            return -0.;
        }

        // println!("{:#010b}", x.0);
        let mut result: u32 = 0;
        let sgn: u32 = x.get_sgn().into();
        let exp: u32 = x.get_exp().into();
        let man: u32 = x.get_man().into();

        // println!("{:#034b}\n{:#034b}\n{:#034b}", sgn, exp, significand);

        // Setting sgn
        result |= sgn << 31;

        // Setting exp
        result |= (exp + (F32_EXP_BIAS - F8_EXP_BIAS) as u32) << F32_MAN_LEN;

        // Setting significand
        result |= man << (F32_MAN_LEN - F8_MAN_LEN);

        let u = Conv32 { bits: result };
        unsafe {
            // println!("{:#034b} {}", u.bits, u.value);
            u.value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let f = f8::new_from_byte(0b01111111);
        assert_eq!(31.0_f32, f32::from(f));

        let f = f8::new_from_byte(0b11111111);
        assert_eq!(-31.0_f32, f32::from(f));

        let f = f8::new_from_byte(0b00111111);
        assert_eq!(1.9375_f32, f32::from(f));

        let f = f8::new_from_byte(0b10111111);
        assert_eq!(-1.9375_f32, f32::from(f));

        let f = f8::new_from_byte(0b00000000);
        assert_eq!(0_f32, f32::from(f));

        let f = f8::new_from_byte(0b10000000);
        assert_eq!(0_f32, f32::from(f));
    }
}
