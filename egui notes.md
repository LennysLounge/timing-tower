* `ui::interact` adds additional space around the rect to make interaction easier
This makes it difficult to interact with precise regions in the ui.
`ui::interact` uses `Context::interact` internally but this is only `pub(crate)`. It would be nice if it was possible to interaction with a response without the added space either through the ctx or some other method.

* It is not possible to make a whole layer transparent. This is sometimes usefull for creating drag and drop ghosts. The ghost should be transparent to not obstruct the view of the ui behind it. This is only possible by setting the transparency of the painted shapes directly. In a library context, should a ghost include ui elements from the user it is not possible for the library to give the transparency effect to the user created elements.
`Painter::fade_out_color` exists but has some problems:
    * it is `pub(crate)`. It is possible to get around this in three steps:
        1) Set the `fade_out_color` in the ui visuals.
        2) Set the ui to be disabled. This sets the `fade_out_color` on the painter.
        3) Set the ui to enabled. This does not actually reset the fade out color.
    * it does not work with transparency.
* Layers ... dont really exist. The `Order`'s do exists. However they are of no real use when rendering a complex widget for example. The state right now is that two `LayerId`'s in the same `Order` are drawn in essentially random order. This is because the `PaintList` that is referenced by the `LayerId` is extracted out of a hashmap. Values in a hashmap are in essentially random order. It is possible to infulence the order by chaning the `Id` of the `LayerId` but that only appears to solve the problem. Accessing the values of a `HashMap` is never assumed to be in any particular order.
Adding a `priority: u8` to a `LayerId` could solve this problem as it allows to extract the `PaintList`'s and then order afterwords.
