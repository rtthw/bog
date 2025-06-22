


use crate::*;



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
        let content_position = visuals.position + self.layout_cache.position();
        self.content.render(Visuals { position: content_position, ..visuals }, pass);
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
