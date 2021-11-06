//! The core and state for laying out groups.

use std::ops::{Deref, DerefMut};

use crate::common::*;
use crate::layout::*;
use crate::renderer::*;

#[derive(Clone)]
struct Group {
    //
    // layout info
    //
    rect: Rect,
    layout: Layout,
    cursor: Vector,
    //
    // rendering info
    //
    line_cap: LineCap,
}

// for use in doc comment
#[allow(unused)]
use crate::build;

/// UI state. This is what's used for laying out groups and drawing to the screen.
///
/// # The group stack
///
/// The group stack is what controls positioning at a given moment. Because paws is an _immediate-mode_ GUI library,
/// there is no intermediate state with nodes and what not. Everything is rendered to the screen directly.
///
/// However, during rendering, paws has to keep some information around for layouting. After all, that's what it's for:
/// laying out rectangles on the screen. The way it maintains this state is through a _stack_ of _groups_.
///
/// Groups are essentially just glorified rectangles with some extra layouting and rendering data. When you push a new
/// group, the its position is placed relative to the previous group, at a vector called the _cursor_. When you pop
/// this new group off the stack, the cursor of the group above is offset by the size of the group you just popped off.
/// This allows you to cascade elements, flexbox-style, just without the overhead of holding a hundred or so nodes
/// scattered across heap memory. The group stack grows as you nest more and more groups inside of each other, but old
/// groups are not preserved after they're popped off the stack.
///
/// Of course this paradigm makes laying out simple panels quite easy, but elements that need to preserve state may be
/// a bit more challenging to implement. But worry not, structs come to your rescue! If you want to implement a more
/// complex element that needs to preserve some state across frames, create a struct that'll hold its data:
///
/// ```
/// // let's assume this is our UI type:
/// use paws::NoRenderer;
/// type Ui = paws::Ui<NoRenderer>;
///
/// struct Slider {
///     value: f32,
///     min: f32,
///     max: f32,
/// }
/// ```
///
/// Then, implement a method on this struct that'll render the element onto the screen, and process all incoming input
/// events:
///
/// ```
/// # use paws::NoRenderer;
/// # type Ui = paws::Ui<NoRenderer>;
/// #
/// # struct Slider {
/// #     value: f32,
/// #     min: f32,
/// #     max: f32,
/// # }
/// #
/// use paws::Layout;
///
/// impl Slider {
///     // for brevity, we'll assume no input events need to be processed, as that's outside
///     // of paws's scope.
///     fn process(&mut self, ui: &mut Ui, width: f32) {
///         // create a group that'll span a rectangle with the provided width and the parent
///         // group's height
///         ui.push((width, ui.height()), Layout::Freeform);
///         ui.draw(|ui| {
///             // ... do all the rendering work here ...
///         });
///         ui.pop();
///     }
/// }
/// ```
///
/// Then, simply keep a Slider somewhere outside of your event loop, and call `process()` when you want to place it
/// onto your UI.
///
/// ```
/// # use paws::NoRenderer;
/// # type Ui = paws::Ui<NoRenderer>;
/// #
/// # struct Slider {
/// #     value: f32,
/// #     min: f32,
/// #     max: f32,
/// # }
/// #
/// # use paws::Layout;
/// #
/// # impl Slider {
/// #     fn process(&mut self, ui: &mut Ui, width: f32) {
/// #         ui.push((width, ui.height()), Layout::Freeform);
/// #         ui.draw(|ui| {
/// #         });
/// #         ui.pop();
/// #     }
/// # }
/// #
/// # let window_size = paws::vector(800.0, 600.0);
/// let mut ui = Ui::new(NoRenderer);
/// let mut slider = Slider {
///     value: 0.0,
///     min: 0.0,
///     max: 32.0,
/// };
///
/// // ↓ this is your event loop! usually done by winit, SDL2, or some other library.
/// 'app: loop {
///     // don't forget the root group!
///     ui.root(window_size, Layout::Vertical);
///     // ... other layout stuff goes here ...
///     // when time comes, process() the slider:
///     slider.process(&mut ui, 256.0);
///     # break;
/// }
/// ```
///
/// # Initialization
///
/// Most of the group-related methods described below will panic if there are no groups on the stack, with the exception
/// of [`Ui::new`] (obviously), and [`Ui::root`]. The latter is used to initialize the UI state so that at least one
/// group is present, but also to make sure that the group stack doesn't grow to oblivion if the user forgets to pop a
/// group or two. (This isn't a good excuse for not popping groups, but it'll at least prevent your program from leaking
/// memory if you really do forget.)
///
/// # Panicking
///
/// All functions that mention the "current group" are guaranteed to panic if there are no groups on the stack.
/// Other groups, such as the "parent group", which is the group above the current group, may also appear, but are
/// always followed up with a Panicking section that specify the requirements for these groups' presence.
///
/// # Rendering
///
/// Because there's usually no use to using a UI library without any actual rendering, the [`Ui`] type takes an extra
/// _renderer_ type as a parameter. The methods from this type are available via the `Deref` and `DerefMut` traits,
/// except for names that collide with those defined on `Ui` itself (obviously).
/// The renderer can be retrieved as an immutable reference (for probing and measurements) using [`Ui::renderer`],
/// and as a mutable reference (for doing actual rendering) using [`Ui::render`].
///
/// # `build!`
///
/// For your convenience while building UIs, a macro is available to make all those `push`es and `pop`s get out of your
/// face. See [`build!`]'s documentation for more info.
pub struct Ui<T: Renderer> {
    stack: Vec<Group>,
    renderer: T,
}

