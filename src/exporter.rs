#![allow(dead_code, unused)]

use std::fs;
use std::fs::File;
//use std::io::prelude::*;
//use std::error::Error;
use std::io::{Write, BufReader};
use std::path::Path;

use crate::sid_player::{SidPlayer, PlayerState::Playing, ExportData};

pub struct Exporter {
}

impl Exporter {
	pub fn export(player: &mut SidPlayer) {

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

		let quantsize = player.export_quantsize as u32;


		let mut data_array = vec![vec![0 as u8; 256*256]; 3];
		let mut data_stored = [0; 3];

		let chunk_index = [0x3800, 0xe000, 0xe000];	// 3 bins for splitting output data...for now only 1st bin will be needed
		let chunk_size = [0x9800, 0x1f00, 0x1f00];
		let use_export_bins = 2;

		let mut current_array = vec![0 as u8; 256*256];
		let mut current_length = 0;
		
		let mut idx_array =  vec![vec![vec![0 as u32; (player.ticks_per_16th as usize * 64)/quantsize as usize]; 3]; 64];

		let mut sep_values = vec![vec![0 as u8; 256*4]; 6];
		let mut sep_count = [0; 6];

		let mut sep_stream = vec![vec![0 as u8; 256*256]; 6];
		let mut sep_stream_count = [0; 6];

		let mut sep_idx_array =  vec![vec![vec![vec![0 as u32; (player.ticks_per_16th as usize * 64)/quantsize as usize]; 6]; 3]; 64];
		let mut sep_out_count = 0;
		let mut sep_idx_array_packed =  vec![0 as u32; 256*256];
		let mut sep_idx_pack_count = 0;
		let mut sep_idx_array_ptr =  vec![vec![vec![0 as u32; (player.ticks_per_16th as usize * 64)/quantsize as usize]; 3]; 64];

		if player.state == Playing
		{		
			player.key_space();
		}	

		for i in 0..16 {
			for j in 0..16 {
				player.instruments[j][i].reset();
			}
		}

		//let channel = player.inst_id as usize;
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


			let mut byte_ctr = vec![0; player.ticks_per_16th as usize*64];
			let mut last_bytes_out = 0;
		
			let mut pattern_ctr = 0;
			let mut pattern_idx = if player.song_mode {player.start_pattern} else {player.pattern_idx}; 
			let pattern_idx_end = if player.song_mode {player.end_pattern} else {player.pattern_idx};
			while pattern_idx <= pattern_idx_end 
			{
				tick_counter = 0;
				play_index = 0.0;
				while tick_counter < (player.ticks_per_16th as u32 * 64)
				{
					let play_index_ = ((play_index as u32)>>16)%64;
					
					let mut ed = ExportData{
						kernel_type: 0,
						bytes: [0; 7]
					};					
					for instloop in 0..1
					{
						let inst = (channel*1 + instloop) as usize;
						
						let track_pattern = player.track.patterns[pattern_idx as usize][inst] as usize; 
						let track_patch_pattern = player.track.patch_patterns[pattern_idx as usize][inst] as usize;

						let mut note = player.track.sequences[inst as usize][track_pattern][play_index_ as usize];
						let patch = player.track.patches[inst as usize][track_patch_pattern][play_index_ as usize];
						if player.mute[inst as usize] {
							note = -1;
						}

						// *** reset previous instrument patch? if 
						if patch >= 0 && (player.current_channel_patch[channel] != patch) {
							if player.current_channel_patch[channel] >= 0 {
								player.instruments[inst][player.current_channel_patch[channel] as usize].reset();
							}
							player.current_channel_patch[channel] = patch;
						}

						ed = player.instruments[inst][player.current_channel_patch[channel] as usize].update(tick_counter, channel as u8, note, &mut player.resid2, player.ticks_per_frame, true);
						
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

						let mut chunk = 0;
						let mut idx = find_slice(&(data_array[chunk]), data_stored[chunk], &current_array, last_bytes_out);
						if (idx < 0) 
						{
							if data_stored[chunk+1] > 0
							{
								chunk += 1;
								idx = find_slice(&(data_array[chunk]), data_stored[chunk], &current_array, last_bytes_out);
								if (idx < 0) 
								{
									if data_stored[chunk+1] > 0
									{
										chunk += 1;
										idx = find_slice(&(data_array[chunk]), data_stored[chunk], &current_array, last_bytes_out);
										if (idx < 0) 
										{
											if data_stored[chunk] + last_bytes_out > chunk_size[chunk] {
												println!("chunk3 size reached");
											}
											else {
												for i in 0..last_bytes_out {
													data_array[chunk][data_stored[chunk]+i] = current_array[i];
												}
												idx_array[pattern_ctr][channel][(tick_counter/quantsize) as usize] = data_stored[chunk] as u32 + chunk_index[chunk];
												data_stored[chunk] += last_bytes_out;
											}
										}
										else {
											idx_array[pattern_ctr][channel][(tick_counter/quantsize) as usize] = idx as u32 + chunk_index[chunk];
										}
									}
									else {
										if data_stored[chunk] + last_bytes_out > chunk_size[chunk] {
											println!("chunk2 size reached");
											chunk += 1;
										}
										for i in 0..last_bytes_out {
											data_array[chunk][data_stored[chunk]+i] = current_array[i];
										}
										idx_array[pattern_ctr][channel][(tick_counter/quantsize) as usize] = data_stored[chunk] as u32 + chunk_index[chunk];
										data_stored[chunk] += last_bytes_out;
									}
								}
								else {
									idx_array[pattern_ctr][channel][(tick_counter/quantsize) as usize] = idx as u32 + chunk_index[chunk];
								}								
							}
							else 
							{
								if data_stored[chunk] + last_bytes_out > chunk_size[chunk] {
									println!("chunk1 size reached");
								 	chunk += 1;
								}
								//else 	// remove else for multiple chunks 
								{
									for i in 0..last_bytes_out {
										data_array[chunk][data_stored[chunk]+i] = current_array[i];
									}
									idx_array[pattern_ctr][channel][(tick_counter/quantsize) as usize] = data_stored[chunk] as u32 + chunk_index[chunk];
									data_stored[chunk] += last_bytes_out;
								}
							}
						}
						else {
							idx_array[pattern_ctr][channel][(tick_counter/quantsize) as usize] = idx as u32 + chunk_index[chunk];
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
					//play_index += player.tick_add_f64;
					play_index += player.tick_add_f64 * player.tick_scale as f64;

				}
				pattern_idx += 1;
				pattern_ctr += 1;
			}

		}
		


		let path_name = format!("./c64/data");
		if !Path::new(&path_name).exists() {
			fs::create_dir(&path_name);
		}

		let mut asm_file = File::create(format!("{}/bng.asm", &path_name)).unwrap();

		let mut totalsize = 0;
		let patterns = if player.song_mode {player.end_pattern-player.start_pattern+1} else {1};

		let psize = (player.ticks_per_16th as usize * 64)/quantsize as usize;
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
		writeln!(asm_file, "bng_ticks: .word {}", player.ticks_per_16th as u32 * 64/(quantsize*player.tick_scale as u32));

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
						for i in 0..(player.ticks_per_16th as usize * 64)/quantsize as usize {
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
						for i in 0..(player.ticks_per_16th as usize * 64)/quantsize as usize {
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

					let lf = (player.ticks_per_16th * 2.0) as u32/quantsize;

					for i in 0..(player.ticks_per_16th as usize * 64)/quantsize as usize {
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
			let mut patterns_written = 0;
			for j in 0..patterns as usize {
				if packed_index[channel][j] as usize == j {
					patterns_written += 1;
					writeln!(asm_file);
					writeln!(asm_file);
					writeln!(asm_file, "ch{}_idx_{}:", channel+1, j+1);
					writeln!(asm_file);

					for i in 0..(player.ticks_per_16th as usize * 64)/quantsize as usize {
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
			writeln!(asm_file, "// --- size: {:?} bytes", data_stored[channel] + patterns_written as usize * 2*(player.ticks_per_16th as usize *64)/quantsize as usize);
			writeln!(asm_file);

			totalsize += patterns_written as usize * 2*(player.ticks_per_16th as usize *64)/quantsize as usize;
		}

		for chunk in 0..use_export_bins {
			writeln!(asm_file);
			writeln!(asm_file, ".pc = {}",chunk_index[chunk]);
			//writeln!(asm_file, "ch{}:", chunk+1);
			writeln!(asm_file);
	
			for i in 0..data_stored[chunk] {
				if (i&15) == 0 {
					write!(asm_file, ".byte    {:?}", data_array[chunk][i]);
				} 
				else {
					write!(asm_file, ", {:?}", data_array[chunk][i]);
				}
				if (i&15) == 15 {
					writeln!(asm_file);
				}
				if (i&255) == 255 {
					writeln!(asm_file);
				}
			}
			totalsize += data_stored[chunk];
		}

		writeln!(asm_file);
		writeln!(asm_file, "// --- total: {:?} bytes", totalsize);
		writeln!(asm_file);

		println!("// --- export total: {:?} bytes", totalsize);

	}
}
