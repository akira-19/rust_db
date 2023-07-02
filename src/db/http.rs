use std::io::Read;
use tiny_http::{Method, Request, Response, Server};

struct ApiServer {
    db: BogoDb,
}

impl ApiServer {
    fn new(db: BogoDb) -> ApiServer {
        ApiServer { db }
    }

    fn execute_handler(&self, request: Request) {
        match (request.method(), request.url()) {
            (&Method::Get, "/execute") => {
                let mut query = String::new();
                request.as_reader().read_to_string(&mut query).unwrap();

                // 実行処理
                let response = self
                    .db
                    .execute(&query, request.headers()["User-Agent"].to_string());

                request.respond(Response::from_string(response)).unwrap();
            }
            _ => {
                request.respond(Response::empty(404)).unwrap();
            }
        }
    }

    fn exit_handler(&self, request: Request) {
        if request.url() == "/exit" {
            // 終了処理
            self.db.terminate();

            request.respond(Response::empty(200)).unwrap();
        } else {
            request.respond(Response::empty(404)).unwrap();
        }
    }

    fn host(&self) {
        let server = Server::http("0.0.0.0:32198").unwrap();
        println!("Server started at http://localhost:32198");

        for request in server.incoming_requests() {
            match request.url() {
                "/execute" => self.execute_handler(request),
                "/exit" => self.exit_handler(request),
                _ => {
                    request.respond(Response::empty(404)).unwrap();
                }
            }
        }
    }
}
