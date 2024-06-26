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

* Focusing is wierd.
** Surrendering focus breaks the focus flow. After a widget surrenders focus the focus flow start back at the beginning. This is problematic because disabled ui's will surrender the focus for the diabled widgets. This makes it impossible to correctly navigate through the app with elements are disabled.
** It is not possible to surrender the focus to the next element programmatically. A list widget might want to capture the arrow up and down keys to change the selection of the list. Once the selection reaches the top of the list, the focus should switch away from the list and to the previous element. Same for the bottom of the list. This is not possible since there is no way to surrender the focus to the next/previous element.

* There is no way to make a custom Menu.
** If you need a context menu for a widget there is no (easy) way to make a custom one without relying on the provided `menu_button` or `Response::context_menu` implementations. From what i can tell, these are your only two options to make a menu that behaves like a menu. It might be possible to make a menu using an area, however this could not correctly interact with any other menus. For example it would not be possible to use `Ui::close_menu` or `Ui::menu_button` to create further sub menues.

* No consistent options on how text is wrapped / truncated
** For labels there is the option to truncate a label with "..." but for buttons this does not exists.

* Some methods on Ui cannot be implemented using the public API.
** A menu button for example cannot be implemented using the public api since it carries some state in the ui itself.
As a result, a menu button should behave like a button but it does not. Since it does not implement the Widget trait it 
cannot be added with `add_sized` for example. It is also not possible to have any sort of wrapping behavior a button has.

