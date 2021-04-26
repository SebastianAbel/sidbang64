#![allow(dead_code, unused)]

//#[macro_use]
extern crate conrod_core;
extern crate rand;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::io::BufReader;
use std::path::Path;
use std::cmp::{min,max};

use serde::{Serialize, Deserialize};
use serde_json::Result;

use crate::{
    sid_player::SidPlayer,
};
use crate::oscillator::{Waveform, TICK_FREQ};
use crate::instrument::{Instrument, BaseMode};
use crate::exporter::Exporter;

pub const WIN_W: u32 = (1920.0 * 0.85) as u32;
pub const WIN_H: u32 = (1080.0 * 0.775) as u32;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum KeyboardMode {
    Off,
    QWERTY,    
    QWERTZ,
}

//#[derive(Serialize, Deserialize, Debug)]
pub struct DemoApp {
    session_name: String,
    pub preview_update: u8,
    pub osc_select: [u8; 16],   

    pub kb_mode: KeyboardMode,
    pub selected_octave: [i8; 3],

    pub paste_key: bool,
    pub shift_key: bool,

    pub copy_patch: Instrument,
    pub copy_osc_patch_id: u8,
    pub copy_osc_patch: Instrument,  

    pub copy_seq: Vec<i8>,
    pub copy_patch_seq: Vec<i8>,  

    pub copy_pattern: [i16; 3],
    pub copy_patch_pattern: [i16; 3],

    pub copy4_pattern: [[i16; 3]; 4],
    pub copy4_patch_pattern: [[i16; 3]; 4],

    pub input_keycode: Option<glium::glutin::VirtualKeyCode>,
    pub input_delay: u8,
}


impl DemoApp {
    pub fn new() -> Self {

        DemoApp {
            session_name: "default".to_string(),
            preview_update: 10,
            osc_select: [0; 16],

            kb_mode: KeyboardMode::QWERTY,    
            selected_octave: [0; 3], 

            paste_key: false,
            shift_key: false,

            copy_patch: Instrument::new("copy1".to_string()),
            copy_osc_patch_id: 0,
            copy_osc_patch: Instrument::new("copy2".to_string()),            

            copy_seq: vec![-1; 64],
            copy_patch_seq: vec![-1; 64],

            copy_pattern: [0; 3],
            copy_patch_pattern: [0; 3],

            copy4_pattern: [[0; 3]; 4],
            copy4_patch_pattern: [[0; 3]; 4],

            input_keycode: None,
            input_delay: 0,
        }
    }

    pub fn set_note(&mut self, note: u8) {

    }
/*
    pub fn save_session(&mut self, session_name: &String) -> std::io::Result<()> {
        let path_name = format!("./bng/{}", session_name);
        if !Path::new(&path_name).exists() {
            fs::create_dir(&path_name)?;
        }

        let file = File::create(format!("{}/gui.json", path_name))?;
        let _j = serde_json::to_writer(file, &self);
        Ok(())
    }

    fn read_session_from_file(&mut self, file: File) -> Result<DemoApp> {
        //let file = File::open(path)?;
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `User`.
        let u = serde_json::from_reader(reader)?;

        // Return the `User`.
        Ok(u)
    }

    pub fn load_session(&mut self, session_name: &String) -> std::io::Result<()> {
        let path_name = format!("./bng/{}", session_name);
        if Path::new(&path_name).exists()
        {   
            //println!("{:?}", session_name);
            //println!("{:?}", format!("{}/gui.json", &path_name));
            let file = File::open(format!("{}/gui.json", &path_name))?;
            let s = self.read_session_from_file(file);

            *self = s.unwrap();
        }
        Ok(())
    }       
*/
}


/// A set of reasonable stylistic defaults that works for the `gui` below.
pub fn theme() -> conrod_core::Theme {
    use conrod_core::position::{Align, Direction, Padding, Position, Relative};
    conrod_core::Theme {
        name: "SidBang Theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod_core::color::DARK_CHARCOAL,
        shape_color: conrod_core::color::LIGHT_CHARCOAL,
        border_color: conrod_core::color::BLACK,
        border_width: 0.0,
        label_color: conrod_core::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 14,
        font_size_small: 12,
        widget_styling: conrod_core::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        // The scrollable canvas.
        canvas,
        // The title and introduction widgets.
        title,
        session_name_bg,
        session_name,

        load,
        save,
        export,
        export_q,
        reset,
        app_title,

        patches_title,
        
        pwm_reduction,
        patch_idx[],

        base_note_type_a,
        base_note_a,
        base_note_a2,
        base_note_type_b,
        base_note_b,
        base_note_b2,
        attack,
        decay,
        sustain,
        hold,
        release,

        copy_patch,
        paste_patch,

        attack_osc,
        decay_osc,
        sustain_osc,
        release_osc,

        attack_hold,
        decay_hold,
        sustain_hold,
        release_hold,

        osc_select[],
        copy_osc_patch,
        paste_osc_patch,

        // frequency
        osc1_wav,
        osc1_frq,
        osc1_frq_ticks,
        osc1_ratio,

        // vibrato
        osc2_wav,
        osc2_frq,
        osc2_frq_ticks,
        osc2_ratio,

        vibrato,

        // wavetype
        osc3_wav,
        osc3_frq,
        osc3_frq_ticks,
        osc3_ratio,

        wave_a[],
        wave_b[],

        // pulsewidth
        osc4_wav,
        osc4_frq,
        osc4_frq_ticks,
        osc4_ratio,

        pulsewidth_a,
        pulsewidth_b,

        seq_time,

        player_speed,
        bpm,
        ticks,
        tick_scale,
        pattern_title,
        start_pattern_select,
        end_pattern_select,
        song_length_display,
        song_mode_select,

        insert_loop,
        delete_loop,

        copy_loop,
        paste_loop,

        copy4_loop,
        paste4_loop,

        copy_pattern,
        paste_pattern,



        mute[],
        solo[],
        play_index[],

        kbd_title,
        kbd[],

        vibrato_1,
        vib1_x_add1,
        vib1_x_offset1,

        osc1_add1,
        osc1_note_a,
        osc1_note_b,
      
        osc1,
        wf1[],
        wf2[], 
        wf_cu_1,
        wf_pw_1,

        pw1,
        pw2,

        pw_osc1_add1,

        filter_freq,
        filter_add,
        filter_res,
        filter_voice[],
        filter_type[],
        filter_label[],

        filter_patch_idx[],


        seq_title,

        pattern_idx[],
        seq1[],

        track_sequence[],
        track_inst[],
        trackborder[],

        canvas_scrollbar,

        grid,
        plot,
        grid2,
        plot2,   

        kbd_mode,
    }
}


