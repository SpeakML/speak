//! Speak crate made by Alex G. C. Copyright (c) 2022. See LICENSE for more information about the copyright

// Ok I just read that (Except for addition) using floats is faster than ints, eh?
// look http://www.phys.ufl.edu/~coldwell/MultiplePrecision/fpvsintmult.htm is this real
#[path = "libs/literal.rs"]
mod lit;
use lit::*;

#[path = "libs/mapping.rs"]
mod map;
use map::*;

#[path = "libs/chunks.rs"]
mod chk;
use chk::*;

//
// ────────────────────────────────────────────────────────────────────────────────────── I ──────────
//   :::::: C O N F I G U R A T I O N   A N D   U T I L S : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────────────────────────────────────────
//

// Explicit docs because it uses fancy graphics

#[cfg_attr(doc, aquamarine::aquamarine)]
///## Memory
///
///Every phrase is made up from words. We make a phrase from adding sequences of words together. Well, ///the `memory` parameter is used to define how many words we take into account into analyzing a phrase.
///
///The functions that takes this parameter take into account that maybe the length of the phrase divided ///by the number of words in the phrase is not an integer. So this functions will take into account ///until the last words, and then scan the words between the length of the phrase minus the memory and ///the length of the word.
///
///```mermaid
///graph TD
///	A("Hi,")
///	B("my")
///	C{{"name"}}
///	D("is")
///	E("Alex")
///
///	F["Not found!"]
///
///	style F stroke-dasharray: 5 5
///
///	X["Iteration 1"]
///	Y["Iteration 2"]
///	Z["Bugged iteration 2"]
///
///	X-->A;
///	X-->B;
///	X-->C;
///
///	Y-->C;
///	Y-->D;
///	Y-->E;
///
///	Z-->D;
///	Z-->E;
///	Z-->F;
///```
///
///###### Honestly, I just wanted to show you how it works, and this graph.
pub const DEFAULT_MEMORY: usize = 2;

///## Threshold
///As you know, we divide two values to find their relations. Well, that relation is then checked against the threshold, if it doesn't passes the threshold, the word is not elected.
///This is the operation to determine if a word is elected. As you can see, if the threshold is too low (less than 0.1 is not recommended), the word "spaghetti" and the word "spagetti" will not be relationated. But if the threshold is too high (more than 0.3 is not recommended), a lot of words, even if they are very different, will be relationated and the final result will not have sense.
pub const DEFAULT_THRESHOLD: f32 = 0.1;
pub const DEFAULT_OUTPUT_LENGTH: usize = 2;

fn translate(vec: &Vec<String>) -> Vec<Vec<u16>> {
	let mut result: Vec<Vec<u16>> = Vec::new();
	let mut new_phrase: Vec<u16> = Vec::new();
	let mut sum: u16 = 0;
	for phrase in vec {
		for word in phrase.split_whitespace() {
			for c in word.chars() {
				sum += c as u16;
			}
			new_phrase.push(sum.pow(10 / 9));
			sum = 0;
		}
		result.push(new_phrase.clone());
		new_phrase.clear()
	}

	return result;
}

// fn merge_hashmaps<T: std::hash::Hash + std::cmp::Eq>(map1: HashMap<T, T>, map2: HashMap<T, T>) -> HashMap<T, T> {
// 	map1.into_iter().chain(map2).collect()
// }

macro_rules! checkmem {
	($mem: expr, $($key: expr, $keyname: ident),*) => {
		$(
			$keyname = if $mem > $key.len() {
				$key.len()
			} else {
				$mem
			};
		)*
	};
}

//
// ────────────────────────────────────────────────────────────────── I ──────────
//   :::::: M A I N   F U N C T I O N : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────────────────────
//

#[path = "libs/algorithm/learn.rs"]
mod learn;
pub use learn::*;

// as this is the most complex file in the whole project, I'll give its own file.

#[path = "libs/algorithm/run.rs"]
mod run;
pub use run::*;