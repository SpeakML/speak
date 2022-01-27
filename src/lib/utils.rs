#![allow(dead_code, unused_variables)]

// * /////////////////////////////
// ^ CONFIG //////////////////////
// * /////////////////////////////

pub struct Config {
    pub multiplier: u32,
    pub threshold:  f32,
    pub memory:     i32
}

pub static CONFIG: Config = Config {
    multiplier: 1
    ,threshold: 0.1
    ,memory:    1
};

// * /////////////////////////////
// ^ Maps ////////////////////////
// * /////////////////////////////

// A map only allows these types: String, &str

pub(crate) trait Literal {
    fn literal(&self) -> String;
}

// Using the .literal() method on a string or &str returns the String.

impl Literal for String { fn literal(&self) -> String { return String::from(self)} }
impl Literal for &str { fn literal(&self) -> String { return String::from(*self); } }

pub struct Map<T> { entries: Vec<(T, T)> }
pub(crate) struct Deconstructed<T: Literal> { pub keys: Vec<T>, pub values: Vec<T> }

// Creates a new map

pub(self) fn __new__<T: Literal>() -> Map<T> { return Map { entries: Vec::new() } }

// Creates a new map with the given entries

pub(self) fn __from__<T: Literal>(vec1: Vec<T>, vec2: Vec<T>) -> Map<String> {
    let mut entries: Vec<(String, String)> = Vec::new();
    for i in 0..vec1.len() {
        entries.push((vec1[i].literal().clone(), vec2[i].literal().clone()));
    }
    return Map { entries }; }

pub(crate) fn deconstruct<T: Literal>(map: Map<T>) -> Deconstructed<String> {
    let mut keys: Vec<String> = Vec::new();
    let mut values: Vec<String> = Vec::new();

    for x in 0..map.entries.len() {
        if x % 2 == 0 { keys.push(map.entries[x].0.literal()); }
        else { values.push(map.entries[x].1.literal()); }
    };

    return Deconstructed { keys, values };
}

macro_rules! impl_map {
    ($($T: path),*) => {
        $(
            impl Map<$T> {
                pub fn new() -> Map<$T> { return __new__::<$T>(); }
                pub fn from(vec1: Vec<$T>, vec2: Vec<$T>) -> Map<String> { return __from__::<$T>(vec1, vec2); }
            }
        )*
    };
}

type T = String;
type U = &'static str; 

impl_map!(T, U);

// I'm so proud of this thing.

// * /////////////////////////////
// ^ MISC. ///////////////////////
// * ////////////////////////////

pub(crate) fn translate<L: Literal>(vec: Vec<L>) -> Vec<Vec<u32>> {
    let mut ram: Vec<u32> = Vec::new();
    let mut result: Vec<Vec<u32>> = Vec::new();
    let mut sum: u32 = 0;
    for word in vec {
        let word = word.literal();
        for (i, word) in word.split_whitespace().enumerate() {
            for c in word.chars() {
                sum += CONFIG.multiplier * c as u32;
            };
            ram.push(sum);
            sum = 0;
        };
        result.push(ram.clone());
        ram.clear();
    };
        return result;
}