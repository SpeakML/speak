//
// ────────────────────────────────────────────── I ──────────
//   :::::: R U N : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────
//

use crate::*;

macro_rules! diffcomparison {
	($MChunk: expr, $IChunk: expr, $VChunk: expr) => {
		$MChunk.iter().sum::<f32>() - $IChunk.iter().sum::<u16>() as f32 / $VChunk.iter().sum::<u16>() as f32
	};
}

// pub fn run(rawinput: String, learnt: (Vec<Vec<u16>>, Vec<Vec<f32>>), memory: Option<usize>, threshold: Option<f32>) {
// 	match (memory, threshold) {
// 		(None, None) => __run__(rawinput, learnt, DEFAULT_MEMORY, DEFAULT_THRESHOLD),
// 		(Some(mem), None) => __run__(rawinput, learnt, mem, DEFAULT_THRESHOLD),
// 		(None, Some(thr)) => __run__(rawinput, learnt, DEFAULT_MEMORY, thr),
// 		(Some(mem), Some(thr)) => __run__(rawinput, learnt, mem, thr)
// 	};
// }

pub fn __run__(rawinput: String, learnt: (Map::<Vec<u16>>, Vec<Vec<f32>>), memory: usize, threshold: f32) {
	// * Input translation
	let mut input: Vec<u16> = Vec::new();

	for word in rawinput.split_whitespace() {
		for c in word.chars() {
			input.push((c as u16).pow(11/10));
		};
	};

	// Let's alias some things
	let TMap = learnt.0;
	let Mega = learnt.1;

	// Algorithm fixed (Being
	
		// O(#C(n) * #C(m))
	
	// instead of
	
	// O( sum_{i <= #V} #C(K_i)^#C(V) )
	
	// )

	let IRM :usize;
	let mut KRM :usize;
	let mut VRM :usize;
	let mut MRM :usize;


	checkmem!(memory, input, IRM);

	for IChunk in input.into_chunks(IRM).base {
		for (i, (key, value)) in TMap.iter().enumerate() {
			checkmem!(memory, key, KRM, value, VRM, Mega[i], MRM);
			for KChunk in key.into_chunks(KRM).base {
				for VChunk in value.into_chunks(VRM).base {
					for MChunk in Mega[i].into_chunks(MRM).base {

					}
				}
			}
		}
	}

}