impl<T: Renderer> Ui<T> {
    /// Creates a new UI state with the given renderer.
    pub fn new(renderer: T) -> Self {
        Self {
            stack: Vec::new(),
            renderer,
        }
    }

    //
    // getters
    //

    /// Returns an immutable reference to the renderer.
    pub fn renderer(&self) -> &T {
        &self.renderer
    }

    /// Returns a mutable reference to the renderer.
    pub fn render(&mut self) -> &mut T {
        &mut self.renderer
    }

    //
    // stack getters
    //

    /// Returns the position of the topmost group, in absolute (screen) coordinates.
    pub fn position(&self) -> Point {
        self.top().rect.position
    }

    /// Returns the size of the topmost group.
    pub fn size(&self) -> Vector {
        self.top().rect.size
    }

    /// Returns the width of the topmost group.
    pub fn width(&self) -> f32 {
        self.top().rect.width()
    }

    /// Returns the height of the topmost group.
    pub fn height(&self) -> f32 {
        self.top().rect.height()
    }

    /// Returns the "remaining size" of the current group. This is measured by subtracting the group's cursor from
    /// its size, effectively giving you the size that remains in the group. In reversed layouts, the cursor is added
    /// instead, as it goes into the negative. On the freeform layout, this always returns (0, 0).
    pub fn remaining_size(&self) -> Vector {
        let top = self.top();
        match top.layout {
            Layout::Freeform => vector(0.0, 0.0),
            Layout::Horizontal | Layout::Vertical => top.rect.size - top.cursor,
            Layout::HorizontalRev | Layout::VerticalRev => top.rect.size + top.cursor,
        }
    }

    /// Returns the "remaining width" of the current group, as per the convention described in
    /// [`Ui::remaining_size`]'s documentation.
    pub fn remaining_width(&self) -> f32 {
        let top = self.top();
        match top.layout {
            Layout::Freeform => 0.0,
            Layout::Horizontal => top.rect.width() - top.cursor.x,
            Layout::Vertical => top.rect.width(),
            Layout::HorizontalRev => top.rect.width() + top.cursor.x,
            Layout::VerticalRev => top.rect.width(),
        }
    }

    /// Returns the "remaining height" of the current group, as per the convention described in
    /// [`Ui::remaining_size`]'s documentation.
    pub fn remaining_height(&self) -> f32 {
        let top = self.top();
        match top.layout {
            Layout::Freeform => 0.0,
            Layout::Horizontal => top.rect.height(),
            Layout::Vertical => top.rect.height() - top.cursor.y,
            Layout::HorizontalRev => top.rect.height(),
            Layout::VerticalRev => top.rect.height() + top.cursor.y,
        }
    }

    //
    // stack manipulation
    //

    /// Clears the group stack and pushes the root group onto the stack, with the given size and layout.
    /// The root group is the first group that should be pushed onto the stack. It defines the size of the window,
    /// and the initial layout to be used.
    ///
    /// Note that this root group **must not** be popped off manually, as it gets popped off every frame anyways,
    /// because the stack is cleared upon calling this function.
    pub fn root(&mut self, size: impl Into<Vector>, layout: Layout) {
        self.stack.clear();
        self.stack.push(Group {
            rect: Rect::new(point(0.0, 0.0), size),
            layout,
            cursor: vector(0.0, 0.0),
            line_cap: LineCap::Butt,
        });
    }

    /// Pushes a group onto the group stack, with the given size and layout.
    pub fn push(&mut self, size: impl Into<Vector>, layout: Layout) {
        let size = size.into();
        let top = self.top().clone();
        let position = match top.layout {
            Layout::Freeform | Layout::Horizontal | Layout::Vertical => {
                top.rect.position + top.cursor
            }
            Layout::HorizontalRev => top.rect.top_right() + top.cursor - point(size.x, 0.0),
            Layout::VerticalRev => top.rect.bottom_left() + top.cursor - point(0.0, size.y),
        };
        self.stack.push(Group {
            rect: Rect::new(position, size),
            layout,
            cursor: point(0.0, 0.0),
            ..top
        });
    }

