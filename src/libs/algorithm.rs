// Speak algorithm made by Alex G. C. (blyxyas) visit github.com/blyxyas/speak for more information.

use crate::*;

/// # WARNING
/// Do not use this struct, just use it in with the `run(...)` function.

pub struct Learnt {
	pub learn_vec: Vec<Vec<f32>>,
	pub translated_deconstructed: Deconstructed<Vec<u32>>,
	pub raw_deconstructed: Deconstructed<String>
}

//
// ──────────────────────────────────────────────────────────────────── I ──────────
//   :::::: T R A I N   F U N C T I O N : :  :   :    :     :        :          :
// ──────────────────────────────────────────────────────────────────────────────
//

pub(crate) fn __learn__<T: Literal>(rawdata: Map<T>, memory: usize) -> Learnt {
    let dec: Deconstructed<String> = deconstruct::<T>(rawdata);
    let decdata: Deconstructed<Vec<u32>> = Deconstructed {
        keys: translate(&dec.keys),
        values: translate(&dec.values),
    };

	println!("{:#?}", decdata.keys);

	let mut kvec_length: usize;
	let mut kmem: usize;

	let mut vvec_length: usize;
	let mut vmem: usize;

	let mut ram: Vec<f32> = Vec::new();
	let mut learn_vec: Vec<Vec<f32>> = Vec::new();

	for kvec in &decdata.keys {
		kvec_length = kvec.len();
		kmem = if memory >= kvec_length {
			kvec_length
		} else {
			memory
		};
		for X in (kmem..kvec_length).step_by(kmem) {
			for vvec in &decdata.values {
				vvec_length = vvec.len();
				vmem = if memory >= vvec_length {
					vvec_length - 1
				} else {
					memory
				};

				for Y in (vmem..vvec_length).step_by(vmem) {
					ram.push(
						kvec[(X - kmem)..X].iter().sum::<u32>() as f32 /
						vvec[(Y - vmem)..Y].iter().sum::<u32>() as f32
					);
				}
			}
		}
		learn_vec.push(ram.clone());
		ram.clear();
	}

println!("{:#?}", learn_vec);

    return Learnt {
		learn_vec,
        translated_deconstructed: decdata,
        raw_deconstructed: dec,
	}
}

//
// ──────────────────────────────────────────────────────────────── I ──────────
//   :::::: R U N   F U N C T I O N : :  :   :    :     :        :          :
// ──────────────────────────────────────────────────────────────────────────
//
/*
pub(crate) fn __run__(
	input: String,               // The input string
    learnt_data: Learnt,        // The learnt data
    threshold: f32,              // The threshold (default: 0.4)
    memory: usize,
) -> String {
	let mut result: String = String::new();
    // First, we translate the input into a vector
    let mut inputvec: Vec<u32> = Vec::new();
    {
		let mut sum: u32 = 0;
        for word in input.split_whitespace() {
            for char in word.chars() {
                sum += char as u32;
            }
            inputvec.push(sum);
            sum = 0;
        }
    };

    // Then, we calculate the distance between the input and the learning data.

    let mut int_chunk: &[u32];
	
	let mut vvec_length: usize;
	let mut vvec_memory: usize;
	let mut vvec_chunk: &[u32];

	let mut key_length: usize;
	let mut key_memory: usize;
	let mut key_chunk: &[f32];

	let mut key_chunk_raw: Vec<f32>; // As you can probably notice, I don't like to use Vecs, but I need to do it...

	let input_memory: usize;
    let inputvec_length: usize = inputvec.len() - 1;
    
    input_memory = if memory >= inputvec_length {
        inputvec.len()
    } else {
        memory
    };

	for X in (input_memory..inputvec_length).step_by(input_memory) {
		int_chunk = &inputvec[X - input_memory .. X];
		for (IVVEC, vvec) in learnt_data
						.translated_deconstructed
						.values.iter().enumerate() {

			vvec_length = vvec.len();
			vvec_memory = if memory >= vvec_length { vvec_length } else { memory };
			
			for Y in (vvec_memory..vvec_length).step_by(vvec_memory) {
				vvec_chunk = &vvec[Y - vvec_memory .. Y];
				//[Y * keys_length



key_length = learnt_data.translated_deconstructed.keys.len();

key_chunk_raw = learnt_data.learn_vec[IVVEC];

if ((
	(int_chunk.iter().sum::<u32>() as f32) /
	(vvec_chunk.iter().sum::<u32>() as f32)) /
	key_chunk.iter().sum::<f32>()
	- 1.0).abs() <= threshold {
		//result.push_str()
};

			};
		};
	};
	return result;
}
*/