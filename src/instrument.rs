use std::cmp;
use resid::*;
use serde::{Serialize, Deserialize};

use crate::oscillator::{Oscillator, Waveform, TICK_FREQ};
use crate::sid_player::{NOTE_LO, NOTE_HI, ATTACK_TIME, DECREL_TIME, ExportData};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum BaseMode {
	Zero,
	NoteIn,
	Replace,
	Add,
	Sub,
}
use BaseMode::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum OsciName {
	Notes,
	Vibrato,
	Wave,
	Pulsewidth,
}


#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Instrument {
	
	pub current_note: i8,
	pub note_start_tick: u32,
	pub playing: bool,

	pub adsr_state: i8,

	pub attack: u8,	
	pub decay: u8,
	pub sustain: u8,
	pub hold: u32,
	pub release: u8,

	pub osc_map: [u8; 4],
	pub osc_hold: [u32; 4],

	pub base_a: [i8; 4],
	pub mode_a: [BaseMode; 4],
	pub base_b: [i8; 4],
	pub mode_b: [BaseMode; 4],

	pub vibrato: [f64; 4],

	pub wave_a: [u8; 4],
	pub wave_b: [u8; 4],

	pub pulsewidth_a: [f64; 4], 
	pub pulsewidth_b: [f64; 4],

	pub oscillators: [Oscillator; 16],
	pub osc_reset: bool,
}

impl Instrument {

	pub fn new(_name: String) -> Self {
		Instrument {
			playing: false,

			current_note: -1,
	    	note_start_tick: 0,

	    	adsr_state: -1,

	    	attack: 0,
	    	decay: 0,
	    	sustain: 0x0f,
	    	hold: 0,
	    	release: 0,

	    	osc_map: [0; 4],
	    	osc_hold: [0; 4],

			base_a: [0; 4],
			mode_a: [BaseMode::NoteIn; 4],
			base_b: [0; 4],
			mode_b: [BaseMode::NoteIn; 4],

			vibrato: [0.0; 4],

			wave_a: [1; 4],
			wave_b: [1; 4],

			pulsewidth_a: [0.0; 4],
			pulsewidth_b: [0.0; 4],

			oscillators: [Oscillator::new(Waveform::Rect, TICK_FREQ/1.0, 0.0, 0.5, false); 16],
			osc_reset: true,
	  	}
	}

	pub fn reset(&mut self) {
		self.playing = false;
		self.current_note = -1;
		self.note_start_tick = 0;
		self.adsr_state = -1;

		for i in 0..self.oscillators.len() { 	
			self.oscillators[i].reset();
		}

	}

