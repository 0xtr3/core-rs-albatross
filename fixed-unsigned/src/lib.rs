///
/// fixed-unsigned - A crate for fixed-point unsigned big integers
///
/// This was written to behave like bignumber.js
///
/// Not all traits you would expect for a number are implemented. Some things are implemented
/// just to work with Nimiq.
///
/// TODO: Parameterize all functions that use scale_down with the rounding mode.
///


extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::identities::{One, Zero};
use num_traits::{ToPrimitive, pow};

use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::marker::PhantomData;
use std::str::FromStr;
use std::fmt;
use std::error::Error;
use std::char::from_digit;
use std::cmp::Ordering;

pub mod types;
#[cfg(feature = "bigdecimal")]
pub mod bigdecimal;


/// Maximum number of digits a decimal number can have in a `u64`
const U64_MAX_DIGITS: u64 = 19u64;
/// Maximum decimal number that can be represented with an `u64`
/// `U64_MAX_DECIMAL` = 10<sup>`U64_MAX_DIGITS`</sup>
/// NOTE: If we go with 15 decimal places, this can safely fit a Javascript Integer
const U64_MAX_DECIMAL: u64 = 10000000000000000000u64;

/// Trait for a fixed scale. It only has one associated constant that must be implemented:
/// `SCALE`, which defines the place of the decimal point.
pub trait FixedScale {
    const SCALE: u64;
}

/// A trait for rounding when scaling down
pub trait RoundingMode {
    /// Scale on `int_value` where `carrier` is the last digit that was already dropped.
    fn round(int_value: BigUint, carrier: u8) -> BigUint;
}

/// Round half up - i.e. 0.4 -> 0 and 0.5 -> 1
struct RoundHalfUp {}
impl RoundingMode for RoundHalfUp{
    #[inline]
    fn round(int_value: BigUint, carrier: u8) -> BigUint {
        int_value + if carrier >= 5u8 { 1u64 } else { 0u64 }
    }
}

/// Round down - i.e. truncate
struct RoundDown {}
impl RoundingMode for RoundDown {
    #[inline]
    fn round(int_value: BigUint, carrier: u8) -> BigUint {
        int_value
    }
}



/// Error returned when a string representation can't be parse into a FixedUnsigned.
/// TODO: Attach string to it for error message
#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    Invalid
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        "Parsing failed"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

/// A trait to convert between `FixedUnsigned`s of different scale
trait ConvertScale<S, T>
    where S: FixedScale, T: FixedScale
{
    fn convert_scale(from: &FixedUnsigned<S>) -> FixedUnsigned<T>;
}

/// A fixed point unsigned integer
///
/// This is a `num_bigint::BigUint` with a fixed scale.
///
/// The fixed scale is determined by the generic `S` which implements `FixedScale` and
/// provides the constant `FixedScale::SCALE`.
#[derive(Clone, Hash)]
pub struct FixedUnsigned<S>
    where S: FixedScale
{
    int_value: BigUint,
    scale: PhantomData<S>
}

