//! Speak crate made by Alex G. C. Copyright (c) 2022. See LICENSE for more information about the copyright

// Ok I just read that (Except for addition) using floats is faster than ints, eh?
// look http://www.phys.ufl.edu/~coldwell/MultiplePrecision/fpvsintmult.htm is this real
#[path = "libs/literal.rs"]
mod lit;
use std::{ops::Deref, collections::btree_map::OccupiedEntry, ffi::FromVecWithNulError};

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
	($mem: expr, $($key: ident, $keyname: ident),*) => {
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
// ────────────────────────────────────────────────── I ──────────
//   :::::: L E A R N : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────
//

// __learn__(...) wrapper
///# `Learn(...)`
///
///`learn(...)` is one of the main functions in the library, this function is used to train the algorithm with a given dataset. This **dataset being a `Hashmap<T, T>`** (in `std::collections::HashMap`).
///
///## Parameters and types
///
///`data: Hashmap<T, T>`: The dataset to train the algorithm with.
///`T`: The type of the dataset. (`String` or `&str`).
///`memory: usize`: Especial parameter. (Optional).
///
///As always, I recommend to use the default values for the special parameters using `None` as those parameters.
///
///For more information about the special parameters, see [special parameters](special-parameters.md)
///
///**T** is the main type of the dataset, it must satisfy the following traits:
///
///* `speak::Literal<String>`
///* `Clone`
///* `ToString`
///
///Both `&str` and `String` satisfies all these traits. In both of the main functions every single string is converted to a `String`.
///
///## Example
///
///```rust
///let map: HashMap<&str, &str> = HashMap::from(
///	vec![
///		("hello", "world"),
///		("hola", "mundo")
///	]
///);
///
///// I want to use the default parameter, so I'll use 'None'.
///
///let learned = learn(map, None);
///```
///
///## Details
///
///**Be careful with this function**, because this function takes time.
///If you need to create a closed feedback loop (training with newly created data), you can use the `relearn_direct(...)` function. In the case that you want to add data and still hash the dataset, you can use `relearn_indirect(...)`, this will return a `HashMap` and you can serialize and store it somewhere.
///
pub fn learn<T: Literal<String> + Clone + ToString>(
	data: &std::collections::HashMap<T, T>,
	memory: Option<usize>,
) -> (Vec<Vec<f32>>, Map<Vec<u16>>, Map<String>) {
	let x: Map<T> = data.clone().to_map();
	let new_map: Map<String> = Map::<String> {
		keys: x.keys.literal(),
		values: x.values.literal(),
	};

	print!("{:?}", new_map.keys);
	print!("{:?}", new_map.values);

	if let Some(mem) = memory {
		return __learn__(new_map, mem);
	} else {
		return __learn__(new_map, DEFAULT_MEMORY);
	}
}

// The main algorithm
fn __learn__<'a>(rawdata: Map<String>, memory: usize) -> (Vec<Vec<f32>>, Map<Vec<u16>>, Map<String>) {
	// First, we translate `data`
	let data: Map<Vec<u16>> = Map::<Vec<u16>> {
		keys: translate(&rawdata.keys),
		values: translate(&rawdata.values),
	};

	println!(
		"{:?}",
		translate(&vec!["a".to_owned(), "b".to_owned(), "c".to_owned()])
	);

	let mut mega: Vec<Vec<f32>> = Vec::new();
	let mut ram: Vec<f32> = Vec::new();

	let mut krealmem: usize;
	let mut vrealmem: usize;

	for (key, value) in data.keys.iter().zip(&data.values) {
		checkmem!(memory, key, krealmem, value, vrealmem);
		// We divide the keys and the values
		for key_chunk in key.into_chunks(krealmem).base {
			for value_chunk in value.into_chunks(vrealmem).base {
				println!("%K{:?}", key_chunk);
				println!("%V{:?}", value_chunk);
				ram.push(
					key_chunk.iter().sum::<u16>() as f32 / value_chunk.iter().sum::<u16>() as f32,
				);
			};
		};
		mega.push(ram.clone());
		ram.clear();
	};

	println!("{:?}", mega);

	return (mega, data, rawdata);
}

