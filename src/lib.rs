#![allow(dead_code)]

//------------------------------------------------------------------------------
/// # Sokoke
///
/// Web application framework for Rust.
///
///
/// ## Usage
///
/// ```rust
/// use sokoke::{ App, Router, Body, Request, Response } };
///
/// async fn handler( req: Request<Body> ) -> Response<Body>
/// {
///    Response::new(Body::from("Hello World!"))
/// }
///
/// fn main()
/// {
///    let router = Router::new()
///        .add(Route::get("/", handler));
///
///    let app = App::new();
///    app.run();
/// }
/// ```
//------------------------------------------------------------------------------

mod router;

pub use router::{ Router, Route };

pub use hyper::{ Body, Request, Response };

use std::env;

use tokio::runtime::Builder;
use tokio::net::TcpListener;


//------------------------------------------------------------------------------
/// Application
//------------------------------------------------------------------------------
pub struct App
{
}

impl App
{
    //--------------------------------------------------------------------------
    /// Creates a new Sokoke application.
    //--------------------------------------------------------------------------
    pub fn new() -> Self
    {
        Self {}
    }

    //--------------------------------------------------------------------------
    /// Returns the version of the application.
    //--------------------------------------------------------------------------
    pub fn version( &self ) -> String
    {
        env::var("CARGO_PKG_VERSION")
            .unwrap_or_else(|_| String::from("unknown"))
    }

    //--------------------------------------------------------------------------
    /// Runs the application.
    //--------------------------------------------------------------------------
    pub fn run( &self )
    {
        // Creates a TCP listener which will listen for incoming connections.

        // Creates a multi-threaded Tokio runtime.
        let runtime = Builder::new_multi_thread()
            .enable_all()
            .worker_threads(4)
            .build()
            .unwrap();
        runtime.block_on(async
        {
            println!("sokoke: {}", self.version());

            // Starts listening for incoming connections.
            let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
            self.serve(listener).await;
        });
    }

    //--------------------------------------------------------------------------
    /// Serves the service with the given listener.
    //--------------------------------------------------------------------------
    async fn serve( &self, tcp_listener: TcpListener )
    {
        loop
        {
            // Accepts a new connection.
            let (_socket, _addr) = match tcp_listener.accept().await
            {
                Ok(ok) => ok,
                Err(err) =>
                {
                    println!("Socket Error: {}", err);
                    continue;
                }
            };

            tokio::spawn(async {});
        }
    }
}


//------------------------------------------------------------------------------
/// Tests
//------------------------------------------------------------------------------
#[cfg(test)]
mod tests
{
    use super::*;

    //--------------------------------------------------------------------------
    /// Test handler.
    //--------------------------------------------------------------------------
    async fn test_handler( req: Request<Body> ) -> Response<Body>
    {
        Response::new(Body::from("Hello World!"))
    }

    //--------------------------------------------------------------------------
    /// Tests the application.
    //--------------------------------------------------------------------------
    #[test]
    fn test_app()
    {
        let router = Router::new()
            .add(Route::get("/", test_handler));

        let app = App::new();
        app.run();
    }
}
