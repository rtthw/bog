


use bog::*;
use fonts::*;
use graphics::*;
use math::vec2;
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
    let mut test_glyph_mesh = parsed_font.glyph_mesh(test_glyph_id, 40.0).unwrap();
    for v in test_glyph_mesh.vertices.iter_mut() {
        v.x += 10.0;
        v.y += 10.0;
    }

    let mut painter = Painter::new(&graphics);
    let paints = vec![
        // Rectangle {
        //     pos: vec2(0.0, 0.0),
        //     size: vec2(100.0, 50.0),
        //     color: 0x2b2b33ff,
        //     corner_radii: [7.0, 19.0, 1.0, 45.0],
        // }.to_mesh(),
        PaintMesh::glyph(test_glyph_mesh, 0xaaaaabff),
    ];

    event_loop.run(move |event, control_flow| {
        match event {
            WindowManagerEvent::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    control_flow.exit();
                }
                WindowEvent::Resized(new_size) => {
                    graphics.window().request_redraw();
                    if new_size.width > 0 && new_size.height > 0 {
                        graphics.resize(vec2(new_size.width as _, new_size.height as _));
                    }
                }
                WindowEvent::RedrawRequested => {
                    graphics
                        .render(|render_pass| {
                            painter.prepare(&graphics, &paints);
                            painter.render(render_pass, &paints);
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
