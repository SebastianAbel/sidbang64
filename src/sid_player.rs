#![allow(dead_code, unused)]
//use std::time::{Duration, Instant};
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use cpal::{StreamData, UnknownTypeOutputBuffer};

use resid::*;

use triple_buffer::TripleBuffer;

use std::fs;
use std::fs::File;
//use std::io::prelude::*;
//use std::error::Error;
use std::io::{Write, BufReader};
use std::path::Path;

use serde::{Serialize, Deserialize};
use serde_json::Result;

use std::cmp;

use crate::oscillator::{TICK_FREQ,Oscillator,Waveform};

use crate::track::Track;
use crate::instrument::Instrument;

const BUFFERSIZE : usize = 1024;

/*
 * Notetable: these values represents notes on a C64
 * SID chip. Pick a value from each vector for correct
 * frequency parameters, note_hi[x] = $d400, note_lo[x] = $d401
 * The numbers in the C64 hardware reference manual are simply
 * WRONG. Index 0 = C-0, index 36 = C-3 (flat C), 
 * index 57 = A-4 (flat A), index 95 = A-7 (last B in octave 8
 * is not possible to replay with c64)
 *
 * Public Domain - Linus Walleij 2001
 */

pub const NOTE_HI: [u8; 96] = [
  0x01,0x01,0x01,0x01,0x01,
  0x01,0x01,0x01,0x01,0x01,0x01,
  0x02,0x02,0x02,0x02,0x02,0x02,
  0x02,0x03,0x03,0x03,0x03,0x03,
  0x04,0x04,0x04,0x04,0x05,0x05,
  0x05,0x06,0x06,0x06,0x07,0x07,
  0x08,0x08,0x09,0x09,0x0a,0x0a,
  0x0b,0x0c,0x0d,0x0d,0x0e,0x0f,
  0x10,0x11,0x12,0x13,0x14,0x15,
  0x17,0x18,0x1a,0x1b,0x1d,0x1f,
  0x20,0x22,0x24,0x27,0x29,0x2b,
  0x2e,0x31,0x34,0x37,0x3a,0x3e,
  0x41,0x45,0x49,0x4e,0x52,0x57,
  0x5c,0x62,0x68,0x6e,0x75,0x7c,
  0x83,0x8b,0x93,0x9c,0xa5,0xaf,
  0xb9,0xc4,0xd0,0xdd,0xea,0xf8,
  0xff
];

pub const NOTE_LO: [u8; 96] = [
  0x16,0x27,0x38,0x4b,0x5e,
  0x73,0x89,0xa1,0xba,0xd4,0xf0,
  0x0d,0x2c,0x4e,0x71,0x96,0xbd,
  0xe7,0x13,0x42,0x74,0xa8,0xe0,
  0x1b,0x59,0x9c,0xe2,0x2c,0x7b,
  0xce,0x27,0x84,0xe8,0x51,0xc0,
  0x36,0xb3,0x38,0xc4,0x59,0xf6,
  0x9d,0x4e,0x09,0xd0,0xa2,0x81,
  0x6d,0x67,0x70,0x88,0xb2,0xed,
  0x3a,0x9c,0x13,0xa0,0x44,0x02,
  0xda,0xce,0xe0,0x11,0x64,0xda,
  0x75,0x38,0x26,0x40,0x89,0x04,
  0xb4,0x9c,0xc0,0x22,0xc8,0xb4,
  0xeb,0x71,0x4c,0x80,0x12,0x08,
  0x68,0x38,0x80,0x45,0x90,0x68,
  0xd6,0xe3,0x98,0x00,0x24,0x10,
  0xff
];

/*
	A 		D 		R 
0 	2 ms 	6 ms 	6 ms
1 	8 ms 	24 ms 	24 ms
2 	16 ms 	48 ms 	48 ms
3 	24 ms 	72 ms 	72 ms
4 	38 ms 	114 ms 	114 ms
5 	56 ms 	168 ms 	168 ms
6 	68 ms 	204 ms 	204 ms
7 	80 ms 	240 ms 	240 ms
8 	100 ms 	0.3 s 	0.3 s
9 	0.25 s 	0.75 s 	0.75 s
10 	0.5 s 	1.5 s 	1.5 s
11 	0.8 s 	2.4 s 	2.4 s
12 	1 s 	3 s 	3 s
13 	3 s 	9 s 	9 s
14 	5 s 	15 s 	15 s
15 	8 s 	24 s 	24 s 
*/

pub const ATTACK_TIME: [f64; 16] = [
	0.002, 0.008, 0.016, 0.024,
	0.038, 0.056, 0.068, 0.080,
	0.100, 0.250, 0.500, 0.800,
	1.000, 3.000, 5.000, 8.000
];

pub const DECREL_TIME: [f64; 16] = [
	0.006, 0.024, 0.048, 0.072,
	0.114, 0.168, 0.204, 0.240,
	0.300, 0.750, 1.500, 2.400,
	3.000, 9.000, 15.000, 24.000
];

const PAL_PHI : f64  = 985248.0;
const NTSC_PHI : f64  = 1022727.0; //This is for machines with 6567R8 VIC. 6567R56A is slightly different.
const SID_CONSTANT : f64 = (256^(3)) as f64/ PAL_PHI; //Select the constant appropriate for your machine (PAL vs NTSC).
//SID_FREQ = SID_CONSTANT * FREQ_HZ; //Calculate SID freq for a certain note (specified in Hz).


#[derive(PartialEq, Eq)]
pub enum PlayerState {
	Paused,
	Playing,
}
use PlayerState::*;

#[derive(Clone, Copy)]
pub struct ExportData {
	// 1=frq_loq 1
	// 2=frq_hi 2
	// 3=pw_low/hi 4
	// 4=ctrl 8
	// 5=adsr 16
	pub kernel_type: u8,
	pub bytes: [u8; 7],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectData {
	pub name: String,
	pub player_speed: u8,
	pub bpm: f64,
	pub tick_scale: u8,
	pub export_quantsize: u16,
	pub start_pattern: u32,
	pub end_pattern: u32,	
}

pub struct SidPlayer {
	pub resid: Sid,

