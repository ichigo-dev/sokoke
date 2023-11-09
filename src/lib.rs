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
/// use sokoke::App;
///
/// fn main()
/// {
///    let app = App::new();
///    app.run();
/// }
/// ```
//------------------------------------------------------------------------------

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
            let (_stream, _remote_addr) = tcp_listener.accept().await.unwrap();
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
    /// Tests the application.
    //--------------------------------------------------------------------------
    #[test]
    fn test_app()
    {
        let app = App::new();
        app.run();
    }
}
