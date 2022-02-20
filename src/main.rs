//Cellular Automaton
//This project is just for fun.
//The only pupose of this project is to improve my Rust programming skills.
//Checkout "Conway's Game Of Life"

extern crate sdl2;

use std::mem;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use rand::prelude::*;

//Length of a unit given in pixels
const  UNIT_SIDE_LENGTH: usize = 5;
//NUM_UNITS will be always the same as "width" and "height" of buffer
const NUM_UNITS: usize = 100; //WINDOW_WIDTH/UNIT_SIDE_LENGTH;


fn main() {

    //Timing
    let duration = std::time::Duration::from_millis(100);

    //false=cell is dead, true=cell lives
    let mut front_buffer = [false; NUM_UNITS*NUM_UNITS]; 
    let mut back_buffer  = [false; NUM_UNITS*NUM_UNITS];

    //Init SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Conway's Game Of Life", (NUM_UNITS*UNIT_SIDE_LENGTH) as u32, (NUM_UNITS*UNIT_SIDE_LENGTH) as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.set_draw_color(Color::WHITE);

    //Init state of life
    init_life_state(&mut front_buffer);

    let mut event_pump = sdl_context.event_pump().unwrap();

    'main_loop: loop {
        //TODO: Pack all updating logic here...
        for event in event_pump.poll_iter(){
            match event{
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {break 'main_loop},
                _ => {}
            }
        }


        //Clear back buffer before writing the new values to it
        //TODO: Make this better
        for i in 0..back_buffer.len() {
            back_buffer[i] = false;
        }

        //Update world state
        for i in 0..front_buffer.len(){
            let row = i / NUM_UNITS;
            let col = i - row*NUM_UNITS;

            //Get state of current cell
            let cell_living = front_buffer[row*NUM_UNITS+col];
            //Get number of living neighbours
            let num_living_neighs = get_num_living_neighs(&front_buffer, row, col);
            
            //Decide if the current cell lives or dies
            if cell_living {
                if num_living_neighs==2 || num_living_neighs==3{
                    back_buffer[i] = true;
                }
                else{
                    back_buffer[i] = false;
                }
            }
            else{
                if num_living_neighs==3 {
                    back_buffer[i] = true
                }
            }

            if back_buffer[i] { update_canvas(&mut canvas, row, col); }

        }

        //Swap back and front buffer
        mem::swap(&mut front_buffer, &mut back_buffer);

        //Draw canvas according to world state
        canvas.present();
    
        //TODO: Replace this with framerate independent code
        std::thread::sleep(duration);

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(Color::WHITE);
    }
}

//Counts the number of "living" cells in the given buffer, for the given cell
fn get_num_living_neighs(buffer: &[bool], row: usize, col: usize) -> u8 {
    let mut num_neighs = 0;

    //We have at most 8 possible neighbour cells

    //Right neigh
    if (col < (NUM_UNITS-1)) && buffer[row*NUM_UNITS+col+1] {
        num_neighs += 1;
    }
    //Left neigh
    if (col > 0) && buffer[row*NUM_UNITS+col-1] {
        num_neighs += 1;
    }
    //Top neigh
    if (row > 0) && buffer[(row-1)*NUM_UNITS+col] {
        num_neighs += 1;
    }
    //Bottom neigh
    if (row < (NUM_UNITS-1)) && buffer[(row+1)*NUM_UNITS+col] {
        num_neighs += 1;
    }

    //Top right
    if (row > 0) && (col < (NUM_UNITS-1)) && buffer[(row-1)*NUM_UNITS+col+1] {
        num_neighs += 1;
    }
    //Top left
    if (row > 0) && (col > 0) && buffer[(row-1)*NUM_UNITS+col-1] {
        num_neighs += 1;
    }
    //Bottom right
    if (row < (NUM_UNITS-1)) && (col < (NUM_UNITS-1)) && buffer[(row+1)*NUM_UNITS+col+1] {
        num_neighs += 1;
    }

    //Bottom left
    if (row < (NUM_UNITS-1)) && (col > 0) && buffer[(row+1)*NUM_UNITS+col-1] {
        num_neighs += 1;
    }

    return num_neighs;
}

//This is slow, but only done once
fn init_life_state(buffer: &mut [bool]){

    let mut rng = rand::thread_rng();

    for i in 0..buffer.len() {
        if rng.gen() {
            let random_idx = rng.gen_range(0..buffer.len());
             buffer[random_idx*rng.gen_range(0..10)%buffer.len()] = rng.gen(); 
             
        };
    }
}

//Update canvas on the given position
fn update_canvas(canvas: &mut Canvas<sdl2::video::Window>, row: usize, col: usize) {

    let rect = Rect::new( (col*UNIT_SIDE_LENGTH).try_into().unwrap(),
        (row*UNIT_SIDE_LENGTH).try_into().unwrap(),
        UNIT_SIDE_LENGTH as u32,
        UNIT_SIDE_LENGTH as u32);
    
    canvas.fill_rect(rect).unwrap();
}