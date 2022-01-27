use druid::widget::{Button, Flex, Label, TextBox, ViewSwitcher};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};

mod encoder;
mod decode;
mod render;

#[derive(Clone, Data, Lens)]
struct AppState {
    intent: i32,                    //Which button was clicked.

    current_text: String,           //File path / file name.
    file_path: String,              //File path.
    new_file_name: String,          //New file name.
    convert_to: String              //Format to convert.

}


//The purpose of this file is to curate the GUI and provide an interface for the decoder, encoder, and renderer.




pub fn main() {
    
    //Init window.
    let main_window = WindowDesc::new(make_ui).title(LocalizedString::new("BHI"));


    //Init context object.
    let data: AppState = AppState {
        intent: -1,
        current_text:   String::new(),
        file_path:      String::new(),
        new_file_name:  String::new(),
        convert_to:     String::new()
    };


    //Start application.
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}


///UI builder.
fn make_ui() -> impl Widget<AppState> {
    let mut switcher_column = Flex::column();


    //Main header text.
    switcher_column.add_child(
        Label::new(|_: &i32, _env: &Env| String::from("Convert to or from BHI."))
            .lens(AppState::intent),
    );




    //Append buttons.

    for i in 0..3 {
        switcher_column.add_spacer(80.);


        //All 3 buttons.

        let msg = match i {
            0 => String::from("Convert to BHI"),
            1 => String::from("Convert from BHI"),
            2 => String::from("Show Image"),
            _ => continue
        };


        
        //Add button column.

        switcher_column.add_child(
            Button::new(msg)
            .on_click(move |_event, intent: &mut i32, _env| {
                *intent = i;
            })
            .lens(AppState::intent),
        );
    }


    //Main event loop (on button click).


    let view_switcher = ViewSwitcher::new(

        |data: &AppState, _env| data.intent,


        //On click.

        |selector, _data, _env| match selector {

            0 => Box::new(
                Flex::column()
                    //To BHI conversion.
                    .with_flex_child(Label::new("To BHI").center(), 1.0)

                    .with_flex_child(
                        Button::new("Convert").on_click(|_event, _data, _env| {

                            //Call conversion function.
                            encoder::convert_to_BHI(AppState::file_path.with(_data, move |data: &String| {
                                data.clone()
                            }), AppState::new_file_name.with(_data, move |data: &String| {
                                data.clone()
                            }));
                        }),
                        1.0,
                    )



                    //Text boxes.


                    .with_flex_child(
                        Label::new(|_data: &String, _env: &Env| String::from("File Path: "))
                            .lens(AppState::current_text),
                        1.0,
                    ).with_flex_child(TextBox::new().lens(AppState::file_path), 3.0)


                    .with_flex_child(
                        Label::new(|_data: &String, _env: &Env| String::from("New File Name: "))
                            .lens(AppState::current_text),
                        1.0
                    ).with_flex_child(TextBox::new().lens(AppState::new_file_name), 10.0)

            ),

            1 => Box::new(
                Flex::column()
                    //Convert from BHI to other formats.
                    .with_flex_child(Label::new("From BHI").center(), 1.0)

                    .with_flex_child(
                        Button::new("Convert").on_click(|_event, _data, _env| {

                            //Call conversion function.

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


                    //Text boxes.

                    .with_flex_child(
                        Label::new(|_data: &String, _env: &Env| String::from("File Path: "))
                            .lens(AppState::current_text),
                        1.0,
                    ).with_flex_child(TextBox::new().lens(AppState::file_path), 3.0)


                    .with_flex_child(
                        Label::new(|_data: &String, _env: &Env| String::from("New File Name: "))
                            .lens(AppState::current_text),
                        1.0
                    ).with_flex_child(TextBox::new().lens(AppState::new_file_name), 10.0)


                    .with_flex_child(
                        Label::new(|_data: &String, _env: &Env| String::from("Convert To: "))
                            .lens(AppState::current_text),
                        1.0
                    ).with_flex_child(TextBox::new().lens(AppState::convert_to), 10.0),
            ),

            2 => Box::new(
                Flex::column()
                    //Render image.

                    .with_flex_child(Label::new("Show Image").center(), 1.0)

                    .with_flex_child(
                        Button::new("Display").on_click(|_event, _data, _env| {

                            //Call image rendering function, pass context to close main window.

                            render::render_BHI(AppState::file_path.with(_data, move |data: &String| {
                                data.clone()
                            }), _event.window());
                        }),
                        1.0,
                    )

                    //Text box.

                    .with_flex_child(
                        Label::new(|_data: &String, _env: &Env| String::from("File Path: "))
                            .lens(AppState::current_text),
                        1.0,
                    ).with_flex_child(TextBox::new().lens(AppState::file_path), 3.0)

            ),

            _ => Box::new(Label::new("").center()),
        },
    );

    //Return context.

    Flex::row()
        .with_child(switcher_column)
        .with_flex_child(view_switcher, 1.0)
}