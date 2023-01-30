extern crate sdl2;

use std::f64;

use std::collections::HashMap;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use sdl2::gfx::primitives::DrawRenderer;

const WINDOW_WIDHT: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

#[derive(Debug, Clone, Copy)]
enum InputType {
    Button(sdl2::controller::Button),
    Axis(sdl2::controller::Axis),
}

struct Lavanka {
    buttons: HashMap<sdl2::controller::Button, i16>,
    axes: HashMap<sdl2::controller::Axis, i16>,
}

fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

impl Lavanka {
    fn new() -> Self {
        let mut buttons: HashMap<sdl2::controller::Button, i16> = HashMap::new();
        let mut axes: HashMap<sdl2::controller::Axis, i16> = HashMap::new();

        buttons.insert(sdl2::controller::Button::A, 0);
        buttons.insert(sdl2::controller::Button::B, 0);
        buttons.insert(sdl2::controller::Button::X, 0);
        buttons.insert(sdl2::controller::Button::Y, 0);
        buttons.insert(sdl2::controller::Button::Back, 0);
        buttons.insert(sdl2::controller::Button::Guide, 0);
        buttons.insert(sdl2::controller::Button::Start, 0);
        buttons.insert(sdl2::controller::Button::LeftStick, 0);
        buttons.insert(sdl2::controller::Button::RightStick, 0);
        buttons.insert(sdl2::controller::Button::LeftShoulder, 0);
        buttons.insert(sdl2::controller::Button::RightShoulder, 0);
        buttons.insert(sdl2::controller::Button::DPadUp, 0);
        buttons.insert(sdl2::controller::Button::DPadDown, 0);
        buttons.insert(sdl2::controller::Button::DPadLeft, 0);
        buttons.insert(sdl2::controller::Button::DPadRight, 0);
        buttons.insert(sdl2::controller::Button::Misc1, 0);
        buttons.insert(sdl2::controller::Button::Paddle1, 0);
        buttons.insert(sdl2::controller::Button::Paddle2, 0);
        buttons.insert(sdl2::controller::Button::Paddle3, 0);
        buttons.insert(sdl2::controller::Button::Paddle4, 0);
        buttons.insert(sdl2::controller::Button::Touchpad, 0);

        axes.insert(sdl2::controller::Axis::LeftX, 0);
        axes.insert(sdl2::controller::Axis::LeftY, 0);
        axes.insert(sdl2::controller::Axis::RightX, 0);
        axes.insert(sdl2::controller::Axis::RightY, 0);
        axes.insert(sdl2::controller::Axis::TriggerLeft, 0);
        axes.insert(sdl2::controller::Axis::TriggerRight, 0);

        Lavanka { buttons, axes }
    }

    fn handle_controller_input(self: &mut Self, button: InputType, value: i16) {
        println!("{:?} {:?}", button, value);
        match button {
            InputType::Button(button) => {
                self.buttons.insert(button, value);
                // println!("{:?}", self.buttons);
                // println!("{:?} {}", button, if value != 0 { "down" } else { "up" });
            }
            InputType::Axis(axis) => {
                self.axes.insert(axis, value);
                // println!("{:?} {}", axis, value)
            }
        }
    }

