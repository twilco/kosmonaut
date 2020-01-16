## Kosmonaut

[![Build Status](https://travis-ci.com/twilco/kosmonaut.svg?branch=master)](https://travis-ci.com/twilco/kosmonaut) [![Join the chat at https://gitter.im/kosmonaut-browser/community](https://badges.gitter.im/kosmonaut-browser/community.svg)](https://gitter.im/kosmonaut-browser/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Kosmonaut is a web browser prioritizing speed and privacy, serving as the vehicle for your journey across the world wide web.

> The road to the stars is steep and dangerous.  But we're not afraid...space flights can't be stopped.
> 
> \- Yuri Gagarin

### Build

Kosmonaut is built with Rust using OpenGL bindings via [gl-rs](https://github.com/brendanzab/gl-rs), [Glutin](https://github.com/rust-windowing/glutin) for window management and OpenGL context creation, Servo's [html5ever](https://github.com/servo/html5ever) and [cssparser](https://github.com/servo/rust-cssparser) for HTML and CSS parsing, and various other auxiliary libraries.

To build from source:

1. Install Rust: https://www.rust-lang.org/tools/install
2. `cargo build`
 
### License and credits

Kosmonaut's current implementation is heavily inspired by [Servo](https://github.com/servo/servo), sometimes taking code directly from it.  Thus, Kosmonaut is licensed with the [Mozilla Public License 2.0](https://www.mozilla.org/en-US/MPL/2.0/).

Kosmonaut also takes inspiration from [Robinson](https://github.com/mbrubeck/robinson).  Thanks to [mbrubeck](https://github.com/mbrubeck) for their great series of articles on browser engines.

Finally, Kosomonaut's DOM implementation was taken from [Kuchiki](https://github.com/kuchiki-rs/kuchiki) and has been slightly modified to fit our needs.
