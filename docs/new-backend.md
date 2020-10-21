## Creating a New Backend

All the code generation backends are in the compiler directory. To create a new backend, the easiest thing to do is to use the example backend (which doesn't do anything...). I really want all the official backends to follow the same pattern, so I will not include any new backends that do not. This does not mean they have to implement every single instruction, however.

### The Process

First, copy and rename the "example" directory in the compiler folder. Open the "Cargo.toml" in your new backend and change the name.

Next, open the root level "Cargo.toml" and add the backend 1) as a dependency, and 2) in the workspace.

Finally, open the the Dash entry point (at the time of writing, "src/main.rs" and add an option to invoke your new backend). You'll have to add your architecture to the Arch enum, which is in the transform layer. If your architecture needs RISC optimization, update "run" function in "transform/src/lib.rs" accordingly.


