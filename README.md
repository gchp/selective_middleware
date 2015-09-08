# Iron Selective Middleware

This crate allows you to selectively add middleware to specific handlers in an
Iron application.

## Motivation

I needed a way to add middleware to specific Iron routes. Using the Iron
Router and Chain, it was only possible to add middleware to the entire Router
as a whole. I created this crate to allow middleware to be added to any handler
in the application.

The use case I had was authentication middleware. I needed to add this
middleware to every route which required the user to be authenticated. I didn't
want to add it to the login handler, though. SelectiveMiddleWare solves this
problem for me.

## Installation

Add the following to the `[dependencies]` section of you  `Cargo.toml`:

```toml
selective_middleware = "*"
```

## Usage

```rust
extern crate iron;
extern crate router;
extern crate selective_middleware;

use iron::{Iron, status, IronResult, Request};
use router::Router;
use selective_middleware::SelectiveMiddleWare;

fn handler(req: &mut Request) -> IronResult<()> {
    Ok(Response::with((status::Ok, "Hello"))
}

fn main() {
    let mut router = Router::new();

    router.get("/", handler);
    router.get("/foo", SelectiveMiddleWare::new(handler, MyMiddleware));

    Iron::new(router).http("localhost:3000").unwrap();
}
```

With the above example, visiting `http://localhost:3000/` will invoke the
`handler` method. Visiting `http://localhost:3000/foo` will invoke `MyMiddleware`
then the `handler` method.

There is also the `with_middleware!` convenience macro:

```rust
#[macro_use(with_middleware)]
extern crate selective_middleware;

fn main() {
    let mut router = Router::new();

    router.get("/", handler);
    router.get("/foo", with_middleware!(handler, MyMiddleware));

    Iron::new(router).http("localhost:3000").unwrap();
}
```
