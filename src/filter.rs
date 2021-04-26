use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct FilterPatch {
    pub filter_freq: u32,
    pub filter_res: u8,
    pub filter_mask: u8,
    pub filter_type: u8,

    pub filter_freq_add: u32,
}

impl FilterPatch {

	pub fn new() -> Self {
		FilterPatch {
			filter_freq: 0x7ff,
			filter_res: 0x00,
			filter_mask: 0x00,
			filter_type: 0x01,

			filter_freq_add: 0x00,
	  	}
	}
}