    /// Pops a group off the group stack, updating the cursor of the group under it.
    pub fn pop(&mut self) {
        let group = self
            .stack
            .pop()
            .expect("the root group got popped of the stack");
        let top = self.top_mut();
        match top.layout {
            Layout::Freeform => (),
            Layout::Horizontal => top.cursor.x += group.rect.width(),
            Layout::Vertical => top.cursor.y += group.rect.height(),
            Layout::HorizontalRev => top.cursor.x -= group.rect.width(),
            Layout::VerticalRev => top.cursor.y -= group.rect.height(),
        }
    }

    //
    // group manipulation
    //

    /// Returns the cursor position of the current group.
    pub fn cursor(&self) -> Vector {
        self.top().cursor
    }

    /// Sets the cursor position of the current group. This is most useful with freeform layouts.
    pub fn set_cursor(&mut self, new_cursor: Vector) {
        self.top_mut().cursor = new_cursor;
    }

    /// Offsets the cursor by the given amount.
    pub fn offset(&mut self, by: Vector) {
        self.top_mut().cursor += by;
    }

    /// Pads the current group with some amount of padding.
    pub fn pad(&mut self, padding: impl Into<Padding>) {
        let padding = padding.into();
        let rect = &mut self.top_mut().rect;
        rect.position.x += padding.left;
        rect.position.y += padding.top;
        rect.size.x -= padding.left + padding.right;
        rect.size.y -= padding.top + padding.bottom;
    }

    /// Aligns the current group in the parent group, with the provided alignment.
    ///
    /// # Panics
    /// If there are less than two groups (the parent and the subject) on the stack.
    pub fn align(&mut self, alignment: Alignment) {
        let parent = self
            .stack
            .get(self.stack.len() - 2)
            .expect("no parent group on the stack to align to")
            .rect;
        let subject = &mut self
            .stack
            .last_mut()
            .expect("no group on the stack to align")
            .rect;
        subject.position.x = match alignment.0 {
            Left => parent.left(),
            Center => parent.center_x() - subject.width() / 2.0,
            Right => parent.right() - subject.width(),
        };
        subject.position.y = match alignment.1 {
            Top => parent.top(),
            Middle => parent.center_y() - subject.height() / 2.0,
            Bottom => parent.bottom() - subject.height(),
        };
    }

    /// Inserts empty space between subgroups, by increasing or decreasing the cursor position by the given amount.
    ///
    /// # Panics
    ///  - If there are no groups.
    ///  - On freeform layout, as it's not clear which direction the spacing should be performed in.
    pub fn space(&mut self, amount: f32) {
        let top = self.top_mut();
        match top.layout {
            Layout::Freeform => panic!("using space() on Freeform layout is forbidden"),
            Layout::Horizontal => top.cursor.x += amount,
            Layout::Vertical => top.cursor.y += amount,
            Layout::HorizontalRev => top.cursor.x -= amount,
            Layout::VerticalRev => top.cursor.y -= amount,
        }
    }

    /// Resizes the current group to fit its children. This function considers a few cases:
    ///  - on `Freeform` layout, it sets the width and height to the cursor,
    ///  - on `Horizontal` layout, it sets the width to the cursor's X position,
    ///  - on `Vertical` layout, it sets the height to the cursor's Y position.
    ///  - on reversed layouts, it panics, as layouting there works _a bit backwards_ and fitting currently doesn't work
    ///    properly. This might get solved in a future release.
    ///
    /// # Panics
    ///  - If there are no groups.
    ///  - On reversed layouts, as noted above.
    pub fn fit(&mut self) {
        let top = self.top_mut();
        match top.layout {
            Layout::Freeform => top.rect.size = top.cursor,
            Layout::Horizontal => top.rect.size.x = top.cursor.x,
            Layout::Vertical => top.rect.size.y = top.cursor.y,
            Layout::HorizontalRev | Layout::VerticalRev => {
                panic!("reverse layout containers can't be fit()ted")
            }
        }
    }

    //
    // internal getters
    //

    fn top(&self) -> &Group {
        self.stack
            .last()
            .expect("no groups on the stack to read from. check your push() and pop()s")
    }

    fn top_mut(&mut self) -> &mut Group {
        self.stack
            .last_mut()
            .expect("no groups on the stack left to modify. check your push() and pop()s")
    }
}

