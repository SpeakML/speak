// TODOS
// - .specific_chunk(...): Returns an specific chunk instead of the whole list of chunks (Improve performance multiple times)

#[must_use]
pub(crate) struct Chunks<'a, T> {
	pub(crate) base: Vec<&'a [T]>,
}

impl<'a, T> Chunks<'a, T> {
	pub(crate) fn new() -> Chunks<'a, T> {
		return Chunks::<T> { base: Vec::new() };
	}
}

pub(crate) trait Chunkable<'a, T> {
	#[must_use]
	fn into_chunks(&'a self, memory: usize) -> Chunks<'a, T>;
	// fn specific_chunk(&'a self, memory: usize, index: usize) -> &'a [T];
}

impl<'a, T> Chunkable<'a, T> for Vec<T> {
	#[must_use]
	fn into_chunks(&'a self, memory: usize) -> Chunks<T> {
		let mut chunks: Vec<&'a [T]> = Vec::new();
		for i in (memory..self.len() + 1).step_by(memory) {
			chunks.push(&self[i - memory..i]);
		}

		if self.len() % memory != 0 {
			chunks.push(&self[self.len() - memory..]);
		};

		return Chunks::<T> { base: chunks };
	}

	// fn specific_chunk(&'a self, memory: usize, index: usize) -> &'a [T] {
	// 	return &self[index * memory .. (index + 1) * memory];
	// }
}
