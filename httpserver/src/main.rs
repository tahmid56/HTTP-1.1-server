use server::Server;
mod handler;
mod server;
mod router;

fn main() {
    let server = Server::new("127.0.0.1:3000");
    server.run();
}
