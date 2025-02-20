use makepad_widgets::*; // Import Makepad Widgets package

// Define live_design macro for declaring UI components and layout
live_design! {
    // import Makepad theme and shaders, 以及 widgets
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    ZooTitle = <View> { // Define ZooTitle component inheriting from View
        width: Fill,    // Fill width
        height: Fit,    // Fit height 
        // flow: Down,
        // align: { x: 0.0, y: 0.5},
        margin: 10.0,
        // spacing: 10.,
        // show_bg: true,
        title = <H2> {
            text: "Makepad UI Zoo"
        }
    }

    ZooHeader = <View> {
        width: Fill, height: Fit,
        flow: Down,
        spacing: (THEME_SPACE_1),
        margin: <THEME_MSPACE_H_3> {}
        divider = <Hr> { }
        title = <H3> { text: "Header" }
    }

    App = {{App}} {
        ui: <Window> {
            show_bg: true,
            draw_bg: {
                // color: #970707    // dark red
                // color: vec3(0.592, 0.027, 0.027) // dark red as above
                // color: vec3(1, 1, 0) // yellow
                // color: #000 // black
                }
            window: {
                title: "Makepad UI zoo"
            },
            caption_bar = {
                visible: true,
                margin: {left: -500},
                caption_label = { label = {text: "Makepad book UI Zoo caption bar"} },
            },

            body = <View> {
                width: Fill, height: Fill,
                flow: Down,
                spacing: 10.,
                margin: 0.,
                scroll_bars: <ScrollBars> {}

                <ZooTitle> {}
            }
        }
    }
}


// Define App struct containing UI and counter
#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef // UI component reference
}

// Implement LiveRegister trait for registering live design
impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        // Register Makepad Widgets' live design
        makepad_widgets::live_design(cx);
    }
}

// Implement AppMain trait for handling events
impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        // Handle UI events
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

// Define application entry point
app_main!(App);