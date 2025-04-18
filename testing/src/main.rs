


use std::collections::HashMap;

use bog::*;
use fonts::*;
use graphics::*;
use gui::*;
use layout::*;
use math::*;
use painter::*;
use window::*;



fn main() -> Result<()> {
    let window_system = WindowingSystem::new()?;
    let mut app = App { window: None, display: None, gui: None };

    window_system.run_client(&mut app)?;

    Ok(())
}



struct App<'w> {
    window: Option<Window>,
    display: Option<Display<'w>>,
    gui: Option<Gui>,
}

impl<'w> Client for App<'w> {
    fn on_resume(&mut self, wm: WindowManager) {
        if self.window.is_none() {
            for monitor in wm.available_monitors() {
                println!(
                    "[bog] INFO: MONITOR FOUND: \"{}\"",
                    monitor.name().unwrap_or(String::new()),
                );
                let size = monitor.size();
                println!("\t... SIZE: {}x{}px", size.width, size.height);
                let pos = monitor.position();
                println!("\t... POSITION: ({}, {})", pos.x, pos.y);
                if let Some(mhz) = monitor.refresh_rate_millihertz() {
                    println!("\t... REFRESH RATE: {} mHz", mhz);
                } else {
                    println!("\t... UNKNOWN REFRESH RATE");
                }
            }

            self.window = Some(wm.create_window(WindowDescriptor {
                title: "Bog Testing",
                ..Default::default()
            }).unwrap());
        }
        if self.display.is_none() && self.window.is_some() {
            let graphics = futures::executor::block_on(async {
                WindowGraphics::from_window(self.window.clone().unwrap()).await
            }).unwrap();

            let font = load_font_face(include_bytes!("../data/JetBrainsMonoNerdFont_Regular.ttf"))
                .unwrap();
            let parsed_font = font.parse().unwrap();
            let indicator_glyph_id = parsed_font.char_glyph('A').unwrap();
            let indicator_glyph_mesh = parsed_font.glyph_mesh(indicator_glyph_id, 20.0).unwrap();

            let painter = Painter::new(&graphics);
            let mut gui = Gui::new(Layout::default()
                .flex_row()
                .flex_wrap()
                .fill_width()
                .fill_height()
                .gap_x(10.0)
                .gap_y(5.0)
                .align_content_center()
                .align_items_center());
            let mut elements = HashMap::with_capacity(5);
            let mut paints = Vec::with_capacity(5);
            paints.push(PaintMesh::glyph(indicator_glyph_mesh, 0xaaaaabff)); // Dragging indicator.
            for (index, layout) in [
                Layout::default().width(70.0).height(50.0),
                Layout::default().width(100.0).height(30.0),
                Layout::default().width(50.0).height(70.0),
                Layout::default().width(40.0).height(70.0),
                Layout::default().width(20.0).height(40.0),
                ].into_iter().enumerate()
            {
                let element = gui.push_element_to_root(layout);
                elements.insert(element, index + 1); // Index 0 reserved for dragging indicator.
                paints.push(PaintMesh::quad(vec2(0.0, 0.0), vec2(0.0, 0.0), 0xaaaaabff));
            }

            self.display = Some(Display {
                graphics,
                painter,
                paints,
                elements,
            });
            self.gui = Some(gui);
        }
    }

    fn on_window_event(&mut self, wm: WindowManager, _id: WindowId, event: WindowEvent) {
        let Some(display) = &mut self.display else { return; };
        let Some(gui) = &mut self.gui else { return; };

        match event {
            WindowEvent::CloseRequested => {
                wm.exit();
            }
            WindowEvent::Resized(new_size) => {
                display.graphics.window().request_redraw();
                if new_size.width > 0 && new_size.height > 0 {
                    let size = vec2(new_size.width as _, new_size.height as _);
                    display.graphics.resize(size);
                    gui.handle_resize(display, size);
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let pos = vec2(position.x as _, position.y as _);
                gui.handle_mouse_move(display, pos);
            }
            WindowEvent::MouseInput { button: MouseButton::Left, state, .. } => {
                if state.is_pressed() {
                    gui.handle_mouse_down(display);
                } else {
                    gui.handle_mouse_up(display);
                }
            }
            WindowEvent::RedrawRequested => {
                display.graphics
                    .render(|render_pass| {
                        display.painter.prepare(&display.graphics, &display.paints);
                        display.painter.render(render_pass, &display.paints);
                    })
                    .unwrap();
            }
            _ => {}
        }
    }
}



struct Display<'w> {
    graphics: WindowGraphics<'w>,
    painter: Painter,
    paints: Vec<PaintMesh>,
    elements: HashMap<Element, usize>,
}