impl<S> FixedUnsigned<S>
    where S: FixedScale
{
    fn new(int_value: BigUint) -> Self {
        Self { int_value, scale: PhantomData }
    }

    /// Scales up a `BigUint` by `scale`
    /// TODO: measure performance with/without inline
    /// TODO: We could start with scaling down 10^16, then 2^8, etc. to have logarithmic runtime
    #[inline]
    pub fn scale_up(mut int_value: BigUint, mut scale: u64) -> BigUint {
        while scale >= U64_MAX_DIGITS {
            int_value *= U64_MAX_DECIMAL;
            scale -= U64_MAX_DIGITS;
        }
        while scale > 0u64 {
            int_value *= 10u64;
            scale -= 1;
        }
        int_value
    }

    /// Scales down a `BigUint` by `scale`
    /// TODO: Rounding?
    #[inline]
    pub fn scale_down<R: RoundingMode>(mut int_value: BigUint, mut scale: u64) -> BigUint {
        // scale down by 10<sup>19</sup> as long as possible
        while scale >= U64_MAX_DIGITS {
            int_value /= U64_MAX_DECIMAL;
            scale -= U64_MAX_DIGITS;
        }
        // scale down by 10 until we have to potentially round
        while scale > 1u64 {
            int_value /= 10u64;
            scale -= 1;
        }
        // round
        if scale > 0u64 {
            // unwrap is safe, since `(int_value % 10u64)` < 10
            let carrier = (&int_value % 10u64).to_u8().unwrap();
            int_value /= 10u64;
            int_value = R::round(int_value, carrier);
        }
        int_value
    }

    /// Returns the integer part as `BigUint` (i.e. the part before the decimal point)
    pub fn int_part(&self) -> BigUint {
        Self::scale_down::<RoundDown>(self.int_value.clone(), S::SCALE)
    }

    pub fn frac_part(&self) -> BigUint {
        unimplemented!();
    }

    /// Returns the scale of the `FixedUnsigned` as u64.
    ///
    /// Note that the scale is fixed by type, so for a `FixedUnsigned<S>` you can also get it as
    /// constant by `S::Scale`. This function is only for convenience.
    pub fn scale(&self) -> u64 {
        S::SCALE
    }

    pub fn from_bytes_be(bytes: &[u8]) -> Self {
        Self::new(BigUint::from_bytes_be(bytes))
    }

    pub fn from_bytes_le(bytes: &[u8]) -> Self {
        Self::new(BigUint::from_bytes_be(bytes))
    }

    pub fn to_bytes_be(&self) -> Vec<u8> {
        self.int_value.to_bytes_be()
    }

    pub fn to_bytes_le(&self) -> Vec<u8> {
        self.int_value.to_bytes_le()
    }

    /// Converts the `FixedUnsigned` to a string representation in base `radix`.
    ///
    /// # Panics
    ///
    /// This function will panic if the radix is 0 or greater than 36.
    pub fn to_radix_string(&self, radix: u8, uppercase: bool) -> String {
        if radix == 0 || radix > 36 {
            panic!("Radix too large: {}", radix);
        }
        let digits = self.int_value.to_radix_be(radix as u32);
        let mut string: String = String::new();
        let decimal_place = digits.len().checked_sub(S::SCALE as usize);
        if let Some(0) = decimal_place {
            string.push('0');
        }
        for (i, d) in digits.iter().enumerate() {
            match decimal_place {
                Some(dp) if dp == i => string.push('.'),
                _ => ()
            }
            // This unwrap is safe, because we to_radix_be only gives us digits in the right radix
            let c = from_digit(*d as u32, radix as u32).unwrap();
            string.push(if uppercase { c.to_ascii_uppercase() } else { c }); // NOTE: `form_digit` returns lower-case characters
        }
        string
    }

    /// Converts a string representation to a `FixedUnsigned` with base `radix`.
    ///
    /// # Panics
    ///
    /// This function will panic if the radix is 0 or greater than 36.
    pub fn from_radix_string(string: &str, radix: u8) -> Result<Self, ParseError> {
        if radix == 0 || radix > 36 {
            panic!("Radix too large: {}", radix);
        }
        let mut digits: Vec<u8> = Vec::new();
        let mut decimal_place = None;
        for (i, c) in string.chars().enumerate() {
            if c == '.' {
                if decimal_place.is_some() {
                    return Err(ParseError::Invalid)
                }
                decimal_place = Some(i)
            }
            else {
                digits.push(c.to_digit(radix as u32).unwrap() as u8)
            }
        }
        if digits.is_empty() {
            return Err(ParseError::Invalid);
        }
        // unscaled `int_value`
        let int_value = BigUint::from_radix_be(digits.as_slice(), radix as u32)
            .ok_or(ParseError::Invalid)?;
        // the scale of the string representation
        // NOTE: `string.len() - 1` is the number of digits. One is being subtracted for the decimal point
        let scale = decimal_place.map(|p| string.len() - p - 1).unwrap_or(0) as u64;
        // scale the unscaled `int_value` to the correct scale
        let int_value = if scale < S::SCALE {
            Self::scale_up(int_value, S::SCALE - scale)
        }
        else if scale > S::SCALE {
            Self::scale_down::<RoundDown>(int_value, scale - S::SCALE)
        }
        else {
            int_value
        };
        Ok(Self::new(int_value))
    }

    /// Converts from a BigUint to FixedScale. This scales the value appropriately, thus the
    /// result will have 0 for decimal places.
    fn from_biguint(int_value: BigUint) -> Self {
        Self::new(Self::scale_up(int_value, S::SCALE))
    }

    /// Converts to a BigUint losing the decimal places
    ///
    /// NOTE: This is not implemented as a `Into`/`From` trait to make the loss of precision implicit
    pub fn into_biguint(self) -> BigUint {
        Self::scale_down::<RoundDown>(self.int_value, S::SCALE)
    }

    pub fn into_biguint_without_scale(self) -> BigUint {
        self.int_value
    }

    pub fn bits(&self) -> usize {
        self.int_value.bits()
    }

    pub fn bytes(&self) -> usize {
        let bits = self.bits();
        return bits / 8 + if bits % 8 == 0 {0} else {1};
    }
}

