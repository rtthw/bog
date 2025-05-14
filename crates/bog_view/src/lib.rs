//! Bog View



use bog_collections::NoHashMap;
use bog_layout::Layout;



pub trait View {
    fn build(&mut self) -> Model<Self> where Self: Sized;
}



/// A view model really just a tree of [`Element`]s that have been
pub struct Model<V: View> {
    elements: NoHashMap<u64, ElementProxy<V>>,
    root_node: u64,
}

/// An element proxy is useful because we need
struct ElementProxy<V: View> {
    object: Option<Box<dyn Object<View = V>>>,
}



/// An element is essentially just a way of attaching an [`Object`] to a [`Model`], which is just a
/// tree of objects.
pub struct Element<V: View> {
    object: Option<Box<dyn Object<View = V>>>,
    layout: Layout,
    children: Vec<Element<V>>,
}



#[allow(unused)]
pub trait Object {
    type View: View;

    fn render(&mut self, view: &mut Self::View, cx: Context<Self::View>) {}
}



pub struct Context<'a, V: View> {
    pub model: &'a mut Model<V>,
}