	pub resid2: Sid,

	device: cpal::Device,
	sample_rate: u32,
	out_channels: u8,
	format: cpal::Format,
	
	sample_time: f32,
	sample_length: usize,
	buffer: [i16; BUFFERSIZE],
	buffer2: [i16; BUFFERSIZE],	
	sidbuffer: Vec<f32>,

	b_in: triple_buffer::Input<Vec<f32>>,
	update: std::sync::Arc<std::sync::atomic::AtomicBool>,
	length: std::sync::Arc<std::sync::atomic::AtomicI32>,

	pub session_name: String,

	pub bpm: f64,
	pub ticks_per_16th: f64,

	pub ticks_per_frame: u8,
	ticks_per_second: f64,
    tick_add: u32,
    tick_add_f64: f64,
    pub tick_scale: u8,

    pub state: PlayerState,
    pub play_index: u32,      // 16 bit fixed point
    tick_counter: u32,


    pub filter_freq: u32,
    pub filter_res: u8,
    pub filter_mask: u8,
    pub filter_type: u8,

	pub instruments: Vec<[Instrument; 16]>,
	pub instrument_notes: [i8; 16],
	pub track: Track,

	pub start_pattern: u32,
	pub end_pattern: u32,
	pub pattern_idx: u32,
	pub song_mode: bool,

	pub mute: [bool; 16],
	pub solo: [bool; 16],

	pub inst_id: u16,
	pub inst_patch_id: [u8; 16],

	current_channel_patch: [i8; 16],

	pub preview: Vec<f32>,

	pub export_quantsize: u16,
}

impl SidPlayer {

	pub fn new(sidmodel: u8, resampling: u8, filter: bool, buffersize: f64) -> Self {
		let host = cpal::default_host();

		let device = host.default_output_device().expect("Failed to get default output device");
	    //let output_format = device.default_output_format().expect("Failed to get default output format");
	
		let mut supported_formats_range = device.supported_output_formats()
	    	.expect("error while querying formats");
		let output_format = supported_formats_range.next()
		    .expect("no supported format?!")
		    .with_max_sample_rate();


	    let sample_rate = output_format.sample_rate.0 as u32;
	    //println!("{:?}", output_format);

	    let sample_time = buffersize as f32;
	    let sample_length = (sample_time * sample_rate as f32) as usize;

	    let t_buffer = TripleBuffer::new(vec![0f32; sample_length+1024]);
	   	let (b_in, _b_out) = t_buffer.split();

	   	let ticks_per_frame = 16;

        let ticks_per_second = ticks_per_frame as f64*TICK_FREQ;
        let bpm = 170.0;
        let bps = bpm/60.0;
        let beat_length = 1.0/bps;
        let ticks_per_beat = ticks_per_second*beat_length;
        let ticks_per_16th = ticks_per_beat/4.0;

   		let preview_length = 44100*2;

	    let mut player = SidPlayer {
	    	resid: Sid::new(if sidmodel == 0 {resid::ChipModel::Mos6581} else {resid::ChipModel::Mos8580}),
	    	resid2: Sid::new(if sidmodel == 0 {resid::ChipModel::Mos6581} else {resid::ChipModel::Mos8580}),

	    	device: device,
	    	sample_rate: sample_rate,
	    	out_channels: output_format.channels as u8,
	    	format: output_format,
	    	
	    	sample_time: sample_time,
	    	sample_length: sample_length,
	    	buffer: [0; BUFFERSIZE],
	    	buffer2: [0; BUFFERSIZE],
	    	sidbuffer: vec![0f32; sample_length+1024],

	    	session_name: "default".to_string(),

	    	bpm: 170.0,
	    	ticks_per_16th: ticks_per_16th, 

	    	ticks_per_frame: ticks_per_frame,
            ticks_per_second: ticks_per_second,
            tick_add: ((1.0/ticks_per_16th)*256.0*256.0) as u32,
            tick_add_f64: ((1.0/ticks_per_16th)*256.0*256.0),
            tick_scale: 1,

            state: Paused,
            play_index: 0,
	    	tick_counter: 0,
	    	
	    	filter_freq: 0x7ff,
	    	filter_res: 0x00,
	    	filter_mask: 0x00,
	    	filter_type: 0x01,

	    	instruments: vec![[Instrument::new("".to_string());16];16],
	    	instrument_notes: [0; 16],

	    	track: Track {
				//channels: vec![[0;16];3],
				patterns: vec![[0; 16]; 64],
				patch_patterns: vec![[0; 16]; 64],
				sequences: vec![ vec![ vec![-1; 64]; 32] ; 16],
				patches: vec![ vec![ vec![-1; 64]; 32] ; 16],
			},

			start_pattern: 0,
			end_pattern: 0,
	    	pattern_idx: 0,
	    	song_mode: false,

	    	mute: [false; 16],
	    	solo: [false; 16],

			inst_id: 0,
			inst_patch_id: [0; 16],

	    	b_in: b_in,
	    	update: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
	    	length: std::sync::Arc::new(std::sync::atomic::AtomicI32::new(0)),

	    	current_channel_patch: [0; 16],

	    	preview: vec![0f32; preview_length],

	    	export_quantsize: 10,
	  	};
	  	player.set_speed_from_ticks(70.0);

		player.resid.set_sampling_parameters(match resampling{1=>SamplingMethod::Interpolate,2=>SamplingMethod::Resample,3=>SamplingMethod::ResampleFast,_=> SamplingMethod::Fast}, 985_248, sample_rate);
		//player.resid.reset(); 
		player.resid.enable_external_filter(false);
		player.resid.enable_filter(filter);

		

		player.resid.write(0x15, (player.filter_freq & 0x07) as u8);
		player.resid.write(0x16, ((player.filter_freq>>3) & 0xff) as u8);

		player.resid.write(0x17, ((player.filter_res & 0x0f)<<4) | player.filter_mask);
		player.resid.write(0x18, ((player.filter_type & 0x0f)<<4) | 0x0f);

	    player.resid.write(0x05, 0x00); // ad
	    player.resid.write(0x06, 0xf0); // sr
	    player.resid.write(0x05+7, 0x00); // ad
	    player.resid.write(0x06+7, 0xf0); // sr
	    player.resid.write(0x05+14, 0x00); // ad
	    player.resid.write(0x06+14, 0xf0); // sr

		player.resid2.set_sampling_parameters(SamplingMethod::Fast, 985_248, 44100);
		player.resid2.reset(); 
		player.resid2.write(0x18, 0x0f);
	    player.resid2.write(0x05, 0x00); // ad
	    player.resid2.write(0x06, 0xf0); // sr
	    player.resid2.write(0x05+7, 0x00); // ad
	    player.resid2.write(0x06+7, 0xf0); // sr
	    player.resid2.write(0x05+14, 0x00); // ad
	    player.resid2.write(0x06+14, 0xf0); // sr	    	    
		
		player
	}

