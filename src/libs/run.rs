use crate::{chk::*, DEFAULT_MEMORY, DEFAULT_THRESHOLD};

macro_rules! diffcomparison {
	($MChunk: expr, $IChunk: expr, $VChunk: expr) => {
		$MChunk.iter().sum::<f32>() - $IChunk.iter().sum::<u16>() as f32 / $VChunk.iter().sum::<u16>() as f32
	};
}

pub fn run(rawinput: String, learnt: (Vec<Vec<u16>>, Vec<Vec<f32>>), memory: Option<usize>, threshold: Option<f32>) {
	match (memory, threshold) {
		(None, None) => __run__(rawinput, learnt, DEFAULT_MEMORY, DEFAULT_THRESHOLD),
		(Some(mem), None) => __run__(rawinput, learnt, mem, DEFAULT_THRESHOLD),
		(None, Some(thr)) => __run__(rawinput, learnt, DEFAULT_MEMORY, thr),
		(Some(mem), Some(thr)) => __run__(rawinput, learnt, mem, thr)
	};
}

pub fn __run__(rawinput: String, learnt: (Vec<Vec<u16>>, Vec<Vec<f32>>), memory: usize, threshold: f32) {
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

	let mut BestMatch: Option<(usize, usize, usize, f32)> = None;
	/*
	Best Match = (
		Value Index,
		Mega Index,
		Mega Chunk Index,
		Best Calculation at that point
	)
	*/

	// for IChunk in input.into_chunks(memory).base {
	// 	for Value in &TValuesVec {
	// 		checkmem!(memory, Value, VRealMem);
	// 		for (i, VChunk) in Value.into_chunks(memory).base.iter().enumerate() {
	// 			for (j, MegaVec) in Mega.iter().enumerate() {
	// 				checkmem!(memory, MegaVec, MRealMem);
	// 				for (k, &MChunk) in MegaVec.into_chunks(MRealMem).base.iter().enumerate() {
	// 					// * Calculating the result
	// 					Calculation = diffcomparison!(MChunk, IChunk, VChunk);

	// 					if Calculation <= threshold {
	// 						// Check if BestMatch is None
	// 						if BestMatch == None {
	// 							BestMatch = Some((i, j, k, Calculation));
	// 						} else {
	// 							if Calculation < BestMatch.unwrap().3 {
	// 								BestMatch = Some((i, j, k, Calculation));
	// 							}
	// 						}
	// 					}
	// 				}
	// 			};
	// 		};
	// 	};
	// }

	// Actually I messed up with the algorithm hehe
}
