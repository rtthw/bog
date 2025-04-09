


use std::collections::HashMap;

use bog::*;
use color::Color;
use fonts::{Font, Fonts};
use graphics::*;
use layout::*;
use math::vec2;
use new_renderer::{Mesh2D, Renderer2D, Shape, Tessellator};
use ui::{Placement, Ui, UiHandler, UiModel};



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
    fn on_layout(&mut self, node: u64, placement: &Placement) {
        if let Some(obj) = self.objects.get_mut(&node) {
            let Object::Button { row_height, text_mesh, pane_mesh } = obj; // else { return; };
            let mut new_mesh = Mesh2D::new();
            Tessellator.tessellate_shape(Shape::Rect {
                pos: placement.position(),
                size: vec2(
                    placement.layout.size.width
                        + placement.layout.padding.horizontal_components().sum()
                        + placement.layout.border.horizontal_components().sum(),
                    placement.layout.size.height
                        + placement.layout.padding.vertical_components().sum()
                        + placement.layout.border.vertical_components().sum(),
                ),
                color: Color::from_rgb(23, 23, 29),
            }, &mut new_mesh);
            std::mem::swap(pane_mesh, &mut new_mesh);

            let (size, min_pos, _max_pos) = text_mesh.compute_info();
            let pos = placement.content_position();
            text_mesh.translate(
                pos.x - min_pos.x,
                pos.y
                    + ((*row_height - size.y) / 2.0)
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
            text_mesh.change_color(Color::from_rgb(139, 139, 149));
            // pane_mesh.translate(0.0, 1.0);
            // text_mesh.translate(0.0, 1.0);
        }
    }

    fn on_mouse_up(&mut self, node: u64, _model: &mut UiModel) {
        if let Some(obj) = self.objects.get_mut(&node) {
            let Object::Button { text_mesh, pane_mesh, .. } = obj; // else { return; };
            pane_mesh.change_color(Color::from_rgb(23, 23, 29));
            text_mesh.change_color(Color::from_rgb(163, 163, 173));
            // pane_mesh.translate(0.0, -1.0);
            // text_mesh.translate(0.0, -1.0);
        }
    }

    fn on_drag_start(&mut self, node: u64, _model: &mut UiModel) {
        println!("on_drag_start(from={node});");
    }

    // NOTE: We can't just swap the two objects in `self.objects` because the sizing system is
    //       tied to the layout model. If `node` and `other` have different widths, then it
    //       won't display properly.
    // TODO: This is probably written terribly, should redo it with a clear head.
    fn on_drag_end(&mut self, node: u64, other: Option<u64>, model: &mut UiModel) {
        let Some(other) = other else { return; };
        if node == other { return; };
        println!("on_drag_end(from={node}, to={other:?});");

        let parent = model.parent(node.into()).unwrap();
        let mut children = model.children(parent).unwrap()
            .into_iter()
            .map(|n| n.into())
            .collect::<Vec<u64>>();
        let Some((node_index, _)) = children.iter().enumerate()
            .find(|(_, n)| *n == &node) else { return; };
        let Some((other_index, _)) = children.iter().enumerate()
            .find(|(_, n)| *n == &other) else { return; };
        children.swap(node_index, other_index);
        let real_children = children.clone().into_iter().map(|n| n.into()).collect::<Vec<_>>();
        model.set_children(parent, &real_children).unwrap();
        // for node in &children[(node_index.min(other_index))..=(node_index.max(other_index))] {
        //     self.on_resize(*node, model);
        // }
    }

    fn on_drag_update(&mut self, node: u64, model: &mut UiModel, delta_x: f32, delta_y: f32) {
        if let Some(obj) = self.objects.get_mut(&node) {
            let Object::Button { text_mesh, pane_mesh, .. } = obj; // else { return; };
            pane_mesh.change_color(Color::from_rgb(59, 59, 67));
            text_mesh.change_color(Color::from_rgb(139, 139, 149));
            let layout = model.layout(node.into()).unwrap();
            let parent_layout = model.layout(model.parent(node.into()).unwrap()).unwrap();
            let (_, min_pos, _) = text_mesh.compute_info();
            text_mesh.translate(
                (parent_layout.location.x + layout.content_box_x() + delta_x) - min_pos.x,
                (parent_layout.location.y + layout.content_box_y() + delta_y) - min_pos.y,
            );
        }
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

        let node = ui.tree().push_to_root(
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