	pub fn reset(&mut self) {
		if self.state == Playing {
			self.key_space();
		}
		self.restart();
		self.resid.reset();
		

		self.resid.write(0x15, (self.filter_freq & 0x07) as u8);
		self.resid.write(0x16, ((self.filter_freq>>3) & 0xff) as u8);
		self.resid.write(0x17, ((self.filter_res & 0x0f)<<4) | self.filter_mask);
		self.resid.write(0x18, ((self.filter_type & 0x0f)<<4) | 0x0f);


	    self.resid.write(0x05, 0x00); // ad
	    self.resid.write(0x06, 0xf0); // sr
	    self.resid.write(0x05+7, 0x00); // ad
	    self.resid.write(0x06+7, 0xf0); // sr
	    self.resid.write(0x05+14, 0x00); // ad
	    self.resid.write(0x06+14, 0xf0); // sr		
		for i in 0..16 {
			for j in 0..16 {
				self.instruments[j][i].reset();
			}
		}
	}

	pub fn save_session(&mut self) -> std::io::Result<()> {
		let path_name = format!("./bng/{}", self.session_name);
		if !Path::new(&path_name).exists() {
			fs::create_dir(&path_name)?;
		}

		let file1 = File::create(format!("{}/instruments.json", &path_name))?;
		serde_json::to_writer(file1, &self.instruments).unwrap();		
		let file2 = File::create(format!("{}/track.json", &path_name))?;
		serde_json::to_writer(file2, &self.track).unwrap();

		let pd = ProjectData{
			name: self.session_name.to_string(),
			player_speed: self.ticks_per_frame, 
			bpm: self.bpm, 
			tick_scale: self.tick_scale, 
			export_quantsize: self.export_quantsize,
			start_pattern: self.start_pattern,
			end_pattern: self.end_pattern,
		};
		let file3 = File::create(format!("{}/project.json", &path_name))?;
		serde_json::to_writer(file3, &pd).unwrap();		

        Ok(())
	}

	fn read_instruments_from_file(&mut self, file: File) -> Result<Vec<[Instrument; 16]>> {
	    let reader = BufReader::new(file);
	    let result = serde_json::from_reader(reader)?;
	    Ok(result)
	}	

	fn read_track_from_file(&mut self, file: File) -> Result<Track> {
	    let reader = BufReader::new(file);
	    let result = serde_json::from_reader(reader)?;
		Ok(result)
	}

	fn read_project_from_file(&mut self, file: File) -> Result<ProjectData> {
	    let reader = BufReader::new(file);
	    let result = serde_json::from_reader(reader)?;
		Ok(result)
	}

	pub fn load_session(&mut self) -> std::io::Result<()> {

		let path_name = format!("./bng/{}", self.session_name);
		if Path::new(&path_name).exists()
		{	
			let file1 = File::open(format!("{}/instruments.json", &path_name))?;
			let i = self.read_instruments_from_file(file1);
			self.instruments = i.unwrap();

	        let file2 = File::open(format!("{}/track.json", &path_name))?;
			let t = self.read_track_from_file(file2);
			self.track = t.unwrap();	

	        let file3 = File::open(format!("{}/project.json", &path_name))?;
			let t = self.read_project_from_file(file3);
			let p = t.unwrap();		
			self.session_name = p.name;	
			self.set_speed(p.bpm);
			self.set_ticks_per_frame(p.player_speed);
			self.tick_scale = p.tick_scale;	
			self.export_quantsize = p.export_quantsize;	
			self.start_pattern = p.start_pattern;
			self.end_pattern = p.end_pattern;
		};

        Ok(())
	}	

	pub fn get_trigger(&mut self, channel: u32, idx: usize) -> i8
	{
		self.track.sequences[channel as usize][self.track.patterns[self.pattern_idx as usize][channel as usize] as usize][idx as usize]
		//self.sequencer.seq_01[idx%16][idx/16][track as usize] = trigger;
	}

	pub fn set_trigger(&mut self, channel: u32, idx: usize, trigger: i8)
	{
		self.track.sequences[channel as usize][self.track.patterns[self.pattern_idx as usize][channel as usize] as usize][idx as usize] = trigger;
		//self.sequencer.seq_01[idx%16][idx/16][track as usize] = trigger;
	}

	pub fn get_patch(&mut self, channel: u32, idx: usize) -> i8
	{
		self.track.patches[channel as usize][self.track.patch_patterns[self.pattern_idx as usize][channel as usize] as usize][idx as usize]
		//self.sequencer.seq_01[idx%16][idx/16][track as usize] = trigger;
	}

	pub fn set_patch(&mut self, channel: u32, idx: usize, instrument: i8)
	{
		self.track.patches[channel as usize][self.track.patch_patterns[self.pattern_idx as usize][channel as usize] as usize][idx as usize] = instrument;
		//self.sequencer.seq_01[idx%16][idx/16][track as usize] = trigger;
	}

