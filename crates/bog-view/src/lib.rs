//! Prototyping



pub mod layout;

use bog_core::{InputEvent, Vec2, Xy};
use bog_render::{RenderPass, Renderer};

use bog_window::Window;
use layout::{LayoutCache, Style};



pub struct Visuals<'a> {
    pub renderer: &'a mut Renderer,
    pub window: &'a Window,
}

#[allow(unused)]
pub trait Element {
    fn render<'a>(&'a mut self, visuals: Visuals, pass: &mut RenderPass<'a>) {}

    fn input(&mut self, visuals: Visuals, event: InputEvent) {}

    fn style(&self) -> &Style;

    fn style_mut(&mut self) -> &mut Style;

    fn layout_cache(&self) -> &LayoutCache;

    fn layout_cache_mut(&mut self) -> &mut LayoutCache;

    fn measure(&self, known_dimensions: Xy<Option<f32>>, available_space: Vec2) -> Option<Vec2> {
        None
    }

    fn child_count(&self) -> usize {
        0
    }

    fn child_at(&self, index: usize) -> &Box<dyn Element>;

    fn child_at_mut(&mut self, index: usize) -> &mut Box<dyn Element>;
}



mod builtin {
    use super::*;

    pub struct Panel {
        content: Box<dyn Element>,
        style: Style,
        layout_cache: LayoutCache,
    }

    impl Panel {
        pub fn new(content: impl Into<Box<dyn Element>>) -> Self {
            Self {
                content: content.into(),
                style: Style::new(),
                layout_cache: LayoutCache::new(),
            }
        }
    }

    impl Element for Panel {
        fn render<'a>(&'a mut self, visuals: Visuals, pass: &mut RenderPass<'a>) {
            self.content.render(visuals, pass);
        }

        fn input(&mut self, visuals: Visuals, event: InputEvent) {
            self.content.input(visuals, event);
        }

        fn style(&self) -> &Style {
            &self.style
        }

        fn style_mut(&mut self) -> &mut Style {
            &mut self.style
        }

        fn layout_cache(&self) -> &LayoutCache {
            &self.layout_cache
        }

        fn layout_cache_mut(&mut self) -> &mut LayoutCache {
            &mut self.layout_cache
        }

        fn child_count(&self) -> usize {
            1
        }

        fn child_at(&self, _index: usize) -> &Box<dyn Element> {
            &self.content
        }

        fn child_at_mut(&mut self, _index: usize) -> &mut Box<dyn Element> {
            &mut self.content
        }
    }
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
