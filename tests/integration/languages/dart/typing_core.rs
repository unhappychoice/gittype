use crate::typing_core_test_with_parser;

typing_core_test_with_parser!(
    dart_function_with_comment,
    "dart",
    r#"int add(int a, int b) {
  return a + b; // Return sum
}"#
);

typing_core_test_with_parser!(
    dart_class_with_comments,
    "dart",
    r#"class Calculator {
  int _result = 0; // Store result
  
  Calculator add(int value) { // Add method
    _result += value;
    return this;
  }
  
  int get result {
    return _result; // Return current result
  }
}"#
);

typing_core_test_with_parser!(
    dart_widget_with_comments,
    "dart",
    r#"class MyWidget extends StatelessWidget {
  final String title; // Widget title
  
  const MyWidget({Key? key, required this.title}) : super(key: key);
  
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(title), // Set app bar title
      ),
      body: Center(
        child: Text('Hello, World!'), // Main content
      ),
    );
  }
}"#
);

typing_core_test_with_parser!(
    dart_async_function_with_comments,
    "dart",
    r#"Future<String> fetchData(String url) async {
  /* Fetch data from URL
     and return response */
  try {
    final response = await http.get(Uri.parse(url));
    return response.body; // Return response body
  } catch (e) {
    return 'Error: $e'; // Return error message
  }
}"#
);

typing_core_test_with_parser!(
    dart_enum_with_comments,
    "dart",
    r#"enum Status {
  loading,  // Loading state
  success,  // Success state
  error,    // Error state
}

extension StatusExtension on Status {
  String get message {
    switch (this) {
      case Status.loading:
        return 'Loading...';      // Loading message
      case Status.success:
        return 'Success!';        // Success message
      case Status.error:
        return 'Error occurred';  // Error message
    }
  }
}"#
);
