


use std::collections::HashMap;

use bog::*;
use color::*;
use event::*;
use graphics::*;
use gui::*;
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
                title: "Bog Showcase",
                ..Default::default()
            }).unwrap());
        }

        if self.display.is_none() && self.window.is_some() {
            let (graphics, device, queue, format) = futures::executor::block_on(async {
                WindowGraphics::from_window(self.window.clone().unwrap()).await
            }).unwrap();

            let mut renderer = Renderer::new(device, queue, format);
            renderer.load_font(
                include_bytes!("../data/JetBrainsMonoNerdFont_Regular.ttf").to_vec(),
            );
            let mut gui = Gui::new(Layout::default()
                .fill_width()
                .fill_height()
                .gap_x(23.0)
                .padding(11.0));

            let left_panel_layout = Layout::default()
                .width_percent(0.2)
                .fill_height();
            let right_panel_layout = Layout::default()
                .flex_row()
                .flex_wrap()
                .fill_width()
                .fill_height()
                .gap_x(11.0)
                .align_items_center()
                .justify_content_center();

            let left_panel = gui.push_element_to_root(left_panel_layout);
            let right_panel = gui.push_element_to_root(right_panel_layout);

            let mut elements = HashMap::with_capacity(7);
            elements.insert(left_panel, Button {
                quad: Quad {
                    bounds: Rect::new(Vec2::ZERO, vec2(10.0, 10.0)),
                    border: Border {
                        color: GRAY_5,
                        width: 3.0,
                        radius: [7.0, 3.0, 11.0, 19.0],
                    },
                    shadow: Shadow {
                        color: GRAY_3,
                        offset: vec2(2.0, 5.0),
                        blur_radius: 3.0,
                    },
                    bg_color: GRAY_2,
                },
                text: Text {
                    content: "LEFT".to_string(),
                    pos: Vec2::ZERO,
                    size: 50.0,
                    color: GRAY_8,
                    line_height: 50.0 * 1.2,
                    font_family: FontFamily::Name("JetBrainsMono Nerd Font"),
                    font_style: FontStyle::Italic,
                    bounds: Vec2::new(100.0, 100.0),
                },
                draggable: false,
            });
            elements.insert(right_panel, Button {
                quad: Quad {
                    bounds: Rect::new(Vec2::ZERO, vec2(10.0, 10.0)),
                    border: Border {
                        color: GRAY_5,
                        width: 3.0,
                        radius: [7.0, 3.0, 11.0, 19.0],
                    },
                    shadow: Shadow {
                        color: GRAY_3,
                        offset: vec2(2.0, 5.0),
                        blur_radius: 3.0,
                    },
                    bg_color: GRAY_2,
                },
                text: Text {
                    content: "LEFT".to_string(),
                    pos: Vec2::ZERO,
                    size: 50.0,
                    color: GRAY_8,
                    line_height: 50.0 * 1.2,
                    font_family: FontFamily::Name("JetBrainsMono Nerd Font"),
                    font_style: FontStyle::Italic,
                    bounds: Vec2::new(100.0, 100.0),
                },
                draggable: false,
            });
            for layout in [
                Layout::default().width(70.0).height(50.0).padding(5.0),
                Layout::default().width(100.0).height(30.0).padding(5.0),
                Layout::default().width(50.0).height(70.0).padding(5.0),
                Layout::default().width(40.0).height(70.0).padding(5.0),
                Layout::default().width(20.0).height(40.0).padding(5.0),
            ] {
                let element = gui.push_element(right_panel, layout);
                elements.insert(element, Button {
                    quad: Quad {
                        bounds: Rect::new(Vec2::ZERO, vec2(10.0, 10.0)),
                        border: Border {
                            color: GRAY_5,
                            width: 3.0,
                            radius: [7.0, 3.0, 11.0, 19.0],
                        },
                        shadow: Shadow {
                            color: GRAY_3,
                            offset: vec2(2.0, 5.0),
                            blur_radius: 3.0,
                        },
                        bg_color: GRAY_6,
                    },
                    text: Text {
                        content: "=>".to_string(),
                        pos: Vec2::ZERO,
                        size: 20.0,
                        color: GRAY_6,
                        line_height: 20.0 * 1.2,
                        font_family: FontFamily::Name("JetBrainsMono Nerd Font"),
                        font_style: FontStyle::Normal,
                        bounds: Vec2::new(100.0, 100.0),
                    },
                    draggable: true,
                });
            }

            self.display = Some(Display {
                graphics,
                renderer,
                viewport: Viewport::default(),
                elements,
                drag_indicator: None,
            });
            self.gui = Some(gui);
        }
    }

    fn on_suspend(&mut self, _wm: WindowManager) {}

    fn on_event(&mut self, wm: WindowManager, _id: WindowId, event: WindowEvent) {
        let Some(display) = &mut self.display else { return; };
        let Some(gui) = &mut self.gui else { return; };

        match event {
            WindowEvent::Resize { width, height } => {
                display.graphics.window().request_redraw();
                if width > 0 && height > 0 {
                    let physical_size = vec2(width as _, height as _);
                    display.viewport.resize(physical_size);
                    display.graphics.resize(display.renderer.device(), physical_size);
                    display.renderer.resize(physical_size);
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
                display.renderer.clear();
                display.renderer.start_layer(display.viewport.rect());
                display.renderer.fill_quad(Quad {
                    bounds: display.viewport.rect(),
                    border: Border::NONE,
                    shadow: Shadow::NONE,
                    bg_color: GRAY_0,
                });
                display.renderer.end_layer();
                display.renderer.start_layer(display.viewport.rect());
                for button in display.elements.values() {
                    display.renderer.fill_quad(button.quad);
                }
                display.renderer.end_layer();
                display.renderer.start_layer(display.viewport.rect());
                for button in display.elements.values() {
                    display.renderer.fill_text(button.text.clone());
                }
                display.renderer.end_layer();
                if let Some(drag_indicator) = &display.drag_indicator {
                display.renderer.start_layer(display.viewport.rect());
                    display.renderer.fill_quad(*drag_indicator);
                    display.renderer.end_layer();
                }

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
        if !button.draggable { return; }
        button.quad.bg_color = GRAY_7;
        button.text.size = 30.0;
    }

    fn on_mouse_leave(&mut self, element: Element, state: &GuiState) {
        self.graphics.window().request_redraw();
        if !state.is_dragging {
            self.graphics.window().set_cursor(CursorIcon::Default);
        }
        let Some(button) = self.elements.get_mut(&element) else { return; };
        if !button.draggable { return; }
        button.quad.bg_color = GRAY_6;
        button.quad.border.color = GRAY_5;
        button.text.size = 20.0;
    }

    fn on_mouse_down(&mut self, element: Element, _state: &GuiState) {
        self.graphics.window().request_redraw();
        let Some(button) = self.elements.get_mut(&element) else { return; };
        if !button.draggable { return; }
        button.quad.bg_color = GRAY_3;
    }

    fn on_mouse_up(&mut self, element: Element, _state: &GuiState) {
        self.graphics.window().request_redraw();
        let Some(button) = self.elements.get_mut(&element) else { return; };
        if !button.draggable { return; }
        button.quad.bg_color = GRAY_6;
    }

    fn on_drag_update(
        &mut self,
        element: Element,
        _tree: &mut LayoutTree,
        delta: Vec2,
        hovered: Option<Element>,
    ) {
        self.graphics.window().request_redraw();
        let Some(button) = self.elements.get(&element) else { return; };
        if !button.draggable { return; }
        self.drag_indicator = Some(Quad {
            bounds: Rect::new(button.quad.bounds.position() + delta, button.quad.bounds.size()),
            ..button.quad
        });
        if let Some(button) = hovered.and_then(|e| self.elements.get_mut(&e)) {
            if !button.draggable { return; }
            button.quad.border.color = GRAY_9;
        }
    }

    fn on_drag_start(&mut self, _element: Element, _tree: &mut LayoutTree) {
        self.graphics.window().request_redraw();
        self.graphics.window().set_cursor(CursorIcon::Grab);
    }

    fn on_drag_end(&mut self, _element: Element, _tree: &mut LayoutTree) {
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
        button.text.pos = placement.content_position();
        button.text.bounds = placement.content_size();
    }
}



struct Button {
    quad: Quad,
    text: Text,
    draggable: bool,
}