impl<S> Add for FixedUnsigned<S>
    where S: FixedScale
{
    type Output = Self;

    fn add(self, rhs: FixedUnsigned<S>) -> Self::Output {
        Self::new(self.int_value + rhs.int_value)
    }
}

impl<'a, 'b, S> Add<&'b FixedUnsigned<S>> for &'a FixedUnsigned<S>
    where S: FixedScale
{
    type Output = FixedUnsigned<S>;

    fn add(self, rhs: &'b FixedUnsigned<S>) -> FixedUnsigned<S> {
        FixedUnsigned::new(&self.int_value + &rhs.int_value)
    }
}

impl<S> AddAssign for FixedUnsigned<S>
    where S: FixedScale
{
    fn add_assign(&mut self, other: Self) {
        self.int_value += other.int_value;
    }
}

impl<S> Sub for FixedUnsigned<S>
    where S: FixedScale
{
    type Output = Self;

    fn sub(self, rhs: FixedUnsigned<S>) -> Self::Output {
        Self::new(self.int_value - rhs.int_value)
    }
}

impl<'a, 'b, S> Sub<&'b FixedUnsigned<S>> for &'a FixedUnsigned<S>
    where S: FixedScale
{
    type Output = FixedUnsigned<S>;

    fn sub(self, rhs: &'b FixedUnsigned<S>) -> FixedUnsigned<S>  {
        FixedUnsigned::new(&self.int_value - &rhs.int_value)
    }
}

impl<S> SubAssign for FixedUnsigned<S>
    where S: FixedScale
{
    fn sub_assign(&mut self, other: Self) {
        self.int_value -= other.int_value;
    }
}

impl<S> Mul for FixedUnsigned<S>
    where S: FixedScale
{
    type Output = Self;

    fn mul(self, rhs: FixedUnsigned<S>) -> Self::Output {
        Self::new(Self::scale_down::<RoundHalfUp>(self.int_value * rhs.int_value, S::SCALE))
    }
}

impl<'a, 'b, S> Mul<&'b FixedUnsigned<S>> for &'a FixedUnsigned<S>
    where S: FixedScale
{
    type Output = FixedUnsigned<S>;

    fn mul(self, rhs: &'b FixedUnsigned<S>) -> FixedUnsigned<S>  {
        FixedUnsigned::new(Self::Output::scale_down::<RoundHalfUp>(&self.int_value * &rhs.int_value, S::SCALE))
    }
}

impl<S> MulAssign for FixedUnsigned<S>
    where S: FixedScale
{
    fn mul_assign(&mut self, other: Self) {
        self.int_value = Self::scale_down::<RoundHalfUp>(&self.int_value * other.int_value, S::SCALE);
    }
}

impl<S> Div for FixedUnsigned<S>
    where S: FixedScale
{
    type Output = Self;

    fn div(self, rhs: FixedUnsigned<S>) -> Self::Output {
        Self::new(Self::scale_up(self.int_value, S::SCALE) / rhs.int_value)
    }
}

impl<'a, 'b, S> Div<&'b FixedUnsigned<S>> for &'a FixedUnsigned<S>
    where S: FixedScale
{
    type Output = FixedUnsigned<S>;

    fn div(self, rhs: &'b FixedUnsigned<S>) -> FixedUnsigned<S>  {
        FixedUnsigned::new(Self::Output::scale_up(self.int_value.clone(), S::SCALE) / &rhs.int_value)
    }
}

