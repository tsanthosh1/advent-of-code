use std::str::Lines;
use intcode::IntCode;
use std::collections::HashMap;
use std::io::{stdout, Write};
use itertools::Itertools;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

type Tile = (i64, i64);

pub fn main() {

    let contents = include_str!("../../data/thirteen.data");
//    draw_tiles(contents);
    play(contents)
}

fn play(contents: &str) {

    let mut intcode = IntCode::initialize(contents, None, true);
    intcode.program[0] = 2;
    let mut score = 0;
    let mut paddle_position = (0, 0);
    let mut ball_position = (0, 0);
    let mut tiles: HashMap<Tile, i64> = HashMap::new();
    let mut step: i32 = 0;
    while !intcode.is_terminated {
        step += 1;
        for i in 0..3 {
            if paddle_position.0 < ball_position.0 { intcode.set_input(1) }
            else if paddle_position.0 > ball_position.0 { intcode.set_input(-1) }
            else { intcode.set_input(0) }
            intcode.execute();
        }

        println!("{:?}", intcode.output_string());

        if !intcode.has_output() {
            break;
        }

        let output = intcode.take_output();

        if output[0] == -1 && output[1] == 0 {
            println!("score {}", score);
            score = output[2];
        } else {
            tiles.insert((output[0], output[1]), output[2]);
        }

        if output[2] == 3 {
            paddle_position = (output[0], output[1]);

        } else if output[2] == 4 {
            ball_position = (output[0], output[1]);
        }

        let mut img = ImageBuffer::from_fn(44 * 7, 25 * 7, |x, y| {
            let tile_type_optional = tiles.get(&((x/7) as i64,(y/7) as i64));
            if tile_type_optional.is_none() { return image::Rgb([0,0,0]) }
            let tile_type = tile_type_optional.unwrap();
            let red = [220,20,60];
            let yellow = [255,215,0];
            let aqua = [0,255,255];
            let green = [50,205,50];
            if *tile_type == 3 {
                image::Rgb(aqua)
            } else if *tile_type == 4 {
                image::Rgb(red)
            } else if *tile_type == 1 {
                image::Rgb(yellow)
            } else if *tile_type == 2 {
                image::Rgb(green)
            } else { image::Rgb([0,0,0]) }
        });

        img.save(format!("/tmp/out/part13_{}.png", step)).unwrap();


    }


    println!("Score {}", score);
    println!("Score {}", score);
    println!("\r");




    println!("\r");
    println!("Score 1");
    println!("Score 1");
}


fn draw_tiles(contents: &str) {
    let mut intcode = IntCode::initialize(contents, None, true);
    let mut tiles: HashMap<Tile, i64> = HashMap::new();
    loop {
        for i in 0..3 {
            intcode.execute();
        }

        if !intcode.has_output() {
            break;
        }
        let output = intcode.take_output();
        tiles.insert((output[0], output[1]), output[2]);
    }
    println!("{:?}", tiles.values()
        .filter(|&&tile_value| tile_value == 2)
        .map(|x| *x)
        .collect::<Vec<i64>>().len());
}

