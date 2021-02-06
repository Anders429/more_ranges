#![no_std]

pub struct RangeFromExclusive<Idx> {
    start: Idx,
}

pub struct RangeFromExclusiveToInclusive<Idx> {
    start: Idx,
    end: Idx,
}

pub struct RangeFromExclusiveToExclusive<Idx> {
    start: Idx,
    end: Idx,
}
