//------------------------------------------------------------------------------
/// # Sokoke Route
//------------------------------------------------------------------------------

use hyper::Method;

use super::Path;
use super::Handler;


//------------------------------------------------------------------------------
/// Route
//------------------------------------------------------------------------------
pub struct Route
{
    method: Method,
    path: Path,
    handler: Handler,
}

impl Route
{
    //--------------------------------------------------------------------------
    /// Create a new route for OPTIONS.
    //--------------------------------------------------------------------------
    pub fn options( path: &str, handler: Handler ) -> Self
    {
        Self::from(Method::OPTIONS, path, handler)
    }

    //--------------------------------------------------------------------------
    /// Create a new route for GET.
    //--------------------------------------------------------------------------
    pub fn get( path: &str, handler: Handler ) -> Self
    {
        Self::from(Method::GET, path, handler)
    }

    //--------------------------------------------------------------------------
    /// Create a new route for POST.
    //--------------------------------------------------------------------------
    pub fn post( path: &str, handler: Handler ) -> Self
    {
        Self::from(Method::POST, path, handler)
    }

    //--------------------------------------------------------------------------
    /// Create a new route for PUT.
    //--------------------------------------------------------------------------
    pub fn put( path: &str, handler: Handler ) -> Self
    {
        Self::from(Method::PUT, path, handler)
    }

    //--------------------------------------------------------------------------
    /// Create a new route for DELETE.
    //--------------------------------------------------------------------------
    pub fn delete( path: &str, handler: Handler ) -> Self
    {
        Self::from(Method::DELETE, path, handler)
    }

    //--------------------------------------------------------------------------
    /// Create a new route for HEAD.
    //--------------------------------------------------------------------------
    pub fn head( path: &str, handler: Handler ) -> Self
    {
        Self::from(Method::HEAD, path, handler)
    }

    //--------------------------------------------------------------------------
    /// Create a new route for TRACE.
    //--------------------------------------------------------------------------
    pub fn trace( path: &str, handler: Handler ) -> Self
    {
        Self::from(Method::TRACE, path, handler)
    }

    //--------------------------------------------------------------------------
    /// Create a new route for CONNECT.
    //--------------------------------------------------------------------------
    pub fn connect( path: &str, handler: Handler ) -> Self
    {
        Self::from(Method::CONNECT, path, handler)
    }

    //--------------------------------------------------------------------------
    /// Create a new route for PATCH.
    //--------------------------------------------------------------------------
    pub fn patch( path: &str, handler: Handler ) -> Self
    {
        Self::from(Method::PATCH, path, handler)
    }

    //--------------------------------------------------------------------------
    /// Create a new route.
    //--------------------------------------------------------------------------
    pub fn from( method: Method, path: &str, handler: Handler ) -> Self
    {
        Self
        {
            method,
            path: Path::new(path),
            handler,
        }
    }
}