	pub fn set_ticks_per_frame(&mut self, ticks_per_frame: u8) {
		self.ticks_per_frame = ticks_per_frame;
		self.ticks_per_second =  ticks_per_frame as f64*TICK_FREQ;
        self.set_speed(self.bpm);    
	}

	pub fn set_speed(&mut self, bpm: f64)
	{
        let bps = bpm/60.0;
        let beat_length = 1.0/bps;
        let ticks_per_beat = self.ticks_per_second*beat_length;
        let ticks_per_16th = ticks_per_beat/4.0;	

        self.ticks_per_16th = ticks_per_16th;
        self.bpm = bpm;

        self.tick_add = ((1.0/ticks_per_16th)*256.0*256.0) as u32;
        self.tick_add_f64 = ((1.0/ticks_per_16th)*256.0*256.0);

	}

	pub fn set_speed_from_ticks(&mut self, ticks_per_16th: f64)
	{
        let ticks_per_beat = ticks_per_16th*4.0;
        let beat_length = ticks_per_beat/self.ticks_per_second;
        let bps = 1.0/beat_length;
        let bpm = bps*60.0;

        self.ticks_per_16th = ticks_per_16th;
        self.bpm = bpm;

        self.tick_add = ((1.0/ticks_per_16th)*256.0*256.0) as u32;
        self.tick_add_f64 = ((1.0/ticks_per_16th)*256.0*256.0);

	}


	pub fn note_off(&mut self) {
	    self.resid.write(0x04, 0x00); // cr
	}

	pub fn render(&mut self) -> usize {

		let raw_input = self.b_in.raw_input_buffer();
		let mut samples_out = 0;
		while samples_out < self.sample_length {

			if self.state == Playing
			{
				let play_index = (self.play_index>>16)%64;
				for channel in 0..3 {
					
					let mut channel_free = true;
					for instloop in 0..1
					{
						let inst = (channel*1 + instloop) as usize;
						
						let track_pattern = self.track.patterns[self.pattern_idx as usize][inst] as usize; 
						let track_patch_pattern = self.track.patch_patterns[self.pattern_idx as usize][inst] as usize;

						let mut note = self.track.sequences[inst as usize][track_pattern][play_index as usize];
						let patch = self.track.patches[inst as usize][track_patch_pattern][play_index as usize];
						if self.mute[inst as usize] {
							note = -1;
						}

						// *** reset previous instrument patch? if 
						if patch >= 0 && (self.current_channel_patch[channel] != patch) {
							if self.current_channel_patch[channel] >= 0 {
								self.instruments[inst][self.current_channel_patch[channel] as usize].reset();
							}
							self.current_channel_patch[channel] = patch;
						}

						self.instruments[inst][self.current_channel_patch[channel] as usize].update(self.tick_counter, channel as u8, note, &mut self.resid, self.ticks_per_frame, false);
					}
				}

				self.tick_counter += 1;
			    self.play_index += self.tick_add * self.tick_scale as u32;
			    if ((self.play_index>>16)%64) < play_index {
				    if self.song_mode {
				    	self.pattern_idx += 1;
				    	if self.pattern_idx > self.end_pattern {
				    		self.pattern_idx = self.start_pattern;
							for i in 0..16 {
								for j in 0..16 {
									self.instruments[j][i].reset();
								}
							}
				    	}
				    }
				    else {
						for i in 0..16 {
							for j in 0..16 {
								self.instruments[j][i].reset();
							}
						}
				    }
				}
			}

			self.resid.write(0x15, (self.filter_freq & 0x07) as u8);
			self.resid.write(0x16, ((self.filter_freq>>3) & 0xff) as u8);

			self.resid.write(0x17, ((self.filter_res & 0x0f)<<4) | self.filter_mask);
			if self.state != Paused {
				self.resid.write(0x18, ((self.filter_type & 0x0f)<<4) | 0x0f);
			}
			let (samples, _next_delta) = self.resid.sample((985248.0/(self.ticks_per_frame as f64 * TICK_FREQ)) as u32, &mut self.buffer[..], 1);
		    for i in 0..samples {
		        let sidsample = self.buffer[i] as f32 / std::i16::MAX as f32;
		        raw_input[samples_out] = sidsample;
		        samples_out += 1;
		    }			
		}
		self.b_in.raw_publish();
		
		samples_out
	}


	pub fn update_preview(&mut self) -> usize {
		let mut samples_out = 0;
		let mut tick_counter = 0;
		let mut play_index = 0;
		//if self.state != Playing
		{
			self.instruments[self.inst_id as usize][self.inst_patch_id[self.inst_id as usize] as usize].reset();

	    	self.resid2.write(0x05, 0x00); // ad
		    self.resid2.write(0x06, 0x00); // sr
		    self.resid2.write(0x04, 0x08);	// gate off
		    for i in 0..(self.ticks_per_frame as u32 *50) {
			    self.resid2.sample((985248.0/(self.ticks_per_frame as f64 * TICK_FREQ)) as u32, &mut self.buffer2[..], 1);
			}
			
			while samples_out < self.preview.len()
			{
				let bar = (play_index>>16)%64;

				self.instruments[self.inst_id as usize][self.inst_patch_id[self.inst_id as usize] as usize].update(tick_counter, 0, if bar == 0 {self.instrument_notes[self.inst_id as usize]} else {-1}, &mut self.resid2, self.ticks_per_frame, false);				
				tick_counter += 1;
				play_index += self.tick_add;
			
				let (samples, _next_delta) = self.resid2.sample((985248.0/(self.ticks_per_frame as f64 * TICK_FREQ)) as u32, &mut self.buffer2[..], 1);
		    	for i in 0..samples {
		        	let sidsample = self.buffer2[i] as f32 / std::i16::MAX as f32;
		        	
		        	if samples_out < self.preview.len() {
		        		self.preview[samples_out] = sidsample;
		        	}
		        	samples_out += 1;
		    	}			
			}
		}
		samples_out
	}


