mod args;
mod helpers;
mod point;

use args::Args;
use std::env::consts::OS;
use std::process::exit;

use helpers::read_file;
use point::Point;

use clap::Parser;
use imgui::{Context, FontSource};
use raylib::core::texture::Image;
use raylib::prelude::*;
use raylib_imgui_rs::Renderer;

use crate::helpers::norm_to_reg;

fn main() {
    let args = Args::parse();
    let mut values: Vec<Point> = Vec::new();

    let file_contents = match read_file(&args.input) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("ERROR: Failed to read '{}':", args.input);
            eprintln!("{}", e);
            exit(1);
        }
    };

    let split_delim = if OS == "windows" { "\r\n" } else { "\n" };

    for line in file_contents.split(split_delim) {
        for raw in line.trim().split(",") {
            values.push(Point {
                label: raw.to_string(),
                count: 0,
            });
        }
    }

    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Spread Maker")
        .vsync()
        .build();

    let mut imgui = Context::create();
    imgui
        .fonts()
        .add_font(&[FontSource::DefaultFontData { config: None }]);

    let mut renderer = Renderer::create(&mut imgui, &mut rl, &thread);

    let mut myself_idx = 0;
    let mut output_path_buffer = String::from("output.png");
    let mut self_color: [f32; 3] = [
        Color::ORANGE.r as f32 / 255.0,
        Color::ORANGE.g as f32 / 255.0,
        Color::ORANGE.b as f32 / 255.0,
    ];
    let mut background_color: [f32; 3] = [
        Color::GRAY.r as f32 / 255.0,
        Color::GRAY.g as f32 / 255.0,
        Color::GRAY.b as f32 / 255.0,
    ];
    let mut team_color: [f32; 3] = [0.0, 0.0, 1.0];
    let mut line_color: [f32; 3] = [1.0, 1.0, 1.0];
    let mut font_color: [f32; 3] = [1.0, 1.0, 1.0];
    const SIZE: i32 = 20;

    while !rl.window_should_close() {
        renderer.update(&mut imgui, &mut rl);

        {
            let ui = imgui.new_frame();

            ui.window("Config").build(|| {
                for (index, point) in values.iter_mut().enumerate() {
                    let id = ui.push_id_int(index as i32);
                    ui.slider(&point.label, 0, 4, &mut point.count);

                    if point.count > 0 {
                        ui.radio_button("Myself", &mut myself_idx, index);
                    }
                    id.end();
                }

                ui.color_edit3("Background Color", &mut background_color);
                ui.color_edit3("Divider Color", &mut line_color);
                ui.color_edit3("Font Color", &mut font_color);
                ui.color_edit3("Myself Color", &mut self_color);
                ui.color_edit3("Team Color", &mut team_color);

                ui.input_text("Output", &mut output_path_buffer).build();

                if ui.button("Generate") {
                    let mut img = Image::gen_image_color(
                        (60 + values.len() * 100) as i32,
                        675,
                        norm_to_reg(background_color),
                    );

                    img.draw_line(
                        30,
                        600,
                        50 + values.len() as i32 * 100,
                        600,
                        norm_to_reg(line_color),
                    );

                    for (i, point) in values.iter_mut().enumerate() {
                        img.draw_text(
                            &point.label,
                            40 + i as i32 * 100,
                            650,
                            SIZE,
                            norm_to_reg(font_color),
                        );

                        for j in 0..point.count {
                            let color = if j == 0 && i == myself_idx {
                                norm_to_reg(self_color)
                            } else {
                                norm_to_reg(team_color)
                            };

                            img.draw_circle(40 + i as i32 * 100, 550 - j * 100, SIZE, color);
                        }
                    }

                    img.export_image(&output_path_buffer);
                }
            });
        }

        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(norm_to_reg(background_color));
            d.draw_line(
                30,
                600,
                50 + (values.len() * 100) as i32,
                600,
                norm_to_reg(line_color),
            );

            for (i, point) in values.iter().enumerate() {
                d.draw_text(
                    &point.label,
                    40 + i as i32 * 100,
                    650,
                    SIZE,
                    norm_to_reg(font_color),
                );

                for j in 0..values[i].count {
                    let color = if j == 0 && i == myself_idx {
                        norm_to_reg(self_color)
                    } else {
                        norm_to_reg(team_color)
                    };

                    d.draw_circle(40 + i as i32 * 100, 550 - j * 100, SIZE as f32, color);
                }
            }

            renderer.render(&mut imgui, &mut d);
        }
    }
}
