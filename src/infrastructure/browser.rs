#[cfg(not(feature = "test-mocks"))]
mod real_impl {
    /// Opens a URL in the default browser.
    pub fn open_url(url: &str) -> Result<(), Box<dyn std::error::Error>> {
        open::that(url)?;
        Ok(())
    }
}

#[cfg(feature = "test-mocks")]
mod mock_impl {
    /// Mock implementation that doesn't actually open browsers
    pub fn open_url(_url: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

#[cfg(not(feature = "test-mocks"))]
pub use real_impl::open_url;

#[cfg(feature = "test-mocks")]
pub use mock_impl::open_url;
