use serde::{Serialize, Deserialize};
use std::f64::consts::PI;
use std::cmp::{min,max};

pub const TICK_FREQ: f64 = 50.12454212;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Waveform {
	Triangle,
	Saw,	
	Rect,
	Noise,
	InverseSaw,
	Sinus
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Oscillator {
	pub waveform: Waveform,
	pub frequency: f64,
	pub phase_offset: f64,
	pub ratio: f64,
	pub oneshot: bool,
	
	pub ticks: u32,

	pub x: f64,
	pub y: f64,
}

impl Oscillator {
	pub fn new(waveform: Waveform, frequency: f64, phase_offset: f64, ratio: f64, oneshot: bool) -> Self {
		let mut osc = Oscillator {
			waveform: waveform,
			frequency: frequency,
			phase_offset: phase_offset,
			ratio: ratio,
			oneshot: oneshot,

			ticks: 0,

			x: 0.0,
			y: 0.0,
		};
		osc.reset();
		osc
	}

	pub fn y_normalized(&self) -> f64 {
		(self.y + 1.0) * 0.5
	}

	pub fn frequency_by_ticks(&self, ticks_per_frame: u8) -> f64 {
		ticks_per_frame as f64 * TICK_FREQ / self.frequency
	}

	pub fn reset(&mut self) {
		self.ticks = 0;
		self.x = 0.0;
		self.set_y();
	}

	pub fn set_frequency_from_ticks(&mut self, ticks: u32, ticks_per_frame: u8) {
		self.frequency = ticks_per_frame as f64 * TICK_FREQ / max(2,ticks) as f64;
	}

	pub fn set_frequency_from_note(&mut self, note: u32) {
		const OCTAVE4: [f64; 12] = [
			261.63, //	C4	261.63	131.87
			277.18, //	 C#4/Db4 	277.18	124.47
			293.66, //	D4	293.66	117.48
			311.13, //	 D#4/Eb4 	311.13	110.89
			329.63, //	E4	329.63	104.66
			349.23, //	F4	349.23	98.79
			369.99, //	 F#4/Gb4 	369.99	93.24
			392.00, //	G4	392.00	88.01
			415.30, //	 G#4/Ab4 	415.30	83.07
			440.00, //	A4	440.00	78.41
			466.16, //	 A#4/Bb4 	466.16	74.01
			493.88, //	B4	493.88	69.85
		];
		let mut oct = min(4,note/12);
		let mut f = OCTAVE4[(note%12) as usize];
		while oct < 4 {
			f *= 0.5;
			oct +=1;
		}
		self.frequency = f;
	}


	fn set_y(&mut self) {
		let mut x = self.x + self.phase_offset;
		while x > 1.0 {
			x -= 1.0;
		}		
		match self.waveform {
			Waveform::Triangle => {
				if x < self.ratio {
					self.y = -1.0 + if self.ratio > 0.0 {2.0 * x/self.ratio} else {0.0};
				}
				else {
					self.y = 1.0 -  if self.ratio < 1.0 {2.0 * (x - self.ratio)/(1.0-self.ratio)} else {0.0};
				}
			}

			Waveform::Saw => {
				while x >= 1.0 {
					x -= 1.0;
				}
				self.y = -1.0 + 2.0 * x;
			}

			Waveform::Rect => {
				if !self.oneshot {
					while x >= 1.0 {
						x -= 1.0;
					}
				}
				self.y = if x < self.ratio {-1.0} else {1.0};
			}

			Waveform::InverseSaw => {
				while x >= 1.0 {
					x -= 1.0;
				}
				self.y = 1.0 - 2.0 * x;
			}

			Waveform::Sinus => {
				while x >= 1.0 {
					x -= 1.0;
				}
				self.y = (x * 2.0 * PI).sin();
			}

			_ => {
				self.y = 0.0;
			}
		}
	}

	pub fn tick(&mut self, ticks_per_frame: u8) {
		self.ticks += 1;
		self.x += self.frequency/(ticks_per_frame as f64 * TICK_FREQ);
		if !self.oneshot {
			while self.x >= 1.0 {
				self.x -= 1.0;
			}
		}
		else if self.x > 1.0 {
			self.x = 1.0;
		}
		self.set_y();

		// /println!("t: {:?} - x: {:?} - y: {:?}", self.ticks, self.x, self.y);
	}
}