impl<T: Renderer> Ui<T> {
    /// Allows one to draw in the current group by translating the renderer's matrix to the group's position.
    /// The renderer can be obtained inside of the callback by using [`Ui::render`].
    pub fn draw<F>(&mut self, do_draw: F)
    where
        F: FnOnce(&mut Self),
    {
        let translation = self.top().rect.position;
        self.render().push();
        self.render().translate(translation);
        do_draw(self);
        self.render().pop();
    }

    /// Clips drawing to only occur inside of the current group.
    ///
    /// Any pixels outside of the group are discarded. Note that to undo the clip,
    /// [`self.render().push()`][Renderer::push] and [`self.render().pop()`][Renderer::pop] must be used.
    pub fn clip(&mut self) {
        let rect = self.top().rect;
        self.render().clip(rect);
    }

    /// Draws a rectangle that fills the current group with the given color.
    pub fn fill(&mut self, color: impl Into<Color>) {
        self.fill_rounded(color, 0.0);
    }

    /// Draws a rounded rectangle that fills the current group, with the given color and corner radius.
    pub fn fill_rounded(&mut self, color: impl Into<Color>, radius: f32) {
        let rect = self.top().rect;
        self.render().fill(rect, color.into(), radius);
    }

    /// Draws a rectangle outline that creates a border around the current group, with the given color and
    /// line thickness.
    pub fn outline(&mut self, color: impl Into<Color>, thickness: f32) {
        self.outline_rounded(color, 0.0, thickness);
    }

    /// Draws a rounded rectangle outline that creates a border around the current group, with the given color,
    /// corner radius, and line thickness.
    pub fn outline_rounded(&mut self, color: impl Into<Color>, radius: f32, thickness: f32) {
        let rect = self.top().rect;
        self.render().outline(rect, color.into(), radius, thickness);
    }

    /// Returns the current group's line cap.
    pub fn line_cap(&self) -> LineCap {
        self.top().line_cap
    }

    /// Sets the current group's line cap for rendering lines. The root group's default line cap is [`LineCap::Butt`].
    pub fn set_line_cap(&mut self, new_line_cap: LineCap) {
        self.top_mut().line_cap = new_line_cap;
    }

    /// Helper function for drawing borders around the current group.
    fn border(&mut self, a: Point, b: Point, color: Color, thickness: f32) {
        let line_cap = self.top().line_cap;
        self.render().line(a, b, color, line_cap, thickness);
    }

    /// Draws a line spanning the left side of the current group, with the given color and line thickness.
    pub fn border_left(&mut self, color: impl Into<Color>, thickness: f32) {
        let rect = self.top().rect;
        self.border(rect.top_left(), rect.bottom_left(), color.into(), thickness);
    }

    /// Draws a line spanning the top side of the current group, with the given color and line thickness.
    pub fn border_top(&mut self, color: impl Into<Color>, thickness: f32) {
        let rect = self.top().rect;
        self.border(rect.top_left(), rect.top_right(), color.into(), thickness);
    }

    /// Draws a line spanning the right side of the current group, with the given color and line thickness.
    pub fn border_right(&mut self, color: impl Into<Color>, thickness: f32) {
        let rect = self.top().rect;
        self.border(
            rect.top_right(),
            rect.bottom_right(),
            color.into(),
            thickness,
        );
    }

    /// Draws a line spanning the bottom side of the current group, with the given color and line thickness.
    pub fn border_bottom(&mut self, color: impl Into<Color>, thickness: f32) {
        let rect = self.top().rect;
        self.border(
            rect.bottom_left(),
            rect.bottom_right(),
            color.into(),
            thickness,
        );
    }

    /// Draws text inside of the current group, with the given color and alignment inside of the group's rectangle.
    ///
    /// # Panics
    /// If there are no groups on the stack, or if a font isn't set.
    pub fn text(
        &mut self,
        font: &T::Font,
        text: &str,
        color: impl Into<Color>,
        alignment: Alignment,
    ) {
        let rect = self.top().rect;
        self.render()
            .text(rect, font, text, color.into(), alignment);
    }
}

/// Any `Ui` instance acts as if it were the underlying renderer.
/// In case any conflicts occur (such as with [`Ui::text`] and [`Renderer::text`], [`Ui::render`] may be used to
/// specify that the renderer method should be called instead.
impl<T: Renderer> Deref for Ui<T> {
    /// The renderer type.
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.renderer
    }
}

/// Any mutable `Ui` instance acts as if it were the underlying renderer.
/// In case any conflicts occur (such as with [`Ui::text`] and [`Renderer::text`], [`Ui::renderer`] may be used to
/// specify that the renderer method should be called instead.
impl<T: Renderer> DerefMut for Ui<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.renderer
    }
}
