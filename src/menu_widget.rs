use druid::widget::{ Widget, Checkbox, Align, Flex, Label, Button };
use druid::{ WidgetExt, Env };
use crate::physics_simulation_widget::appdata::{ Params, AppData };

pub fn get_menu() -> impl Widget<AppData> {
    Flex::column()
        .with_flex_spacer(10.)
        .with_child(Align::left(Checkbox::new("Zero Gravity").lens(Params::zero_gravity)).lens(AppData::params))
        .with_spacer(10.)
        .with_child(Align::left(Checkbox::new("Walls").lens(Params::walls)).lens(AppData::params))
        .with_spacer(30.)
        .with_child(Align::left(Label::new("Left click:")))
        .with_spacer(10.)
        .with_child(
            Align::left(Checkbox::new("Spawn ball").lens(Params::spawn_tool))
            .lens(AppData::params)
            .on_click(|_, data, _| {
                data.params.move_tool = false;
            }))
        .with_spacer(10.)
        .with_child(
            Align::left(Checkbox::new("Move ball").lens(Params::move_tool))
            .lens(AppData::params)
            .on_click(|_, data, _| {
                data.params.spawn_tool = false;
            }))
        .with_spacer(30.)
        .with_child(Align::left(Label::new("Right click:")))
        .with_spacer(10.)
        .with_child(
            Align::left(Checkbox::new("Attract balls").lens(Params::attraction_tool)
                .lens(AppData::params))
                .on_click(|_, data, _| {
                    data.params.delete_tool = false;
                }))
        .with_spacer(10.)
        .with_child(
            Align::left(Checkbox::new("Delete ball").lens(Params::delete_tool))
                .lens(AppData::params)
                .on_click(|_, data, _| {
                    data.params.attraction_tool = false;
                }))
        .with_flex_spacer(10.)
        .with_child(
            Align::centered(
                Button::new(|data: &AppData, _: &Env| format!("{}", if data.params.paused {"Resume"} else {"Pause"}))
                    .on_click(|_, data, _| {
                        data.params.paused = !data.params.paused;
                    })
            ))
        .with_spacer(10.)
}