use super::ElmExport;

#[allow(dead_code)]
struct Primitives {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    e: usize,
    f: i8,
    g: i16,
    h: i32,
    i: i64,
    j: isize,
}

impl ElmExport for Primitives {}
