use interface::hyper::header::{AccessControlAllowOrigin};
use interface::iron::prelude::*;
use interface::iron::middleware::AfterMiddleware;

pub struct CorsMiddleware {}

impl AfterMiddleware for CorsMiddleware {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
        add_cors(&mut res);
        Ok(res)
    }

    fn catch(&self, _: &mut Request, mut err: IronError) -> IronResult<Response> {
        add_cors(&mut err.response);
        Err(err)
    }
}

fn add_cors(res: &mut Response) {
    res.headers.set(
        AccessControlAllowOrigin::Any
    );
}
