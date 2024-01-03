use std::string;

use druid::widget::{Align, Flex, Label, TextBox, List};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("File Explorer");

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
    directory: String,
    files: std::sync::Arc<Vec<String>>,
}

pub fn run(path: &String, fileList: &Vec<String>) {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = HelloState {
        name: "".into(),
        directory: path.into(),
        files: std::sync::Arc::new(fileList.clone()),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<HelloState> {
    // a label that will determine its text based on the current app data.
    let label = Label::new(|data: &HelloState, _env: &Env| format!("{}", data.directory));
    // a textbox that modifies `name`.
    let textbox = TextBox::new()
        .with_placeholder("Search")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::name);

    let file_list = List::new(|| {
        Label::new(|file: &String, _env: &Env| file.clone())
    })
    .lens(HelloState::files);

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(label)
        .with_child(file_list);

    // center the two widgets in the available space
    Align::centered(layout)
}