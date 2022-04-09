use crate::*;

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
) -> Vec<f32> {
	let x: Map<T> = data.clone().to_map();
	let new_map: Map<String> = Map::<String> {
		keys: x.keys.literal(),
		values: x.values.literal(),
	};

	// print!("{:?}", new_map.keys);
	// print!("{:?}", new_map.values);

	if let Some(mem) = memory {
		return __learn__(new_map, mem);
	} else {
		return __learn__(new_map, DEFAULT_MEMORY);
	}
}

// The main algorithm
fn __learn__<'a>(rawdata: Map<String>, memory: usize) -> Vec<f32> {

	let mut mega: Vec<f32> = Vec::new();

	let mut vrm: usize;
	let mut krm: usize;
	
	// Now, we convert the rawdata to a translated data
	let data: Map<Vec<u16>> = Map::<Vec<u16>> {
		keys: translate(&rawdata.keys),
		values: translate(&rawdata.values),
	};

	// Now, we crate the relations between the values
	for (key, value) in data.keys.iter().zip(data.values.iter()) {
		checkmem!(memory, key, krm, value, vrm);
		for KChunk in key.into_chunks(krm).base {
			for VChunk in value.into_chunks(vrm).base {
				mega.push(KChunk.iter().sum::<u16>() as f32 / VChunk.iter().sum::<u16>() as f32);
			}
		}
	}
	return mega;
}