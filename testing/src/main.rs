


use bog::*;
// use fonts::*;
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
    let graphics = futures::executor::block_on(async {
        WindowGraphics::from_window(&window).await
    })?;

    // let font = load_font_face(include_bytes!("../data/JetBrainsMonoNerdFont_Regular.ttf"))
    //     .unwrap();
    // let parsed_font = font.parse().unwrap();

    let mut painter = Painter::new(&graphics);
    let mut gui = Gui::new(Layout::default()
        .flex_row()
        .flex_wrap()
        .fill_width()
        .fill_height()
        .gap_x(10.0)
        .gap_y(5.0)
        .align_content_center()
        .align_items_center());
    gui.push_element_to_root(0, Layout::default()
        .width(70.0)
        .height(50.0));
    gui.push_element_to_root(1, Layout::default()
        .width(100.0)
        .height(30.0));
    gui.push_element_to_root(2, Layout::default()
        .width(50.0)
        .height(70.0));
    gui.push_element_to_root(3, Layout::default()
        .width(40.0)
        .height(10.0));
    gui.push_element_to_root(4, Layout::default()
        .width(20.0)
        .height(20.0));
    let mut app = App {
        graphics,
        paints: vec![
            PaintMesh::quad(vec2(0.0, 0.0), vec2(0.0, 0.0), 0xaaaaabff),
            PaintMesh::quad(vec2(0.0, 0.0), vec2(0.0, 0.0), 0xaaaaabff),
            PaintMesh::quad(vec2(0.0, 0.0), vec2(0.0, 0.0), 0xaaaaabff),
            PaintMesh::quad(vec2(0.0, 0.0), vec2(0.0, 0.0), 0xaaaaabff),
            PaintMesh::quad(vec2(0.0, 0.0), vec2(0.0, 0.0), 0xaaaaabff),
        ],
    };

    event_loop.run(move |event, control_flow| {
        match event {
            WindowManagerEvent::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    control_flow.exit();
                }
                WindowEvent::Resized(new_size) => {
                    app.graphics.window().request_redraw();
                    if new_size.width > 0 && new_size.height > 0 {
                        let size = vec2(new_size.width as _, new_size.height as _);
                        app.graphics.resize(size);
                        gui.handle_resize(&mut app, size);
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let pos = vec2(position.x as _, position.y as _);
                    gui.handle_mouse_move(&mut app, pos);
                }
                WindowEvent::MouseInput { button: MouseButton::Left, state, .. } => {
                    if state.is_pressed() {
                        gui.handle_mouse_down(&mut app);
                    } else {
                        gui.handle_mouse_up(&mut app);
                    }
                }
                WindowEvent::RedrawRequested => {
                    app.graphics
                        .render(|render_pass| {
                            painter.prepare(&app.graphics, &app.paints);
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



struct App<'w> {
    graphics: WindowGraphics<'w>,
    paints: Vec<PaintMesh>,
}

impl<'w> GuiHandler for App<'w> {
    type Element = usize;

    fn on_mouse_move(&mut self, _pos: math::Vec2) {}

    fn on_mouse_enter(&mut self, element: &mut Self::Element) {
        self.graphics.window().request_redraw();
        self.graphics.window().set_cursor_icon(CursorIcon::Pointer);
        self.paints[*element].change_color(0xb7b7c0ff);
    }

    fn on_mouse_leave(&mut self, element: &mut Self::Element) {
        self.graphics.window().request_redraw();
        self.graphics.window().set_cursor_icon(CursorIcon::Default);
        self.paints[*element].change_color(0xaaaaabff);
    }

    fn on_mouse_down(&mut self, element: &mut Self::Element) {
        self.graphics.window().request_redraw();
        self.paints[*element].change_color(0x3c3c44ff);
    }

    fn on_mouse_up(&mut self, element: &mut Self::Element) {
        self.graphics.window().request_redraw();
        self.paints[*element].change_color(0xb7b7c0ff);
        println!("Element #{element} clicked");
    }

    fn on_drag_update(&mut self, element: &mut Self::Element, hovered: Option<LayoutNode>, delta: Vec2) {
    }

    fn on_drag_start(&mut self, element: &mut Self::Element) {
        self.graphics.window().request_redraw();
        self.graphics.window().set_cursor_icon(CursorIcon::Grab);
    }

    fn on_drag_end(&mut self, element: &mut Self::Element) {
        self.graphics.window().request_redraw();
        self.graphics.window().set_cursor_icon(CursorIcon::Default);
    }

    fn on_resize(&mut self, _size: math::Vec2) {}

    fn on_element_layout(&mut self, element: &mut Self::Element, placement: &Placement) {
        self.paints[*element] = Rectangle {
            pos: placement.position(),
            size: vec2(placement.layout.size.width, placement.layout.size.height),
            color: 0xaaaaabff,
            corner_radii: [3.0, 3.0, 3.0, 3.0],
        }.to_mesh();
    }
}
