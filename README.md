**Still WIP**

This repository is a simple demonstration of Spin's component dependencies feature.
This example uses two component `animal-facts-app` and `animal-facts-dep` where the former
uses the latter as a dependency.

The `animal-facts-app` component targets this world:
```
package component:animal-facts;

interface animal-facts {
    /// Get a random animal fact.
    get-random-fact: func() -> result<string, string>;
}

world animal-facts-app {
    import animal-facts;
}
```

While the `animal-facts-dep` targets this world:
```
package component:animal-facts;

interface animal-facts {
    /// Get a random animal fact.
    get-random-fact: func() -> result<string, string>;
}

world animal-facts-dep {
    export animal-facts;
}
```

If you look at the `spin.toml` you will see the `spin-animal-facts` component specifies
a dependency on the `animal-facts-dep` component. Furthermore the `animal-cat-facts` component
enable configuration inheritance via the `dependencies_inherit_configuration` field allowing
the `animal-facts-dep` component to inherit configuration defined by the `animal-cat-facts` component.

TODO: Move this into spin/examples

To build and run:
```
# Build the animal-facts-dep
$ cargo component build --release --manifest-path animal-facts-dep/Cargo.toml
$ spin up
```