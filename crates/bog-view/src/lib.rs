//! Prototyping



pub mod layout;

use bog_core::{InputEvent, Vec2, Xy};
use bog_render::{RenderPass, Renderer};

use layout::{LayoutCache, Style};



pub struct Visuals<'a> {
    pub renderer: &'a mut Renderer,
}

#[allow(unused)]
pub trait Element {
    fn render<'a>(&'a mut self, visuals: Visuals, pass: &mut RenderPass<'a>) {}
    fn style(&self) -> &Style;
    fn style_mut(&mut self) -> &mut Style;
    fn layout_cache(&self) -> &LayoutCache;
    fn layout_cache_mut(&mut self) -> &mut LayoutCache;
    fn measure(&self, known_dimensions: Xy<Option<f32>>, available_space: Vec2) -> Vec2;
    fn num_children(&self) -> usize;
    fn child_at(&self, index: usize) -> &Box<dyn Element>;
    fn child_at_mut(&mut self, index: usize) -> &mut Box<dyn Element>;
}

#[allow(unused)]
pub trait View {
    type Event;
    type Context;

    fn event(&mut self, event: Self::Event) {}
    fn render<'a>(&'a mut self, cx: Self::Context, pass: &mut RenderPass<'a>) {}
    fn input(&mut self, cx: Self::Context, event: InputEvent) {}
}



#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use bog_render::{RenderPass, Text};

    use crate::View;

    struct ContextWithLifetime<'a> {
        field: &'a mut String,
    }

    #[derive(Default)]
    struct TestView<'cx> {
        counter: usize,
        _lifetime_constraint: PhantomData<&'cx ()>,
    }

    impl<'cx> View for TestView<'cx> {
        type Context = ContextWithLifetime<'cx>;
        type Event = ();

        fn render<'a>(&'a mut self, cx: ContextWithLifetime<'a>, pass: &mut RenderPass<'a>) {
            self.counter += cx.field.len();
            *cx.field = "Thing".to_string();
            pass.fill_text(Text {
                content: cx.field.as_str().into(),
                ..Default::default()
            });
        }
    }

    #[test]
    fn works() {
        let mut field = "Something".to_string();
        let mut view = TestView::default();

        {
            let cx = ContextWithLifetime {
                field: &mut field,
            };

            view.render(cx, &mut RenderPass::new());
        }

        assert!(view.counter == 9);
        assert!(field.as_str() == "Thing");
    }
}
