#[cfg(test)]
mod tests {
    use gittype::infrastructure::console::{Console, ConsoleImpl};

    #[test]
    fn test_mock_console_new() {
        let console = ConsoleImpl::new();
        assert!(console.get_output().is_empty());
        assert!(console.get_error_output().is_empty());
    }

    #[test]
    fn test_mock_console_default() {
        let console = ConsoleImpl::default();
        assert!(console.get_output().is_empty());
    }

    #[test]
    fn test_print() {
        let console = ConsoleImpl::new();
        console.print("hello").unwrap();
        assert_eq!(console.get_output(), vec!["hello"]);
    }

    #[test]
    fn test_println() {
        let console = ConsoleImpl::new();
        console.println("world").unwrap();
        assert_eq!(console.get_output(), vec!["world"]);
    }

    #[test]
    fn test_eprintln() {
        let console = ConsoleImpl::new();
        console.eprintln("error msg").unwrap();
        assert_eq!(console.get_error_output(), vec!["error msg"]);
    }

    #[test]
    fn test_print_multiple_messages() {
        let console = ConsoleImpl::new();
        console.print("a").unwrap();
        console.println("b").unwrap();
        assert_eq!(console.get_output(), vec!["a", "b"]);
    }

    #[test]
    fn test_eprintln_multiple_messages() {
        let console = ConsoleImpl::new();
        console.eprintln("e1").unwrap();
        console.eprintln("e2").unwrap();
        assert_eq!(console.get_error_output(), vec!["e1", "e2"]);
    }

    #[test]
    fn test_flush() {
        let console = ConsoleImpl::new();
        assert!(console.flush().is_ok());
    }

    #[test]
    fn test_read_line_with_input() {
        let console = ConsoleImpl::new();
        console.set_input_lines(vec!["line1".to_string(), "line2".to_string()]);

        let mut buf = String::new();
        console.read_line(&mut buf).unwrap();
        assert_eq!(buf, "line1\n");

        let mut buf2 = String::new();
        console.read_line(&mut buf2).unwrap();
        assert_eq!(buf2, "line2\n");
    }

    #[test]
    fn test_read_line_with_newline_suffix() {
        let console = ConsoleImpl::new();
        console.set_input_lines(vec!["already\n".to_string()]);

        let mut buf = String::new();
        console.read_line(&mut buf).unwrap();
        assert_eq!(buf, "already\n");
    }

    #[test]
    fn test_read_line_empty_input() {
        let console = ConsoleImpl::new();
        let mut buf = String::new();
        console.read_line(&mut buf).unwrap();
        assert_eq!(buf, "");
    }

    #[test]
    fn test_set_input_lines_replaces() {
        let console = ConsoleImpl::new();
        console.set_input_lines(vec!["first".to_string()]);
        console.set_input_lines(vec!["replaced".to_string()]);

        let mut buf = String::new();
        console.read_line(&mut buf).unwrap();
        assert_eq!(buf, "replaced\n");
    }

    #[test]
    fn test_output_isolation() {
        let console = ConsoleImpl::new();
        console.print("stdout").unwrap();
        console.eprintln("stderr").unwrap();

        assert_eq!(console.get_output(), vec!["stdout"]);
        assert_eq!(console.get_error_output(), vec!["stderr"]);
    }
}
