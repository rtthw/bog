


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
        let style = StyleClass::base(&mut theme, Styling {
            fg_color: Some(Color::new(139, 139, 149, 255)),
            bg_color: Some(Color::new(43, 43, 53, 255)),
            border_width: Some(Unit::Px(0.0)),
            border_radius: Some(BorderRadius::Uniform(0.0)),
            text_height: Some(Unit::Px(40.0)),
            text_slant: Some(TextSlant::Italic),
            ..Default::default()
        });
        style.with_hover(&mut theme, Styling {
            fg_color: Some(Color::new(163, 163, 173, 255)),
            ..Default::default()
        });

        Model::new(
            Element::new()
                .child(panel()
                    .style(style)
                    .layout(Layout::default()
                        .flex_auto()
                        .align_items_center()
                        .justify_content_center())
                    .child(
                        static_label("Hello, world!").style(style)
                    )),
            layout_map,
            theme,
        )
    }
}
