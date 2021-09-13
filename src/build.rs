//! Implementation of the `paws::build!` macro.

// used in doc comments
#[allow(unused)]
use crate::Ui;

/// Convenience macro for `push`ing and `pop`ping groups automatically.
///
/// Since it can be quite annoying to have to call [`Ui::push`] and [`Ui::pop`] manually, this macro aims to solve that
/// by offloading the work to the Rust compiler. It also helps show how groups are nested, further aiding readability.
///
/// # Syntax
///
/// The syntax for using this macro is:
/// ```
/// # use paws::{Ui, NoRenderer, Layout};
/// # struct Window;
/// # impl Window {
/// #     fn begin(&mut self, _ui: &mut Ui<NoRenderer>, (): ()) {}
/// #     fn end(&mut self) {}
/// # }
/// # let mut ui = Ui::new(NoRenderer);
/// # ui.root((800.0, 600.0), Layout::Freeform);
/// # let mut element = Window;
/// # let args = ();
/// paws::build! {
///     group ui => ((800.0, 600.0), Layout::Freeform) { }
///     process element, &mut ui => (args) { }
///     // or any old statement
/// }
/// ```
///
/// ## `group`
///
/// `group`'s primary use case is for pushing and popping UI groups. A `group` is transformed like so:
/// ```ignore
/// paws::build! {
///     group ui => ((300.0, 300.0), Layout::Freeform) {
///         println!("hi!");
///     }
/// }
/// // becomes
/// ui.push((300.0, 300.0), Layout::Freeform);
/// {
///     println!("hi!");
/// }
/// ui.pop();
/// ```
///
/// This module's examples use a real `Ui`, but `group` will work with anything that implements methods called `push`
/// and `pop`, so you can even use it with `Vec<T>` (although that doesn't mean you should):
/// ```
/// let mut stack: Vec<i32> = Vec::new();
/// paws::build! {
///     group stack => (3) {
///         assert_eq!(*stack.last().unwrap(), 3);
///     }
/// }
/// ```
///
/// ## `process`
///
/// `process`'s primary use case is for user elements that may nest other groups, like panels, menus, windows, etc.
/// Contrary to `group`, this will call methods called `begin` and `end`, as they better signify what is actually done.
/// Here's an example of how one might implement a window:
/// ```
/// # use paws::{Ui, NoRenderer, Layout};
/// use paws::Vector;
///
/// // of course, your window will contain actual data like its position, maybe also size, etc.
/// struct Window;
///
/// impl Window {
///     // for this example though, the size of the window will be defined by the caller.
///     fn begin(&mut self, _ui: &mut Ui<NoRenderer>, _size: impl Into<Vector>, _title: &str) {}
///     fn end(&mut self) {}
/// }
/// # let mut ui = Ui::new(NoRenderer);
/// # ui.root((800.0, 600.0), Layout::Freeform);
///
/// let mut window = Window;
/// paws::build! {
///     process window, &mut ui => ((300.0, 500.0), "Hello, world") {
///         // draw stuff inside the window
///     }
/// }
/// ```
///
/// The reason why the element comes before the UI instance is to aid readability. In this case, it's clear that which
/// element is being processed is more important than the UI instance that's being used, as there's usually only one
/// of the latter, while there may be many other elements present.
///
/// A `process` is transformed just like a `group`, with a few key differences:
/// ```ignore
/// paws::build! {
///     process window, &mut ui => ((300.0, 500.0), "Hello, world") {
///         println!("inside the window!");
///     }
/// }
/// // becomes
/// window.begin(&mut ui, (300.0, 500.0), "Hello, world");
/// {
///     println!("inside the window!");
/// }
/// window.end();
/// ```
///
/// The first, probably most noticable difference, is that the method names are `begin` and `end`, instead of `push` and
/// `pop`, as previously mentioned. The second difference is that `begin` receives the UI instance before all other
/// parameters.
#[macro_export]
macro_rules! build {
    ( group $ui:expr => $args:tt $then:tt $($rest:tt)* ) => {
        $ui.push $args;
        $crate::build! $then
        $ui.pop();
        $crate::build!($($rest)*)
    };
    ( process $element:expr , $ui:expr => ( $($args:expr),* ) $then:tt $($rest:tt)* ) => {
        $element.begin($ui, $($args,)*);
        $crate::build! $then
        $element.end();
        $crate::build!($($rest)*)
    };
    ( $statement:stmt ; $($rest:tt)* ) => {
        $statement
        $crate::build!($($rest)*)
    };
    () => {};
}

#[cfg(test)]
mod tests {
    use crate::{Ui, NoRenderer, Layout, Vector};

    #[test]
    fn empty() {
        let mut ui = Ui::new(NoRenderer);

        ui.root((800.0, 600.0), Layout::Vertical);
        build! {}
    }

    #[test]
    fn just_statements() {
        let mut ui = Ui::new(NoRenderer);

        ui.root((800.0, 600.0), Layout::Vertical);
        build! {
            println!("hello");
            println!("world");
            println!("my name jeff");
        }
    }

    #[test]
    fn group() {
        let mut ui = Ui::new(NoRenderer);

        ui.root((800.0, 600.0), Layout::Vertical);
        build! {
            println!("in build!");
            group ui => (ui.size(), Layout::Freeform) {
                println!("in group");
            }
            println!("after group");
        }
    }

    #[test]
    fn nested_group() {
        let mut ui = Ui::new(NoRenderer);
        ui.root((800.0, 600.0), Layout::Freeform);
        build! {
            group ui => (ui.size(), Layout::Vertical) {
                println!("before nest");
                group ui => (ui.size(), Layout::Horizontal) {
                    println!("nesting tested!");
                }
                println!("after nest");
            }
        }
    }

    struct Window;

    impl Window {
        fn begin(&mut self, _ui: &mut Ui<NoRenderer>, _size: Vector, _title: &str) {}
        fn end(&mut self) {}
    }

    #[test]
    fn process() {
        let mut ui = Ui::new(NoRenderer);
        let mut window = Window;
        ui.root((800.0, 600.0), Layout::Freeform);
        build! {
            let size = ui.size();
            process window, &mut ui => (size, "Hello") {
                println!("inside of window");
                group ui => ((size / 2.0), Layout::Vertical) {
                    println!("group inside of window");
                }
            }
        }
    }
}

