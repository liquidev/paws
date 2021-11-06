# paws

paws is a very simple, bring-your-own-backend UI library built for quick
prototyping, a small memory footprint, and easy embedding within existing
projects.

This project is still a work in progress, expect breaking changes as the API
is not final.

```rs
use paws::rgb;

type Ui = paws::Ui<MyRenderer>;

let mut ui = Ui::new(MyRenderer::new());

ui.root((800, 600), Layout::Freeform);

ui.push();
ui.pad(8);

ui.fill(rgb(0, 127, 255));

// draw more components here

ui.pop();
```

The whole idea behind paws is that the layout is built at the same time as it's drawn, by using
a stack of rectangles with extra metadata – **groups**.

For more information on how to lay out elements, please read the [documentation](https://docs.rs/paws/latest/paws/struct.Ui.html).
