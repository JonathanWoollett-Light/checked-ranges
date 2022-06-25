use num::Zero;
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    convert::TryFrom,
    mem,
    ops::{Deref, Range},
};

const START_AFTER_END_ERR: &str =
    "The start of the given range is greater than the end of the given range";
const END_GREATER_THAN_ZERO: &str = "The end of the given range is greater than zero";
const START_LESS_THAN_ZERO: &str = "The start of the given range is less than zero";

pub trait RangeExt {
    /// `self` covers `other`.
    fn covers(&self, other: &Self) -> bool;
    /// `self` intersects `other`.
    fn intersects(&self, other: &Self) -> bool;
}

impl<T: Ord> RangeExt for Range<T> {
    fn covers(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }
    fn intersects(&self, other: &Self) -> bool {
        self.start <= other.end && self.end <= other.start
    }
}

/// An ascending range from `start` to `end` (exclusive) where `start` is positive.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PositiveAscendingRange<T: Zero + Ord>(Range<T>);
impl<T: Zero + Ord> Deref for PositiveAscendingRange<T> {
    type Target = Range<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: Zero + Ord> TryFrom<Range<T>> for PositiveAscendingRange<T> {
    type Error = &'static str;
    fn try_from(r: Range<T>) -> Result<Self, Self::Error> {
        match r.start.cmp(&r.end) {
            Less | Equal if r.start >= T::zero() => Ok(Self(r)),
            Less | Equal => Err(START_LESS_THAN_ZERO),
            Greater => Err(START_AFTER_END_ERR),
        }
    }
}
impl<T: Zero + Ord> PositiveAscendingRange<T> {
    pub fn start(&self) -> &T {
        &self.start
    }
    pub fn end(&self) -> &T {
        &self.end
    }
    pub fn set_start(&mut self, x: T) -> Result<T, &str> {
        if x <= self.0.end && x > T::zero() {
            Ok(mem::replace(&mut self.0.start, x))
        } else {
            Err("Start greater than end or less than zero")
        }
    }
    pub fn set_end(&mut self, x: T) -> Result<T, &str> {
        if x >= self.0.start {
            Ok(mem::replace(&mut self.0.end, x))
        } else {
            Err("End less than start")
        }
    }
}

/// An ascending range from `start` to `end` (exclusive) where `end` is negative.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NegativeAscendingRange<T: Zero + Ord>(Range<T>);
impl<T: Zero + Ord> Deref for NegativeAscendingRange<T> {
    type Target = Range<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: Zero + Ord> TryFrom<Range<T>> for NegativeAscendingRange<T> {
    type Error = &'static str;
    fn try_from(r: Range<T>) -> Result<Self, Self::Error> {
        match r.start.cmp(&r.end) {
            Less | Equal if r.end <= T::zero() => Ok(Self(r)),
            Less | Equal => Err(END_GREATER_THAN_ZERO),
            Greater => Err(START_AFTER_END_ERR),
        }
    }
}
impl<T: Zero + Ord> NegativeAscendingRange<T> {
    pub fn start(&self) -> &T {
        &self.start
    }
    pub fn end(&self) -> &T {
        &self.end
    }
    pub fn set_start(&mut self, x: T) -> Result<T, &str> {
        if x <= self.0.end {
            Ok(mem::replace(&mut self.0.start, x))
        } else {
            Err("Start greater than end")
        }
    }
    pub fn set_end(&mut self, x: T) -> Result<T, &str> {
        if x >= self.0.start && x < T::zero() {
            Ok(mem::replace(&mut self.0.end, x))
        } else {
            Err("End less than start or greater than zero")
        }
    }
}

/// An ascending range from `start` to `end` (exclusive).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AscendingRange<T: Ord>(Range<T>);
impl<T: Ord> Deref for AscendingRange<T> {
    type Target = Range<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: Ord> TryFrom<Range<T>> for AscendingRange<T> {
    type Error = &'static str;
    fn try_from(r: Range<T>) -> Result<Self, Self::Error> {
        match r.start.cmp(&r.end) {
            Less | Equal => Ok(Self(r)),
            Greater => Err(START_AFTER_END_ERR),
        }
    }
}
impl<T: Zero + Ord> AscendingRange<T> {
    pub fn start(&self) -> &T {
        &self.start
    }
    pub fn end(&self) -> &T {
        &self.end
    }
    pub fn set_start(&mut self, x: T) -> Result<T, &str> {
        if x <= self.0.end {
            Ok(mem::replace(&mut self.0.start, x))
        } else {
            Err("Start greater than end")
        }
    }
    pub fn set_end(&mut self, x: T) -> Result<T, &str> {
        if x >= self.0.start {
            Ok(mem::replace(&mut self.0.end, x))
        } else {
            Err("End less than start")
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
