use crate::Result;

/// Trait for console I/O abstraction
pub trait Console {
    fn print(&self, message: &str) -> Result<()>;
    fn println(&self, message: &str) -> Result<()>;
    fn eprintln(&self, message: &str) -> Result<()>;
    fn read_line(&self, buffer: &mut String) -> Result<()>;
    fn flush(&self) -> Result<()>;
}

#[cfg(not(feature = "test-mocks"))]
mod real_impl {
    use super::*;
    use std::io::{self, Write};

    #[derive(Debug, Clone)]
    pub struct RealConsole;

    impl RealConsole {
        pub fn new() -> Self {
            Self
        }
    }

    impl Default for RealConsole {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Console for RealConsole {
        fn print(&self, message: &str) -> Result<()> {
            print!("{}", message);
            Ok(())
        }

        fn println(&self, message: &str) -> Result<()> {
            println!("{}", message);
            Ok(())
        }

        fn eprintln(&self, message: &str) -> Result<()> {
            eprintln!("{}", message);
            Ok(())
        }

        fn read_line(&self, buffer: &mut String) -> Result<()> {
            io::stdin().read_line(buffer)?;
            Ok(())
        }

        fn flush(&self) -> Result<()> {
            io::stdout().flush()?;
            Ok(())
        }
    }
}

#[cfg(feature = "test-mocks")]
mod mock_impl {
    use super::*;
    use std::cell::RefCell;

    #[derive(Debug, Clone)]
    pub struct MockConsole {
        pub output: RefCell<Vec<String>>,
        pub error_output: RefCell<Vec<String>>,
        pub input_lines: RefCell<Vec<String>>,
    }

    impl MockConsole {
        pub fn new() -> Self {
            Self {
                output: RefCell::new(Vec::new()),
                error_output: RefCell::new(Vec::new()),
                input_lines: RefCell::new(Vec::new()),
            }
        }

        pub fn set_input_lines(&self, lines: Vec<String>) {
            *self.input_lines.borrow_mut() = lines;
        }

        pub fn get_output(&self) -> Vec<String> {
            self.output.borrow().clone()
        }

        pub fn get_error_output(&self) -> Vec<String> {
            self.error_output.borrow().clone()
        }
    }

    impl Default for MockConsole {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Console for MockConsole {
        fn print(&self, message: &str) -> Result<()> {
            self.output.borrow_mut().push(message.to_string());
            Ok(())
        }

        fn println(&self, message: &str) -> Result<()> {
            self.output.borrow_mut().push(message.to_string());
            Ok(())
        }

        fn eprintln(&self, message: &str) -> Result<()> {
            self.error_output.borrow_mut().push(message.to_string());
            Ok(())
        }

        fn read_line(&self, buffer: &mut String) -> Result<()> {
            let mut input_lines = self.input_lines.borrow_mut();
            if let Some(line) = input_lines.first() {
                buffer.push_str(line);
                if !line.ends_with('\n') {
                    buffer.push('\n');
                }
                input_lines.remove(0);
            }
            Ok(())
        }

        fn flush(&self) -> Result<()> {
            Ok(())
        }
    }
}

#[cfg(not(feature = "test-mocks"))]
pub use real_impl::RealConsole as ConsoleImpl;

#[cfg(feature = "test-mocks")]
pub use mock_impl::MockConsole as ConsoleImpl;
