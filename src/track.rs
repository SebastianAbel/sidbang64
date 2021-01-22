use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
	pub patterns: Vec<[i16; 16]>,				// vector of sequence indices
	pub patch_patterns: Vec<[i16; 16]>,			// vector of sequence indices
	pub sequences: Vec<Vec<Vec<i8>>>,			// vector of patterns sequences for each channel
	pub patches: Vec<Vec<Vec<i8>>>,				// vector of patterns of instrument-patches for each channel
}