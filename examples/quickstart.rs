


use bog::prelude::*;



fn main() -> Result<()> {
    run_app(QuickstartApp)
}

struct QuickstartApp;

impl AppHandler for QuickstartApp {
    fn window_desc(&self) -> WindowDescriptor {
        WindowDescriptor {
            title: "Quickstart",
            ..Default::default()
        }
    }
}

impl View for QuickstartApp {
    fn build(&mut self, layout_map: &mut LayoutMap) -> Model<Self> {
        Model::new(
            Element::new()
                .layout(Layout::default()
                    .align_items_center()
                    .justify_content_center())
                .child(static_paragraph(
                    Text {
                        content: "Hello, World!",
                        size: 40.0,
                        color: Color::new(163, 163, 173, 255),
                        line_height: 45.0,
                        font_family: FontFamily::SansSerif,
                        text_slant: TextSlant::Normal,
                        ..Default::default()
                    },
                    Layout::default(),
                )),
            layout_map,
        )
    }
}