impl<S> DivAssign for FixedUnsigned<S>
    where S: FixedScale
{
    fn div_assign(&mut self, other: Self) {
        self.int_value = Self::scale_up(&self.int_value * other.int_value, S::SCALE);
    }
}

impl<S> PartialEq for FixedUnsigned<S>
    where S: FixedScale
{
    fn eq(&self, other: &Self) -> bool {
        self.int_value.eq(&other.int_value)
    }
}

impl<S> Eq for FixedUnsigned<S>
    where S: FixedScale
{

}

impl<S> PartialOrd for FixedUnsigned<S>
    where S: FixedScale
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.int_value.partial_cmp(&other.int_value)
    }
}

impl<S> Ord for FixedUnsigned<S>
    where S: FixedScale
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.int_value.cmp(&other.int_value)
    }
}

/*
NOTE: Conflicts with implementation from crate `alloc`, because we implemented `Display`

impl<S> ToString for FixedUnsigned<S>
    where S: FixedScale
{
    fn to_string(&self) -> String {
        self.to_radix_string(10, false)
    }
}
*/

impl<S> FromStr for FixedUnsigned<S>
    where S: FixedScale
{
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_radix_string(s, 10)
    }
}

impl<S> fmt::Debug for FixedUnsigned<S>
    where S: FixedScale
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FixedUnsigned({}, scale={})", self.to_radix_string(10, false), S::SCALE)
    }
}

impl<S> fmt::Display for FixedUnsigned<S>
    where S: FixedScale
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_radix_string(10, false))
    }
}

impl<S> From<BigUint> for FixedUnsigned<S>
    where S: FixedScale
{
    fn from(int_value: BigUint) -> Self {
        Self::from_biguint(int_value)
    }
}

/*
    XXX While this is a nice thing to have, it causes a lot of conflicts

impl<S, T> From<T> for FixedUnsigned<S>
    where S: FixedScale, BigUint: From<T>, T:
{
    fn from(x: T) -> Self {
        Self::from_biguint(BigUint::from(x))
    }
}
*/

impl<S: FixedScale> From<u64> for FixedUnsigned<S> {
    fn from(x: u64) -> Self {
        Self::from_biguint(BigUint::from(x))
    }
}

impl<S: FixedScale> From<u32> for FixedUnsigned<S> {
    fn from(x: u32) -> Self {
        Self::from_biguint(BigUint::from(x))
    }
}

impl<S: FixedScale> From<u16> for FixedUnsigned<S> {
    fn from(x: u16) -> Self {
        Self::from_biguint(BigUint::from(x))
    }
}

impl<S: FixedScale> From<u8> for FixedUnsigned<S> {
    fn from(x: u8) -> Self {
        Self::from_biguint(BigUint::from(x))
    }
}

impl<S, T> ConvertScale<S, T> for FixedUnsigned<S>
    where S: FixedScale, T: FixedScale
{
    fn convert_scale(from: &FixedUnsigned<S>) -> FixedUnsigned<T> {
        unimplemented!();
    }
}

impl<S> Zero for FixedUnsigned<S>
    where S: FixedScale
{
    fn zero() -> Self {
        Self::new(BigUint::zero())
    }

    fn is_zero(&self) -> bool {
        self.int_value.is_zero()
    }
}

impl<S> One for FixedUnsigned<S>
    where S: FixedScale
{
    fn one() -> Self {
        Self::from(1u64)
    }
}

impl<S> Default for FixedUnsigned<S>
    where S: FixedScale
{
    fn default() -> Self {
        Self::zero()
    }
}

/// Converts a `f64` to a `FixedUnsigned`
///
/// TODO: Do checked operations and panic if anything fails or return an Option
impl<S: FixedScale> From<f64> for FixedUnsigned<S> {
    fn from(x: f64) -> Self {
        // scale up the float and drop the decimals (i.e. cast to u64)
        // let scaled = x * 10f64.powi(S::SCALE as i32);
        let scaled = x * pow(10f64, S::SCALE as usize);
        Self::new(BigUint::from(scaled as u64))
    }
}
