// Copyright 2019 the Druid Authors
// SPDX-License-Identifier: Apache-2.0

//! Example of switches

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

#[allow(deprecated)]
use druid::widget::Parse;
use druid::widget::{
    Checkbox, Flex, Label, LensWrap, MainAxisAlignment, Padding, Stepper, Switch, TextBox,
    WidgetExt,
};
use druid::{AppLauncher, Data, Lens, LensExt, LocalizedString, Widget, WindowDesc};

#[derive(Clone, Data, Lens)]
struct DemoState {
    value: bool,
    stepper_value: f64,
}

fn build_widget() -> impl Widget<DemoState> {
    let mut col = Flex::column();
    let mut row = Flex::row();
    let switch = LensWrap::new(Switch::new(), DemoState::value);
    let check_box = LensWrap::new(Checkbox::new(""), DemoState::value);
    let switch_label = Label::new("Setting label");

    row.add_child(Padding::new(5.0, switch_label));
    row.add_child(Padding::new(5.0, switch));
    row.add_child(Padding::new(5.0, check_box));

    let stepper = LensWrap::new(
        Stepper::new()
            .with_range(0.0, 10.0)
            .with_step(0.5)
            .with_wraparound(false),
        DemoState::stepper_value,
    );

    let mut textbox_row = Flex::row();
    // TODO: Replace Parse usage with TextBox::with_formatter
    #[allow(deprecated)]
    let textbox = LensWrap::new(
        Parse::new(TextBox::new()),
        DemoState::stepper_value.map(|x| Some(*x), |x, y| *x = y.unwrap_or(0.0)),
    );
    textbox_row.add_child(Padding::new(5.0, textbox));
    textbox_row.add_child(Padding::new(5.0, stepper.center()));

    let mut label_row = Flex::row();

    let label = Label::new(|data: &DemoState, _env: &_| {
        format!("Stepper value: {0:.2}", data.stepper_value)
    });

    label_row.add_child(Padding::new(5.0, label));

    col.set_main_axis_alignment(MainAxisAlignment::Center);
    col.add_child(Padding::new(5.0, row));
    col.add_child(Padding::new(5.0, textbox_row));
    col.add_child(Padding::new(5.0, label_row));
    col.center()
}

pub fn main() {
    let window = WindowDesc::new(build_widget())
        .title(LocalizedString::new("switch-demo-window-title").with_placeholder("Switch Demo"));
    AppLauncher::with_window(window)
        .log_to_console()
        .launch(DemoState {
            value: true,
            stepper_value: 1.0,
        })
        .expect("launch failed");
}
