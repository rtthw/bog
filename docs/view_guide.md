


## Chapter 1: Introduction

A *view* is ...



## Chapter 2: Elements

An *element* is ...



## Chapter 3: Processing

*This chapter covers the technical details of how Bog processes views.*

### 3.1: Building the model

A view's *model* is its concrete, in-memory representation. As such, it must be pre-processed due to technical limitations with querying and manipulating a raw element tree. This pre-processing step is called "building the model".

The process of building a view model involves "digesting" a tree of element representations into more managable mappings of layout nodes to resources like styles and event handlers. Tree digestion is essentially a way of converting user-defined trees of elements with styling and event handlers into usable types for more low-level computations.

The digestion process for each element is as follows:

1. Separate the element's style into two parts: the layout-defining properties (width, font family, etc.) and the non-layout-defining properties (color, border radius, etc.). When event handlers change style properties, we need to know when to re-compute the layout. For example, if an event handler only changes the element's background color, re-computing the layout would be a waste of resources.
2. Use the layout-defining properties and element's tree position (ancestors, descendents, etc.) to create layout nodes that will be used to identify the now-broken element. Remember, elements are only used to build concrete representations of themselves, so the model needs identifiers for the discrete components that define the abstract elements that will no longer be represented in memory as a single "thing".
3. Place the element's components into their respective mappings. Event handlers go into the object map, styles go into the style map, and layouts are now already stored in the layout map. At this point, the "element" no longer exists and its builder is dropped.



## Appendix A: Definitions
