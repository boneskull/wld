# wld

`wld` is a library which will read and write Terraria world (`.wld`) files.

## Layout

Models (structs) live in the [`models`] module, and enums live in the
[`enums`] module.  Additionally, some constants (corresponding to colors
used by the [map-rendering functionality](World::render)) are available in
the [`constants`] module.

You shouldn't need to instantiate _any_ struct or model directly.  Instead,
use [`parse_world`] to create a [`World`] instance, and use the methods
and properties on that instance.

License: Apache-2.0