// //
// // ────────────────────────────────────────────────────── I ──────────
// //   :::::: R E L E A R N : :  :   :    :     :        :          :
// // ────────────────────────────────────────────────────────────────
// //

// /*

// The user can obtain new learning data, well, we can add that data to one of two things, we can add that to the HashMap / Map or we can that to the `mega` Vec<u16>. These two options are the functions `relearn_indirect(...)` & `relearn_direct(...)`

// ~ `relearn_indirect(...)`:

// * Returns the original HashMap
// * All the original data (being literals) are still legible
// * Doesn't really 'relearn' the stuff, you have to feed it to the `learn(...)` function.

// * It is meant for the need of writing it in another file, reset the RAM and still have all the data.

// ~ `relearn_direct(...)`:

// * Returns a Vec<u16> for feeding directly into the `run(...)` function.
// * Not suited for external writing (just a list of u16 values)
// * Doesn't recompute all the values

// */
// // __relearn_direct__(...) wrapper

// // We use include_str! as this docs doesn't use mermaid
// #[doc = include_str!("../docs/relearn.md")]
// pub fn relearn_direct<

// T: Literal<String>
// + Clone
// + ToString
// + std::hash::Hash
// + std::cmp::Eq

// >(data: HashMap<T, T>, new_data: HashMap<T, T>, memory: Option<usize>) -> Vec<f32> {
// 	if let Some(mem) = memory {
// 		return __relearn_direct__(data, new_data, mem);
// 	} else {
// 		return __relearn_direct__(data, new_data, DEFAULT_MEMORY);
// 	}

// }

// fn __relearn_direct__<

// 	T: Literal<String>
// 	+ Clone
// 	+ ToString
// 	+ std::hash::Hash
// 	+ std::cmp::Eq

// 	>(data: HashMap<T, T>, new_data: HashMap<T, T>, memory: usize) -> Vec<f32> {
// 	// First, we merge maps
// 	let old_length = data.len();
// 	let old_key_length = data.keys().len();
// 	let x = merge_hashmaps::<T>(data, new_data).to_map();

// 	// Now, we translate it.

// 	let map: Map<Vec<u16>> = Map::<Vec<u16>> {
// 		keys: translate(x.keys.literal()),
// 		values: translate(x.values.literal())
// 	};

// 	let mut mega: Vec<f32> = Vec::new();

// 	// Now, we relearn the releations JUST in the unknown section

// 	let mut krealmem: usize;
// 	let mut vrealmem: usize;

// 	let mut key_length: usize;
// 	let mut value_length: usize;

// 	for i in (0 .. map.keys.len()).step_by(old_key_length) {
// 		let key = &map.keys[i];
// 		key_length = key.len();
// 		krealmem = if memory >= key_length {
// 			key_length
// 		} else {
// 			memory
// 		};

// 		for key_chunk in key.into_chunks(krealmem).iterate() {
// 			for value in &map.values[old_length ..] {
// 				value_length = value.len();
// 				vrealmem = if memory >= value_length {
// 					value_length
// 				} else {
// 					memory
// 				};

// 				for value_chunk in value.into_chunks(vrealmem).iterate() {
// 					mega.push(value_chunk.iter().sum::<u16>() as f32 / key_chunk.iter().sum::<u16>() as f32);
// 				};
// 			};
// 		};
// 	};

// 	return mega;
// }

// pub fn relearn_indirect<

// T: Literal<String> +
// std::hash::Hash +
// std::cmp::Eq

// >(data: HashMap<T, T>, new_data: HashMap<T, T>) -> HashMap<T, T> {
// 	return merge_hashmaps(data, new_data);
// }

//
// ────────────────────────────────────────────── I ──────────
//   :::::: R U N : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────
//

// as this is the most complex file in the whole project, I'll give its own file.

#[path = "libs/run.rs"]
mod run;
pub use run::*;