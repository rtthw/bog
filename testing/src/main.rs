


use std::collections::HashMap;

use bog::*;
use color::Color;
use fonts::{Font, Fonts};
use graphics::*;
use layout::*;
use math::vec2;
use new_renderer::{Mesh2D, Renderer2D, Shape, Tessellator};
use ui::{Ui, UiHandler, UiModel};



fn main() -> Result<()> {
    let (screen_width, screen_height) = (1200.0, 800.0);
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Bog Testing")
        .with_inner_size(winit::dpi::LogicalSize::new(screen_width, screen_height))
        .build(&event_loop)
        .unwrap();
    let graphics = WindowGraphics::from_winit_window(&window, GraphicsConfig::new(1200, 800))?;
    let mut fonts = Fonts::default();

    fonts.load_font(
        "mono",
        include_bytes!("../data/JetBrainsMonoNerdFont_Regular.ttf").to_vec(),
        20.0,
        false, // Don't load all glyphs on each run.
    ).unwrap();

    let bg_color = Color::from_rgb(43, 43, 53);

    let mut ui = Ui::new(Layout::default()
        .flex_row()
        .flex_wrap()
        .gap_x(11.0)
        .margin(11.0)
        .fill_width()
        .fill_height()
        .align_content_center());
    let mut something = Something {
        window,
        renderer: Renderer2D::new(graphics.context()),
        objects: HashMap::with_capacity(10),
    };

    let font = fonts.get_font_mut("mono").unwrap();

    // Only load the glyphs we use.
    font.load_text_glyphs("┃ This is @_ |> test for text #_(o) ┃ ... *** =>>").unwrap();
    for word in ["┃ This", "is", "@_ |>", "test", "for", "text", "#_(o)", "┃ ...", "***", "=>>"] {
        something.spawn_button(&mut ui, &font, word);
    }

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                }
                winit::event::WindowEvent::Resized(new_size) => {
                    let (width, height) = new_size.into();
                    ui.handle_resize(&mut something, width, height);
                    graphics.resize(new_size);
                    something.window.request_redraw();
                }
                winit::event::WindowEvent::CursorMoved { position, .. } => {
                    let (x, y): (f32, f32) = position.into();
                    ui.handle_mouse_move(&mut something, x, y);
                    something.window.request_redraw();
                }
                winit::event::WindowEvent::MouseInput { state, button, .. } => {
                    if state == winit::event::ElementState::Pressed {
                        ui.handle_mouse_down(&mut something, button);
                    } else {
                        ui.handle_mouse_up(&mut something, button);
                    }
                    something.window.request_redraw();
                }
                _ => {}
            }
            winit::event::Event::RedrawRequested(_) => {
                let (width, height) = something.window.inner_size().into();
                let viewport = Viewport::new_at_origo(width, height);
                let [r, g, b, a] = bg_color.into();
                RenderTarget::screen(graphics.context(), width, height)
                    .clear(ClearState::color_and_depth(r, g, b, a, 1.0))
                    .write(|| -> Result<()> {
                        for obj in something.objects.values() {
                            match obj {
                                Object::Button { text_mesh, pane_mesh, .. } => {
                                    something.renderer.render(viewport, text_mesh);
                                    something.renderer.render(viewport, pane_mesh);
                                }
                            }
                        }

                        Ok(())
                    })
                    .unwrap();
                graphics.swap_buffers().unwrap();
            }
            _ => {}
        }
    });
}



struct Something {
    window: winit::window::Window,
    renderer: Renderer2D,
    objects: HashMap<u64, Object>,
}

impl UiHandler for Something {
    fn on_resize(&mut self, node: u64, model: &mut UiModel) {
        if let Some(obj) = self.objects.get_mut(&node) {
            let Object::Button { row_height, text_mesh, pane_mesh } = obj; // else { return; };
            let layout = model.layout(node.into()).unwrap();
            let parent_layout = model.layout(model.parent(node.into()).unwrap()).unwrap();
            let mut new_mesh = Mesh2D::new();
            Tessellator.tessellate_shape(Shape::Rect {
                pos: vec2(
                    parent_layout.location.x + layout.location.x,
                    parent_layout.location.y + layout.location.y,
                ),
                size: vec2(
                    layout.size.width
                        + layout.padding.horizontal_components().sum()
                        + layout.border.horizontal_components().sum(),
                    layout.size.height
                        + layout.padding.vertical_components().sum()
                        + layout.border.vertical_components().sum(),
                ),
                color: Color::from_rgb(23, 23, 29),
            }, &mut new_mesh);
            std::mem::swap(pane_mesh, &mut new_mesh);

            let (size, min_pos, _max_pos) = text_mesh.compute_info();
            text_mesh.translate(
                (parent_layout.location.x
                    // + text_offset.x
                    + layout.content_box_x())
                - min_pos.x,
                (parent_layout.location.y
                    // + text_offset.y
                    + ((*row_height - size.y) / 2.0)
                    + layout.content_box_y())
                - min_pos.y,
            );
        }
    }

