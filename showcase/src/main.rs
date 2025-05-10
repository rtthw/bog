



use bog::*;

use app::*;
use color::*;
use layout::*;
use math::*;
use render::*;
use window::*;



pub const GRAY_0: Color = Color::new(13, 13, 23, 255); // 0d0d17
pub const GRAY_1: Color = Color::new(29, 29, 39, 255); // 1d1d27
pub const GRAY_2: Color = Color::new(43, 43, 53, 255); // 2b2b35
pub const GRAY_3: Color = Color::new(59, 59, 67, 255); // 3b3b43
pub const GRAY_4: Color = Color::new(73, 73, 83, 255); // 494953
pub const GRAY_5: Color = Color::new(89, 89, 109, 255); // 59596d
pub const GRAY_6: Color = Color::new(113, 113, 127, 255); // 71717f
pub const GRAY_7: Color = Color::new(139, 139, 149, 255); // 8b8b95
pub const GRAY_8: Color = Color::new(163, 163, 173, 255); // a3a3ad
pub const GRAY_9: Color = Color::new(191, 191, 197, 255); // bfbfc5



fn main() -> Result<()> {
    run_app(Showcase {
        drag_indicator: None,
    })?;

    Ok(())
}



struct Showcase {
    drag_indicator: Option<Quad>,
}

impl AppHandler for Showcase {
    fn render(
        &mut self,
        view: &mut View,
        renderer: &mut Renderer,
        root_placement: Placement<'_>,
        viewport_rect: Rect,
    ) {
        renderer.clear();

        { // Background layer.
            renderer.start_layer(viewport_rect);
            renderer.fill_quad(Quad {
                bounds: viewport_rect,
                bg_color: GRAY_0,
                ..Default::default()
            });
            renderer.end_layer();
        }

        { // Main layer.
            renderer.start_layer(viewport_rect);
            for placement in root_placement.children() {
                renderer.fill_quad(Quad {
                    bounds: placement.rect(),
                    bg_color: GRAY_3,
                    ..Default::default()
                });
            }
            renderer.end_layer();
        }

        // Overlay layer.
        if let Some(drag_indicator) = &self.drag_indicator {
            renderer.start_layer(viewport_rect);
            renderer.fill_quad(*drag_indicator);
            renderer.end_layer();
        }
    }

    fn view(&mut self) -> Element {
        Element::new()
            .layout(Layout::default()
                .width(1280.0)
                .height(720.0)
                .gap_x(11.0)
                .padding(11.0))
            .child(Element::new()
                .on_render(|renderer, placement| {
                    renderer.fill_quad(Quad {
                        bounds: placement.rect(),
                        bg_color: GRAY_2,
                        ..Default::default()
                    });
                })
                .on_mouse_enter(|_obj, _app| {
                    println!("Mouse entered left panel!");
                })
                .on_mouse_leave(|_obj, _app| {
                    println!("Mouse left left panel!");
                })
                .layout(Layout::default()
                    .flex_initial()
                    .width(300.0)
                    .padding(7.0)))
            .child(Element::new()
                .on_render(|renderer, placement| {
                    renderer.fill_quad(Quad {
                        bounds: placement.rect(),
                        bg_color: GRAY_3,
                        ..Default::default()
                    });
                })
                .layout(Layout::default()
                    .flex_auto()
                    .flex_wrap()
                    .gap_x(11.0)
                    .padding(7.0)
                    .align_items_center()
                    .justify_content_center()))
    }

    fn window_desc(&self) -> WindowDescriptor {
        WindowDescriptor {
            title: "Bog Showcase",
            inner_size: Vec2::new(1280.0, 720.0),
            ..Default::default()
        }
    }
}
