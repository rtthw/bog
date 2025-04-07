


use std::collections::HashMap;

use bog::*;
use graphics::*;
use layout::Layout;
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
    let mut graphics = WindowGraphics::from_winit_window(&window, GraphicsConfig::new(1200, 800))?;

    graphics.renderer_mut().load_font(
        "mono",
        include_bytes!("../data/JetBrainsMonoNerdFont_Regular.ttf").to_vec(),
        20.0,
    )?;

    let bg_color = Srgba::new_opaque(43, 43, 53);

    let mut ui = Ui::new(Layout::default()
        .flex_row()
        .flex_wrap()
        .gap_x(19.0)
        .padding(11.0)
        .fill_width()
        .fill_height()
        .align_content_center());
    let mut something = Something {
        renderer: Renderer2D::new(graphics.renderer().gl()),
        meshes: HashMap::with_capacity(10),
        objects: HashMap::with_capacity(10),
    };

    for word in ["┃ This", "is", "@_ |>", "test", "for", "text", "#_(o)", "┃ ...", "***", "=>>"] {
        something.spawn_button(&mut ui, graphics.renderer(), word);
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
                    window.request_redraw();
                }
                winit::event::WindowEvent::CursorMoved { position, .. } => {
                    let (x, y): (f32, f32) = position.into();
                    ui.handle_mouse_move(&mut something, x, y);
                    window.request_redraw();
                }
                winit::event::WindowEvent::MouseInput { state, button, .. } => {
                    if state == winit::event::ElementState::Pressed {
                        ui.handle_mouse_down(&mut something, button);
                    } else {
                        ui.handle_mouse_up(&mut something, button);
                    }
                }
                _ => {}
            }
            winit::event::Event::RedrawRequested(_) => {
                let (width, height) = window.inner_size().into();
                let viewport = Viewport::new_at_origo(width, height);
                let [r, g, b, a] = bg_color.into();
                RenderTarget::screen(graphics.renderer(), width, height)
                    .clear(ClearState::color_and_depth(r, g, b, a, 1.0))
                    .write(|| -> Result<()> {
                        for mesh in something.meshes.values() {
                            something.renderer.render(viewport, mesh);
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
    renderer: Renderer2D,
    meshes: HashMap<u64, Mesh2D>,
    objects: HashMap<u64, Object>,
}

impl UiHandler for Something {
    fn on_resize(&mut self, node: u64, model: &mut UiModel) {
        if let Some(mesh) = self.meshes.get_mut(&node) {
            let layout = model.layout(node.into()).unwrap();
            let parent_layout = model.layout(model.parent(node.into()).unwrap()).unwrap();
            let (_size, min_pos, _max_pos) = mesh.compute_info();
            mesh.translate(
                (parent_layout.location.x + layout.content_box_x()) - min_pos[0],
                (parent_layout.location.y + layout.content_box_y()) - min_pos[1],
            );
        }
    }

    fn on_mouse_enter(&mut self, node: u64, _model: &mut UiModel) {
        if let Some(obj) = self.objects.get(&node) {
            let Object::Button { pane_node: _, text_node } = obj; // else { return; };
            if let Some(text_mesh) = self.meshes.get_mut(text_node) {
                text_mesh.change_color(Srgba::new_opaque(191, 191, 197));
            }
        }
    }

    fn on_mouse_leave(&mut self, node: u64, _model: &mut UiModel) {
        if let Some(obj) = self.objects.get(&node) {
            let Object::Button { pane_node: _, text_node } = obj; // else { return; };
            if let Some(text_mesh) = self.meshes.get_mut(text_node) {
                text_mesh.change_color(Srgba::new_opaque(163, 163, 173));
            }
        }
    }

    fn on_click(&mut self, node: u64, _model: &mut UiModel) {
        println!("on_click({node});");
    }
}

impl Something {
    fn spawn_button(&mut self, ui: &mut Ui, renderer: &Renderer, text: &str) {
        let text_wireframe = renderer
            .text_wireframe("mono", text, None)
            .unwrap();
        let mut text_mesh = Mesh2D::from_wireframe(
            text_wireframe,
            Srgba::new_opaque(163, 163, 173),
        );
        text_mesh.invert_y();
        let (text_size, _, _) = text_mesh.compute_info();
        let row_height = renderer.get_font("mono").unwrap().row_height();

        let mut pane_mesh = Mesh2D::new();
        Tessellator.tessellate_shape(Shape::Rect {
            pos: vec2(0.0, 0.0),
            size: vec2(text_size[0], row_height),
            color: Srgba::new_opaque(23, 23, 29),
        }, &mut pane_mesh);
        // pane_mesh.invert_y();

        let pane_node = ui.push_to_root(
            Layout::default()
                .align_items_center()
                .width(text_size[0])
                .height(row_height),
            true,
        );
        let text_node = ui.push_to(
            Layout::default()
                .width(text_size[0])
                .height(text_size[1]),
            pane_node,
            false,
        );

        self.meshes.insert(pane_node, pane_mesh);
        self.meshes.insert(text_node, text_mesh);
        self.objects.insert(pane_node, Object::Button { pane_node, text_node });
    }
}

enum Object {
    Button {
        pane_node: u64,
        text_node: u64,
    },
}