pub fn gui(ui: &mut conrod_core::UiCell, ids: &mut Ids, app: &mut DemoApp, player: &mut SidPlayer) {
    use conrod_core::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
    //use std::iter::once;

    const MARGIN: conrod_core::Scalar = 30.0;
    const SHAPE_GAP: conrod_core::Scalar = 50.0;
    const TITLE_SIZE: conrod_core::FontSize = 24;
    const SUBTITLE_SIZE: conrod_core::FontSize = 20;
    const TEXT_SIZE_TINY: conrod_core::FontSize = 12;
    const TEXT_SIZE_MED: conrod_core::FontSize = 16;
    const TEXT_SIZE_SMALL: conrod_core::FontSize = 14;

    const WIDGET_SIZE: f64 = 20.0;
    const WIDGET_DISTANCE: f64 = 16.0;

    let osc_types = ["tri", "saw", "rct", "nse", "isw", "sin"];

    let list_count4 = [" 0 ", " 1 ", " 2 ", " 3 "];
    let note_names_a = ["  C-", "  C#", "  D-", "  D#", "  E-", "  F-", "  F#", "  G-", "  G#", "  A-", "  A#", "  B-"];
    let octave_names_a = ["0 ", "1 ", "2 ", "3 ", "4 ", "5 ", "6 ", "7 "];
    let note_names = ["C-", "C#", "D-", "D#", "E-", "F-", "F#", "G-", "G#", "A-", "A#", "B-"];

    let base_mode_types = ["zro", "inp", "rep", "add", "sub"];
    let base_note_types = [" - ", "rep", "add", "sub", "src", "dest"];

    let track_sequence_ids = ["00", "01", "02", "03", "04", "05", "06", "07", "08", 
        "09", "10", "11", "12", "13", "14", "15", "16",
        "17", "18", "19", "20", "21", "22", "23", "24",
        "25", "26", "27", "28", "29", "30", "31",
        ];

    let tick_scales = ["1/16", "1/32", "1/64"];

    let kbd_modes = [" off ", "QWERTY", "QWERTZ"];

    if app.preview_update > 0 
    {
        app.preview_update -= 1;
        if app.preview_update == 0 {
            player.update_preview();
        }    
    }
    
    widget::Canvas::new().pad(MARGIN).scroll_kids_vertically().set(ids.canvas, ui);
    widget::Text::new("").font_size(TITLE_SIZE).left_justify().top_left_of(ids.canvas).set(ids.title, ui);

    widget::BorderedRectangle::new([WIDGET_SIZE*10.0, WIDGET_SIZE])
        .right_from(ids.title, 0.0)
        .color(conrod_core::color::CHARCOAL)
        .set(ids.session_name_bg, ui);

    for edit in widget::TextEdit::new(&app.session_name)
        .right_from(ids.title, WIDGET_SIZE*0.5)
        .color(conrod_core::color::GREY)
        .w_h(WIDGET_SIZE*9.0, WIDGET_SIZE)
        .set(ids.session_name, ui)
    {
        app.session_name = edit;       
        app.input_delay = 0; 
    }

    for _press in widget::Button::new()
        .right(WIDGET_DISTANCE*4.0)
        .color(conrod_core::color::BLUE)
        .w_h(WIDGET_SIZE*4.0, WIDGET_SIZE)
        .label(&format!("LOAD"))
        .set(ids.load, ui)
    {
       player.session_name = app.session_name.to_string();
       player.load_session().unwrap();
       //app.load_session(&app.session_name.to_string()).unwrap();

       app.preview_update = 10;
    }

    for _press in widget::Button::new()
        .right(WIDGET_DISTANCE)
        .color(conrod_core::color::RED)
        .w_h(WIDGET_SIZE*4.0, WIDGET_SIZE)
        .label(&format!("SAVE"))
        .set(ids.save, ui)
    {
        player.session_name = app.session_name.to_string();
        player.save_session().unwrap();
        //app.save_session(&app.session_name.to_string()).unwrap();
    }

    for _press in widget::Button::new()
        .right(WIDGET_DISTANCE)
        .color(conrod_core::color::LIGHT_RED)
        .w_h(WIDGET_SIZE*4.0, WIDGET_SIZE)
        .label(&format!("EXPORT"))
        .set(ids.export, ui)
    {
        Exporter::export(player);
    }    

    for dialed in widget::NumberDialer::new(player.export_quantsize as f64, 0.0, 255.0, 0)
        .right(WIDGET_DISTANCE)
        .w_h(WIDGET_SIZE*3.0, WIDGET_SIZE)
        .label(&format!("Q"))
        .set(ids.export_q, ui)
    {
        player.export_quantsize = dialed as u16;
    }    

    for _press in widget::Button::new()
        .right(WIDGET_DISTANCE*4.8)
        //.color(conrod_core::color::LIGHT_GREEN)
        .w_h(WIDGET_SIZE*4.0, WIDGET_SIZE)
        .label(&format!("SIDRESET"))
        .set(ids.reset, ui)
    {
        player.reset();
    }  


    widget::Text::new("sidbang64 v0.6.5a / w4rp8 / 2021")
        .font_size(TEXT_SIZE_TINY)
        .color(conrod_core::color::BLUE)
        .right(WIDGET_DISTANCE*39.0)
        .set(ids.app_title, ui);

    widget::Text::new("Patch:")
        .font_size(TEXT_SIZE_MED)
        .top_left_with_margin_on(ids.canvas, 0.0)
        .down_from(ids.title, WIDGET_DISTANCE)
        .set(ids.patches_title, ui);
  

    ids.patch_idx.resize(16, &mut ui.widget_id_generator());
    for i in 0..16 {          
        for _press in widget::Button::new()
            .color(if i == player.inst_patch_id[player.inst_id as usize] {conrod_core::color::LIGHT_BLUE} else {conrod_core::color::DARK_GREY})
            .mid_left_with_margin_on(ids.canvas, WIDGET_SIZE*4.0 + WIDGET_SIZE * i as f64 + (2*i + 4*(i/4) + 6*(i/16)) as f64)
            .down_from(ids.title, 18.0)
            .w_h(WIDGET_SIZE*1.0, WIDGET_SIZE*1.0)
            .set(ids.patch_idx[i as usize], ui)
        {
            player.inst_patch_id[player.inst_id as usize] = i;
            app.preview_update = 10;
        }   
    }

    let min_x = 0.0;
    let max_x = 1.0;
    let min_y = -0.5;
    let max_y = 0.5;

    let quarter_lines = widget::grid::Lines::step(0.5).thickness(2.0);
    let sixteenth_lines = widget::grid::Lines::step(0.25).thickness(0.5);
    let lines = &[
        quarter_lines.x(),
        quarter_lines.y(),
        sixteenth_lines.x(),
        sixteenth_lines.y(),
    ];

    let get_y = |x:f64| -> f64 {
       player.get_preview(x, 1.0)
    };

    widget::Grid::new(min_x, max_x, min_y, max_y, lines.iter().cloned())
        .color(conrod_core::color::rgb(0.1, 0.12, 0.15))
        .right(WIDGET_DISTANCE * 10.0)
        .w_h(WIDGET_SIZE*48.0, WIDGET_SIZE*7.0)
        .set(ids.grid, ui);
    widget::PlotPath::new(min_x, max_x, min_y, max_y, get_y)
        .color(conrod_core::color::LIGHT_BLUE)
        .thickness(1.0)
        .wh_of(ids.grid)
        .middle_of(ids.grid)
        .set(ids.plot, ui)
        ;


    //let instrument_ = &mut player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize];

    for dialed in widget::NumberDialer::new(player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].attack as f64, 0.0, 15.0, 0)
        .top_left_with_margin_on(ids.canvas, 0.0)
        .down_from(ids.patch_idx[0], WIDGET_SIZE)
        //.right(WIDGET_DISTANCE*2.0)
        .w_h(WIDGET_SIZE*2.0, WIDGET_SIZE)
        .label(&format!("A"))
        .set(ids.attack, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].attack = dialed as u8;
        app.preview_update = 10;
    }

    for dialed in widget::NumberDialer::new(player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].decay as f64, 0.0, 15.0, 0)
        .right(WIDGET_DISTANCE)
        .w_h(WIDGET_SIZE*2.0, WIDGET_SIZE)
        .label(&format!("D"))
        .set(ids.decay, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].decay = dialed as u8;
        app.preview_update = 10;
    }

    for dialed in widget::NumberDialer::new(player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].sustain as f64, 0.0, 15.0, 0)
        .right(WIDGET_DISTANCE)
        .w_h(WIDGET_SIZE*2.0, WIDGET_SIZE)
        .label(&format!("S"))
        .set(ids.sustain, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].sustain = dialed as u8;
        app.preview_update = 10;
    }

    for dialed in widget::NumberDialer::new(player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].release as f64, 0.0, 15.0, 0)
        .right(WIDGET_DISTANCE)
        .w_h(WIDGET_SIZE*2.0, WIDGET_SIZE)
        .label(&format!("R"))
        .set(ids.release, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].release = dialed as u8;
        app.preview_update = 10;
    }

    for dialed in widget::NumberDialer::new(player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].hold as f64, 0.0, 16000.0, 0)
        .right(WIDGET_DISTANCE)
        .w_h(WIDGET_SIZE*4.0, WIDGET_SIZE)
        .label(&format!("H"))
        .set(ids.hold, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].hold = dialed as u32;
        app.preview_update = 10;
    }


    for _press in widget::Button::new()
        .right(WIDGET_DISTANCE*2.0)
        .color(conrod_core::color::BLUE)
        .w_h(WIDGET_SIZE*2.5, WIDGET_SIZE)
        .label(&format!("COPY"))
        .set(ids.copy_patch, ui)
    {
        app.copy_patch = player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize];
    }


    for _press in widget::Button::new()
        .right(WIDGET_DISTANCE*1.0)
        .color(conrod_core::color::RED)
        .w_h(WIDGET_SIZE*2.5, WIDGET_SIZE)
        .label(&format!("PASTE"))
        .set(ids.paste_patch, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize] = 
            app.copy_patch; 
     
        app.preview_update = 10;
    }


   
    for selected in widget::DropDownList::new(&list_count4, Some(player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].osc_map[0] as usize))
        .top_left_with_margin_on(ids.canvas, 0.0)
        //.down_from(ids.attack_hold, WIDGET_SIZE*0.5)
        .down_from(ids.attack, WIDGET_SIZE*0.5)
        .w_h(WIDGET_SIZE*2.0, WIDGET_SIZE)
        .set(ids.attack_osc, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].osc_map[0] = selected as u8;
        app.preview_update = 10;
    }

    for selected in widget::DropDownList::new(&list_count4, Some(player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].osc_map[1] as usize))
        .right(WIDGET_DISTANCE)
        .w_h(WIDGET_SIZE*2.0, WIDGET_SIZE)
        .set(ids.decay_osc, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].osc_map[1] = selected as u8;
        app.preview_update = 10;
    }

    for selected in widget::DropDownList::new(&list_count4, Some(player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].osc_map[2] as usize))
        .right(WIDGET_DISTANCE)
        .w_h(WIDGET_SIZE*2.0, WIDGET_SIZE)
        .set(ids.sustain_osc, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].osc_map[2] = selected as u8;
        app.preview_update = 10;
    }

    for selected in widget::DropDownList::new(&list_count4, Some(player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].osc_map[3] as usize))
        .right(WIDGET_DISTANCE)
        .w_h(WIDGET_SIZE*2.0, WIDGET_SIZE)
        .set(ids.release_osc, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].osc_map[3] = selected as u8;
        app.preview_update = 10;
    }    


    ids.osc_select.resize(4, &mut ui.widget_id_generator());  
    let mut i = 0;
    for &id in ids.osc_select.iter() {     
        if i == 0 {   
            for selected in widget::Toggle::new(app.osc_select[player.inst_id as usize] == i)
                .color(if app.osc_select[player.inst_id as usize] == i {conrod_core::color::LIGHT_BLUE} else {conrod_core::color::DARK_GREY})
                .top_left_with_margin_on(ids.canvas, 0.0)
                .down_from(ids.attack_osc, WIDGET_SIZE*2.0)
                .w_h(WIDGET_SIZE, WIDGET_SIZE)
                .set(id, ui)
            {
                if selected {
                     app.osc_select[player.inst_id as usize] = i;
                }
            }   
        }
        else {
            for selected in widget::Toggle::new(app.osc_select[player.inst_id as usize] == i)
                .color(if app.osc_select[player.inst_id as usize] == i {conrod_core::color::LIGHT_BLUE} else {conrod_core::color::DARK_GREY})
                .right(2.0)
                .w_h(WIDGET_SIZE, WIDGET_SIZE)
                .set(id, ui)
            {
                if selected {
                     app.osc_select[player.inst_id as usize] = i;
                }
            } 
        }
        i += 1;
    }


    for _press in widget::Button::new()
        .right(WIDGET_DISTANCE*2.0)
        .color(conrod_core::color::BLUE)
        .w_h(WIDGET_SIZE*2.5, WIDGET_SIZE)
        .label(&format!("COPY"))
        .set(ids.copy_osc_patch, ui)
    {
        app.copy_osc_patch = player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize];
        app.copy_osc_patch_id = app.osc_select[player.inst_id as usize];
    }


    for _press in widget::Button::new()
        .right(WIDGET_DISTANCE*1.0)
        .color(conrod_core::color::RED)
        .w_h(WIDGET_SIZE*2.5, WIDGET_SIZE)
        .label(&format!("PASTE"))
        .set(ids.paste_osc_patch, ui)
    {
        for i in 0..4 {
            player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(i + 4*app.osc_select[player.inst_id as usize]) as usize] = 
                app.copy_osc_patch.oscillators[(i + 4*app.copy_osc_patch_id) as usize]; 
        };
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].base_a[(app.osc_select[player.inst_id as usize]) as usize] = 
            app.copy_osc_patch.base_a[app.copy_osc_patch_id as usize];
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].base_b[(app.osc_select[player.inst_id as usize]) as usize] = 
            app.copy_osc_patch.base_b[app.copy_osc_patch_id as usize];
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].mode_a[(app.osc_select[player.inst_id as usize]) as usize] = 
            app.copy_osc_patch.mode_a[app.copy_osc_patch_id as usize];
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].mode_b[(app.osc_select[player.inst_id as usize]) as usize] = 
            app.copy_osc_patch.mode_b[app.copy_osc_patch_id as usize];

        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].vibrato[(app.osc_select[player.inst_id as usize]) as usize] = 
            app.copy_osc_patch.vibrato[app.copy_osc_patch_id as usize];

        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].wave_a[(app.osc_select[player.inst_id as usize]) as usize] = 
            app.copy_osc_patch.wave_a[app.copy_osc_patch_id as usize];
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].wave_b[(app.osc_select[player.inst_id as usize]) as usize] = 
            app.copy_osc_patch.wave_b[app.copy_osc_patch_id as usize];

        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].pulsewidth_a[(app.osc_select[player.inst_id as usize]) as usize] = 
            app.copy_osc_patch.pulsewidth_a[app.copy_osc_patch_id as usize];
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].pulsewidth_b[(app.osc_select[player.inst_id as usize]) as usize] = 
            app.copy_osc_patch.pulsewidth_b[app.copy_osc_patch_id as usize];

        app.preview_update = 10;
    }




    //-------------------------------------------------------------
    // wave
    for selected in widget::DropDownList::new(&osc_types, Some((player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(2 + 4*app.osc_select[player.inst_id as usize]) as usize].waveform as usize) as usize))
        .top_left_with_margin_on(ids.canvas, 0.0)
        .down_from(ids.osc_select[0], MARGIN*0.5)
        .w_h(WIDGET_SIZE*2.0, WIDGET_SIZE)
        .set(ids.osc3_wav, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(2 + 4*app.osc_select[player.inst_id as usize]) as usize].waveform =
            match selected {
                0 => Waveform::Triangle,
                1 => Waveform::Saw,
                2 => Waveform::Rect,
                3 => Waveform::Noise,
                4 => Waveform::InverseSaw,
                5 => Waveform::Sinus,
                _ => player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(2 + 4*app.osc_select[player.inst_id as usize]) as usize].waveform
            };
        app.preview_update = 10;
    }

    for dialed in widget::NumberDialer::new(player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(2 + 4*app.osc_select[player.inst_id as usize]) as usize].frequency, 0.0, player.ticks_per_frame as f64 * 50.0, 3)
        .right(WIDGET_DISTANCE)
        .w_h(WIDGET_SIZE*5.0, WIDGET_SIZE)
        .label("f")
        .set(ids.osc3_frq, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(2 + 4*app.osc_select[player.inst_id as usize]) as usize].frequency = dialed;
        app.preview_update = 10;
    }

    for dialed in widget::NumberDialer::new(player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(2 + 4*app.osc_select[player.inst_id as usize]) as usize].frequency_by_ticks(player.ticks_per_frame) + 0.5, 1.0, player.ticks_per_frame as f64 * 50.0, 0)
        .right(WIDGET_DISTANCE)
        .w_h(WIDGET_SIZE*3.0, WIDGET_SIZE)
        .label("t")
        .set(ids.osc3_frq_ticks, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(2 + 4*app.osc_select[player.inst_id as usize]) as usize].set_frequency_from_ticks(dialed as u32, player.ticks_per_frame);
        app.preview_update = 10;
    }

    for dialed in widget::Slider::new(1.0 - player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(2 + 4*app.osc_select[player.inst_id as usize]) as usize].ratio, 0.0, 1.0)
        .right(WIDGET_DISTANCE)
        .w_h(WIDGET_SIZE*5.0, WIDGET_SIZE)
        .label(&format!("r: {:.2}", player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(2 + 4*app.osc_select[player.inst_id as usize]) as usize].ratio))
        .set(ids.osc3_ratio, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(2 + 4*app.osc_select[player.inst_id as usize]) as usize].ratio = 1.0 - dialed;
        app.preview_update = 10;
    }


    ids.wave_a.resize(4, &mut ui.widget_id_generator());  
    let mut i = 0;
    for &id in ids.wave_a.iter() {        
        for selected in widget::Toggle::new((player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].wave_a[app.osc_select[player.inst_id as usize] as usize] & 1<<(3-i)) != 0)
            .right(if i==0 {WIDGET_DISTANCE*2.0} else {2.0})
            .w_h(WIDGET_SIZE, WIDGET_SIZE)
            .set(id, ui)
        {
            if selected {
                 player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].wave_a[app.osc_select[player.inst_id as usize] as usize] |= 1<<(3-i);
            }
            else {
                player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].wave_a[app.osc_select[player.inst_id as usize] as usize] &= 1<<(3-i) ^ 0x0f;
            }
            app.preview_update = 10;
        }   
        i += 1;
    }
   
    ids.wave_b.resize(4, &mut ui.widget_id_generator());
    let mut i = 0;
    for &id in ids.wave_b.iter() {        
        for selected in widget::Toggle::new((player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].wave_b[app.osc_select[player.inst_id as usize] as usize] & 1<<(3-i)) != 0)
            .right(if i==0 {WIDGET_DISTANCE*1.8} else {2.0})
            .w_h(WIDGET_SIZE, WIDGET_SIZE)
            .set(id, ui)
        {
            if selected {
                 player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].wave_b[app.osc_select[player.inst_id as usize] as usize] |= 1<<(3-i);
            }
            else {
                player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].wave_b[app.osc_select[player.inst_id as usize] as usize] &= 1<<(3-i) ^ 0x0f;
            }
            app.preview_update = 10;
        }   
        i += 1;
    }




    for selected in widget::DropDownList::new(&osc_types, Some((player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(4*app.osc_select[player.inst_id as usize]) as usize].waveform as usize) as usize))
        .top_left_with_margin_on(ids.canvas, 0.0)
        .down_from(ids.osc3_wav, WIDGET_SIZE*0.5)
        .w_h(WIDGET_SIZE*2.0, WIDGET_SIZE)
        .set(ids.osc1_wav, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(4*app.osc_select[player.inst_id as usize]) as usize].waveform =
            match selected {
                0 => Waveform::Triangle,
                1 => Waveform::Saw,
                2 => Waveform::Rect,
                3 => Waveform::Noise,
                4 => Waveform::InverseSaw,
                5 => Waveform::Sinus,
                _ => player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(4*app.osc_select[player.inst_id as usize]) as usize].waveform
            };
        app.preview_update = 10;
    }

    for dialed in widget::NumberDialer::new(player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(4*app.osc_select[player.inst_id as usize]) as usize].frequency, 0.0, player.ticks_per_frame as f64 * 50.0, 3)
        .right(WIDGET_DISTANCE)
        .w_h(WIDGET_SIZE*5.0, WIDGET_SIZE)
        .label("f")
        .set(ids.osc1_frq, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(4*app.osc_select[player.inst_id as usize]) as usize].frequency = dialed;
        app.preview_update = 10;
    }

    for dialed in widget::NumberDialer::new(player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(4*app.osc_select[player.inst_id as usize]) as usize].frequency_by_ticks(player.ticks_per_frame) + 0.5, 1.0, player.ticks_per_frame as f64 * 50.0, 0)
        .right(WIDGET_DISTANCE)
        .w_h(WIDGET_SIZE*3.0, WIDGET_SIZE)
        .label("t")
        .set(ids.osc1_frq_ticks, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(4*app.osc_select[player.inst_id as usize]) as usize].set_frequency_from_ticks(dialed as u32, player.ticks_per_frame);
        app.preview_update = 10;
    }

    for dialed in widget::Slider::new(1.0 - player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(4*app.osc_select[player.inst_id as usize]) as usize].ratio, 0.0, 1.0)
        .right(WIDGET_DISTANCE)
        .w_h(WIDGET_SIZE*5.0, WIDGET_SIZE)
        .label(&format!("r: {:.2}", player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(4*app.osc_select[player.inst_id as usize]) as usize].ratio))
        .set(ids.osc1_ratio, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(4*app.osc_select[player.inst_id as usize]) as usize].ratio = 1.0 - dialed;
        app.preview_update = 10;
    }    


    // note / freq
    for selected in widget::DropDownList::new(&base_mode_types, Some((player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].mode_a[app.osc_select[player.inst_id as usize] as usize]) as usize))
        .right(WIDGET_DISTANCE * 2.0)
        .w_h(WIDGET_SIZE*2.05, WIDGET_SIZE)
        .set(ids.base_note_type_a, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].mode_a[app.osc_select[player.inst_id as usize] as usize] = match selected {
            0 => BaseMode::Zero,
            1 => BaseMode::NoteIn,
            2 => BaseMode::Replace,
            3 => BaseMode::Add,
            4 => BaseMode::Sub,
            _ => player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].mode_a[app.osc_select[player.inst_id as usize] as usize]
        };
        app.preview_update = 10;
    };

    for selected in widget::DropDownList::new(&note_names_a, Some((player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].base_a[app.osc_select[player.inst_id as usize] as usize] % 12 ) as usize))
        .right(WIDGET_DISTANCE*0.5)
        .w_h(WIDGET_SIZE*1.5, WIDGET_SIZE)
        .set(ids.base_note_a, ui)
    {
        let base = std::cmp::max(0, player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].base_a[app.osc_select[player.inst_id as usize] as usize]);
        let octave = base / 12;
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].base_a[app.osc_select[player.inst_id as usize] as usize] = selected as i8 + 12 * octave;
        app.preview_update = 10;
    };
    
    for selected in widget::DropDownList::new(&octave_names_a, Some((player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].base_a[app.osc_select[player.inst_id as usize] as usize] / 12) as usize))
        .right(0.0)
        .w_h(WIDGET_SIZE*1.0, WIDGET_SIZE)
        .set(ids.base_note_a2, ui)
    {
        let base = std::cmp::max(0, player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].base_a[app.osc_select[player.inst_id as usize] as usize]);
        let note = base % 12;
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].base_a[app.osc_select[player.inst_id as usize] as usize] = (selected as i8) * 12 + note;
        app.preview_update = 10;
    };

    for selected in widget::DropDownList::new(&base_mode_types, Some((player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].mode_b[app.osc_select[player.inst_id as usize] as usize]) as usize))
        .right(WIDGET_DISTANCE * 1.0)
        .w_h(WIDGET_SIZE*2.05, WIDGET_SIZE)
        .set(ids.base_note_type_b, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].mode_b[app.osc_select[player.inst_id as usize] as usize] = match selected {
            0 => BaseMode::Zero,
            1 => BaseMode::NoteIn,
            2 => BaseMode::Replace,
            3 => BaseMode::Add,
            4 => BaseMode::Sub,
            _ => player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].mode_b[app.osc_select[player.inst_id as usize] as usize]
        };
        app.preview_update = 10;
    };

    for selected in widget::DropDownList::new(&note_names_a, Some((player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].base_b[app.osc_select[player.inst_id as usize] as usize] % 12 ) as usize))
        .right(WIDGET_DISTANCE*0.5)
        .w_h(WIDGET_SIZE*1.7, WIDGET_SIZE)
        .set(ids.base_note_b, ui)
    {
        let octave = player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].base_b[app.osc_select[player.inst_id as usize] as usize] / 12;
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].base_b[app.osc_select[player.inst_id as usize] as usize] = selected as i8 + 12 * octave;
        app.preview_update = 10;
    };
    
    for selected in widget::DropDownList::new(&octave_names_a, Some((player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].base_b[app.osc_select[player.inst_id as usize] as usize] / 12) as usize))
        .right(0.0)
        .w_h(WIDGET_SIZE*1.0, WIDGET_SIZE)
        .set(ids.base_note_b2, ui)
    {
        let note = player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].base_b[app.osc_select[player.inst_id as usize] as usize] % 12;
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].base_b[app.osc_select[player.inst_id as usize] as usize] = (selected as i8) * 12 + note;
        app.preview_update = 10;
    };




    // vibrato
    for selected in widget::DropDownList::new(&osc_types, Some((player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(1 + 4*app.osc_select[player.inst_id as usize]) as usize].waveform as usize) as usize))
        .top_left_with_margin_on(ids.canvas, 0.0)
        .down_from(ids.osc1_wav, WIDGET_SIZE*0.5)
        .w_h(WIDGET_SIZE*2.0, WIDGET_SIZE)
        .set(ids.osc2_wav, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(1 + 4*app.osc_select[player.inst_id as usize]) as usize].waveform =
            match selected {
                0 => Waveform::Triangle,
                1 => Waveform::Saw,
                2 => Waveform::Rect,
                3 => Waveform::Noise,
                4 => Waveform::InverseSaw,
                5 => Waveform::Sinus,
                _ => player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(1 + 4*app.osc_select[player.inst_id as usize]) as usize].waveform
            };
        app.preview_update = 10;
    }

    for dialed in widget::NumberDialer::new(player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(1 + 4*app.osc_select[player.inst_id as usize]) as usize].frequency, 0.0, player.ticks_per_frame as f64 * 50.0, 3)
        .right(WIDGET_DISTANCE)
        .w_h(WIDGET_SIZE*5.0, WIDGET_SIZE)
        .label("f")
        .set(ids.osc2_frq, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(1 + 4*app.osc_select[player.inst_id as usize]) as usize].frequency = dialed;
        app.preview_update = 10;
    }

    for dialed in widget::NumberDialer::new(player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(1 + 4*app.osc_select[player.inst_id as usize]) as usize].frequency_by_ticks(player.ticks_per_frame) + 0.5, 1.0, player.ticks_per_frame as f64 * 50.0, 0)
        .right(WIDGET_DISTANCE)
        .w_h(WIDGET_SIZE*3.0, WIDGET_SIZE)
        .label("t")
        .set(ids.osc2_frq_ticks, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(1 + 4*app.osc_select[player.inst_id as usize]) as usize].set_frequency_from_ticks(dialed as u32, player.ticks_per_frame);
        app.preview_update = 10;
    }

    for dialed in widget::Slider::new(1.0 - player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(1 + 4*app.osc_select[player.inst_id as usize]) as usize].ratio, 0.0, 1.0)
        .right(WIDGET_DISTANCE)
        .w_h(WIDGET_SIZE*5.0, WIDGET_SIZE)
        .label(&format!("r: {:.2}", player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(1 + 4*app.osc_select[player.inst_id as usize]) as usize].ratio))
        .set(ids.osc2_ratio, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(1 + 4*app.osc_select[player.inst_id as usize]) as usize].ratio = 1.0 - dialed;
        app.preview_update = 10;
    }


    for dialed in widget::Slider::new(player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].vibrato[app.osc_select[player.inst_id as usize] as usize], 0.0, 1.0)
        .right(WIDGET_DISTANCE * 2.0)
        .w_h(WIDGET_SIZE*5.0, WIDGET_SIZE)
        .label(&format!("v: {:.2}", player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].vibrato[app.osc_select[player.inst_id as usize] as usize]))
        .set(ids.vibrato, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].vibrato[app.osc_select[player.inst_id as usize] as usize] = dialed;
        app.preview_update = 10;
    }





    // pulsewidth
    for selected in widget::DropDownList::new(&osc_types, Some((player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(3 + 4*app.osc_select[player.inst_id as usize]) as usize].waveform as usize) as usize))
        .top_left_with_margin_on(ids.canvas, 0.0)
        .down_from(ids.osc2_wav, WIDGET_SIZE*0.5)
        .w_h(WIDGET_SIZE*2.0, WIDGET_SIZE)
        .set(ids.osc4_wav, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(3 + 4*app.osc_select[player.inst_id as usize]) as usize].waveform =
            match selected {
                0 => Waveform::Triangle,
                1 => Waveform::Saw,
                2 => Waveform::Rect,
                3 => Waveform::Noise,
                4 => Waveform::InverseSaw,
                5 => Waveform::Sinus,
                _ => player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(3 + 4*app.osc_select[player.inst_id as usize]) as usize].waveform
            };
        app.preview_update = 10;
    }

    for dialed in widget::NumberDialer::new(player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(3 + 4*app.osc_select[player.inst_id as usize]) as usize].frequency, 0.0, player.ticks_per_frame as f64 * 50.0, 3)
        .right(WIDGET_DISTANCE)
        .w_h(WIDGET_SIZE*5.0, WIDGET_SIZE)
        .label("f")
        .set(ids.osc4_frq, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(3 + 4*app.osc_select[player.inst_id as usize]) as usize].frequency = dialed;
        app.preview_update = 10;
    }

    for dialed in widget::NumberDialer::new(player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(3 + 4*app.osc_select[player.inst_id as usize]) as usize].frequency_by_ticks(player.ticks_per_frame) + 0.5, 1.0, player.ticks_per_frame as f64 * 50.0, 0)
        .right(WIDGET_DISTANCE)
        .w_h(WIDGET_SIZE*3.0, WIDGET_SIZE)
        .label("t")
        .set(ids.osc4_frq_ticks, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(3 + 4*app.osc_select[player.inst_id as usize]) as usize].set_frequency_from_ticks(dialed as u32, player.ticks_per_frame);
        app.preview_update = 10;
    }

    for dialed in widget::Slider::new(1.0 - player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(3 + 4*app.osc_select[player.inst_id as usize]) as usize].ratio, 0.0, 1.0)
        .right(WIDGET_DISTANCE)
        .w_h(WIDGET_SIZE*5.0, WIDGET_SIZE)
        .label(&format!("r: {:.2}", player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(3 + 4*app.osc_select[player.inst_id as usize]) as usize].ratio))
        .set(ids.osc4_ratio, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].oscillators[(3 + 4*app.osc_select[player.inst_id as usize]) as usize].ratio = 1.0 - dialed;
        app.preview_update = 10;
    }

    for dialed in widget::Slider::new(player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].pulsewidth_a[app.osc_select[player.inst_id as usize] as usize], 0.0, 1.0)
        .right(WIDGET_DISTANCE * 2.0)
        .w_h(WIDGET_SIZE*5.0, WIDGET_SIZE)
        .label(&format!("pw1: {:.2}", player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].pulsewidth_a[app.osc_select[player.inst_id as usize] as usize]))
        .set(ids.pulsewidth_a, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].pulsewidth_a[app.osc_select[player.inst_id as usize] as usize] = dialed;
        app.preview_update = 10;
    }

    for dialed in widget::Slider::new(player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].pulsewidth_b[app.osc_select[player.inst_id as usize] as usize], 0.0, 1.0)
        .right(WIDGET_DISTANCE * 1.0)
        .w_h(WIDGET_SIZE*5.0, WIDGET_SIZE)
        .label(&format!("pw2: {:.2}", player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].pulsewidth_b[app.osc_select[player.inst_id as usize] as usize]))
        .set(ids.pulsewidth_b, ui)
    {
        player.instruments[player.inst_id as usize][player.inst_patch_id[player.inst_id as usize] as usize].pulsewidth_b[app.osc_select[player.inst_id as usize] as usize] = dialed;
        app.preview_update = 10;
    }


    ids.filter_label.resize(4, &mut ui.widget_id_generator()); 
    widget::Text::new("Filter Chn.:")
        .font_size(TEXT_SIZE_SMALL)
        .right_from(ids.osc3_wav, WIN_W as f64 * 0.5 - (WIDGET_SIZE*1.5))
        .parent(ids.canvas)
        .set(ids.filter_label[0], ui);

    widget::Text::new("Filter Type:")
        .font_size(TEXT_SIZE_SMALL)
        .down_from(ids.filter_label[0], WIDGET_SIZE*0.8)
        .parent(ids.canvas)
        .set(ids.filter_label[1], ui);

    widget::Text::new("Freq:")
        .font_size(TEXT_SIZE_SMALL)
        .down_from(ids.filter_label[1], WIDGET_SIZE*0.8)
        .parent(ids.canvas)
        .set(ids.filter_label[2], ui);

    widget::Text::new("Reso:")
        .font_size(TEXT_SIZE_SMALL)
        .down_from(ids.filter_label[2], WIDGET_SIZE*0.8)
        .parent(ids.canvas)
        .set(ids.filter_label[3], ui);        


    ids.filter_voice.resize(3, &mut ui.widget_id_generator());  
    let mut i = 0;
    for &id in ids.filter_voice.iter() {        
        for selected in widget::Toggle::new(
            player.filter_matrix[player.filter_patch_idx as usize].filter_mask & (1<<(i)) != 0)
            .right_from(ids.osc3_wav, WIN_W as f64 * 0.5 + (WIDGET_SIZE*3.0) + (WIDGET_SIZE+2.0) * i as f64)
            .w_h(WIDGET_SIZE, WIDGET_SIZE)
            .set(id, ui)
        {
            if selected {
                player.filter_matrix[player.filter_patch_idx as usize].filter_mask |= 1<<(i);
            }
            else {
                player.filter_matrix[player.filter_patch_idx as usize].filter_mask &= 0x7 ^ (1<<(i));
            }
            //app.preview_update = 10;
        }   
        i += 1;
    }

    ids.filter_type.resize(3, &mut ui.widget_id_generator());  
    let mut i = 0;
    for &id in ids.filter_type.iter() {        
        for selected in widget::Toggle::new(
            player.filter_matrix[player.filter_patch_idx as usize].filter_type & (1<<(2-i)) != 0)
            .right_from(ids.osc1_wav, WIN_W as f64 * 0.5 + (WIDGET_SIZE*3.0) + (WIDGET_SIZE+2.0) * i as f64)
            .w_h(WIDGET_SIZE, WIDGET_SIZE)
            .set(id, ui)
        {
            if selected {
                player.filter_matrix[player.filter_patch_idx as usize].filter_type |= 1<<(2-i);
            }
            else {
                player.filter_matrix[player.filter_patch_idx as usize].filter_type &= 0x7 ^ (1<<(2-i));
            }
            //app.preview_update = 10;
        }   
        i += 1;
    }

    let mut val = player.filter_matrix[player.filter_patch_idx as usize].filter_freq as f64 / 0x7ff as f64;
    for dialed in widget::Slider::new(val, 0.0, 1.0)
        //.right_from(ids.osc3_wav, WIN_W as f64 * 0.5)
        .down_from(ids.filter_type[0], WIDGET_SIZE*0.5)
        .w_h(WIDGET_SIZE*5.0, WIDGET_SIZE)
        .label(&format!("f: {}", player.filter_matrix[player.filter_patch_idx as usize].filter_freq))
        .set(ids.filter_freq, ui)
    {

        player.filter_matrix[player.filter_patch_idx as usize].filter_freq = (dialed * 0x7ff as f64) as u32;
        //app.preview_update = 10;
    }   

    for dialed in widget::NumberDialer::new(player.filter_matrix[player.filter_patch_idx as usize].filter_freq_add as f64, 0.0, 0xffff as f64, 0)
        .right(WIDGET_DISTANCE)
        .w_h(WIDGET_SIZE*4.0, WIDGET_SIZE)
        //.label(&format!("S"))
        .set(ids.filter_add, ui)
    {
        player.filter_matrix[player.filter_patch_idx as usize].filter_freq_add = dialed as u32;
        //app.preview_update = 10;
    }

    for dialed in widget::Slider::new(player.filter_matrix[player.filter_patch_idx as usize].filter_res as f64, 0.0, 0xf as f64)
        .down_from(ids.filter_freq, WIDGET_SIZE*0.5)
        .w_h(WIDGET_SIZE*5.0, WIDGET_SIZE)
        .label(&format!("r: {}", player.filter_matrix[player.filter_patch_idx as usize].filter_res))
        .set(ids.filter_res, ui)
    {

        player.filter_matrix[player.filter_patch_idx as usize].filter_res = dialed as u8;
        //app.preview_update = 10;
    }   





    ids.filter_patch_idx.resize(64, &mut ui.widget_id_generator());
    for i in 0..16 {          
        for _press in widget::Button::new()
            .right_from(ids.filter_voice[0], (WIDGET_SIZE*13.0) + 6.0 + WIDGET_SIZE * i as f64 + (2*i + 4*(i/4) + 6*(i/16)) as f64)
            .color(if i == player.filter_patch_idx {conrod_core::color::LIGHT_BLUE} else {conrod_core::color::DARK_GREY})
            .w_h(WIDGET_SIZE*1.0, WIDGET_SIZE*1.0)
            .set(ids.filter_patch_idx[i as usize], ui)
        {
            player.filter_patch_idx = i;
        }   
    }
    for j in 1..4 {
        for i in 0..16 { 
            let idx = j*16 + i;         
            for _press in widget::Button::new()
                .down_from(ids.filter_patch_idx[idx-16], WIDGET_SIZE*0.5)
                .color(if idx as u16 == player.filter_patch_idx {conrod_core::color::LIGHT_BLUE} else {conrod_core::color::DARK_GREY})
                .w_h(WIDGET_SIZE*1.0, WIDGET_SIZE*1.0)
                .set(ids.filter_patch_idx[idx as usize], ui)
            {
                player.filter_patch_idx = idx as u16;
            }   
        }
    }    






    let mut time = player.ticks_per_16th*64.0 / player.ticks_per_second;
    if player.song_mode {
        time *= (1 + player.end_pattern - player.start_pattern) as f64;
    }
    time /= player.tick_scale as f64;
    widget::Text::new("")
        .font_size(SUBTITLE_SIZE)
        .top_left_with_margin_on(ids.canvas, 0.0)
        .down_from(ids.osc4_wav, WIDGET_SIZE*1.25)
        .parent(ids.canvas)
        .set(ids.seq_title, ui);

    for dialed in widget::NumberDialer::new(player.ticks_per_frame as f64, 4.0, 32.0, 0)
        .right_from(ids.seq_title, 0.0)
        .parent(ids.canvas)
        .w_h(WIDGET_SIZE*2.0, WIDGET_SIZE)
        .label("x")
        .set(ids.player_speed, ui)
    {
        player.set_ticks_per_frame(dialed as u8);
    }  

    for dialed in widget::NumberDialer::new(player.bpm, 62.5, 200.0, 1)
        .right(WIDGET_DISTANCE)
        .parent(ids.canvas)
        .w_h(WIDGET_SIZE*5.0, WIDGET_SIZE)
        .label("BPM")
        .set(ids.bpm, ui)
    {
        player.set_speed(dialed);
    }    

    for dialed in widget::NumberDialer::new(player.ticks_per_16th as f64, 1.0, 16.0*50.0, 2)
        .right(WIDGET_DISTANCE)
        .parent(ids.canvas)
        .w_h(WIDGET_SIZE*5.0, WIDGET_SIZE)
        .label("1/16")
        .set(ids.ticks, ui)
    {
        player.set_speed_from_ticks(dialed);
    }    

    for selected in widget::DropDownList::new(&tick_scales, Some(match player.tick_scale {2=>1,4=>2,_=>0}))
        .right(WIDGET_DISTANCE)
        .parent(ids.canvas)
        .w_h(WIDGET_SIZE*5.0, WIDGET_SIZE)
        .set(ids.tick_scale, ui)
    {
        player.tick_scale = match selected {
            1 => 2,
            2 => 4,
            _ => 1,
        };
        app.preview_update = 10;
    }


    widget::Text::new(&format!("{}:{}.{}",time as u32/60, time as u32%60, (time-(time as u32)as f64) as u32))
        .right(WIDGET_DISTANCE)
        .font_size(TEXT_SIZE_SMALL)
        .w_h(WIDGET_SIZE*5.0, WIDGET_SIZE)
        .parent(ids.canvas)
        .set(ids.seq_time, ui);


    for _press in widget::Button::new()
        .color(if player.song_mode {conrod_core::color::LIGHT_RED} else {conrod_core::color::BLUE})
        .label(if player.song_mode{"Song"} else {"Loop"})
        .right(WIDGET_DISTANCE*4.0)
        .w_h(WIDGET_SIZE*4.4, WIDGET_SIZE)
        .set(ids.song_mode_select, ui)
    {
        player.song_mode = !player.song_mode;
    }


    if !player.song_mode {
        for _press in widget::Button::new()
            .right(WIDGET_DISTANCE*2.0)
            .color(conrod_core::color::BLUE)
            .w_h(WIDGET_SIZE*2.5, WIDGET_SIZE)
            .label(&format!("INS"))
            .set(ids.insert_loop, ui)
        {
            for i in 0..63-player.pattern_idx as usize {
                for j in 0..3 {
                    player.track.patterns[63-i][j] = player.track.patterns[62-i][j];
                    player.track.patch_patterns[63-i][j] = player.track.patch_patterns[62-i][j];
                }
            }
            if player.pattern_idx < 63 {
                player.pattern_idx += 1;
            }
        }


        for _press in widget::Button::new()
            .right(WIDGET_DISTANCE*1.0)
            .color(conrod_core::color::RED)
            .w_h(WIDGET_SIZE*2.5, WIDGET_SIZE)
            .label(&format!("DEL"))
            .set(ids.delete_loop, ui)
        {
            for i in player.pattern_idx as usize..63 {
                for j in 0..3 {
                    player.track.patterns[i][j] = player.track.patterns[i+1][j];
                    player.track.patch_patterns[i][j] = player.track.patch_patterns[i+1][j];
                }
            }
        }


        for _press in widget::Button::new()
            .right(WIDGET_DISTANCE*2.0)
            .color(conrod_core::color::BLUE)
            .w_h(WIDGET_SIZE*2.5, WIDGET_SIZE)
            .label(&format!("COPY"))
            .set(ids.copy_loop, ui)
        {
            for i in 0..3 {
                app.copy_pattern[i] = player.track.patterns[player.pattern_idx as usize][i];
                app.copy_patch_pattern[i] = player.track.patch_patterns[player.pattern_idx as usize][i];
            }
        }


        for _press in widget::Button::new()
            .right(WIDGET_DISTANCE*1.0)
            .color(conrod_core::color::RED)
            .w_h(WIDGET_SIZE*2.5, WIDGET_SIZE)
            .label(&format!("PASTE"))
            .set(ids.paste_loop, ui)
        {
            for i in 0..3 {
                player.track.patterns[player.pattern_idx as usize][i] = app.copy_pattern[i];
                player.track.patch_patterns[player.pattern_idx as usize][i] = app.copy_patch_pattern[i];
            }
        }


        for _press in widget::Button::new()
            .right(WIDGET_DISTANCE*2.0)
            .color(conrod_core::color::BLUE)
            .w_h(WIDGET_SIZE*2.5, WIDGET_SIZE)
            .label(&format!("COPY4"))
            .set(ids.copy4_loop, ui)
        {
            for i in 0..3 {
                for j in 0..4 {
                    app.copy4_pattern[j][i] = player.track.patterns[((player.pattern_idx & 0xfffc) + j as u32) as usize][i];
                    app.copy4_patch_pattern[j][i] = player.track.patch_patterns[((player.pattern_idx & 0xfffc) + j as u32) as usize][i];
                }
            }
        }


        for _press in widget::Button::new()
            .right(WIDGET_DISTANCE*1.0)
            .color(conrod_core::color::RED)
            .w_h(WIDGET_SIZE*2.5, WIDGET_SIZE)
            .label(&format!("PASTE4"))
            .set(ids.paste4_loop, ui)
        {
            for i in 0..3 {
                for j in 0..4 {
                    player.track.patterns[((player.pattern_idx & 0xfffc) + j as u32) as usize][i] = app.copy4_pattern[j][i];
                    player.track.patch_patterns[((player.pattern_idx & 0xfffc) + j as u32) as usize][i] = app.copy4_patch_pattern[j][i];
                }
            }            
        }

    }
    else {
        for dialed in widget::NumberDialer::new(player.start_pattern as f64, 0.0, 63.0, 0)
            .right(WIDGET_DISTANCE*2.0)
            .parent(ids.canvas)
            .w_h(WIDGET_SIZE*1.8, WIDGET_SIZE)
            .set(ids.start_pattern_select, ui)
        {
            if dialed as u32 <= 63 {
                player.start_pattern = dialed as u32;
                if player.end_pattern < player.start_pattern {
                    player.end_pattern = player.start_pattern;
                }
            }
        }

        for dialed in widget::NumberDialer::new(player.end_pattern as f64, 0.0, 63.0, 0)
            .right(WIDGET_DISTANCE)
            .parent(ids.canvas)
            .w_h(WIDGET_SIZE*1.8, WIDGET_SIZE)
            .set(ids.end_pattern_select, ui)
        {
            if dialed as u32 <= 63 && dialed as u32 >= player.start_pattern {     
                player.end_pattern = dialed as u32;
            }
        }
    }

    widget::Text::new("Pattern:")
        .font_size(TEXT_SIZE_MED)
        .top_left_with_margin_on(ids.canvas, 0.0)
        .down_from(ids.seq_title, 18.0)
        .set(ids.pattern_title, ui);

    if player.song_mode {
        let mut i = player.start_pattern;
        let left = WIDGET_SIZE*4.0 -2.0 + WIDGET_SIZE * i as f64 + (2*i + 4*(i/4) + 6*(i/16)) as f64;
        i = player.end_pattern;
        let right = WIDGET_SIZE*5.0 + 2.0 + WIDGET_SIZE * i as f64 + (2*i + 4*(i/4) + 6*(i/16)) as f64;

        widget::BorderedRectangle::new([right-left, WIDGET_SIZE+8.0])
                .color(conrod_core::color::YELLOW)
                .mid_left_with_margin_on(ids.canvas, left)
                .down_from(ids.seq_title, 14.0)
                .set(ids.song_length_display, ui);
    }


    ids.pattern_idx.resize(64, &mut ui.widget_id_generator());
    for i in 0..64 {          
        for _press in widget::Button::new()
            .color(if i == player.pattern_idx {conrod_core::color::LIGHT_BLUE} else {conrod_core::color::DARK_GREY})
            .mid_left_with_margin_on(ids.canvas, WIDGET_SIZE*4.0 + WIDGET_SIZE * i as f64 + (2*i + 4*(i/4) + 6*(i/16)) as f64)
            .down_from(ids.seq_title, 18.0)
            .w_h(WIDGET_SIZE*1.0, WIDGET_SIZE*1.0)
            .set(ids.pattern_idx[i as usize], ui)
        {
            player.pattern_idx = i;
        }   
    }


    for _press in widget::Button::new()
        .down_from(ids.pattern_idx[0], 14.0)
        .color(conrod_core::color::BLUE)
        .w_h(WIDGET_SIZE*2.5, WIDGET_SIZE)
        .label(&format!("COPY"))
        .set(ids.copy_pattern, ui)
    {
        for i in 0..64 {
            app.copy_seq[i] = player.get_trigger(player.inst_id as u32, i);
            app.copy_patch_seq[i] = player.get_patch(player.inst_id as u32, i);
        }
    }


    for _press in widget::Button::new()
        .right(WIDGET_DISTANCE*1.0)
        .color(conrod_core::color::RED)
        .w_h(WIDGET_SIZE*2.5, WIDGET_SIZE)
        .label(&format!("PASTE"))
        .set(ids.paste_pattern, ui)
    {
        for i in 0..64 {
            player.set_trigger(player.inst_id as u32, i, app.copy_seq[i]);
            player.set_patch(player.inst_id as u32, i, app.copy_patch_seq[i]);
        }
    }


    let inst_prev = player.inst_id;

    ids.trackborder.resize(3, &mut ui.widget_id_generator());
    ids.track_sequence.resize(6, &mut ui.widget_id_generator());    
    ids.track_inst.resize(3, &mut ui.widget_id_generator());
    ids.play_index.resize(6, &mut ui.widget_id_generator());
    ids.mute.resize(3, &mut ui.widget_id_generator());

    ids.seq1.resize(64*16, &mut ui.widget_id_generator());

    let y_offset = WIDGET_SIZE+WIDGET_DISTANCE*3.0;

    for s in 0..6 {
        let play_index = (player.play_index>>16)%64;

        if (s & 1) == 0 {
            for _press in widget::Button::new()
                .color(if s as u16 / 2 == player.inst_id {conrod_core::color::LIGHT_BLUE} else {conrod_core::color::DARK_CHARCOAL})
                .mid_left_with_margin_on(ids.canvas, -MARGIN)
                .down_from(ids.seq_title, y_offset + 18.0 - WIDGET_DISTANCE * 0.5 + (WIDGET_DISTANCE+WIDGET_SIZE) * s as f64)
                .w_h(WIN_W as f64 - MARGIN*0.0, (WIDGET_DISTANCE+WIDGET_SIZE)*2.0 - WIDGET_DISTANCE*0.25)
                .set(ids.trackborder[s/2], ui)
            {
                player.inst_id = s as u16 / 2;
                if player.inst_id != inst_prev {
                    app.preview_update = 10;
                }
            }
        }

  
         // index-marker
        widget::BorderedRectangle::new([WIDGET_SIZE+4.0, WIDGET_SIZE+4.0])
            .mid_left_with_margin_on(ids.canvas, WIDGET_SIZE*4.0 + -2.0 + WIDGET_SIZE * play_index as f64 + (2*play_index + 4*(play_index/4) + 6*(play_index/16)) as f64)
            .down_from(ids.seq_title, y_offset + 18.0 + (WIDGET_DISTANCE+WIDGET_SIZE) * (s&254) as f64 + (WIDGET_DISTANCE*0.5+WIDGET_SIZE) * (s&1) as f64)
            .set(ids.play_index[s], ui);

        if (s & 1) == 0 {
            for selected in widget::Toggle::new(!player.mute[s/2])
                .mid_left_with_margin_on(ids.canvas, WIDGET_SIZE * 0.0)
                .down_from(ids.seq_title, y_offset + 20.0 + (WIDGET_DISTANCE+WIDGET_SIZE) * s as f64)
                .w_h(WIDGET_SIZE, WIDGET_SIZE)
                .set(ids.mute[s/2], ui)
            {
                //player.inst_id = s as u16;
                player.mute[s/2] = !selected;
            } 

            for selected in widget::DropDownList::new(&track_sequence_ids, Some((player.track.patterns[player.pattern_idx as usize][s/2]) as usize))
                .mid_left_with_margin_on(ids.canvas, (WIDGET_DISTANCE+WIDGET_SIZE) * 1.0)
                .down_from(ids.seq_title, y_offset + 20.0 + (WIDGET_DISTANCE+WIDGET_SIZE) * (s&254) as f64 + (WIDGET_DISTANCE*0.5+WIDGET_SIZE) * (s&1) as f64)
                .w_h(WIDGET_SIZE*1.5, WIDGET_SIZE)
                .set(ids.track_sequence[s], ui)
            {
                player.inst_id = s as u16 /2;        
                if player.inst_id != inst_prev {
                    app.preview_update = 10;
                }    
                player.track.patterns[player.pattern_idx as usize][s/2] = (selected) as i16;
            };

        }
        else {
            /*
            for selected in widget::DropDownList::new(&track_sequence_ids, Some((player.track.patterns[player.pattern_idx as usize][s/2]) as usize))
                .mid_left_with_margin_on(ids.canvas, WIDGET_DISTANCE* 0.0)
                .down_from(ids.seq_title, y_offset + 20.0 + (WIDGET_DISTANCE+WIDGET_SIZE) * (s&254) as f64 + (WIDGET_DISTANCE*0.5+WIDGET_SIZE) * (s&1) as f64)
                .w_h(WIDGET_SIZE*1.5, WIDGET_SIZE)
                .set(ids.track_inst[s/2], ui)
            {
                player.inst_id = s as u16 / 2;   
                if player.inst_id != inst_prev {
                    app.preview_update = 10;
                }                         
                //player.track.patterns[player.pattern_idx as usize][s/2] = (selected) as i16;
            }; 
            */
            for selected in widget::DropDownList::new(&track_sequence_ids, Some((player.track.patch_patterns[player.pattern_idx as usize][s/2]) as usize))
                .mid_left_with_margin_on(ids.canvas, (WIDGET_DISTANCE+WIDGET_SIZE) * 1.0)
                .down_from(ids.seq_title, y_offset + 20.0 + (WIDGET_DISTANCE+WIDGET_SIZE) * (s&254) as f64 + (WIDGET_DISTANCE*0.5+WIDGET_SIZE) * (s&1) as f64)
                .w_h(WIDGET_SIZE*1.5, WIDGET_SIZE)
                .set(ids.track_sequence[s], ui)
            {
                player.inst_id = s as u16 / 2;   
                if player.inst_id != inst_prev {
                    app.preview_update = 10;
                }                         
                player.track.patch_patterns[player.pattern_idx as usize][s/2] = (selected) as i16;
            };           
        }


        
        for i in 0..64 {  
            if (s & 1) == 0 {
                let track_pattern = player.track.patterns[player.pattern_idx as usize][s/2] as usize;
                let note = player.track.sequences[s/2][track_pattern][i];        
                let label = if note as i8 >= 0 {
                    format!("{}{:?}", note_names[(note % 12) as usize], note / 12)
                    }
                    else
                    {
                        "".to_string()
                    };
                for selected in widget::Toggle::new(note >= 0)
                    .label(&label)
                    .label_font_size(10)
                    .mid_left_with_margin_on(ids.canvas, WIDGET_SIZE*4.0 + WIDGET_SIZE * i as f64 + (2*i + 4*(i/4) + 6*(i/16)) as f64)
                    .down_from(ids.seq_title, y_offset + 20.0 + (WIDGET_DISTANCE+WIDGET_SIZE) * s as f64)
                    .w_h(WIDGET_SIZE, WIDGET_SIZE)
                    .set(ids.seq1[s*64  + i], ui)
                {
                    player.inst_id = (s/2) as u16;
                    if player.inst_id != inst_prev {
                        app.preview_update = 10;
                    }                    
                    let prev = player.get_trigger((s/2) as u32, i);
                    player.set_trigger((s/2) as u32, i, if selected {player.instrument_notes[player.inst_id as usize]} else {-1});    
                    if app.shift_key {
                        let mut scan = true;
                        let mut j:i32 = i as i32 -1;
                        while (j>=0) && scan {
                            let cur = player.get_trigger((s/2) as u32, j as usize);
                            if cur == prev {
                                player.set_trigger((s/2) as u32, j as usize, if selected {player.instrument_notes[player.inst_id as usize]} else {-1});
                            }
                            else {
                                scan = false;
                            }
                            j -= 1;
                        }
                    }
                } 
            }  
            else {
                let track_patch_pattern = player.track.patch_patterns[player.pattern_idx as usize][s/2] as usize;
                let patch = player.track.patches[s/2][track_patch_pattern][i];        
                let label = if patch as i8 >= 0 {
                    format!("{:?}", patch % 16)
                    }
                    else
                    {
                        "".to_string()
                    };
                for selected in widget::Toggle::new(patch >= 0)
                    .label(&label)
                    .label_font_size(10)
                    .mid_left_with_margin_on(ids.canvas, WIDGET_SIZE*4.0 + WIDGET_SIZE * i as f64 + (2*i + 4*(i/4) + 6*(i/16)) as f64)
                    .down_from(ids.seq_title, y_offset + 20.0 + (WIDGET_DISTANCE+WIDGET_SIZE) * (s&254) as f64 + (WIDGET_DISTANCE*0.5+WIDGET_SIZE) * (s&1) as f64)
                    .w_h(WIDGET_SIZE, WIDGET_SIZE)
                    .set(ids.seq1[s*64  + i], ui)
                {
                    player.inst_id = (s/2) as u16;
                    if player.inst_id != inst_prev {
                        app.preview_update = 10;
                    }                    
                    let prev = player.get_patch((s/2) as u32, i);
                    player.set_patch((s/2) as u32, i, if selected {player.inst_patch_id[player.inst_id as usize] as i8} else {-1});    
                    if app.shift_key {
                        //println!("shift1");
                        let mut scan = true;
                        let mut j:i32 = i as i32 -1;
                        while (j>=0) && scan {
                            //println!("shift2");
                            let cur = player.get_trigger((s/2) as u32, j as usize);
                            if cur == prev {
                                player.set_trigger((s/2) as u32, j as usize, if selected {player.inst_patch_id[player.inst_id as usize] as i8} else {-1});
                            }
                            else {
                                scan = false;
                            }
                            j -= 1;
                        }
                    }
                } 
            }
        }
    }

    let octave = player.instrument_notes[player.inst_id as usize] / 12;
    let note = player.instrument_notes[player.inst_id as usize] % 12;
    let ostr = octave_names_a[octave as usize].to_string();
    let nstr = note_names[note as usize].to_string();
    let label = format!("Note: {}{}", nstr, ostr);
    widget::Text::new(&label)
        .font_size(TEXT_SIZE_TINY)
        .top_left_with_margin_on(ids.canvas, 0.0)
        .down_from(ids.trackborder[2], WIDGET_DISTANCE)
        .set(ids.kbd_title, ui);

    for selected in widget::DropDownList::new(&kbd_modes, Some(app.kb_mode as usize))
        .top_left_with_margin_on(ids.canvas, 0.0)
        //.down_from(ids.attack_hold, WIDGET_SIZE*0.5)
        .down_from(ids.trackborder[2], WIDGET_DISTANCE*0.5)
        .right(WIDGET_SIZE*2.0)
        .w_h(WIDGET_SIZE*5.0, WIDGET_SIZE)
        .set(ids.kbd_mode, ui)
    {

        app.kb_mode = match selected {
            1 => KeyboardMode::QWERTY,
            2 => KeyboardMode::QWERTZ,
            _ => KeyboardMode::Off,
        };
        app.preview_update = 10;
    }

    let note_map = [
        0, 2, 4, 5, 7, 9, 11,
        1, 3, 6, 8, 10,
     ];

    let octave = 0;
    let key_d = ((WIN_W-40) / (8*7)) as usize;
    let key_w = (key_d-4) as f64;

    ids.kbd.resize(8*12, &mut ui.widget_id_generator());
    for o in 0..8 {
        for i in 0..7 {
            //let label = if i == 0 {format!("C{:?}", o)} else {"".to_string()};
            let note = ((octave+o as u8)*12 + note_map[i]) as i8;
            for _press in widget::Button::new()
                .color(if note == player.instrument_notes[player.inst_id as usize] {conrod_core::color::LIGHT_BLUE}
                    else if app.kb_mode as usize == 0 || (app.selected_octave[player.inst_id as usize] == o as i8) || ((app.selected_octave[player.inst_id as usize]+1 == o as i8) && ((note%12) < 4)) {conrod_core::color::WHITE}
                    else {conrod_core::color::GREY})
                //.label(&label)
                .mid_left_with_margin_on(ids.canvas, (key_d*i) as f64 + (7*key_d*o) as f64)
                .down_from(ids.kbd_title, 8.0)
                .w_h(key_w, 100.0)
                .set(ids.kbd[o*12+i], ui)

            {
                player.instrument_notes[player.inst_id as usize] = note;
                app.selected_octave[player.inst_id as usize] = o as i8;
                app.preview_update = 10;
            }        
        }

        for i in 7..9 {
            let note = ((octave+o as u8)*12 + note_map[i]) as i8;
            for _press in widget::Button::new()
                .color(if note == player.instrument_notes[player.inst_id as usize] {conrod_core::color::BLUE} 
                    else {conrod_core::color::BLACK})
                .mid_left_with_margin_on(ids.canvas, ((key_d>>1) + key_d*(i-7)) as f64 + (7*key_d*o) as f64)
                .down_from(ids.kbd_title, 8.0)
                .w_h(key_w, 60.0)
                .set(ids.kbd[o*12+i], ui)
            {
                player.instrument_notes[player.inst_id as usize] = note;
                app.selected_octave[player.inst_id as usize] = o as i8;
                app.preview_update = 10;
            }        
        }

        for i in 9..12 {
            let note = ((octave+o as u8)*12 + note_map[i]) as i8;
            for _press in widget::Button::new()
                .color(if note == player.instrument_notes[player.inst_id as usize] {conrod_core::color::BLUE} 
                    else {conrod_core::color::BLACK})
                .mid_left_with_margin_on(ids.canvas, (key_d+(key_d>>1) + key_d*(i-7)) as f64 + (7*key_d*o) as f64)
                .down_from(ids.kbd_title, 8.0)
                .w_h(key_w, 60.0)
                .set(ids.kbd[o*12+i], ui)
            {
                player.instrument_notes[player.inst_id as usize] = note;
                app.selected_octave[player.inst_id as usize] = o as i8;
                app.preview_update = 10;
            }        
        }    
    }

    widget::Scrollbar::y_axis(ids.canvas).auto_hide(true).set(ids.canvas_scrollbar, ui);

    let keyboard_keys = [
        glium::glutin::VirtualKeyCode::A,
        glium::glutin::VirtualKeyCode::W,
        glium::glutin::VirtualKeyCode::S,

        glium::glutin::VirtualKeyCode::E,
        glium::glutin::VirtualKeyCode::D,
        glium::glutin::VirtualKeyCode::F,

        glium::glutin::VirtualKeyCode::T,
        glium::glutin::VirtualKeyCode::G,
        glium::glutin::VirtualKeyCode::Y,

        glium::glutin::VirtualKeyCode::H,
        glium::glutin::VirtualKeyCode::U,
        glium::glutin::VirtualKeyCode::J,

        glium::glutin::VirtualKeyCode::K,
        glium::glutin::VirtualKeyCode::O,
        glium::glutin::VirtualKeyCode::L,

        glium::glutin::VirtualKeyCode::P,
    ];

    let _keyboard_keys_qwertz = [
        glium::glutin::VirtualKeyCode::A,
        glium::glutin::VirtualKeyCode::W,
        glium::glutin::VirtualKeyCode::S,

        glium::glutin::VirtualKeyCode::E,
        glium::glutin::VirtualKeyCode::D,
        glium::glutin::VirtualKeyCode::F,

        glium::glutin::VirtualKeyCode::T,
        glium::glutin::VirtualKeyCode::G,
        glium::glutin::VirtualKeyCode::Z,

        glium::glutin::VirtualKeyCode::H,
        glium::glutin::VirtualKeyCode::U,
        glium::glutin::VirtualKeyCode::J,

        glium::glutin::VirtualKeyCode::K,
        glium::glutin::VirtualKeyCode::O,
        glium::glutin::VirtualKeyCode::L,

        glium::glutin::VirtualKeyCode::P,            
    ];

    let keyboard_keys_nr = [
        glium::glutin::VirtualKeyCode::Key1,
        glium::glutin::VirtualKeyCode::Key2,
        glium::glutin::VirtualKeyCode::Key3,
        glium::glutin::VirtualKeyCode::Key4,
        glium::glutin::VirtualKeyCode::Key5,
        glium::glutin::VirtualKeyCode::Key6,
        glium::glutin::VirtualKeyCode::Key7,
        glium::glutin::VirtualKeyCode::Key8,

    ];        



    if app.input_delay > 0 {
        app.input_delay -= 1;

        if app.input_delay == 0 {
            match app.input_keycode{
                Some(glium::glutin::VirtualKeyCode::Return) => {
                    player.key_return();
                },
                Some(glium::glutin::VirtualKeyCode::Space) => {
                    player.key_space();
                },
                _ => {}
            }

            match app.kb_mode {
                KeyboardMode::QWERTY => {
                    for i in 0..keyboard_keys.len() {
                        if app.input_keycode == Some(keyboard_keys[i]) {
                            if (app.selected_octave[player.inst_id as usize]*12 + i as i8) < 96 {
                                player.instrument_notes[player.inst_id as usize] = app.selected_octave[player.inst_id as usize]*12 + i as i8;
                                app.preview_update = 10;
                            }
                        }
                    };
                    for i in 0..keyboard_keys_nr.len() {
                        if app.input_keycode == Some(keyboard_keys_nr[i]) {
                            //player.instrument_notes[player.inst_id as usize] = i as i8;
                            app.selected_octave[player.inst_id as usize] = i as i8;
                            if (player.instrument_notes[player.inst_id as usize] % 12 + app.selected_octave[player.inst_id as usize]*12) < 96 {
                                player.instrument_notes[player.inst_id as usize] = player.instrument_notes[player.inst_id as usize] % 12 + app.selected_octave[player.inst_id as usize]*12;
                                app.preview_update = 10;
                            }
                        }
                    };
                },
                KeyboardMode::QWERTZ => {
                    for i in 0..keyboard_keys.len() {
                        if app.input_keycode == Some(_keyboard_keys_qwertz[i]) {
                            if (app.selected_octave[player.inst_id as usize]*12 + i as i8) < 96 {
                                player.instrument_notes[player.inst_id as usize] = app.selected_octave[player.inst_id as usize]*12 + i as i8;
                                app.preview_update = 10;
                            }
                        }
                    };
                    for i in 0..keyboard_keys_nr.len() {
                        if app.input_keycode == Some(keyboard_keys_nr[i]) {
                            //player.instrument_notes[player.inst_id as usize] = i as i8;
                            app.selected_octave[player.inst_id as usize] = i as i8;
                            if (player.instrument_notes[player.inst_id as usize] % 12 + app.selected_octave[player.inst_id as usize]*12) < 96 {
                                player.instrument_notes[player.inst_id as usize] = player.instrument_notes[player.inst_id as usize] % 12 + app.selected_octave[player.inst_id as usize]*12;
                                app.preview_update = 10;
                            }
                        }
                    };
                },                                

                _ => {}
            }
        }    
    }
}
