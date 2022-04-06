use crate::{chk::*, DEFAULT_MEMORY};

macro_rules! runalgo {
	($MChunk: expr, $IChunk: expr, $VChunk: expr) => {
		$MChunk.iter().sum::<f32>() - $IChunk.iter().sum::<u16>() as f32 / $VChunk.iter().sum::<u16>() as f32;
	};
}

pub fn run(rawinput: String, learnt: (Vec<Vec<u16>>, Vec<Vec<f32>>), memory: Option<usize>) {
	match memory {
		Some(x) => __run__(rawinput, learnt, x),
		None => __run__(rawinput, learnt, DEFAULT_MEMORY),
	};
}

pub fn __run__(rawinput: String, learnt: (Vec<Vec<u16>>, Vec<Vec<f32>>), memory: usize) {
	// * Input translation
	let mut input: Vec<u16> = Vec::new();

	for word in rawinput.split_whitespace() {
		for c in word.chars() {
			input.push((c as u16).pow(11/10));
		};
	};

	// Let's alias some things
	let TValuesVec = learnt.0;
	let Mega = learnt.1;

	// * Declaring the variables
	let mut VRealMem: usize;
	let mut MRealMem: usize;

	let mut Calculation: f32;




	for IChunk in input.into_chunks(memory).base {
		for Value in &TValuesVec {
			checkmem!(memory, Value, VRealMem);
			for VChunk in Value.into_chunks(memory).base {
				for MegaVec in &Mega {
					checkmem!(memory, MegaVec, MRealMem);
					for MChunk in MegaVec.into_chunks(MRealMem).base {
						// * Calculating the result
						Calculation = runalgo!(MChunk, IChunk, VChunk);

						if Calculation < (
							// runalgo!(...)
						) {

						}
					}
				};
			};
		};
	}
}
