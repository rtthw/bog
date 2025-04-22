


use std::collections::HashMap;

use bog::*;
use color::*;
use event::*;
use fonts::*;
use graphics::*;
use gui::*;
use layout::*;
use math::*;
use render::*;
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
            let (graphics, device, queue, format) = futures::executor::block_on(async {
                WindowGraphics::from_window(self.window.clone().unwrap()).await
            }).unwrap();

            let renderer = Renderer::new(device, queue, format);
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
            for layout in [
                Layout::default().width(70.0).height(50.0),
                Layout::default().width(100.0).height(30.0),
                Layout::default().width(50.0).height(70.0),
                Layout::default().width(40.0).height(70.0),
                Layout::default().width(20.0).height(40.0),
            ] {
                let element = gui.push_element_to_root(layout);
                elements.insert(element, Button {
                    quad: Quad {
                        bounds: Rect::new(Vec2::ZERO, Vec2::ZERO),
                        border: Border {
                            color: Color::from_u32(0xb7b7c0ff),
                            width: 3.0,
                            radius: [7.0, 3.0, 11.0, 19.0],
                        },
                        shadow: Shadow {
                            color: Color::from_u32(0x3c3c44ff),
                            offset: vec2(2.0, 5.0),
                            blur_radius: 3.0,
                        },
                        bg_color: Color::from_u32(0xaaaaabff),
                    }
                });
            }

            self.display = Some(Display {
                graphics,
                renderer,
                viewport: Viewport {
                    physical_size: vec2(1.0, 1.0),
                    logical_size: vec2(1.0, 1.0),
                    scale_factor: 1.0,
                    projection: Mat4::IDENTITY,
                },
                elements,
                drag_indicator: None,
            });
            self.gui = Some(gui);
        }
    }

    fn on_event(&mut self, wm: WindowManager, _id: WindowId, event: WindowEvent) {
        let Some(display) = &mut self.display else { return; };
        let Some(gui) = &mut self.gui else { return; };

        match event {
            WindowEvent::Resize { width, height } => {
                display.graphics.window().request_redraw();
                if width > 0 && height > 0 {
                    let physical_size = vec2(width as _, height as _);
                    display.viewport.scale_factor = display.graphics.window().scale_factor();

                    display.graphics.resize(display.renderer.device(), physical_size);
                    gui.handle_resize(display, physical_size);
                }
            }
            // WindowEvent::KeyDown { code, repeat } => {}
            // WindowEvent::KeyUp { code } => {}
            WindowEvent::MouseMove { x, y } => {
                gui.handle_mouse_move(display, vec2(x, y));
            }
            WindowEvent::MouseDown { code } => {
                if code == 0 {
                    gui.handle_mouse_down(display);
                }
            }
            WindowEvent::MouseUp { code } => {
                if code == 0 {
                    gui.handle_mouse_up(display);
                }
            }
            WindowEvent::CloseRequest => {
                wm.exit();
            }
            WindowEvent::RedrawRequest => {
                display.renderer.start_layer(
                    Rect::new(Vec2::ZERO, display.viewport.physical_size),
                );
                for button in display.elements.values() {
                    display.renderer.fill_quad(button.quad);
                }
                display.renderer.end_layer();

                let texture = display.graphics.get_current_texture();
                let target = texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
                display.renderer.render(&target, &display.viewport);
                texture.present();
            }
            _ => {}
        }
    }
}



struct Display<'w> {
    graphics: WindowGraphics<'w>,
    renderer: Renderer,
    viewport: Viewport,
    elements: HashMap<Element, Button>,
    drag_indicator: Option<Quad>,
}

impl<'w> GuiHandler for Display<'w> {
    fn on_mouse_move(&mut self, _pos: math::Vec2) {}

    fn on_mouse_enter(&mut self, element: Element, state: &GuiState) {
        self.graphics.window().request_redraw();
        if !state.is_dragging {
            self.graphics.window().set_cursor(CursorIcon::Pointer);
        }
        let Some(button) = self.elements.get_mut(&element) else { return; };
        button.quad.bg_color = Color::from_u32(0xb7b7c0ff);
    }

    fn on_mouse_leave(&mut self, element: Element, state: &GuiState) {
        self.graphics.window().request_redraw();
        if !state.is_dragging {
            self.graphics.window().set_cursor(CursorIcon::Default);
        }
        let Some(button) = self.elements.get_mut(&element) else { return; };
        button.quad.bg_color = Color::from_u32(0xaaaaabff);
    }

    fn on_mouse_down(&mut self, element: Element, _state: &GuiState) {
        self.graphics.window().request_redraw();
        let Some(button) = self.elements.get_mut(&element) else { return; };
        button.quad.bg_color = Color::from_u32(0x3c3c44ff);
    }

    fn on_mouse_up(&mut self, element: Element, _state: &GuiState) {
        self.graphics.window().request_redraw();
        let Some(button) = self.elements.get_mut(&element) else { return; };
        button.quad.bg_color = Color::from_u32(0xb7b7c0ff);
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
            self.drag_indicator = Some(Quad {
                bounds: Rect::new(pos, vec2(10.0, 10.0)),
                border: Border {
                    color: Color::from_u32(0xb7b7c0ff),
                    width: 3.0,
                    radius: [7.0, 3.0, 11.0, 19.0],
                },
                shadow: Shadow {
                    color: Color::from_u32(0x3c3c44ff),
                    offset: vec2(2.0, 5.0),
                    blur_radius: 3.0,
                },
                bg_color: Color::from_u32(0xaaaaabff),
            });
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
            self.drag_indicator = Some(Quad {
                bounds: Rect::new(pos, vec2(10.0, 10.0)),
                border: Border {
                    color: Color::from_u32(0xb7b7c0ff),
                    width: 3.0,
                    radius: [7.0, 3.0, 11.0, 19.0],
                },
                shadow: Shadow {
                    color: Color::from_u32(0x3c3c44ff),
                    offset: vec2(2.0, 5.0),
                    blur_radius: 3.0,
                },
                bg_color: Color::from_u32(0xaaaaabff),
            });
        }
    }

    fn on_drag_end(&mut self, _element: Element) {
        self.graphics.window().request_redraw();
        self.graphics.window().set_cursor(CursorIcon::Default);
        self.drag_indicator = None;
    }

    fn on_resize(&mut self, _size: math::Vec2) {}

    fn on_element_layout(&mut self, element: Element, placement: &Placement) {
        let Some(button) = self.elements.get_mut(&element) else { return; };
        button.quad.bounds = Rect::new(
            placement.position(),
            vec2(placement.layout.size.width, placement.layout.size.height),
        );
    }
}



struct Button {
    quad: Quad,
}
