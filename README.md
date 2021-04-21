# paws

paws is a very simple, bring-your-own-backend UI library built for quick
prototyping, a small memory footprint, and easy embedding within existing
projects.

This project is still a work in progress, expect breaking changes as the API
is not final.

## Roadmap

- Actually test if it works! Most of the code is taken directly from
  [NetCanv](https://github.com/liquidev/netcanv), but tweaked to be more general.
  I haven't tested these tweaks yet as I don't have a graphical backend at hand.
- `paws-skia` – standalone Skia backend
- `paws-build` – attribute macro that helps you `push()` and `pop()` groups
- generalizations in `paws::Renderer`
