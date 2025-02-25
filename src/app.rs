use makepad_widgets::*; // Import Makepad Widgets package

// Define live_design macro for declaring UI components and layout
live_design! {
    // import Makepad theme and shaders, and widgets
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // define color of background container
    COLOR_CONTAINER = (THEME_COLOR_D_1)

    // define colors for demo blocks
    DEMO_COLOR_1 = #8f0
    DEMO_COLOR_2 = #0f8
    DEMO_COLOR_3 = #80f

    ZooGroup = <RoundedView> {
        height: Fit, width: Fill,
        flow: Right,
        align: { x: 0.0, y: 0.5},
        margin: 0.,
        show_bg: false;
        draw_bg: { color: (COLOR_CONTAINER) }
    }

    ZooBlock = <RoundedView> {
        width: 50., height: 50.
        margin: 0.,
        spacing: 0.,

        show_bg: true;
        draw_bg: {
            // return color based on position
            fn get_color(self) -> vec4 {
                return mix(self.color, self.color*0.5, self.pos.y);
            }
            
            // a float value for the corner radius
            radius: (THEME_CONTAINER_CORNER_RADIUS)
        }
    }

    ZooTitle = <View> { // Define ZooTitle component inheriting from View
        width: Fill,    // Fill width
        height: Fit,    // Fit height 
        margin: 10.0,
        title = <H2> {
            text: "Makepad UI Zoo"
        }
    }

    ZooHeader = <View> {
        width: Fill, height: Fit,
        flow: Down,
        spacing: 10.,
        margin: {top: 0., right: 9, bottom: 0., left: 9}
        divider = <Hr> { }
        title = <H3> { }
    }

    ZooDesc = <P> { }


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
                spacing: 10.,   //spacing between child components
                margin: 0.,     //margin around the component
                scroll_bars: <ScrollBars> {}

                <ZooTitle> {}
                <ZooHeader> {
                    title = {text: "Intro"}
                    <ZooDesc> {
                        text: "Intro."
                    }
                    <View> {
                        width: Fill, height: Fit,
                        flow: Down,
                        <P> { text: "- Shader-based: what does that mean for how things work." }
                        <P> { text: "- Inheritance mechanisms in the DSL." }
                        <P> { text: "- Introduction to the layout system." }
                        <P> { text: "- Base theme parameters." }
                        <P> { text: "- Typographic system. Base font-size and contrast." }
                        <P> { text: "- Space constants to control denseness of the design." }
                        <P> { text: "- Transparency mechanism of the widgets. Nesting for structure." }
                    }
                }

                <ZooHeader> {
                    title = {text: "Control Heights & Text Baselines"}
                    <ZooDesc> {
                        text: "Control heights and text baselines"
                    }
                    <View> {
                        width: Fill, height: Fit,
                        align: { x: 0., y: 0.}
                        flow: Right,
                        spacing: (THEME_SPACE_2)
                        <P> { text: "TestLabel", width: Fit}
                        <Vr> {} 
                        <LinkLabel> { text: "TestButton", width: Fit}
                        <CheckBox> { text: "TestButton"}
                        <CheckBoxToggle> { text: "TestButton"}
                        <ButtonFlat> { text: "TestButton"}
                        <Button> { text: "TestButton"}
                        <TextInput> { text: "TestButton"}
                        <Slider> { text: "TestButton"}
                        <SliderBig> { text: "TestButton"}
                    }
                }

                <ZooHeader> {
                    title = {text: "Typography"}
                    <ZooDesc> {
                        text: "Typography."
                    }
                    <View> {
                        width: Fill, height: Fit,
                        flow: Down,

                        <H1> { text: "H1 headline" }
                        <H1italic> { text: "H1 italic headline" }
                        <H2> { text: "H2 headline" }
                        <H2italic> { text: "H2 italic headline" }
                        <H3> { text: "H3 headline" }
                        <H3italic> { text: "H3 italic headline" }
                        <H4> { text: "H4 headline" }
                        <H4italic> { text: "H4 italic headline" }
                        <P> { text: "P copy text" }
                        <Pitalic> { text: "P italic copy text" }
                        <Pbold> { text: "P bold copy text" }
                        <Pbolditalic> { text: "P bold italic copy text" }
                    }
                }

                <ZooHeader> {
                    title = {text: "<View>" }
                    <ZooDesc> {text:"This is a gray view with flow set to Right\nTo show the extend, the background has been enabled using show_bg and a gray pixelshader has been provided to draw_bg."}
                    <View> {
                        height: Fit
                        flow: Right,
                        show_bg: true,
                        draw_bg: { color: (COLOR_CONTAINER) }
                        padding: 10.
                        spacing: 10.
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_1)}}
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_3)}}
                    }

                    <ZooDesc> {text:"This utlizes a <Filler> to separate items."}
                    <View> {
                        height: Fit
                        flow: Right,
                        show_bg: true,
                        draw_bg: { color: (COLOR_CONTAINER) }
                        padding: 10.
                        spacing: 10.
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_1)}}
                        <Filler> {} // placeholder
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_3)}}
                    }
                    
                    <ZooDesc> {text:"This view is bigger on the inside"}
                    <View> {
                        width: 150, height: 150,
                        flow: Right,
                        padding: 10.
                        spacing: 10.

                        show_bg: true,
                        draw_bg: { color: (COLOR_CONTAINER) }
                        scroll_bars: <ScrollBars> {}

                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            show_bg: false,
                            spacing: 10
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_1)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_3)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                        }

                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            show_bg: false,
                            spacing: 10
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_1)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_3)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                        }

                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            show_bg: false,
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_1)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_3)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                        }

                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            show_bg: false,
                            spacing: 10
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_1)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_3)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                        }

                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            show_bg: false,
                            spacing: 10
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_1)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_3)}}
                        }
                    }
                }

                <ZooHeader> {
                    title = {text:"<TextInput> with interaction"}
                    padding: 10.
                        <View> {
                            height: Fit, width: Fill,
                            spacing: (THEME_SPACE_2),
                            textalreadyfilled = <TextInput> {
                                text: "text here"
                            }
                            simpletextinput = <TextInput> { // simpletextinput is the id
                                width: Fill,
                             empty_message: "input" }
                             // chanagble output box
                            simpletextinput_outputbox = <P> {
                                text: "Output"
                            }
                        }
                }

                <ZooHeader> {
                    title = {text:"<Button>"}
                    <ZooDesc> {text:"A small clickable region"}
                    <ZooGroup> {
                        flow: Down,
                        width: Fill, height: Fit,
                        align: { x: 0.0, y: 0.5 }
                        spacing: 10.,

                        <H4> { text: "Default"}
                        <Label> { text: "<Button>"}
                        basicbutton = <Button> { text: "I can be clicked" }

                        <H4> { text: "Button with an icon"}
                        <Label> { text: "<ButtonIcon>"}
                        iconbutton = <ButtonIcon> {
                            draw_icon: {
                                color: #ff0,
                                svg_file: dep("crate://self/resources/Icon_Favorite.svg"),
                            }
                            text: "I can have a icon!"
                        }

                        <H4> { text: "Flat Mode"}
                        <Label> { text: "<ButtonFlat>"}
                        <View> {
                            flow: Right,
                            align: { x: 0., y: 0.5 }
                            width: Fill, height: Fit,
                            <ButtonFlat> {
                                draw_icon: {
                                    color: #f00,
                                    svg_file: dep("crate://self/resources/Icon_Favorite.svg"),
                                }
                                text: "I can have a lovely icon!"
                            }

                            <ButtonFlat> {
                                draw_icon: {
                                    svg_file: dep("crate://self/resources/Icon_Favorite.svg"),
                                }
                            }

                            <ButtonFlat> {
                                flow: Down,
                                icon_walk: { width: 15. }
                                draw_icon: {
                                    svg_file: dep("crate://self/resources/Icon_Favorite.svg"),
                                }
                                text: "Vertical Layout"
                            }
                        }

                            <H4> { text: "Hover"}
                            styledbutton = <Button> {
                                draw_bg: {
                                    fn pixel(self) -> vec4 {
                                        return (THEME_COLOR_MAKEPAD) + self.pressed * vec4(1., 1., 1., 1.)
                                    }
                                }
                                draw_text: {
                                    fn get_color(self) -> vec4 {
                                        return (THEME_COLOR_U_5) - vec4(0., 0.1, 0.4, 0.) * self.hover - self.pressed * vec4(1., 1., 1., 0.);
                                    }
                                }
                                text: "I can be styled!"
                            }
                        
                    
                        }
                }

                <ZooHeader> {
                    title = {text:"<Label>"}
                    <ZooDesc> { text:"Default single line textbox" }
                    <ZooGroup> { <Label> { text: "This is a small line of text" } }
                    <ZooGroup> {
                        <Label> {
                            draw_text: {
                                color: #fff,
                                text_style: {
                                    font: {path: dep("crate://self/resources/XITSOneText-BoldItalic.ttf")},
                                    font_size: 20,
                                }
                            },
                            text: "You can style text using colors and fonts"
                        }
                    }
                    <ZooGroup> {
                        <Label> {
                            draw_text: {
                                fn get_color(self) ->vec4{
                                    return mix((THEME_COLOR_MAKEPAD), (THEME_COLOR_U_HIDDEN), self.pos.x)
                                }
                                color: (THEME_COLOR_MAKEPAD)
                                text_style: {
                                    font_size: 40.,
                                }
                            },
                            text: "OR EVEN SOME PIXELSHADERS"
                        }
                    }
                }

                <ZooHeader> {
                    title = { text:"<Slider>" }
                    <ZooDesc> { text:"A parameter dragger" }
                    <ZooGroup> {
                        width: Fill, height: Fit,
                        flow: Right,
                        spacing: 10.0,
                        align: { x: 0., y: 0.}
                        <View> {
                            width: Fill, height: Fit,
                            flow: Down,
                            <Slider> { text: "Default" }
                            <Slider> { text: "min 0 to max 100", min: 0., max: 100. }
                            <Slider> { text: "precision 7", precision: 4 }  // max precision
                            <Slider> { text: "stepped 0.1", step: 0.1 }
                        }
                        <View> {
                            width: Fill, height: Fit,
                            flow: Down,
                            <SliderBig> { text: "Default 0.2", default: 0.2 }
                            <SliderBig> { text: "min -50 to max 50", min: -50, max: 50. }
                            <SliderBig> { text: "precision 8", precision: 8 }   // when greater than 7, auto jump to 16 and won't changed
                            <SliderBig> { text: "stepped 0.2", step: 0.2 }
                        }
                        <View> {
                            width: Fill, height: Fit,
                            flow: Down,
                            <SliderAlt1> {
                                text: "Colored",
                                draw_slider: {
                                    val_color_a: (#FFCC00),
                                    val_color_b: #f00,
                               }
                            }
                        }
                    }
                }

                <View> {
                    width: Fill, height: Fit,
                    flow: Right,
                    <Rotary> {
                        width: 100, height: 100,
                        text: "Rotary",
                        draw_slider: {
                            gap: 90.,
                            width: 30.
                            padding: 2.,
                        }
                    }
                    <Rotary> {
                        width: 100, height: 100,
                        text: "Change width",
                        draw_slider: {
                            gap: 90.,
                            width: 10.,
                            padding: 2.,
                        }
                    }
                    <Rotary> {
                        width: 100, height: 100,
                        text: "Change gap",
                        draw_slider: {
                            gap: 180.,
                            width: 10.
                            padding: 4,
                        }
                    }
                    <Rotary> {
                        width: 150, height: 150,
                        text: "Change size",
                        draw_slider: {
                            gap: 90.,
                            width: 20.
                            padding: 4.,
                        }
                    }
                }

                <View> {
                    width: Fill, height: Fit,
                    flow: Right,
                    <RotaryFlat> {
                        width: 100., height: 100.,
                        text: "Colored",
                        draw_slider: {
                            gap: 0.,
                            width: 20.
                            padding: 0.,
                        }
                    }
                    <RotaryFlat> {
                        width: 120., height: 120.,
                        text: "Solid",
                        draw_text: {
                            color: #0ff;
                        }
                        draw_slider: {
                            val_color_a: #ff0,
                            val_color_b: #f00,
                            handle_color: #f,
                            gap: 180.,
                            width: 20.,
                            padding: 2.,
                        }
                    }
                    <RotaryFlat> {
                        width: 120., height: 120.,
                        text: "Solid",
                        draw_slider: {
                            val_color_a: #0ff,
                            val_color_b: #ff0,
                            handle_color: #f,
                            gap: 90.,
                            width: 20.,
                            padding: 2.,
                        }
                    }
                    <RotaryFlat> {
                        width: 100., height: 90.,
                        text: "Solid",
                        draw_slider: {
                            gap: 90.,
                            padding: 10.,
                            width: 20.,
                            handle_color: #f0f,
                        }
                    }
                    <RotaryFlat> {
                        width: 150., height: 150.,
                        text: "Solid",
                        draw_slider: {
                            val_color_a: #0ff,
                            val_color_b: #0ff,
                            gap: 180.,
                            padding: 4.,
                            width: 6.,
                        }
                    }
                    <RotaryFlat> {
                        width: Fill, height: 150.,
                        text: "Solid",
                        draw_slider: {
                            val_color_a: #8;
                            val_color_b: #ff0;
                            gap: 75.,
                            width: 40.0,
                            padding: 4.,
                        }
                    }
                }

                <View> {
                    width: Fill, height: Fit,
                    flow: Right,
                    <RotarySolid> {
                        width: 100, height: 100,
                        text: "Colored",
                        draw_slider: {
                            gap: 90.,
                        }
                    }
                    <RotarySolid> {
                        width: 200, height: 150,
                        text: "Colored",
                        draw_slider: {
                            gap: 180.,
                        }
                    }
                    <RotarySolid> {
                        width: Fill, height: 150,
                        text: "Colored",
                        draw_slider: {
                            gap: 60.,
                        }
                    }
                }

                <ZooHeader> {
                    title = {text:"<Html>"}
                    <ZooDesc> {text:"HTML Widget"}
                    <ZooGroup> {
                        <Html> {
                            width:Fill, height:Fit,
                            body:"<H1>H1 Headline</H1><H2>H2 Headline</H2><H3>H3 Headline</H3><H4>H4 Headline</H4><H5>H5 Headline</H5><H6>H6 Headline</H6>This is <b>bold</b>&nbsp;and <i>italic text</i>.<sep><b><i>Bold italic</i></b>, <u>underlined</u>, and <s>strike through</s> text. <p>This is a paragraph</p> <code>A code block</code>. <br/> And this is a <a href='https://www.google.com/'>link</a><br/><ul><li>lorem</li><li>ipsum</li><li>dolor</li></ul><ol><li>lorem</li><li>ipsum</li><li>dolor</li></ol><br/> <blockquote>Blockquote</blockquote> <pre>pre</pre><sub>sub</sub><del>del</del>"
                        }
                    }
                }

                <ZooHeader> {
                    title = {text:"<Markdown>"}
                    <ZooDesc> {text:"Markdown"}
                    <ZooGroup> {
                        <Markdown> {
                            width:Fill, height: Fit,
                            body:"# Headline 1 \n ## Headline 2 \n ### Headline 3 \n #### Headline 4 \n This is standard text with a  \n\n line break a short ~~strike through~~ demo.\n\n *Italic text* \n\n **Bold text** \n\n - Bullet\n - Another bullet\n\n - Third bullet\n\n 1. Numbered list Bullet\n 2. Another list entry\n\n 3. Third list entry\n\n `Monospaced text`\n\n> This is a quote.\n\nThis is `inline code`.\n\n ```code block
                            ```"
                        }
                    }
                }

                <ZooHeader> {
                    title = {text:"<Image>"}
                    <ZooDesc> {text:"A static inline image from a resource."}
                    <ZooGroup> {
                        height: Fit, width: Fill,
                        spacing: (THEME_SPACE_2)
                        scroll_bars: <ScrollBars> {}
                        <View> {
                            width: Fit, height: Fit, flow: Down,
                            <View> {
                                show_bg: true, draw_bg: { color: (THEME_COLOR_BG_CONTAINER)}, width: 125, height: 250, flow: Down,
                                <Image> { source: dep("crate://self/resources/ducky.png" ) }
                            }
                            <P> { text: "Default" }
                        }

                        <View> {
                            width: Fit, height: Fit, flow: Down,
                            <View> {
                                show_bg: true, draw_bg: { color: (THEME_COLOR_BG_CONTAINER)}, width: 125, height: 250,
                                <Image> { width: Fill, height: Fill, source: dep("crate://self/resources/ducky.png"), fit: Stretch }
                            }
                            <P> { text: "fit: Stretch" }
                        }
                        <View> {
                            width: Fit, height: Fit, flow: Down,
                            <View> {
                                show_bg: true, draw_bg: { color: (THEME_COLOR_BG_CONTAINER)}, width: 125, height: 250,
                                <Image> { width: Fill, height: Fill, source: dep("crate://self/resources/ducky.png" ), fit: Horizontal }
                            }
                            <P> { text: "fit: Horizontal" }
                        }
                        <View> {
                            width: Fit, height: Fit, flow: Down,
                            <View> {
                                show_bg: true, draw_bg: { color: (THEME_COLOR_BG_CONTAINER)}, width: 125, height: 250,
                                <Image> { width: Fill, height: Fill, source: dep("crate://self/resources/ducky.png" ), fit: Vertical }
                            }
                            <P> { text: "fit: Vertical" }
                        }
                        <View> {
                            width: Fit, height: Fit, flow: Down,
                            <View> {
                                show_bg: true, draw_bg: { color: (THEME_COLOR_BG_CONTAINER)}, width: 50, height: 250,
                                <Image> { width: Fill, height: Fill, source: dep("crate://self/resources/ducky.png" ), fit: Smallest }
                            }
                            <P> { text: "fit: Smallest" }
                        }
                        <View> {
                            width: Fit, height: Fit, flow: Down,
                            <View> {
                                show_bg: true, draw_bg: { color: (THEME_COLOR_BG_CONTAINER)}, width: 125, height: 250,
                                <Image> { width: Fill, height: Fill, source: dep("crate://self/resources/ducky.png" ), fit: Biggest }
                            }
                            <P> { text: "fit: Biggest" }
                        }
                    }
                }

                <ZooHeader> {
                    title = {text:"<CheckBox>"}
                    <ZooDesc> {text:"Checkbox"}
                    <ZooGroup> {
                        height: Fit
                        spacing: (THEME_SPACE_2)
                        flow: Down,
                        <H4> { text: "Output demo"}
                        <View> {
                            height: Fit
                            flow: Right
                            align: { x: 0.0, y: 0.5}
                            simplecheckbox = <CheckBox> {text:"Check me out!"}
                            simplecheckbox_output = <Label> { text:"hmm" }
                        }
                        <H4> { text: "Standard Mode"}
                        <View> {
                            height: Fit
                            flow: Right
                            spacing: (THEME_SPACE_1)
                            align: { x: 0.0, y: 0.5}
                            <CheckBox> {text:"Check me out!"}
                            <CheckBox> {text:"Check me out!"}
                            <CheckBox> {text:"Check me out!"}
                        }
                        <H4> { text: "Toggle Mode"}
                        <View> {
                            height: Fit
                            flow: Right
                            spacing: (THEME_SPACE_1)
                            align: { x: 0.0, y: 0.5}
                            <CheckBoxToggle> {text:"Check me out!" }
                            <CheckBoxToggle> {text:"Check me out!" }
                            <CheckBoxToggle> {text:"Check me out!" }
                        }

                        <H4> { text: "Custom Icon Mode"}
                        <View> {
                            height: Fit
                            flow: Right
                            spacing: (THEME_SPACE_1)
                            align: { x: 0.0, y: 0.5}
                            <CheckBoxCustom> {
                                text:"Check me out!"
                                draw_check: { check_type: None }
                                draw_icon: {
                                    color_active: #f00,
                                    svg_file: dep("crate://self/resources/Icon_Favorite.svg"),
                                }
                            }
                            <CheckBoxCustom> {
                                text:"Check me out!"
                                draw_check: { check_type: None }
                                draw_icon: {
                                    svg_file: dep("crate://self/resources/Icon_Favorite.svg"),
                                }
                            }
                        }
                    }
                }

                <ZooHeader> {
                    title = {text:"<RadioButton>"}
                    <ZooDesc> {text:"Todo: List the different radio button templates."}
                    <ZooGroup> {
                        flow: Down,
                        spacing: (THEME_SPACE_2)
                        <H4> { text: "Default"}
                        <View> {
                            height: Fit
                            flow: Right
                            align: { x: 0.0, y: 0.5 }
                            radios_demo = <View> {
                                spacing: (THEME_SPACE_2)
                                width: Fit, height: Fit,
                                radio1 = <RadioButton> { text: "Option 1" }
                                radio2 = <RadioButton> { text: "Option 2" }
                                radio3 = <RadioButton> { text: "Option 3" }
                                radio4 = <RadioButton> { text: "Option 4" }
                            }
                        }

                        <H4> { text: "Custom Radios"}
                        <View> {
                            height: Fit
                            flow: Right
                            align: { x: 0.0, y: 0.5 }
                            iconradios_demo = <View> {
                                width: Fit, height: Fit,
                                spacing: (THEME_SPACE_2)
                                flow: Down,

                                radio1 = <RadioButtonCustom> {
                                    text: "Option 1"
                                    icon_walk: {
                                        width: 12.5, height: Fit,
                                    }
                                    draw_icon: {
                                        svg_file: dep("crate://self/resources/Icon_Favorite.svg"),
                                    }
                                }
                                radio2 = <RadioButtonCustom> {
                                    text: "Option 2"
                                    icon_walk: {
                                        width: 12.5, height: Fit,
                                    }
                                    draw_icon: {
                                        color_active: #0f0,
                                        svg_file: dep("crate://self/resources/Icon_Favorite.svg"),
                                    }
                                }
                                radio3 = <RadioButtonCustom> {
                                    text: "Option 3"
                                    icon_walk: {
                                        width: 12.5, height: Fit,
                                    }
                                    draw_icon: {
                                        color_active: #0ff,
                                        svg_file: dep("crate://self/resources/Icon_Favorite.svg"),
                                    }
                                }
                                radio4 = <RadioButtonCustom> {
                                    text: "Option 4"
                                    icon_walk: {
                                        width: 12.5, height: Fit,
                                    }
                                    draw_icon: {
                                        color_active: #f00,
                                        svg_file: dep("crate://self/resources/Icon_Favorite.svg"),
                                    }
                                }
                            }
                        }

                        <H4> { text: "Text only"}
                        <View> {
                            height: Fit
                            flow: Right
                            align: { x: 0.0, y: 0.5 }
                            textonlyradios_demo = <View> {
                                width: Fit, height: Fit,
                                flow: Right,
                                spacing: (THEME_SPACE_2)
                                radio1 = <RadioButtonTextual> { text: "Option 1" }
                                radio2 = <RadioButtonTextual> { text: "Option 2" }
                                radio3 = <RadioButtonTextual> { text: "Option 3" }
                                radio4 = <RadioButtonTextual> { text: "Option 4" }
                            }
                        }


                        <H4> { text: "Button Group"}
                        <ButtonGroup> {
                            height: Fit
                            flow: Right
                            align: { x: 0.0, y: 0.5 }
                            radiotabs_demo = <View> {
                                width: Fit, height: Fit,
                                radio1 = <RadioButtonTab> { text: "Option 1" }
                                radio2 = <RadioButtonTab> { text: "Option 2" }
                                radio3 = <RadioButtonTab> { text: "Option 3" }
                                radio4 = <RadioButtonTab> { text: "Option 4" }
                            }
                        }

                        <H4> { text: "Media"}
                        <View> {
                            height: Fit
                            flow: Right
                            align: { x: 0.0, y: 0.5 }
                            mediaradios_demo = <View> {
                                width: Fit, height: Fit,
                                flow: Right,
                                spacing: (THEME_SPACE_2)
                                radio1 = <RadioButtonImage> {
                                    width: 50, height: 50,
                                    media: Image,
                                    image: <Image> { source: dep("crate://self/resources/ducky.png" ) }
                                }
                                radio2 = <RadioButtonImage> {
                                    width: 50, height: 50,
                                    media: Image,
                                    image: <Image> { source: dep("crate://self/resources/ducky.png" ) }
                                }
                                radio3 = <RadioButtonImage> {
                                    width: 50, height: 50,
                                    media: Image,
                                    image: <Image> { source: dep("crate://self/resources/ducky.png" ) }
                                }
                                radio4 = <RadioButtonImage> {
                                    width: 50, height: 50,
                                    media: Image,
                                    image: <Image> { source: dep("crate://self/resources/ducky.png" ) }
                                }
                            }
                        }
                    }
                }

                <ZooHeader> {
                    title = {text:"<SlidesView>"}
                    width: Fill, height: Fit,
                    <ZooDesc> {text:"Slides View"}
                    <View> {
                        width: Fill, height: Fit,
                        <SlidesView> {
                            width: Fill, height: 400,

                            <SlideChapter> {
                                title = {text: "Hey!"},
                                <SlideBody> {text: "This is the 1st slide. Use your right\ncursor key to show the next slide."}
                            }

                            <Slide> {
                                title = {text: "Second slide"},
                                <SlideBody> {text: "This is the 2nd slide. Use your left\ncursor key to show the previous slide."}
                            }

                        }
                    }
                }

                <H4> { text: "Animation"}
                <Button> {
                // Allows instantiation of customly styled elements as i.e. <MyButton> {}.

                    // BUTTON SPECIFIC PROPERTIES

                    draw_bg: { // Shader object that draws the bg.
                            fn pixel(self) -> vec4 {
                            return mix( // State transition animations.
                                mix(
                                    #800,   
                                    mix(#800, #f, 0.5), // when hover 1
                                    self.hover
                                ),
                                #00f,   // blue when pressed
                                self.pressed
                            )
                        }
                    },

                    draw_icon: { // Shader object that draws the icon.
                        svg_file: dep("crate://self/resources/Icon_Favorite.svg"),
                        // Icon file dependency.

                        fn get_color(self) -> vec4 { // Overwrite the shader's fill method.
                            return mix( // State transition animations.
                                mix(
                                    #f0f,   // pink
                                    #fff,   // white
                                    self.hover
                                ),
                                #000,   // black
                                self.pressed
                            )
                        }
                    }

                    grab_key_focus: true, // Keyboard gets focus when clicked.

                    icon_walk: {
                        margin: 10.,
                        width: 16.,
                        height: Fit
                    }

                    label_walk: {
                        margin: 2.,
                        width: Fit,
                        height: Fit,
                    }

                    text: "I can be clicked", // Text label.

                    height: Fit,
                    // Element assumes the height of its children.

                    width: Fill,
                    // Element assumes the width of its children.

                    margin: 5.0
                    padding: { top: 3.0, right: 6.0, bottom: 3.0, left: 6.0 },
                    // Individual space between the element's border and its content
                    // for top and left.

                    flow: Right,
                    // Stacks children from left to right.

                    spacing: 5.0,
                    // A spacing of 10.0 between children.

                    align: { x: 0.5, y: 0.5 },
                    // Positions children at the left (x) bottom (y) corner of the parent.
                }


            }
        }
    }
}


// Define App struct containing UI and counter
#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef, // UI component reference
    #[rust] counter: usize  // use rust instead of live for counter
}

// Implement LiveRegister trait for registering live design
impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        // Register Makepad Widgets' live design
        makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for App{
    fn handle_actions(&mut self, cx: &mut Cx, actions:&Actions){
        let ui = self.ui.clone();

    
    ui.radio_button_set(ids!(radios_demo.radio1, radios_demo.radio2, radios_demo.radio3, radios_demo.radio4))
    .selected_to_visible(cx, &ui, actions, ids!(radios_demo.radio1, radios_demo.radio2, radios_demo.radio3, radios_demo.radio4));

    ui.radio_button_set(ids!(iconradios_demo.radio1, iconradios_demo.radio2, iconradios_demo.radio3, iconradios_demo.radio4))
        .selected_to_visible(cx, &ui, actions, ids!(iconradios_demo.radio1, iconradios_demo.radio2, iconradios_demo.radio3, iconradios_demo.radio4));

    ui.radio_button_set(ids!(radiotabs_demo.radio1, radiotabs_demo.radio2, radiotabs_demo.radio3, radiotabs_demo.radio4))
        .selected_to_visible(cx, &ui, actions, ids!(radiotabs_demo.radio1, radiotabs_demo.radio2, radiotabs_demo.radio3, radiotabs_demo.radio4));

    ui.radio_button_set(ids!(textonlyradios_demo.radio1, textonlyradios_demo.radio2, textonlyradios_demo.radio3, textonlyradios_demo.radio4))
        .selected_to_visible(cx, &ui, actions, ids!(textonlyradios_demo.radio1, textonlyradios_demo.radio2, textonlyradios_demo.radio3, textonlyradios_demo.radio4));

    ui.radio_button_set(ids!(mediaradios_demo.radio1, mediaradios_demo.radio2, mediaradios_demo.radio3, mediaradios_demo.radio4))
        .selected_to_visible(cx, &ui, actions, ids!(mediaradios_demo.radio1, mediaradios_demo.radio2, mediaradios_demo.radio3, mediaradios_demo.radio4));

    if let Some(txt) = self.ui.text_input(id!(simpletextinput)).changed(&actions){  // when text input changes
        log!("TEXTBOX CHANGED {}", self.counter);   // output to console
        self.counter += 1;
        let lbl = self.ui.label(id!(simpletextinput_outputbox));
        lbl.set_text(cx,&format!("{} {}" , self.counter, txt));
    }

    if self.ui.button(id!(basicbutton)).clicked(&actions) {
        log!("BASIC BUTTON CLICKED {}", self.counter);
        self.counter += 1;
        let btn = self.ui.button(id!(basicbutton));
        btn.set_text(cx,&format!("Clicky clicky! {}", self.counter));
    }

    if self.ui.button(id!(iconbutton)).clicked(&actions) {
        log!("ICON BUTTON CLICKED {}", self.counter);
        self.counter += 1;
        let btn = self.ui.button(id!(iconbutton));
        btn.set_text(cx,&format!("Icon button clicked: {}", self.counter));
    }

    if let Some(check) = self.ui.check_box(id!(simplecheckbox)).changed(actions) {
        log!("CHECK BUTTON CLICKED {} {}", self.counter, check);
        self.counter += 1;
        let lbl = self.ui.label(id!(simplecheckbox_output));
        lbl.set_text(cx,&format!("{} {}" , self.counter, check));
    }
}
}

// Implement AppMain trait for handling events
impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);    
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

// Define application entry point
app_main!(App);