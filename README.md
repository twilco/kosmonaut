## Kosmonaut

Kosmonaut is a web browser prioritizing speed and privacy, serving as the vehicle for your journey across the world wide web.

> The road to the stars is steep and dangerous.  But we're not afraid...space flights can't be stopped.
> 
> \- Yuri Gagarin

### Build

Kosmonaut is built with Rust using bindings to [GTK](https://www.gtk.org/) via [gtk-rs](https://gtk-rs.org/).

1. Install Rust: https://www.rust-lang.org/tools/install
2. Install GTK: `brew install gtk+3` on MacOS, `sudo apt install libgtk-3-dev` on Linux
3. Attempt a `cargo build`.  If presented with an error message about `libffi` not being found in the pkg-config search path, run:
 
 ```bash
brew install libffi 
# Change this if your path is different.  
# Run `brew info libffi` for more information.
export PKG_CONFIG_PATH="/usr/local/opt/libffi/lib/pkgconfig"
 ```

### License

Kosmonaut's current implementation is heavily inspired by [Servo](https://github.com/servo/servo), sometimes taking code directly from it.  Thus, Kosmonaut is licensed with the [Mozilla Public License 2.0](https://www.mozilla.org/en-US/MPL/2.0/).