    fn on_mouse_enter(&mut self, node: u64, _model: &mut UiModel) {
        if let Some(obj) = self.objects.get_mut(&node) {
            let Object::Button { text_mesh, pane_mesh, .. } = obj; // else { return; };
            pane_mesh.change_color(Color::from_rgb(59, 59, 67));
            text_mesh.change_color(Color::from_rgb(191, 191, 197));
            self.window.set_cursor_icon(winit::window::CursorIcon::Hand);
        }
    }

    fn on_mouse_leave(&mut self, node: u64, _model: &mut UiModel) {
        if let Some(obj) = self.objects.get_mut(&node) {
            let Object::Button { text_mesh, pane_mesh, .. } = obj; // else { return; };
            pane_mesh.change_color(Color::from_rgb(23, 23, 29));
            text_mesh.change_color(Color::from_rgb(163, 163, 173));
            self.window.set_cursor_icon(winit::window::CursorIcon::Arrow);
        }
    }

    fn on_mouse_down(&mut self, node: u64, _model: &mut UiModel) {
        if let Some(obj) = self.objects.get_mut(&node) {
            let Object::Button { text_mesh, pane_mesh, .. } = obj; // else { return; };
            pane_mesh.change_color(Color::from_rgb(59, 59, 67));
            pane_mesh.translate(0.0, 1.0);
            text_mesh.change_color(Color::from_rgb(139, 139, 149));
            text_mesh.translate(0.0, 1.0);
        }
    }

    fn on_mouse_up(&mut self, node: u64, _model: &mut UiModel) {
        if let Some(obj) = self.objects.get_mut(&node) {
            let Object::Button { text_mesh, pane_mesh, .. } = obj; // else { return; };
            pane_mesh.change_color(Color::from_rgb(23, 23, 29));
            pane_mesh.translate(0.0, -1.0);
            text_mesh.change_color(Color::from_rgb(163, 163, 173));
            text_mesh.translate(0.0, -1.0);
        }
    }

    fn on_drag_start(&mut self, node: u64, _model: &mut UiModel) {
        println!("on_drag_start({node});");
    }

    fn on_drag_end(&mut self, node: u64, other: Option<u64>, model: &mut UiModel) {
        println!("on_drag_end({node}, {other:?});");
    }

    fn on_drag_update(&mut self, node: u64, _model: &mut UiModel, delta_x: f32, delta_y: f32) {
        // println!("on_drag_update({node}, {delta_x}, {delta_y});");
    }

    fn on_click(&mut self, node: u64, _model: &mut UiModel) {
        println!("on_click({node});");
    }
}

impl Something {
    fn spawn_button(&mut self, ui: &mut Ui, font: &Font, text: &str) {
        let text_wireframe = font.text_wireframe(text, None);
        let mut text_mesh = Mesh2D::from_wireframe(
            text_wireframe,
            Color::from_rgb(163, 163, 173),
        );
        let (text_size, _, _) = text_mesh.compute_info();
        text_mesh.invert_y();
        let row_height = font.row_height();

        let mut pane_mesh = Mesh2D::new();
        Tessellator.tessellate_shape(Shape::Rect {
            pos: vec2(0.0, 0.0),
            size: vec2(text_size.x, row_height),
            color: Color::from_rgb(23, 23, 29),
        }, &mut pane_mesh);
        // pane_mesh.invert_y();

        let node = ui.push_to_root(
            Layout::default()
                .width(text_size.x)
                .height(row_height)
                .padding(3.0),
            true,
        );

        self.objects.insert(node, Object::Button { row_height, text_mesh, pane_mesh });
    }
}

enum Object {
    Button {
        row_height: f32,
        text_mesh: Mesh2D,
        pane_mesh: Mesh2D,
    },
}
