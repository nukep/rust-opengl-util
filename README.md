An OpenGL utility library that I conjured up for my own purposes.
It aims to simplify common OpenGL tasks.

**This is not a crate!**
This is meant to be used as a module inside of the project.
The reason for this is that it requires access to generated OpenGL bindings
within the project.

This library uses `gl-rs` to communicate with OpenGL.
`gl-rs` includes a feature to generate bindings for specific OpenGL versions,
whether it be for OpenGL 2.1, OpenGL ES 3.0, etc.
In other words, there's isn't a single OpenGL crate that can be relied on because
we can't rely on a single set of OpenGL bindings.

At the root of the project crate, the `gl` module/crate must be defined.
If generated, it must be generated as static or global.

To use this library from a Git project, add this repository as a Git submodule.
If you don't use Git, simply add the files manually to your project.
