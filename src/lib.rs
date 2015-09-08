extern crate iron;

use iron::{Handler, IronResult, BeforeMiddleware, Request, Response, Chain};

pub struct SelectiveMiddleWare {
    chain: Chain,
}

impl SelectiveMiddleWare {
    /// Create a new SelectiveMiddleWare instance with the given BeforeMiddleware.
    pub fn new<H: Handler, M: BeforeMiddleware>(handler: H, m: Vec<M>) -> Self {

        let mut chain = Chain::new(handler);
        for item in m.into_iter() {
            chain.link_before(item);
        }

        SelectiveMiddleWare {
            chain: chain
        }
    }
}

impl Handler for SelectiveMiddleWare {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        self.chain.handle(req)
    }
}

#[macro_export]
macro_rules! with_middleware {
    ($handler:ident, before=[$($y:ident),+] ) => {
        {
            use selective_middleware::SelectiveMiddleWare;
            let before = vec!($( $y  ),*);

            SelectiveMiddleWare::new($handler, before)
        }
    };
    ($handler:ident, [$($y:ident),+] ) => {
        with_middleware!($handler, before=[$( $y  ),*])
    }
}