	pub fn get_preview(&self, x: f64, length_scale: f64) -> f64 {
        let bps = self.bpm/60.0;
        let beat_length = 44100.0/bps;
        let preview_length = length_scale*beat_length;

		//println!("x: {:?}", x);
        let i = (x*preview_length) as usize;

		self.preview[i] as f64
	}

	pub fn export(&mut self) {

		fn find_slice(data: &Vec<u8>, data_length: usize, slice: &Vec<u8>, slice_length: usize) -> i32 {
			let mut i = 0;
			let mut result = -1;
			while i < data_length {
				if slice_length > 0 {
					if slice[0] == data[i] {
						let mut j=i+1;
						let mut k=1;
						let mut found = true;
						while (k<slice_length) && (j<data_length){
							if slice[k] != data[j] {
								found = false;
								break;
							}
							k += 1;
							j += 1;
						}
						found &= (k == slice_length) && (j<=data_length);
						if found {
							result = i as i32;
							i = data_length;
						}
					}
				}
				i += 1;
			}
			result
		};
		fn find_slice_u32(data: &Vec<u32>, data_length: usize, slice: &Vec<u32>, slice_length: usize) -> i32 {
			let mut i = 0;
			let mut result = -1;
			while i < data_length {
				if slice_length > 0 {
					if slice[0] == data[i] {
						let mut j=i+1;
						let mut k=1;
						let mut found = true;
						while (k<slice_length) && (j<data_length){
							if slice[k] != data[j] {
								found = false;
								break;
							}
							k += 1;
							j += 1;
						}
						found &= (k == slice_length) && (j<=data_length);
						if found {
							result = i as i32;
							i = data_length;
						}
					}
				}
				i += 1;
			}
			result
		};		

		let quantsize = self.export_quantsize as u32;

		let mut data_array = vec![vec![0 as u8; 256*256]; 3];
		let mut data_stored = [0; 3];
		
		let mut current_array = vec![0 as u8; 256*256];
		let mut current_length = 0;
		
		let mut idx_array =  vec![vec![vec![0 as u32; (self.ticks_per_16th as usize * 64)/quantsize as usize]; 3]; 64];

		let mut sep_values = vec![vec![0 as u8; 256*4]; 6];
		let mut sep_count = [0; 6];

		let mut sep_stream = vec![vec![0 as u8; 256*256]; 6];
		let mut sep_stream_count = [0; 6];

		let mut sep_idx_array =  vec![vec![vec![vec![0 as u32; (self.ticks_per_16th as usize * 64)/quantsize as usize]; 6]; 3]; 64];
		let mut sep_out_count = 0;
		let mut sep_idx_array_packed =  vec![0 as u32; 256*256];
		let mut sep_idx_pack_count = 0;
		let mut sep_idx_array_ptr =  vec![vec![vec![0 as u32; (self.ticks_per_16th as usize * 64)/quantsize as usize]; 3]; 64];

		if self.state == Playing
		{		
			self.key_space();
		}	

		for i in 0..16 {
			for j in 0..16 {
				self.instruments[j][i].reset();
			}
		}

		//let channel = self.inst_id as usize;
		for channel in 0..3 as usize
		{
			let mut samples_out = 0;
			let mut tick_counter = 0;
			let mut play_index = 0.0;
			let mut bytes_out = 0;

			let mut ed_prev = ExportData {
				kernel_type: 0,
				bytes: [0; 7]
			};


			let mut byte_ctr = vec![0; self.ticks_per_16th as usize*64];
			let mut last_bytes_out = 0;
		
			let mut pattern_ctr = 0;
			let mut pattern_idx = if self.song_mode {self.start_pattern} else {self.pattern_idx}; 
			let pattern_idx_end = if self.song_mode {self.end_pattern} else {self.pattern_idx};
			while pattern_idx <= pattern_idx_end 
			{
				tick_counter = 0;
				play_index = 0.0;
				while tick_counter < (self.ticks_per_16th as u32 * 64)
				{
					let play_index_ = ((play_index as u32)>>16)%64;
					
					let mut ed = ExportData{
						kernel_type: 0,
						bytes: [0; 7]
					};					
					for instloop in 0..1
					{
						let inst = (channel*1 + instloop) as usize;
						
						let track_pattern = self.track.patterns[pattern_idx as usize][inst] as usize; 
						let track_patch_pattern = self.track.patch_patterns[pattern_idx as usize][inst] as usize;

						let mut note = self.track.sequences[inst as usize][track_pattern][play_index_ as usize];
						let patch = self.track.patches[inst as usize][track_patch_pattern][play_index_ as usize];
						if self.mute[inst as usize] {
							note = -1;
						}

						// *** reset previous instrument patch? if 
						if patch >= 0 && (self.current_channel_patch[channel] != patch) {
							if self.current_channel_patch[channel] >= 0 {
								self.instruments[inst][self.current_channel_patch[channel] as usize].reset();
							}
							self.current_channel_patch[channel] = patch;
						}

						ed = self.instruments[inst][self.current_channel_patch[channel] as usize].update(tick_counter, channel as u8, note, &mut self.resid2, self.ticks_per_frame, true);
						
						if ed.kernel_type != 0 {
							break;
						}	
					}

					let mut ed_now = ed;

					if tick_counter > 0 {
						if (ed_now.kernel_type & ed_prev.kernel_type & 1) != 0
						{
							if ed_prev.bytes[0] == ed_now.bytes[0] {
								ed_now.kernel_type &= 31^1;
							}
						}
						if (ed_now.kernel_type & ed_prev.kernel_type & 2) != 0
						{
							if ed_prev.bytes[1] == ed_now.bytes[1] {
								ed_now.kernel_type &= 31^2;
							}
						}
						if (ed_now.kernel_type & ed_prev.kernel_type & 4) != 0
						{
							if (ed_prev.bytes[2] == ed_now.bytes[2]) && (ed_prev.bytes[3] == ed_now.bytes[3]) {
								ed_now.kernel_type &= 31^4;
							}
						}
						if (ed_now.kernel_type & ed_prev.kernel_type & 8) != 0
						{
							if ed_prev.bytes[4] == ed_now.bytes[4] {
								ed_now.kernel_type &= 31^8;
							}
						}
						if (ed_now.kernel_type & ed_prev.kernel_type & 16) != 0
						{
							if (ed_prev.bytes[5] == ed_now.bytes[5]) && (ed_prev.bytes[6] == ed_now.bytes[6]) {
								ed_now.kernel_type &= 31^16;
							}
						}															
					}

					sep_values[5][sep_count[5]] = ed_now.kernel_type;
					sep_count[5] += 1;

					//print!(".byte    {:?}", ed_now.kernel_type);
					current_array[last_bytes_out as usize] = ed_now.kernel_type;
					last_bytes_out += 1;
					bytes_out += 1;

					if (ed_now.kernel_type & 16) != 0 {
						//print!(", {:?}, {:?}", ed_now.bytes[5], ed_now.bytes[6]);
						current_array[last_bytes_out as usize] = ed_now.bytes[5];
						last_bytes_out += 1;
						current_array[last_bytes_out as usize] = ed_now.bytes[6];
						last_bytes_out += 1;
						bytes_out += 2;

						sep_values[4][sep_count[4]] = ed_now.bytes[5];
						sep_values[4][sep_count[4]+1] = ed_now.bytes[6];
						sep_count[4] += 2;
					}					
					if (ed_now.kernel_type & 1) != 0 {
						//print!(", {:?}", ed_now.bytes[0]);
						current_array[last_bytes_out as usize] = ed_now.bytes[0];
						last_bytes_out += 1;
						bytes_out += 1;

						sep_values[0][sep_count[0]] = ed_now.bytes[0];
						sep_count[0] += 1;						
					}
					if (ed_now.kernel_type & 2) != 0 {
						//print!(", {:?}", ed_now.bytes[1]);
						current_array[last_bytes_out as usize] = ed_now.bytes[1];
						last_bytes_out += 1;						
						bytes_out += 1;

						sep_values[1][sep_count[1]] = ed_now.bytes[1];
						sep_count[1] += 1;						
					}
					if (ed_now.kernel_type & 4) != 0 {
						//print!(", {:?}, {:?}", ed_now.bytes[2], ed_now.bytes[3]);
						current_array[last_bytes_out as usize] = ed_now.bytes[2];
						last_bytes_out += 1;
						current_array[last_bytes_out as usize] = ed_now.bytes[3];
						last_bytes_out += 1;												
						bytes_out += 2;

						sep_values[2][sep_count[2]] = ed_now.bytes[2];
						sep_values[2][sep_count[2]+1] = ed_now.bytes[3];
						sep_count[2] += 2;						
					}
					if (ed_now.kernel_type & 8) != 0 {
						//print!(", {:?} ", ed_now.bytes[4]);
						current_array[last_bytes_out as usize] = ed_now.bytes[4];
						last_bytes_out += 1;						
						bytes_out += 1;

						sep_values[3][sep_count[3]] = ed_now.bytes[4];
						sep_count[3] += 1;
					}

					
					//println!();

					if (tick_counter % quantsize) == (quantsize-1) {
						byte_ctr[(tick_counter/quantsize) as usize] = last_bytes_out;

						let idx = find_slice(&(data_array[channel]), data_stored[channel], &current_array, last_bytes_out);
						if (idx < 0) 
						{
							for i in 0..last_bytes_out {
								data_array[channel][data_stored[channel]+i] = current_array[i];
							}
							idx_array[pattern_ctr][channel][(tick_counter/quantsize) as usize] = data_stored[channel] as u32;
							data_stored[channel] += last_bytes_out;


						}
						else {
							idx_array[pattern_ctr][channel][(tick_counter/quantsize) as usize] = idx as u32;
						}

						last_bytes_out = 0;

						for j in 0..6 {
							if sep_count[j] == 0 {
								sep_idx_array[pattern_ctr][channel][j][(tick_counter/quantsize) as usize] = 0;
								//sep_out_count += 2;
							}
							else {	
								let idx = find_slice(&(sep_stream[j]), sep_stream_count[j], &sep_values[j], sep_count[j]);
								if (idx < 0)
								{
									for i in 0..sep_count[j] {
										sep_stream[j][sep_stream_count[j]+i] = sep_values[j][i];
									}

									sep_idx_array[pattern_ctr][channel][j][(tick_counter/quantsize) as usize] = sep_stream_count[j] as u32;
									sep_stream_count[j] += sep_count[j];

									sep_out_count += sep_count[j];//+2;
								}
								else {
									sep_idx_array[pattern_ctr][channel][j][(tick_counter/quantsize) as usize] = idx as u32;
									//sep_out_count += 2;
								}
							}

							/*print!("{}: ", sep_count[j]);
							for i in 0..sep_count[j] {
								print!("{}, ", sep_values[j][i]);
							}*/
							sep_count[j] = 0;
							//println!();
						}
						//println!();
						
					}

					ed_prev = ed;

					tick_counter += 1;
					//play_index += self.tick_add_f64;
					play_index += self.tick_add_f64 * self.tick_scale as f64;

				}
				pattern_idx += 1;
				pattern_ctr += 1;
			}

		}
		


		let path_name = format!("./bng");
		if !Path::new(&path_name).exists() {
			fs::create_dir(&path_name);
		}

		let mut asm_file = File::create(format!("{}/{}.asm", &path_name, self.session_name)).unwrap();

		let mut totalsize = 0;
		let patterns = if self.song_mode {self.end_pattern-self.start_pattern+1} else {1};

		let psize = (self.ticks_per_16th as usize * 64)/quantsize as usize;
		let mut packed_index = vec![vec![0; psize]; 3];
		for channel in 0.. 3 {
			for j in 1..patterns as usize {
				let mut equals = -1;
				for testpattern in 0..j {
					equals = testpattern as i32;
					for i in 0..psize {
						if idx_array[j][channel][i] != idx_array[testpattern][channel][i] {
							equals = -1;
							break;
						}
					}
					if equals == testpattern as i32 {
						break;
					}
				}
				packed_index[channel][j] = if equals >= 0 {equals} else {j as i32};
			}
		}	

		writeln!(asm_file, "bng_ptc: .byte {}", patterns);
		writeln!(asm_file, "bng_qsize: .byte {}", quantsize);
		writeln!(asm_file, "bng_ticks: .word {}", self.ticks_per_16th as u32 * 64/(quantsize*self.tick_scale as u32));

		for channel in 0..3 {
			writeln!(asm_file, "ch_idx_ptr{}:", channel+1);
			writeln!(asm_file);
			for i in 0..patterns as usize {
				if (i&7) == 0 {
					write!(asm_file, ".word    ch{}_idx_{}", channel+1, packed_index[channel][i]+1);
				}
				else {
					write!(asm_file, ", ch{}_idx_{}", channel+1, packed_index[channel][i]+1);
				}
				if (i&7) == 7 {
					writeln!(asm_file);
				}
			}
			writeln!(asm_file);
			writeln!(asm_file);


		}
/*
		for channel in 0..6 {
			println!("stream_{}:", channel+1);
			println!();
	
			for i in 0..sep_stream_count[channel] {
				if (i&15) == 0 {
					print!(".byte    {:?}", sep_stream[channel][i]);
				} 
				else {
					print!(", {:?}", sep_stream[channel][i]);
				}
				if (i&15) == 15 {
					println!();
				}
				if (i&255) == 255 {
					println!();
				}
			}
			println!();
		}

		let store5 = true;
		if store5 { 
			for channel in 0..3 {
				for j in 0..patterns as usize {
					if packed_index[channel][j] as usize == j {
						for i in 0..(self.ticks_per_16th as usize * 64)/quantsize as usize {
							let current_slice = vec![sep_idx_array[j][channel][0][i],
									sep_idx_array[j][channel][1][i],
									sep_idx_array[j][channel][2][i],
									sep_idx_array[j][channel][3][i],
									sep_idx_array[j][channel][4][i],
									sep_idx_array[j][channel][5][i]
									];

							let idx = find_slice_u32(&(sep_idx_array_packed), sep_idx_pack_count, &current_slice, 5);
							if (idx < 0)
							{
								for k in 0..5 {
									sep_idx_array_packed[sep_idx_pack_count+k] = current_slice[k];
								}

								sep_idx_array_ptr[j][channel][i] = sep_idx_pack_count as u32;
								sep_idx_pack_count += 5;

								sep_out_count += 12+2;
							}
							else {
								sep_idx_array_ptr[j][channel][i] = idx as u32;
								sep_out_count += 2+2;
							}
						}
					}
				}
			}

			println!();
			println!();
			for i in 0..sep_idx_pack_count/5 {
				println!(".word    {:?}, {:?}, {:?}, {:?}, {:?}", 
					sep_idx_array_packed[i*5+0],
					sep_idx_array_packed[i*5+1],
					sep_idx_array_packed[i*5+2],
					sep_idx_array_packed[i*5+3],
					sep_idx_array_packed[i*5+4]
					);
			}
			println!();

			println!("// {} segments", sep_idx_pack_count/5);
		}
		else {
		
			for channel in 0..3 {
				for j in 0..patterns as usize {
					if packed_index[channel][j] as usize == j {
						for i in 0..(self.ticks_per_16th as usize * 64)/quantsize as usize {
							let current_slice = vec![sep_idx_array[j][channel][0][i],
									sep_idx_array[j][channel][1][i],
									sep_idx_array[j][channel][2][i],
									sep_idx_array[j][channel][3][i],
									sep_idx_array[j][channel][4][i],
									sep_idx_array[j][channel][5][i]
									];

							let idx = find_slice_u32(&(sep_idx_array_packed), sep_idx_pack_count, &current_slice, 6);
							if (idx < 0)
							{
								for k in 0..6 {
									sep_idx_array_packed[sep_idx_pack_count+k] = current_slice[k];
								}

								sep_idx_array_ptr[j][channel][i] = sep_idx_pack_count as u32;
								sep_idx_pack_count += 6;

								sep_out_count += 12+2;
							}
							else {
								sep_idx_array_ptr[j][channel][i] = idx as u32;
								sep_out_count += 2;
							}
						}
					}
				}
			}

			println!();
			println!();
			for i in 0..sep_idx_pack_count/6 {
				println!(".word    {:?}, {:?}, {:?}, {:?}, {:?}, {:?}", 
					sep_idx_array_packed[i*6+0],
					sep_idx_array_packed[i*6+1],
					sep_idx_array_packed[i*6+2],
					sep_idx_array_packed[i*6+3],
					sep_idx_array_packed[i*6+4],
					sep_idx_array_packed[i*6+5]
					);
			}
			println!();

			println!("// {} segments", sep_idx_pack_count/6);
		}

		for channel in 0..3 {
			for j in 0..patterns as usize {
				if packed_index[channel][j] as usize == j {
					println!();
					println!();
					println!("ch{}_idx_{}:", channel+1, j+1);
					println!();

					let lf = (self.ticks_per_16th * 2.0) as u32/quantsize;

					for i in 0..(self.ticks_per_16th as usize * 64)/quantsize as usize {
						if (i as u32%lf) == 0 {
							print!(".word    {:?}", sep_idx_array_ptr[j][channel][i]);
						} 
						else {
							print!(", {:?}", sep_idx_array_ptr[j][channel][i]);
						}
						if (i as u32%lf) == lf-1 {
							println!();
						}
						if (i as u32%lf*4) == lf*4-1 {
							println!();
						}
					}
				}
			}
		}
		println!("// --- schema2 export size: {}", sep_out_count);
*/
		for channel in 0..3 {
			writeln!(asm_file, "ch{}:", channel+1);
			writeln!(asm_file);
	
			for i in 0..data_stored[channel] {
				if (i&15) == 0 {
					write!(asm_file, ".byte    {:?}", data_array[channel][i]);
				} 
				else {
					write!(asm_file, ", {:?}", data_array[channel][i]);
				}
				if (i&15) == 15 {
					writeln!(asm_file);
				}
				if (i&255) == 255 {
					writeln!(asm_file);
				}
			}

			let mut patterns_written = 0;
			for j in 0..patterns as usize {
				if packed_index[channel][j] as usize == j {
					patterns_written += 1;
					writeln!(asm_file);
					writeln!(asm_file);
					writeln!(asm_file, "ch{}_idx_{}:", channel+1, j+1);
					writeln!(asm_file);

					for i in 0..(self.ticks_per_16th as usize * 64)/quantsize as usize {
						if (i%6) == 0 {
							write!(asm_file, ".word    {:?}", idx_array[j][channel][i]);
						} 
						else {
							write!(asm_file, ", {:?}", idx_array[j][channel][i]);
						}
						if (i%6) == 5 {
							writeln!(asm_file);
						}
						if (i%6*4) == 6*4-1 {
							writeln!(asm_file);
						}
					}
				}
			}
			writeln!(asm_file);
			writeln!(asm_file, "// --- size: {:?} bytes", data_stored[channel] + patterns_written as usize * 2*(self.ticks_per_16th as usize *64)/quantsize as usize);
			writeln!(asm_file);

			totalsize += data_stored[channel] + patterns_written as usize * 2*(self.ticks_per_16th as usize *64)/quantsize as usize;
		}
		writeln!(asm_file);
		writeln!(asm_file, "// --- total: {:?} bytes", totalsize);
		writeln!(asm_file);

		println!("// --- export total: {:?} bytes", totalsize);

	}

