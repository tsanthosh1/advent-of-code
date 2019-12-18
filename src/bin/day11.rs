use std::str::Lines;
use intcode::IntCode;
use std::collections::HashMap;
use plotlib::scatter::Scatter;
use plotlib::view::ContinuousView;
use plotlib::scatter;
use plotlib::style::Point;
use plotlib::page::Page;

type Panel = (i32, i32);
const UP: i32 = 0;
const RIGHT: i32 = 1;
const DOWN: i32 = 2;
const LEFT: i32 = 3;

const BLACK: i64 = 0;
const WHITE: i64 = 1;

pub fn main() {

    let contents = include_str!("../../data/eleven.data");

    let intcode = paint_panels(contents);

}

fn paint_panels(contents: &str) {
    let mut panels: HashMap<Panel, i64> = HashMap::new();
    let mut current_panel = (0, 0);
    let mut current_direction = UP;
    panels.insert(current_panel, WHITE);

    let mut intcode = IntCode::initialize(
        contents,
        None,
        true
    );

    loop {
        let current_color_wrapped = panels.get(&current_panel);
        let current_color =
            if current_color_wrapped.is_some() { current_color_wrapped.unwrap() } else { &BLACK };

        intcode.set_input(*current_color);
        intcode.execute();
        if !intcode.has_output() {
            println!("No output exiting: {:?}", panels);
            println!("count {}", panels.len());
            let data2 = panels.iter()
                .filter(|&x| *x.1 == 1)
                .map(|x| (*x.0))
                .map(|x| (x.0 as f64, x.1 as f64))
                .collect::<Vec<(f64, f64)>>();
            let scatter = Scatter::from_slice(&data2)
                .style(scatter::Style::new()
                    .colour("#35C788"));
            let v = ContinuousView::new()
                .add(&scatter)
                .x_range(-44., 44.)
                .y_range(-22., 22.);
            Page::single(&v).save("./src/out/part11.svg");
            println!("{}", Page::single(&v).to_text().unwrap());
            break
        }
        panels.insert(current_panel, intcode.take_output()[0]);

        intcode.execute();
        let direction_command = intcode.take_output()[0];

        current_direction = turn_direction(current_direction, direction_command);
        current_panel = move_panel(&current_panel, current_direction);
    }
}

fn move_panel(current_panel: &(i32, i32), current_direction: i32) -> (i32, i32) {
    match current_direction {
        UP => (current_panel.0, current_panel.1 + 1),
        RIGHT => (current_panel.0 + 1, current_panel.1),
        DOWN => (current_panel.0, current_panel.1 - 1),
        LEFT => (current_panel.0 - 1, current_panel.1),
        _ => panic!()
    }
}

fn turn_direction(current_direction: i32, next_direction: i64) -> i32 {
    let i = match next_direction {
        0 => (4 + current_direction - 1) % 4,
        1 => (4 + current_direction + 1) % 4,
        _ => unreachable!()
    };
    i
}

#[cfg(test)]
mod tests {

    #![test]

    use crate::paint_panels;

    fn paints_panels() {
    }
}