**Still WIP**

This repository is a simple demonstration of Spin's component dependencies features.
This example uses two component `cat-facts-app` and `cat-facts-dep` where the former
uses the latter as a dependency.

The `cat-facts-app` component targets this world:
```
package component:cat-facts;

interface cat-facts {
    /// Get a random cat fact.
    get-random-fact: func() -> result<string, string>;
}

world cat-facts-app {
    import cat-facts;
}
```

While the `cat-facts-dep` targets this world:
```
package component:cat-facts;

interface cat-facts {
    /// Get a random cat fact.
    get-random-fact: func() -> result<string, string>;
}

/// An example world for the component to target.
world cat-facts-dep {
    export cat-facts;
}
```

If you look at the `spin.toml` you will see the `spin-cat-facts` component specifies
a dependency on the `cat-facts-dep` component. Furthermore the `spin-cat-facts` component
enable configuration inheritance via the `dependencies_inherit_configuration` field allowing
the `cat-facts-dep` component to inherit configuration defined by the `spin-cat-facts` component.

Additionally there is a `spin.deny.toml` which expresses the same dependency relationship described above
but omits the `dependencies_inherit_configuration` field. While composing Spin will enforce this isolation
by applying the `spin-virt-deny-all` adapter to each dependency.


TODO: Write `cat-facts-dep` so it uses an `allowed_outbound_hosts` configuration and demonstrate that the configuration inheritance toggle is enforced to allow or disallow access.

TODO: Move this into spin/examples