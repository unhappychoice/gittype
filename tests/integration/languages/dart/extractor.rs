use crate::integration::languages::extractor::test_language_extractor;

test_language_extractor! {
    name: test_dart_function_extraction,
    language: "dart",
    extension: "dart",
    source: r#"
String greet(String name) {
  return 'Hello, $name!';
}

int calculateSum(int a, int b) {
  return a + b;
}

void processData(List<String> data) {
  data.forEach(print);
}
"#,
    total_chunks: 4,
    chunk_counts: {
        Function: 3,
    }
}

test_language_extractor! {
    name: test_dart_class_extraction,
    language: "dart",
    extension: "dart",
    source: r#"
class Person {
  String name;
  int age;

  Person(this.name, this.age);

  String greet() {
    return 'Hello, I am $name!';
  }

  bool get isAdult => age >= 18;
}

class Calculator {
  int add(int a, int b) => a + b;

  int multiply(int a, int b) {
    return a * b;
  }
}
"#,
    total_chunks: 7,
    chunk_counts: {
        Class: 2,
        CodeBlock: 1,
        Function: 3,
    }
}

test_language_extractor! {
    name: test_dart_mixin_extraction,
    language: "dart",
    extension: "dart",
    source: r#"
mixin Flyable {
  void fly() {
    print('Flying...');
  }
}

mixin Swimmable {
  void swim() {
    print('Swimming...');
  }
}

class Duck with Flyable, Swimmable {
  void quack() {
    print('Quack!');
  }
}
"#,
    total_chunks: 8,
    chunk_counts: {
        Class: 3,
        CodeBlock: 1,
        Function: 3,
    }
}

test_language_extractor! {
    name: test_dart_abstract_class_extraction,
    language: "dart",
    extension: "dart",
    source: r#"
abstract class Shape {
  double get area;
  void draw();

  void describe() {
    print('This shape has area: $area');
  }
}

class Circle extends Shape {
  double radius;

  Circle(this.radius);

  @override
  double get area => 3.14159 * radius * radius;

  @override
  void draw() {
    print('Drawing a circle');
  }
}
"#,
    total_chunks: 7,
    chunk_counts: {
        Class: 2,
        CodeBlock: 2,
        Function: 2,
    }
}

test_language_extractor! {
    name: test_dart_enum_extraction,
    language: "dart",
    extension: "dart",
    source: r#"
enum Color {
  red,
  green,
  blue;

  String get name {
    switch (this) {
      case Color.red:
        return 'Red';
      case Color.green:
        return 'Green';
      case Color.blue:
        return 'Blue';
    }
  }
}

enum Status { pending, completed, failed }
"#,
    total_chunks: 5,
    chunk_counts: {
        CodeBlock: 1,
        Conditional: 1,
        Enum: 2,
    }
}

test_language_extractor! {
    name: test_dart_variable_extraction,
    language: "dart",
    extension: "dart",
    source: r#"
const String APP_NAME = 'MyApp';
const int VERSION = 1;

final String sessionId = generateId();
final List<String> items = [];

var globalCounter = 0;
var isLoggedIn = false;

late String userId;
late DatabaseConnection db;
"#,
    total_chunks: 9,
    chunk_counts: {
        Variable: 8,
    }
}

test_language_extractor! {
    name: test_dart_extension_extraction,
    language: "dart",
    extension: "dart",
    source: r#"
extension StringExtensions on String {
  bool get isEmail {
    return contains('@') && contains('.');
  }

  String capitalize() {
    if (isEmpty) return this;
    return this[0].toUpperCase() + substring(1);
  }
}

extension IntExtensions on int {
  bool get isEven => this % 2 == 0;
  bool get isOdd => !isEven;

  int get squared => this * this;
}
"#,
    total_chunks: 6,
    chunk_counts: {
        Class: 2,
        CodeBlock: 2,
        Function: 1,
    }
}

