#![windows_subsystem = "windows"]

use druid::widget::prelude::*;
use druid::{ AppLauncher, LocalizedString, WindowDesc, Widget };

use druid::widget::{ Align, Container, Label, Padding, Split };
use druid::piet::Color;

pub mod physics_simulation_widget;

fn build_app() -> impl Widget<physics_simulation_widget::AppData> {
    Padding::new(
        4.0,
        Container::new(
            Split::columns(
                physics_simulation_widget::PhysicsSimulationWidget::new(60),
                Align::centered(Label::new("Split B")),
            )
            .split_point(0.85)
            .bar_size(5.0)
            .min_bar_area(11.0)
            .draggable(false),
        )
        .border(Color::WHITE, 1.0),
    )
}

pub fn main() {
    let size: Size = Size::new(1100., 700.);
    let window = WindowDesc::new(|| {build_app()})
                    .title(LocalizedString::new("2D_Physics"))
                    .window_size(size)
                    .resizable(false);

    let launcher = AppLauncher::with_window(window);

    launcher
        .use_simple_logger()
        .launch(physics_simulation_widget::get_new_appdata(size))
        .expect("launch failed");
}