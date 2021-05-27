use fastly::http::{header, StatusCode};
use fastly::{Error, Request, Response};

const BACKEND_NAME: &str = "github";

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    // Make any desired changes to the client request.
    req.set_header(header::HOST, "doramatadora.github.io");
    req.set_pass(true);
    req.set_path(&format!("/edgeml{}", &req.get_url().path()));
    // Pattern match on the path.
    match req.get_url().path() {
        "/edgeml/.well-known/fastly/demo-manifest" => Ok(Response::from_status(StatusCode::OK)
            .with_body_text_plain(include_str!("./demo-manifest"))),
        _ => Ok(req.send(BACKEND_NAME)?),
    }
}