test_language_extractor! {
    name: test_dart_complex_algorithm_extraction,
    language: "dart",
    extension: "dart",
    source: r#"
import 'dart:async';
import 'dart:math';

class ProcessedItem {
  final int id;
  final int originalValue;
  final int transformedValue;
  final String category;
  final DateTime timestamp;
  final Map<String, dynamic> metadata;

  ProcessedItem({
    required this.id,
    required this.originalValue,
    required this.transformedValue,
    required this.category,
    required this.timestamp,
    required this.metadata,
  });
}

class DataProcessor {
  final Map<String, ProcessedItem> _cache = {};
  final List<ProcessedItem> _processingLog = [];
  final int _threshold;

  DataProcessor(this._threshold);

  Future<List<ProcessedItem>> processComplexData(List<int> input) async {
    final results = <ProcessedItem>[];
    var processedCount = 0;

    // Main processing algorithm - extractable middle chunk
    for (var i = 0; i < input.length; i++) {
      final value = input[i];
      final cacheKey = 'item_${i}_$value';

      if (_cache.containsKey(cacheKey)) {
        final cachedItem = _cache[cacheKey]!;
        results.add(cachedItem);
        continue;
      }

      late ProcessedItem processedItem;
      if (value > _threshold) {
        final transformedValue = value * 2;
        final category = transformedValue > _threshold * 3 ? 'HIGH' : 'MEDIUM';

        processedItem = ProcessedItem(
          id: i,
          originalValue: value,
          transformedValue: transformedValue,
          category: category,
          timestamp: DateTime.now(),
          metadata: {
            'processed': true,
            'multiplier': 2,
            'processor': 'enhanced',
          },
        );

        processedCount++;

        // Additional processing for high values
        if (transformedValue > 100) {
          processedItem = ProcessedItem(
            id: processedItem.id,
            originalValue: processedItem.originalValue,
            transformedValue: processedItem.transformedValue + 10,
            category: processedItem.category,
            timestamp: processedItem.timestamp,
            metadata: {...processedItem.metadata, 'bonus': true},
          );
        }
      } else if (value > 0) {
        processedItem = ProcessedItem(
          id: i,
          originalValue: value,
          transformedValue: value + _threshold,
          category: 'LOW',
          timestamp: DateTime.now(),
          metadata: {
            'processed': true,
            'adjusted': true,
            'processor': 'basic',
          },
        );
      } else {
        continue; // skip negative values
      }

      _cache[cacheKey] = processedItem;
      _processingLog.add(processedItem);
      results.add(processedItem);

      // Simulate async work
      if (i % 10 == 0) {
        await Future.delayed(Duration(milliseconds: 1));
      }
    }

    // Finalization logic
    if (processedCount > 0) {
      final average = results.map((r) => r.transformedValue).reduce((a, b) => a + b) / results.length;
      print('Processing complete. Average: ${average.toStringAsFixed(2)}');

      // Add processing statistics
      for (final item in results) {
        item.metadata['processing_average'] = average;
      }
    }

    return results;
  }

  Map<String, dynamic> analyzePatterns(List<ProcessedItem> items) {
    final analysis = <String, dynamic>{};
    final categoryGroups = <String, List<ProcessedItem>>{};

    // Group items by category
    for (final item in items) {
      categoryGroups.putIfAbsent(item.category, () => []).add(item);
    }

    // Pattern analysis logic - extractable middle chunk
    for (final entry in categoryGroups.entries) {
      final category = entry.key;
      final categoryItems = entry.value;

      final values = categoryItems.map((i) => i.transformedValue);
      final categoryAnalysis = {
        'count': categoryItems.length,
        'percentage': (categoryItems.length / items.length) * 100,
        'avg_value': values.reduce((a, b) => a + b) / values.length,
        'min_value': values.reduce(min),
        'max_value': values.reduce(max),
      };

      // Time-based analysis
      final now = DateTime.now();
      final recentItems = categoryItems.where((i) =>
        now.difference(i.timestamp).inMinutes < 1
      ).toList();

      if (recentItems.isNotEmpty) {
        final recentValues = recentItems.map((i) => i.transformedValue);
        categoryAnalysis['recent_count'] = recentItems.length;
        categoryAnalysis['recent_avg'] = recentValues.reduce((a, b) => a + b) / recentValues.length;
      }

      // High-value analysis
      final highValueItems = categoryItems.where((i) => i.transformedValue > 1000);
      if (highValueItems.isNotEmpty) {
        categoryAnalysis['high_value_count'] = highValueItems.length;
      }

      analysis[category] = categoryAnalysis;
    }

    analysis['total_items'] = items.length;
    analysis['processing_time'] = DateTime.now().toIso8601String();

    return analysis;
  }
}
"#,
    total_chunks: 41,
    chunk_counts: {
        Class: 2,
        Function: 2,
    }
}
