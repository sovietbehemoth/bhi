use druid::widget::{Button, Flex, Label, TextBox, ViewSwitcher};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};

mod format;
mod decode;
mod render;

#[derive(Clone, Data, Lens)]
struct AppState {
    intent: i32,

    current_text: String,
    file_path: String,
    new_file_name: String,
    convert_to: String

}
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::rect::Point;

use std::time::Duration;
use std::fs::File;
use std::io::Read;
use std::{thread, time};

pub fn main() {
    
    let main_window = WindowDesc::new(make_ui).title(LocalizedString::new("BHI"));

    let data: AppState = AppState {
        intent: -1,
        current_text:   String::new(),
        file_path:      String::new(),
        new_file_name:  String::new(),
        convert_to:     String::new()
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}

fn make_ui() -> impl Widget<AppState> {
    let mut switcher_column = Flex::column();

    switcher_column.add_child(
        Label::new(|_: &i32, _env: &Env| String::from("Convert to or from BHI."))
            .lens(AppState::intent),
    );


    for i in 0..3 {
        switcher_column.add_spacer(80.);

        match i {
            0 => {
                switcher_column.add_child(
                    Button::new(String::from("Convert to BHI"))
                        .on_click(move |_event, intent: &mut i32, _env| {
                            *intent = i;
                        })
                        .lens(AppState::intent),
                );
            },
            1 => {
                switcher_column.add_child(
                    Button::new(String::from("Convert from BHI"))
                        .on_click(move |_event, intent: &mut i32, _env| {
                            *intent = i;
                        })
                        .lens(AppState::intent),
                );
            },
            2 => {
                switcher_column.add_child(
                    Button::new(String::from("Show Image"))
                    .on_click(move |_event, intent: &mut i32, _env| {
                        *intent = i;
                    })
                    .lens(AppState::intent),
                );
            }

            _ => continue
        };

        
    }

    let view_switcher = ViewSwitcher::new(

        |data: &AppState, _env| data.intent,
        |selector, _data, _env| match selector {

            0 => Box::new(
                Flex::column()
                    .with_flex_child(Label::new("To BHI").center(), 1.0)


                    .with_flex_child(
                        Button::new("Convert").on_click(|_event, _data, _env| {

                            format::convert_to_BHI(AppState::file_path.with(_data, move |data: &String| {
                                data.clone()
                            }), AppState::new_file_name.with(_data, move |data: &String| {
                                data.clone()
                            }));
                        }),
                        1.0,
                    )


                    .with_flex_child(
                        Label::new(|data: &String, _env: &Env| String::from("File Path: "))
                            .lens(AppState::current_text),
                        1.0,
                    ).with_flex_child(TextBox::new().lens(AppState::file_path), 3.0)


                    .with_flex_child(
                        Label::new(|data: &String, _env: &Env| String::from("New File Name: "))
                            .lens(AppState::current_text),
                        1.0
                    ).with_flex_child(TextBox::new().lens(AppState::new_file_name), 10.0)

            ),

            1 => Box::new(
                Flex::column()
                    .with_flex_child(Label::new("From BHI").center(), 1.0)


                    .with_flex_child(
                        Button::new("Convert").on_click(|_event, _data, _env| {

                            decode::convert_from_BHI(AppState::file_path.with(_data, move |data: &String| {
                                data.clone()
                            }), AppState::new_file_name.with(_data, move |data: &String| {
                                data.clone()
                            }), AppState::convert_to.with(_data, move |data: &String| {
                                data.clone()
                            }));
                        }),
                        1.0,
                    )
                    .with_flex_child(
                        Label::new(|data: &String, _env: &Env| String::from("File Path: "))
                            .lens(AppState::current_text),
                        1.0,
                    ).with_flex_child(TextBox::new().lens(AppState::file_path), 3.0)


                    .with_flex_child(
                        Label::new(|data: &String, _env: &Env| String::from("New File Name: "))
                            .lens(AppState::current_text),
                        1.0
                    ).with_flex_child(TextBox::new().lens(AppState::new_file_name), 10.0)


                    .with_flex_child(
                        Label::new(|data: &String, _env: &Env| String::from("Convert To: "))
                            .lens(AppState::current_text),
                        1.0
                    ).with_flex_child(TextBox::new().lens(AppState::convert_to), 10.0),
            ),

            2 => Box::new(
                Flex::column()
                    .with_flex_child(Label::new("Show Image").center(), 1.0)


                    .with_flex_child(
                        Button::new("Display").on_click(|_event, _data, _env| {

                            render::render_BHI(AppState::file_path.with(_data, move |data: &String| {
                                data.clone()
                            }), _event.window());
                        }),
                        1.0,
                    )
                    .with_flex_child(
                        Label::new(|data: &String, _env: &Env| String::from("File Path: "))
                            .lens(AppState::current_text),
                        1.0,
                    ).with_flex_child(TextBox::new().lens(AppState::file_path), 3.0)

            ),

            _ => Box::new(Label::new("").center()),
        },
    );

    Flex::row()
        .with_child(switcher_column)
        .with_flex_child(view_switcher, 1.0)
}