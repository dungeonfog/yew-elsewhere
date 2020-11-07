/*!
Because at some point, you'll need to modify something at a different level in the DOM.

This is a fairly simple yew library; heavily inspired by [ember-elsewhere](https://github.com/ef4/ember-elsewhere),
it allows you to place a receiver at some place in the DOM, and then send any
`VNode` to it, which will then be rendered in its place.

This is useful in a lot of scenarios; for example, if you are already integrating
with some global JavaScript library that yields events, you don't have to define
your own agent, but can just place an Elsewhere component somewhere down and send
the values to display to it.

As another example (and the main usecase), depending on which stack you use, you
might not always be in full control over your CSS; as soon as you're using
non-static `position`ing and a bit of `overflow: hidden` somewhere, you'll not be
able to make elements overlay their direct or indirect parents.

In this case, you can use elsewhere to create a global component that places
stuff at a higher place in the DOM, possibly with its own Agent to make it easier.

Examples will be included at some point.

# Implementation
Currently, this crate comes in two parts: The [`ElsewhereService`],
which each component calls upon to register itself, and is subsequently used to
send VNodes to registered components. It is currently implemented as a Singleton,
but with upcoming changes making it possible for Agents to not require their
input to be Serialize and Deserialize, we might soon reimplement it as an Agent.

The other part is the component [`Elsewhere`], which you can place anywhere in
your component hierarchy:

```rust
# use yew::html;
use yew_elsewhere::Elsewhere;
// ...
html! { <>
    <div class="wrapper">
        // Other components...
    </div>
    <Elsewhere name="tooltip" />
    <Elsewhere name="global-dialog" />
    // etc.
</> }
# ;
```

In a different part of the hierarchy, you can then send anything to it:

```rust
# use yew::html;
use yew_elsewhere::ElsewhereService;
// ...
# let content = "";
ElsewhereService::get().send("tooltip", html! {
    <span class="tooltip">
        { content }
    </span>
});
```

This will cause the component named `"tooltip"` to replace its current body and
rerender immediately.

You can also implement your own component, since ElsewhereService exposes the
necessary methods ([`ElsewhereService::register_component`] and
[`ElsewhereService::unregister_component`]). Refer to the method docs or the
code for the [`Elsewhere`] component for more information on how to use them.
*/

mod service;
pub use service::ElsewhereService;
mod component;
pub use component::Elsewhere;
