#![no_std]

mod tests;

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
    pub fn from_byte(byte: u8) -> Self {
        Self(byte)
    }

    #[inline]
    pub fn from_f32(x: f32) -> Self {
        Self::from(x)
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

        let conv = Conv32 { bits: result };
        unsafe {
            // println!("{:#034b} {}", conv.bits, conv.value);
            conv.value
        }
    }
}

impl From<f32> for f8 {
    fn from(x: f32) -> Self {
        let mut result: u8 = 0;
        let conv = Conv32 { value: x };

        let bits = unsafe {
            conv.bits
        };

        let sgn: u8 = (bits >> 31) as u8 & 1_u8;
        let exp: u8 = ((bits >> F32_MAN_LEN) & ((1 << F32_EXP_LEN) - 1)) as u8;
        let man: u8 = ((bits >> (F32_MAN_LEN-F8_MAN_LEN)) & ((1 << F8_MAN_LEN) - 1)) as u8;

        if !(F32_EXP_BIAS - F8_EXP_LEN..=7 + F32_EXP_BIAS - F8_EXP_LEN).contains(&exp){
            return f8::from_byte(0b10000000);
        }

        let exp = exp + F8_EXP_BIAS - F32_EXP_BIAS;

        result |= sgn << 7;
        result |= exp << F8_MAN_LEN;
        result |= man;

        f8::from_byte(result)
    }
}
