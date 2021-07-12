#![no_std]
#![feature(start)]

extern crate alloc;
use ogc::prelude::*;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    // Boilerplate
    let video = Video::init();
    Console::init(&video);
    Video::configure(video.render_config.into());
    Video::set_next_framebuffer(video.framebuffer);
    Video::set_black(false);
    Video::flush();
    Video::wait_vsync();

    // Pad
    Pad::init();

    loop {
        Pad::scan_pads();

        // Check for pressed buttons, fancy way
        let output = match Pad::buttons_down(Controller::One) {
            Button::A => "A was pressed!",
            Button::B => "B was pressed!",
            Button::X => "X was pressed!",
            Button::Y => "Y was pressed!",
            _ => "",
        };
        println!("{}", output);

        // Check for released buttons
        let released = Pad::buttons_up(Controller::One);

        if released == Button::X {
            println!("X was released");
        }

        if released == Button::Z {
            println!("Z was released");
        }

        // Check for held buttons, also shows how to test multiple buttons
        let held = Pad::buttons_held(Controller::One);

        if held == (Button::Up | Button::A) {
            println!("Up and A is being held");
        }

        if held == (Button::Down | Button::B) {
            println!("Down and B is being held");
        }

        // Check for analog stick
        let (stick_x, stick_y) = (Pad::stick_x(Controller::One), Pad::stick_y(Controller::One));

        if stick_x.abs() > 50 || stick_y.abs() > 50 {
            println!("Analog stick: ({}, {})", stick_x, stick_y);
        }

        // Check for c-stick
        let (sub_stick_x, sub_stick_y) = (
            Pad::sub_stick_x(Controller::One),
            Pad::sub_stick_y(Controller::One),
        );

        if sub_stick_x.abs() > 50 || sub_stick_y.abs() > 50 {
            println!("C-stick: ({}, {})", sub_stick_x, sub_stick_y);
        }

        // Check for triggers
        let (trigger_l, trigger_r) = (
            Pad::trigger_l(Controller::One),
            Pad::trigger_r(Controller::One),
        );

        if trigger_l > 50 || trigger_r > 50 {
            println!("Triggers: ({}, {})", trigger_l, trigger_r);
        }

        // Boilerplate
        Video::wait_vsync();
    }
}