	pub fn check_update(&mut self)
	{
		if self.update.load(std::sync::atomic::Ordering::Relaxed) {
			let samples = self.render();

			self.length.store(samples as i32, std::sync::atomic::Ordering::Relaxed);
			self.update.store(false, std::sync::atomic::Ordering::Relaxed);
		}
	}

	pub fn key_space(&mut self) {
		self.state = if self.state == Paused {Playing} else {Paused}; 
		if self.state == Paused {
			self.resid.write(0x18, 0x00);
		}
		else {
			self.resid.write(0x18, ((self.filter_type & 0x0f)<<4) | 0x0f);
		}
	}

	pub fn key_return(&mut self) {
		self.restart();
	}	


	fn restart(&mut self) {
		self.play_index = 0;
	}

	pub fn playback(&mut self) {

		let event_loop = cpal::default_host().event_loop();
	    let stream_id = event_loop.build_output_stream(&self.device, &self.format).unwrap();
	    event_loop.play_stream(stream_id.clone()).unwrap();

	    let mut samples_max = (self.sample_time * self.sample_rate as f32) as usize - 1;
	    //let fmt = self.format.clone();

	    let mut samples_counter = 0;
	    let mut channel_counter = 0;
	    let max_channels = self.out_channels;

	    let update_clone = self.update.clone();
		let length_clone = self.length.clone();

		let t_buffer = TripleBuffer::new(vec![0f32; self.sample_length+1024]);
	   	let (b_in, mut b_out) = t_buffer.split();

	   	self.b_in = b_in;
		
	    self.render();

	    let mut bptr = vec![0f32; self.sample_length+1024];
	    let mut next_value = move || {
	    	if samples_counter == 0	{
	    		b_out.raw_update();
				bptr = b_out.raw_output_buffer().to_vec();
				samples_max = length_clone.load(std::sync::atomic::Ordering::Relaxed) as usize;
	    		update_clone.store(true, std::sync::atomic::Ordering::Relaxed);
			}
	        let sample = bptr[samples_counter];
	        channel_counter += 1;
	        if channel_counter == max_channels
	        {
	        	channel_counter = 0;
		        if samples_counter < samples_max {
		        	samples_counter += 1;
		        }
		        else {
		        	samples_counter = 0;
		        }
		    }
	 		sample
	    };

	    // println!("start");
		std::thread::spawn(move || {
			event_loop.run(move |stream_id, stream_result| {
			    let stream_data = match stream_result {
			        Ok(data) => data,
			        Err(err) => {
			            eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
			            return;
			        }
			    };
			    match stream_data {
			        StreamData::Output { buffer: UnknownTypeOutputBuffer::U16(mut buffer) } => {
			        	//println!("size U16: {}",buffer.iter().count());
			            for elem in buffer.iter_mut() {
			                //*elem = u16::max_value() / 2;
				            let s_val = next_value();
		                	if s_val > 1.0 {
		                		return;
		                	}
		                	else {
			                	let value = ((s_val * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
			                    *elem = value;
			                }
			            }
			        },
			        StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut buffer) } => {
			            //println!("size I16: {}",buffer.iter().count());
			            for elem in buffer.iter_mut() {
			                //*elem = 0;
			                let s_val = next_value();
		                	if s_val > 1.0 {
		                		return;
		                	}
		                	else {
			                	let value = (s_val * std::i16::MAX as f32) as i16;
			                    *elem = value;
			                }
			            }

			        },
			        StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer) } => {
			            //println!("size F32: {}",buffer.iter().count());
			            for elem in buffer.iter_mut() {
			                //*elem = 0.0;
			                let s_val = next_value();
		                	if s_val > 1.0 {
		                		return;
		                		//std::process::exit(1);
		                	}
		                	else {
			                	let value = s_val;
			                    *elem = value;
			                }
			            }
			        },
			        _ => (),
			    }
			})
		});	
		
		//event_loop.destroy_stream(stream_id.clone());
	}

}