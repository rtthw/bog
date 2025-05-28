


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
        let mut theme = Theme::default();
        let style = StyleClass::new(&mut theme, Styling {
            text_height: Some(Unit::Rem(2.0)),
            text_slant: Some(TextSlant::Italic),
            ..Default::default()
        });

        Model::new(
            Element::new()
                .layout(Layout::default()
                    .align_items_center()
                    .justify_content_center())
                .child(static_paragraph(
                    "Hello, World!",
                    Layout::default(),
                )),
            layout_map,
            Theme::default(),
        )
    }
}
