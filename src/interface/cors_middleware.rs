use interface::unicase::UniCase;
use interface::hyper::header::{AccessControlAllowOrigin, AccessControlAllowMethods, AccessControlAllowHeaders};
use interface::hyper::method::Method;
use interface::iron::prelude::*;
use interface::iron::middleware::AfterMiddleware;

pub struct CorsMiddleware {}

impl AfterMiddleware for CorsMiddleware {
    fn after(&self, req: &mut Request, mut res: Response) -> IronResult<Response> {
        add_cors(req, &mut res);
        Ok(res)
    }

    fn catch(&self, req: &mut Request, mut err: IronError) -> IronResult<Response> {
        add_cors(req, &mut err.response);
        Err(err)
    }
}

fn add_cors(_: &Request, res: &mut Response) {
    res.headers.set(
        AccessControlAllowOrigin::Any
    );
    res.headers.set(
        AccessControlAllowMethods(vec![
        Method::Get,
        Method::Post,
        Method::Put,
        Method::Patch,
        Method::Delete,
        Method::Options,
        Method::Connect,
        Method::Head,
        Method::Trace
        ])
    );

    // List from https://github.com/ottoyiu/django-cors-headers/
    let allowed_headers: Vec<UniCase<String>> = vec![
    "accept",
    "accept-encoding",
    "authorization",
    "content-type",
    "dnt",
    "origin",
    "user-agent",
    "x-csrftoken",
    "x-requested-with",
    ].iter().map(|h| UniCase(h.to_string())).collect();
    res.headers.set(
        AccessControlAllowHeaders(allowed_headers)
    );
}
