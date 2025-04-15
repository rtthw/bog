


use bog::*;
use fonts::*;
use graphics::*;
use gui::*;
use layout::*;
use math::*;
use painter::*;
use window::*;



fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Bog Testing")
        .with_inner_size(dpi::LogicalSize::new(1200.0, 800.0))
        .build(&event_loop)?;
    let mut graphics = futures::executor::block_on(async {
        WindowGraphics::from_window(&window).await
    })?;

    let font = load_font_face(include_bytes!("../data/JetBrainsMonoNerdFont_Regular.ttf"))
        .unwrap();
    let parsed_font = font.parse().unwrap();
    let test_glyph_id = parsed_font.char_glyph('g').unwrap();
    let mut test_glyph_mesh = parsed_font.glyph_mesh(test_glyph_id, 60.0).unwrap();
    for v in test_glyph_mesh.vertices.iter_mut() {
        v.x += 100.0;
        v.y += 400.0;
    }

    let mut painter = Painter::new(&graphics);
    let mut gui = Gui::new(Layout::default()
        .fill_width()
        .fill_height()
        .align_content_center()
        .align_items_center());
    gui.push_element_to_root(Element::One, Layout::default()
        .width(70.0)
        .height(50.0));
    gui.push_element_to_root(Element::Two, Layout::default()
        .width(100.0)
        .height(30.0));
    let mut app = App {
        paints: vec![
            Rectangle {
                pos: vec2(0.0, 0.0),
                size: vec2(100.0, 50.0),
                color: 0xaaaaabff,
                corner_radii: [7.0, 19.0, 1.0, 45.0],
            }.to_mesh(),
            Rectangle {
                pos: vec2(0.0, 0.0),
                size: vec2(100.0, 50.0),
                color: 0xaaaaabff,
                corner_radii: [7.0, 19.0, 1.0, 45.0],
            }.to_mesh(),
        ],
    };

    event_loop.run(move |event, control_flow| {
        match event {
            WindowManagerEvent::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    control_flow.exit();
                }
                WindowEvent::Resized(new_size) => {
                    graphics.window().request_redraw();
                    if new_size.width > 0 && new_size.height > 0 {
                        let size = vec2(new_size.width as _, new_size.height as _);
                        graphics.resize(size);
                        gui.handle_resize(&mut app, size);
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let pos = vec2(position.x as _, position.y as _);
                    gui.handle_mouse_move(&mut app, pos);
                }
                WindowEvent::RedrawRequested => {
                    graphics
                        .render(|render_pass| {
                            painter.prepare(&graphics, &app.paints);
                            painter.render(render_pass, &app.paints);
                        })
                        .unwrap();
                }
                _ => {}
            }
            _ => {}
        }
    })?;

    Ok(())
}



struct App {
    paints: Vec<PaintMesh>,
}

impl GuiHandler for App {
    type Element = Element;

    fn on_mouse_move(&mut self, _pos: math::Vec2) {}

    fn on_mouse_enter(&mut self, element: &mut Self::Element) {
        match element {
            Element::One => {
                println!("One hovered");
            }
            Element::Two => {
                println!("Two hovered");
            }
        }
    }

    fn on_mouse_leave(&mut self, element: &mut Self::Element) {
        match element {
            Element::One => {
                println!("One un-hovered");
            }
            Element::Two => {
                println!("Two un-hovered");
            }
        }
    }

    fn on_resize(&mut self, _size: math::Vec2) {}

    fn on_element_layout(&mut self, element: &mut Self::Element, placement: &layout::Placement) {
        match element {
            Element::One => {
                self.paints[0] = Rectangle {
                    pos: placement.position(),
                    size: vec2(placement.layout.size.width, placement.layout.size.height),
                    color: 0xaaaaabff,
                    corner_radii: [3.0, 3.0, 3.0, 3.0],
                }.to_mesh();
            }
            Element::Two => {
                self.paints[1] = Rectangle {
                    pos: placement.position(),
                    size: vec2(placement.layout.size.width, placement.layout.size.height),
                    color: 0xaaaaabff,
                    corner_radii: [3.0, 3.0, 3.0, 3.0],
                }.to_mesh();
            }
        }
    }
}

enum Element {
    One,
    Two,
}