impl<'w> GuiHandler for Display<'w> {
    fn on_mouse_move(&mut self, _pos: math::Vec2) {}

    fn on_mouse_enter(&mut self, element: Element, state: &GuiState) {
        self.graphics.window().request_redraw();
        if !state.is_dragging {
            self.graphics.window().set_cursor(CursorIcon::Pointer);
        }
        let Some(index) = self.elements.get(&element) else { return; };
        self.paints[*index].change_color(0xb7b7c0ff);
    }

    fn on_mouse_leave(&mut self, element: Element, state: &GuiState) {
        self.graphics.window().request_redraw();
        if !state.is_dragging {
            self.graphics.window().set_cursor(CursorIcon::Default);
        }
        let Some(index) = self.elements.get(&element) else { return; };
        self.paints[*index].change_color(0xaaaaabff);
    }

    fn on_mouse_down(&mut self, element: Element, _state: &GuiState) {
        self.graphics.window().request_redraw();
        let Some(index) = self.elements.get(&element) else { return; };
        self.paints[*index].change_color(0x3c3c44ff);
    }

    fn on_mouse_up(&mut self, element: Element, _state: &GuiState) {
        self.graphics.window().request_redraw();
        let Some(index) = self.elements.get(&element) else { return; };
        self.paints[*index].change_color(0xb7b7c0ff);
        println!("Element #{index} clicked");
    }

    fn on_drag_update(
        &mut self,
        tree: &mut LayoutTree,
        _element: Element,
        hovered: Option<Element>,
        _delta: Vec2,
    ) {
        if let Some(placement) = hovered.and_then(|e| tree.placement(e)) {
            let pos = vec2(
                (placement.position().x + placement.layout.size.width / 2.0) - 5.0,
                placement.position().y + placement.layout.size.height + 5.0,
            );
            self.paints[0] = Rectangle {
                pos,
                size: vec2(10.0, 10.0),
                color: 0xaaaaabff,
                corner_radii: [2.0, 2.0, 2.0, 2.0],
            }.to_mesh();
        }
    }

    fn on_drag_start(&mut self, element: Element, tree: &mut LayoutTree) {
        self.graphics.window().request_redraw();
        self.graphics.window().set_cursor(CursorIcon::Grab);
        if let Some(placement) = tree.placement(element) {
            let pos = vec2(
                (placement.position().x + placement.layout.size.width / 2.0) - 5.0,
                placement.position().y + placement.layout.size.height + 5.0,
            );
            self.paints[0] = Rectangle {
                pos,
                size: vec2(10.0, 10.0),
                color: 0xaaaaabff,
                corner_radii: [2.0, 2.0, 2.0, 2.0],
            }.to_mesh();
        }
    }

    fn on_drag_end(&mut self, _element: Element) {
        self.graphics.window().request_redraw();
        self.graphics.window().set_cursor(CursorIcon::Default);
        self.paints[0] = PaintMesh::quad(vec2(-1.0, -1.0), vec2(1.0, 1.0), 0x00000000);
    }

    fn on_resize(&mut self, _size: math::Vec2) {}

    fn on_element_layout(&mut self, element: Element, placement: &Placement) {
        let Some(index) = self.elements.get(&element) else { return; };
        self.paints[*index] = Rectangle {
            pos: placement.position(),
            size: vec2(placement.layout.size.width, placement.layout.size.height),
            color: 0xaaaaabff,
            corner_radii: [3.0, 3.0, 3.0, 3.0],
        }.to_mesh();
    }
}