    fn draw_to(self: &Self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let x_offset: i16 = WINDOW_WIDHT as i16 / 2;
        let y_offset: i16 = WINDOW_HEIGHT as i16 / 2;

        let dpad_size: u32 = 16;
        let dpad_x_offset: i32 = -300;
        let dpad_y_offset: i32 = 0;
        let circle_rad: i16 = 8;
        let abxy_x_offset: i16 = 300;

        let shoulder_x_size: u32 = 32;
        let shoulder_y_size: u32 = 16;

        let back_x_size: u32 = 32;
        let back_y_size: u32 = 16;

        let mut l3_color = Color::RGB(120, 120, 120);
        let mut r3_color = Color::RGB(120, 120, 120);

        for (button, value) in self.buttons.iter() {
            match button {
                sdl2::controller::Button::A => {
                    let btn_x_offset: i16 = 0 + abxy_x_offset;
                    let btn_y_offset: i16 = 20;
                    if *value != 0 {
                        canvas
                            .filled_circle(
                                x_offset + btn_x_offset,
                                y_offset + btn_y_offset,
                                circle_rad,
                                Color::RGB(255, 255, 255),
                            )
                            .unwrap();
                    } else {
                        canvas
                            .circle(
                                x_offset + btn_x_offset,
                                y_offset + btn_y_offset,
                                circle_rad,
                                Color::RGB(255, 255, 255),
                            )
                            .unwrap();
                    }
                }
                sdl2::controller::Button::B => {
                    let btn_x_offset: i16 = 20 + abxy_x_offset;
                    let btn_y_offset: i16 = 0;
                    if *value != 0 {
                        canvas
                            .filled_circle(
                                x_offset + btn_x_offset,
                                y_offset + btn_y_offset,
                                circle_rad,
                                Color::RGB(255, 255, 255),
                            )
                            .unwrap();
                    } else {
                        canvas
                            .circle(
                                x_offset + btn_x_offset,
                                y_offset + btn_y_offset,
                                circle_rad,
                                Color::RGB(255, 255, 255),
                            )
                            .unwrap();
                    }
                }
                sdl2::controller::Button::X => {
                    let btn_x_offset: i16 = -20 + abxy_x_offset;
                    let btn_y_offset: i16 = 0;
                    if *value != 0 {
                        canvas
                            .filled_circle(
                                x_offset + btn_x_offset,
                                y_offset + btn_y_offset,
                                circle_rad,
                                Color::RGB(255, 255, 255),
                            )
                            .unwrap();
                    } else {
                        canvas
                            .circle(
                                x_offset + btn_x_offset,
                                y_offset + btn_y_offset,
                                circle_rad,
                                Color::RGB(255, 255, 255),
                            )
                            .unwrap();
                    }
                }
                sdl2::controller::Button::Y => {
                    let btn_x_offset: i16 = 0 + abxy_x_offset;
                    let btn_y_offset: i16 = -20;
                    if *value != 0 {
                        canvas
                            .filled_circle(
                                x_offset + btn_x_offset,
                                y_offset + btn_y_offset,
                                circle_rad,
                                Color::RGB(255, 255, 255),
                            )
                            .unwrap();
                    } else {
                        canvas
                            .circle(
                                x_offset + btn_x_offset,
                                y_offset + btn_y_offset,
                                circle_rad,
                                Color::RGB(255, 255, 255),
                            )
                            .unwrap();
                    }
                }
                sdl2::controller::Button::Back => {
                    let back_x_offset: i32 = -85;
                    let back_y_offset: i32 = -30;

                    if *value != 0 {
                        canvas
                            .filled_trigon(
                                x_offset + back_x_offset as i16,
                                y_offset + back_y_offset as i16,
                                x_offset + back_x_offset as i16,
                                y_offset + back_y_size as i16 + back_y_offset as i16,
                                x_offset + back_x_size as i16 + back_x_offset as i16,
                                y_offset + (back_y_size as i16 / 2) + back_y_offset as i16,
                                Color::RGB(255, 255, 255),
                            )
                            .unwrap();
                        // canvas.fill_rect(rect).unwrap();
                    } else {
                        canvas
                            .trigon(
                                x_offset + back_x_offset as i16,
                                y_offset + back_y_offset as i16,
                                x_offset + back_x_offset as i16,
                                y_offset + back_y_size as i16 + back_y_offset as i16,
                                x_offset + back_x_size as i16 + back_x_offset as i16,
                                y_offset + (back_y_size as i16 / 2) + back_y_offset as i16,
                                Color::RGB(255, 255, 255),
                            )
                            .unwrap();
                        // canvas.draw_rect(rect).unwrap();
                    }
                }
                sdl2::controller::Button::Guide => {
                    let guide_y_offset = -60;
                    let guide_rad = 20;
                    if *value != 0 {
                        canvas
                            .filled_circle(
                                x_offset,
                                y_offset + guide_y_offset,
                                guide_rad,
                                Color::RGB(255, 255, 255),
                            )
                            .unwrap();
                    } else {
                        canvas
                            .circle(
                                x_offset,
                                y_offset + guide_y_offset,
                                guide_rad,
                                Color::RGB(255, 255, 255),
                            )
                            .unwrap();
                    }
                }
                sdl2::controller::Button::Start => {
                    let start_x_offset: i32 = 85;
                    let start_y_offset: i32 = -30;

                    if *value != 0 {
                        canvas
                            .filled_trigon(
                                x_offset + start_x_offset as i16,
                                y_offset + start_y_offset as i16,
                                x_offset + start_x_offset as i16,
                                y_offset + back_y_size as i16 + start_y_offset as i16,
                                x_offset - back_x_size as i16 + start_x_offset as i16,
                                y_offset + (back_y_size as i16 / 2) + start_y_offset as i16,
                                Color::RGB(255, 255, 255),
                            )
                            .unwrap();
                        // canvas.fill_rect(rect).unwrap();
                    } else {
                        canvas
                            .trigon(
                                x_offset + start_x_offset as i16,
                                y_offset + start_y_offset as i16,
                                x_offset + start_x_offset as i16,
                                y_offset + back_y_size as i16 + start_y_offset as i16,
                                x_offset - back_x_size as i16 + start_x_offset as i16,
                                y_offset + (back_y_size as i16 / 2) + start_y_offset as i16,
                                Color::RGB(255, 255, 255),
                            )
                            .unwrap();
                        // canvas.draw_rect(rect).unwrap();
                    }
                }
                sdl2::controller::Button::LeftStick => {
                    if *value != 0 {
                        l3_color = Color::RGB(255, 255, 255);
                    }
                }
                sdl2::controller::Button::RightStick => {
                    if *value != 0 {
                        r3_color = Color::RGB(255, 255, 255);
                    }
                }
                sdl2::controller::Button::LeftShoulder => {
                    let shoulder_x_offset: i32 = -150;
                    let shoulder_y_offset: i32 = -50;

                    let rect = sdl2::rect::Rect::new(
                        x_offset as i32 + shoulder_x_offset - shoulder_x_size as i32 / 2,
                        y_offset as i32 + shoulder_y_offset - shoulder_y_size as i32 / 2,
                        shoulder_x_size,
                        shoulder_y_size,
                    );

                    if *value != 0 {
                        canvas.fill_rect(rect).unwrap();
                    } else {
                        canvas.draw_rect(rect).unwrap();
                    }
                }
                sdl2::controller::Button::RightShoulder => {
                    let shoulder_x_offset: i32 = 150;
                    let shoulder_y_offset: i32 = -50;

                    let rect = sdl2::rect::Rect::new(
                        x_offset as i32 + shoulder_x_offset - shoulder_x_size as i32 / 2,
                        y_offset as i32 + shoulder_y_offset - shoulder_y_size as i32 / 2,
                        shoulder_x_size,
                        shoulder_y_size,
                    );

                    if *value != 0 {
                        canvas.fill_rect(rect).unwrap();
                    } else {
                        canvas.draw_rect(rect).unwrap();
                    }
                }
                sdl2::controller::Button::DPadUp => {
                    let btn_x_offset: i32 = 0 + dpad_x_offset;
                    let btn_y_offset: i32 = -20 + dpad_y_offset;
                    let rect = sdl2::rect::Rect::new(
                        x_offset as i32 + btn_x_offset - dpad_size as i32 / 2,
                        y_offset as i32 + btn_y_offset - dpad_size as i32 / 2,
                        dpad_size,
                        dpad_size,
                    );

                    if *value != 0 {
                        canvas.fill_rect(rect).unwrap();
                    } else {
                        canvas.draw_rect(rect).unwrap();
                    }
                }
                sdl2::controller::Button::DPadDown => {
                    let btn_x_offset: i32 = 0 + dpad_x_offset;
                    let btn_y_offset: i32 = 20 + dpad_y_offset;
                    let rect = sdl2::rect::Rect::new(
                        x_offset as i32 + btn_x_offset - dpad_size as i32 / 2,
                        y_offset as i32 + btn_y_offset - dpad_size as i32 / 2,
                        dpad_size,
                        dpad_size,
                    );

                    if *value != 0 {
                        canvas.fill_rect(rect).unwrap();
                    } else {
                        canvas.draw_rect(rect).unwrap();
                    }
                }
                sdl2::controller::Button::DPadLeft => {
                    let btn_x_offset: i32 = -20 + dpad_x_offset;
                    let btn_y_offset: i32 = 0 + dpad_y_offset;
                    let rect = sdl2::rect::Rect::new(
                        x_offset as i32 + btn_x_offset - dpad_size as i32 / 2,
                        y_offset as i32 + btn_y_offset - dpad_size as i32 / 2,
                        dpad_size,
                        dpad_size,
                    );

                    if *value != 0 {
                        canvas.fill_rect(rect).unwrap();
                    } else {
                        canvas.draw_rect(rect).unwrap();
                    }
                }
                sdl2::controller::Button::DPadRight => {
                    let btn_x_offset: i32 = 20 + dpad_x_offset;
                    let btn_y_offset: i32 = 0 + dpad_y_offset;
                    let rect = sdl2::rect::Rect::new(
                        x_offset as i32 + btn_x_offset - dpad_size as i32 / 2,
                        y_offset as i32 + btn_y_offset - dpad_size as i32 / 2,
                        dpad_size,
                        dpad_size,
                    );

                    if *value != 0 {
                        canvas.fill_rect(rect).unwrap();
                    } else {
                        canvas.draw_rect(rect).unwrap();
                    }
                }
                sdl2::controller::Button::Misc1 => {}
                sdl2::controller::Button::Paddle1 => {}
                sdl2::controller::Button::Paddle2 => {}
                sdl2::controller::Button::Paddle3 => {}
                sdl2::controller::Button::Paddle4 => {}
                sdl2::controller::Button::Touchpad => {}
            }
        }

        /*LEFT STICK */
        {
            let l_stick_x_val = *self.axes.get(&sdl2::controller::Axis::LeftX).unwrap();
            let l_stick_y_val = *self.axes.get(&sdl2::controller::Axis::LeftY).unwrap();

            let l_stick_x_mapped =
                map_range((-32767.0, 32768.0), (-15.0, 15.0), l_stick_x_val as f64) as i16;
            let l_stick_y_mapped =
                map_range((-32767.0, 32768.0), (-15.0, 15.0), l_stick_y_val as f64) as i16;

            let l_stick_x_offset: i16 = -90;
            let l_stick_y_offset: i16 = 60;

            canvas
                .circle(
                    x_offset + l_stick_x_offset,
                    y_offset + l_stick_y_offset,
                    24,
                    Color::RGB(255, 255, 255),
                )
                .unwrap();

            canvas
                .filled_circle(
                    x_offset + l_stick_x_offset + l_stick_x_mapped,
                    y_offset + l_stick_y_mapped + l_stick_y_offset,
                    20,
                    l3_color,
                )
                .unwrap();
        }

        /*RIGHT STICK */
        {
            let r_stick_x_val = *self.axes.get(&sdl2::controller::Axis::RightX).unwrap();
            let r_stick_y_val = *self.axes.get(&sdl2::controller::Axis::RightY).unwrap();

            let r_stick_x_mapped =
                map_range((-32767.0, 32768.0), (-15.0, 15.0), r_stick_x_val as f64) as i16;
            let r_stick_y_mapped =
                map_range((-32767.0, 32768.0), (-15.0, 15.0), r_stick_y_val as f64) as i16;

            let r_stick_x_offset: i16 = 90;
            let r_stick_y_offset: i16 = 60;

            canvas
                .circle(
                    x_offset + r_stick_x_offset,
                    y_offset + r_stick_y_offset,
                    24,
                    Color::RGB(255, 255, 255),
                )
                .unwrap();

            canvas
                .filled_circle(
                    x_offset + r_stick_x_offset + r_stick_x_mapped,
                    y_offset + r_stick_y_mapped + r_stick_y_offset,
                    20,
                    r3_color,
                )
                .unwrap();
        }

        /*LEFT TRIGGER */
        {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            let l_trigger_x_offset: i32 = -170;
            let l_trigger_y_offset: i32 = -80;

            let l_trigger_x_size: u32 = 16;
            let l_trigger_y_size: u32 = 32;

            let l_trigger_val = *self.axes.get(&sdl2::controller::Axis::TriggerLeft).unwrap();
            let l_trigger_mapped =
                map_range((0.0, 32768.0), (0.0, 32.0), l_trigger_val as f64) as i16;

            let rect = sdl2::rect::Rect::new(
                x_offset as i32 + l_trigger_x_offset - l_trigger_x_size as i32 / 2,
                y_offset as i32 + l_trigger_y_offset - l_trigger_y_size as i32 / 2,
                l_trigger_x_size,
                l_trigger_y_size,
            );

            let fill_rect = sdl2::rect::Rect::new(
                x_offset as i32 + l_trigger_x_offset - l_trigger_x_size as i32 / 2,
                y_offset as i32 + l_trigger_y_size as i32 - l_trigger_mapped as i32
                    + l_trigger_y_offset
                    - l_trigger_y_size as i32 / 2,
                l_trigger_x_size,
                l_trigger_mapped as u32,
            );

            canvas.draw_rect(rect).unwrap();
            canvas.fill_rect(fill_rect).unwrap();
        }

        /*RIGHT TRIGGER */
        {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            let r_trigger_x_offset: i32 = 170;
            let r_trigger_y_offset: i32 = -80;

            let r_trigger_x_size: u32 = 16;
            let r_trigger_y_size: u32 = 32;

            let r_trigger_val = *self
                .axes
                .get(&sdl2::controller::Axis::TriggerRight)
                .unwrap();
            let r_trigger_mapped =
                map_range((0.0, 32768.0), (0.0, 32.0), r_trigger_val as f64) as i16;

            let rect = sdl2::rect::Rect::new(
                x_offset as i32 + r_trigger_x_offset - r_trigger_x_size as i32 / 2,
                y_offset as i32 + r_trigger_y_offset - r_trigger_y_size as i32 / 2,
                r_trigger_x_size,
                r_trigger_y_size,
            );

            let fill_rect = sdl2::rect::Rect::new(
                x_offset as i32 + r_trigger_x_offset - r_trigger_x_size as i32 / 2,
                y_offset as i32 + r_trigger_y_size as i32 - r_trigger_mapped as i32
                    + r_trigger_y_offset
                    - r_trigger_y_size as i32 / 2,
                r_trigger_x_size,
                r_trigger_mapped as u32,
            );

            canvas.draw_rect(rect).unwrap();
            canvas.fill_rect(fill_rect).unwrap();
        }

        // for (axis, value) in self.axes.iter() {
        //     match axis {
        //         sdl2::controller::Axis::LeftX => {}
        //         sdl2::controller::Axis::LeftY => {}
        //         sdl2::controller::Axis::RightX => {}
        //         sdl2::controller::Axis::RightY => {}
        //         sdl2::controller::Axis::TriggerLeft => {}
        //         sdl2::controller::Axis::TriggerRight => {}
        //     }
        // }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let game_controller_subsystem = sdl_context.game_controller().unwrap();
    let window = video_subsystem
        .window("lavanka", WINDOW_WIDHT, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // let controllers = game_controller_subsystem.num_joysticks().unwrap();
    let _controller = game_controller_subsystem.open(0).unwrap();
    // let controller_name = game_controller_subsystem.name_for_index(0).unwrap();
    // println!("{}", controller_name);

    game_controller_subsystem.set_event_state(true);

    let mut lavanka = Lavanka::new();

    'runn: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'runn,
                Event::ControllerButtonDown { button, .. } => {
                    lavanka.handle_controller_input(InputType::Button(button), 1)
                }
                Event::ControllerButtonUp { button, .. } => {
                    lavanka.handle_controller_input(InputType::Button(button), 0)
                }
                Event::ControllerAxisMotion { axis, value, .. } => {
                    lavanka.handle_controller_input(InputType::Axis(axis), value)
                }
                _ => {}
            }
        }

        // draw background
        canvas.set_draw_color(Color::RGB(21, 21, 21));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        lavanka.draw_to(&mut canvas);

        canvas.present();
    }
}