	pub fn update(&mut self, tick: u32, voice: u8, note: i8, resid: &mut Sid, ticks_per_frame: u8, export: bool) -> ExportData {

		let mut exp = ExportData{
			kernel_type: 0,
			bytes: [0;7]
		};

		//println!("osc1: {:?}", self.osc1.y);
		let trigger = (note != self.current_note) && (note >= 0);
		
		if note >= 0
		{
			self.current_note = note;
		}
		let reg_offset = voice*7;

		if trigger && (!self.playing || self.adsr_state == 3)  
		{
			self.playing = true;
			self.note_start_tick = tick;
			if self.osc_reset {
				for i in 0..self.oscillators.len() { 	
					self.oscillators[i].reset();
				}
			}
			self.adsr_state = 0;
			if !export {
				resid.write(0x05 + reg_offset, (self.attack << 4) | self.decay); // ad
				resid.write(0x06 + reg_offset, (self.sustain << 4) | self.release); // sr
			}
			else {
				exp.kernel_type |= 16;
				exp.bytes[5] = (self.attack << 4) | self.decay;
				exp.bytes[6] = (self.sustain << 4) | self.release;
			}
		}

		if self.playing
		{
			let mut gate = 0;
			let hold_cmp = (self.hold as f64 * ticks_per_frame as f64/16.0) as u32;
			match self.adsr_state {
				0 => {
					gate = if hold_cmp > 0 
					{
						if (tick - self.note_start_tick) < hold_cmp {1} else {0}
					}
						else
					{
						if note >= 0 {1} else {0}	// currently set from sequencer
					};

					if gate == 0 {
						self.note_start_tick = tick;
						self.adsr_state = 3;
					}
					else {
						let t = ((tick - self.note_start_tick) as f64 / ticks_per_frame as f64) * (1.0/TICK_FREQ);
						if t > ATTACK_TIME[self.attack as usize] {
							self.note_start_tick = tick;
							self.adsr_state += 1;
						}
					}
				}

				1 => {
					gate = if hold_cmp > 0 
					{
						let t = (ATTACK_TIME[self.attack as usize] * ticks_per_frame as f64 * TICK_FREQ) as u32;
						if (tick - (self.note_start_tick - t)) < hold_cmp {1} else {0}
					}
						else
					{
						if note >= 0 {1} else {0}	// currently set from sequencer
					};
					if gate == 0 {
						self.note_start_tick = tick;
						self.adsr_state = 3;
					}
					else {
						let t = ((tick - self.note_start_tick) as f64 / ticks_per_frame as f64) * (1.0/TICK_FREQ);
						if t > DECREL_TIME[self.decay as usize] {
							self.note_start_tick = tick;
							self.adsr_state += 1;
						}
					}
				}

				2 => {
					gate = if hold_cmp > 0 
					{
						let t = ((ATTACK_TIME[self.attack as usize] + DECREL_TIME[self.decay as usize]) * ticks_per_frame as f64 * TICK_FREQ) as u32;
						if (tick - (self.note_start_tick - t)) < hold_cmp {1} else {0}
					}
						else
					{
						if note >= 0 {1} else {0}	// currently set from sequencer
					};
					if gate == 0 {
						self.note_start_tick = tick;
						self.adsr_state = 3;
						
					}
				}				

				3 => {
					let t = ((tick - self.note_start_tick) as f64 / ticks_per_frame as f64) * (1.0/TICK_FREQ);
					if t > (DECREL_TIME[self.release as usize] + DECREL_TIME[2]) {
						self.adsr_state = -1;
						self.playing = false;
					}
				}
				_ => {}
			}

			let osc_group = if self.adsr_state >= 0 {self.adsr_state} else {3};

			let note_src = match self.mode_a[self.osc_map[osc_group as usize] as usize] {
				Zero => 0,
				NoteIn => self.current_note,
				Replace => self.base_a[self.osc_map[osc_group as usize] as usize],
				Add => cmp::min(95, self.current_note as i16 + self.base_a[self.osc_map[osc_group as usize] as usize] as i16) as i8,
				Sub => cmp::max(0, self.current_note as i16 - self.base_a[self.osc_map[osc_group as usize] as usize] as i16) as i8
			};

			let note_dst = match self.mode_b[self.osc_map[osc_group as usize] as usize] {
				Zero => 0,
				NoteIn => self.current_note,
				Replace => self.base_b[self.osc_map[osc_group as usize] as usize],
				Add => cmp::min(95, self.current_note as i16 + self.base_b[self.osc_map[osc_group as usize] as usize] as i16) as i8,
				Sub => cmp::max(0, self.current_note as i16 - self.base_b[self.osc_map[osc_group as usize] as usize] as i16) as i8
			};

			let frq_src = match self.mode_a[self.osc_map[osc_group as usize] as usize] { 
				Zero => {
					0.0},
				_ => {
					let f1 = (NOTE_LO[note_src as usize] as u32 + ((NOTE_HI[note_src as usize] as u32)<<8)) as f64;
		
					let f_down = if note_src > 0 {
							(NOTE_LO[(note_src-1) as usize] as u32 + ((NOTE_HI[(note_src-1) as usize] as u32)<<8)) as f64
						}
						else {
							f1
						};

					let f_up = if note_src < 95 {
							(NOTE_LO[(note_src+1) as usize] as u32 + ((NOTE_HI[(note_src+1) as usize] as u32)<<8)) as f64
						}
						else {
							f1
						};
					

					let y = self.oscillators[1 + (4*self.osc_map[osc_group as usize]) as usize].y;
					if y < 0.0 {
						f1 + self.vibrato[self.osc_map[osc_group as usize] as usize] * y * (f1 - f_down)
					}
					else {
						f1 + self.vibrato[self.osc_map[osc_group as usize] as usize] * y * (f_up - f1)
					}
				},
			};

			let frq_dst = match self.mode_b[self.osc_map[osc_group as usize] as usize] {
				Zero => 0.0,
				_ => {
					let f1 = (NOTE_LO[note_dst as usize] as u32 + ((NOTE_HI[note_dst as usize] as u32)<<8)) as f64;
		
					let f_down = if note_dst > 0 {
							(NOTE_LO[(note_dst-1) as usize] as u32 + ((NOTE_HI[(note_dst-1) as usize] as u32)<<8)) as f64
						}
						else {
							f1
						};

					let f_up = if note_dst < 95 {
							(NOTE_LO[(note_dst+1) as usize] as u32 + ((NOTE_HI[(note_dst+1) as usize] as u32)<<8)) as f64
						}
						else {
							f1
						};
					

					let y = self.oscillators[1 + (4*self.osc_map[osc_group as usize]) as usize].y;
					if y < 0.0 {
						f1 + self.vibrato[self.osc_map[osc_group as usize] as usize] * y * (f1 - f_down)
					}
					else {
						f1 + self.vibrato[self.osc_map[osc_group as usize] as usize] * y * (f_up - f1)
					}
				},
			};

			let wave = if self.oscillators[2 + (4*self.osc_map[osc_group as usize]) as usize].y < 0.0 {self.wave_a[self.osc_map[osc_group as usize] as usize]} else {self.wave_b[self.osc_map[osc_group as usize] as usize]};

			let frq = frq_src + self.oscillators[0 + (4*self.osc_map[osc_group as usize]) as usize].y_normalized() * (frq_dst - frq_src);
			let frq_out = cmp::min(0xffff, cmp::max(0x00, frq as i32));

			let p1 = self.pulsewidth_a[self.osc_map[osc_group as usize] as usize] + (self.pulsewidth_b[self.osc_map[osc_group as usize] as usize] - self.pulsewidth_a[self.osc_map[osc_group as usize] as usize]) * self.oscillators[3 + (4*self.osc_map[osc_group as usize]) as usize].y_normalized();
			let pulse_ratio = p1 * 0xfff as f64;

			if !export {
				resid.write(0x00 + reg_offset, (frq_out&0xff) as u8); // freqlo
				resid.write(0x01 + reg_offset, (frq_out>>8) as u8); // freqhi
			
				resid.write(0x02 + reg_offset, ((pulse_ratio as u16)&0xff) as u8); // freqlo
				resid.write(0x03 + reg_offset, (((pulse_ratio as u16)>>8)&0xf) as u8); // freqhi

				if self.adsr_state >= 0
				{
					resid.write(0x04 + reg_offset, (wave<<4) | gate);
				}
				else {
					resid.write(0x04 + reg_offset, 0x08);
					resid.write(0x05 + reg_offset, 0); // ad
					resid.write(0x06 + reg_offset, 0); // sr					
				}
			}
			else {
				if (wave & 0x4) != 0 {
					exp.kernel_type |= 1;
					exp.bytes[0] = (frq_out&0xff) as u8;
					exp.kernel_type |= 2;
					exp.bytes[1] = (frq_out>>8) as u8;
					exp.kernel_type |= 4;
					exp.bytes[2] = ((pulse_ratio as u16)&0xff) as u8;
					exp.bytes[3] = (((pulse_ratio as u16)>>8)&0xf) as u8;					
					exp.kernel_type |= 8;
					exp.bytes[4] = if self.adsr_state >= 0 {(wave<<4) | gate} else {0x08};
					if self.adsr_state < 0 {
						exp.kernel_type |= 16;
						exp.bytes[5] = 0;
						exp.bytes[6] = 0;
					}
				}
				else {
					exp.kernel_type |= 0;
					exp.bytes[0] = (frq_out&0xff) as u8;
					exp.kernel_type |= 2;
					exp.bytes[1] = (frq_out>>8) as u8;	
					exp.kernel_type |= 8;
					exp.bytes[4] = if self.adsr_state >= 0 {(wave<<4) | gate} else {0x08};		
					if self.adsr_state < 0 {
						exp.kernel_type |= 16;
						exp.bytes[5] = 0;
						exp.bytes[6] = 0;
					}							
				}
			}
		}
		
		if !self.playing {
			self.current_note = -1;
		}		
		

		for i in 0..self.oscillators.len() { 	
			self.oscillators[i].tick(ticks_per_frame);
		}

		exp
	}
}