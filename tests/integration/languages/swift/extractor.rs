use crate::integration::languages::extractor::test_language_extractor;

test_language_extractor! {
    name: test_swift_class_extraction,
    language: "swift",
    extension: "swift",
    source: r#"
class Calculator {
    private var value: Int = 0

    init() {
        self.value = 0
    }

    deinit {
        print("Calculator deallocated")
    }

    func add(_ number: Int) -> Int {
        value += number
        return value
    }

    func multiply(_ number: Int) -> Int {
        value *= number
        return value
    }
}

class Person {
    let name: String
    let age: Int

    init(name: String, age: Int) {
        self.name = name
        self.age = age
    }

    func greet() -> String {
        return "Hello, I'm \(name)!"
    }
}
"#,
    total_chunks: 11,
    chunk_counts: {
        Class: 2,
        CodeBlock: 5,
        Function: 3,
    }
}

test_language_extractor! {
    name: test_swift_function_extraction,
    language: "swift",
    extension: "swift",
    source: r#"
func greet(name: String) -> String {
    return "Hello, \(name)!"
}

func calculateSum(a: Int, b: Int) -> Int {
    return a + b
}

func processData(items: [String]) {
    for item in items {
        print(item)
    }
}
"#,
    total_chunks: 6,
    chunk_counts: {
        CodeBlock: 1,
        Function: 3,
        Loop: 1,
    }
}

test_language_extractor! {
    name: test_swift_protocol_extraction,
    language: "swift",
    extension: "swift",
    source: r#"
protocol Drawable {
    func draw()
    func area() -> Double
}

protocol Comparable {
    func isGreaterThan(_ other: Self) -> Bool
}
"#,
    total_chunks: 3,
    chunk_counts: {
        Interface: 2,
    }
}

test_language_extractor! {
    name: test_swift_struct_extraction,
    language: "swift",
    extension: "swift",
    source: r#"
struct Point {
    let x: Double
    let y: Double

    func distance(to other: Point) -> Double {
        return sqrt(pow(x - other.x, 2) + pow(y - other.y, 2))
    }
}

struct Rectangle {
    let width: Double
    let height: Double

    func area() -> Double {
        return width * height
    }
}
"#,
    total_chunks: 6,
    chunk_counts: {
        Class: 2,
        CodeBlock: 1,
        Function: 2,
    }
}

test_language_extractor! {
    name: test_swift_enum_extraction,
    language: "swift",
    extension: "swift",
    source: r#"
enum Direction {
    case north, south, east, west

    func opposite() -> Direction {
        switch self {
        case .north: return .south
        case .south: return .north
        case .east: return .west
        case .west: return .east
        }
    }
}

enum Status {
    case pending
    case completed
    case failed
}
"#,
    total_chunks: 6,
    chunk_counts: {
        Class: 2,
        CodeBlock: 1,
        Conditional: 1,
        Function: 1,
    }
}

test_language_extractor! {
    name: test_swift_complex_algorithm_extraction,
    language: "swift",
    extension: "swift",
    source: r#"
import Foundation

struct ProcessedItem {
    let id: Int
    let originalValue: Int
    let transformedValue: Int
    let category: String
    let timestamp: Date
    var metadata: [String: Any]

    init(id: Int, originalValue: Int, transformedValue: Int, category: String, metadata: [String: Any] = [:]) {
        self.id = id
        self.originalValue = originalValue
        self.transformedValue = transformedValue
        self.category = category
        self.timestamp = Date()
        self.metadata = metadata
    }
}

class DataProcessor {
    private var cache: [String: ProcessedItem] = [:]
    private var processingLog: [ProcessedItem] = []
    private let threshold: Int

    init(threshold: Int) {
        self.threshold = threshold
    }

    func processComplexData(_ input: [Int]) -> [ProcessedItem] {
        var results: [ProcessedItem] = []
        var processedCount = 0

        // Main processing algorithm - extractable middle chunk
        for (index, value) in input.enumerated() {
            let cacheKey = "item_\(index)_\(value)"

            if let cachedItem = cache[cacheKey] {
                results.append(cachedItem)
                continue
            }

            let processedItem: ProcessedItem?
            if value > threshold {
                let transformedValue = value * 2
                let category = transformedValue > threshold * 3 ? "HIGH" : "MEDIUM"
                let bonusValue = transformedValue > 100 ? transformedValue + 10 : transformedValue

                processedItem = ProcessedItem(
                    id: index,
                    originalValue: value,
                    transformedValue: bonusValue,
                    category: category,
                    metadata: [
                        "processed": true,
                        "multiplier": 2,
                        "processor": "enhanced"
                    ]
                )
                processedCount += 1
            } else if value > 0 {
                processedItem = ProcessedItem(
                    id: index,
                    originalValue: value,
                    transformedValue: value + threshold,
                    category: "LOW",
                    metadata: [
                        "processed": true,
                        "adjusted": true,
                        "processor": "basic"
                    ]
                )
            } else {
                continue // skip negative values
            }

            if let item = processedItem {
                cache[cacheKey] = item
                processingLog.append(item)
                results.append(item)
            }
        }

        // Finalization logic
        if processedCount > 0 {
            let average = Double(results.map { $0.transformedValue }.reduce(0, +)) / Double(results.count)
            print("Processing complete. Average: \(String(format: "%.2f", average))")

            // Add processing statistics
            for i in 0..<results.count {
                results[i].metadata["processing_average"] = average
            }
        }

        return results
    }

    func analyzePatterns(_ items: [ProcessedItem]) -> [String: [String: Any]] {
        var analysis: [String: [String: Any]] = [:]
        let categoryGroups = Dictionary(grouping: items) { $0.category }

        // Pattern analysis logic - extractable middle chunk
        for (category, categoryItems) in categoryGroups {
            let values = categoryItems.map { $0.transformedValue }
            var categoryAnalysis: [String: Any] = [
                "count": categoryItems.count,
                "percentage": Double(categoryItems.count) / Double(items.count) * 100,
                "avg_value": Double(values.reduce(0, +)) / Double(values.count),
                "min_value": values.min() ?? 0,
                "max_value": values.max() ?? 0
            ]

            // Time-based analysis
            let currentTime = Date()
            let recentItems = categoryItems.filter { currentTime.timeIntervalSince($0.timestamp) < 60 } // last minute
            if !recentItems.isEmpty {
                let recentValues = recentItems.map { $0.transformedValue }
                categoryAnalysis["recent_count"] = recentItems.count
                categoryAnalysis["recent_avg"] = Double(recentValues.reduce(0, +)) / Double(recentValues.count)
            }

            // High-value analysis
            let highValueItems = categoryItems.filter { $0.transformedValue > 1000 }
            if !highValueItems.isEmpty {
                categoryAnalysis["high_value_count"] = highValueItems.count
            }

            analysis[category] = categoryAnalysis
        }

        analysis["total_items"] = items.count
        analysis["processing_time"] = Date().timeIntervalSince1970

        return analysis
    }
}

// Extension for additional functionality
extension DataProcessor {
    func complexTransform(_ item: ProcessedItem) -> ProcessedItem {
        let newValue: Int
        switch item.category {
        case "HIGH":
            newValue = item.transformedValue * 2
        case "MEDIUM":
            newValue = item.transformedValue + 50
        case "LOW":
            newValue = item.transformedValue + 10
        default:
            newValue = item.transformedValue
        }

        return ProcessedItem(
            id: item.id,
            originalValue: item.originalValue,
            transformedValue: newValue,
            category: item.category,
            metadata: item.metadata
        )
    }

    func filterAndSort(_ items: [ProcessedItem], targetCategory: String) -> [ProcessedItem] {
        return items
            .filter { $0.category == targetCategory }
            .sorted { $0.transformedValue > $1.transformedValue }
    }
}

// Protocol for batch processing
protocol BatchProcessable {
    func batchProcess(_ batches: [[Int]], threshold: Int) -> [Result<[ProcessedItem], Error>]
}

extension DataProcessor: BatchProcessable {
    func batchProcess(_ batches: [[Int]], threshold: Int) -> [Result<[ProcessedItem], Error>] {
        return batches.map { batch in
            Result {
                let processor = DataProcessor(threshold: threshold)
                return processor.processComplexData(batch)
            }
        }
    }
}
"#,
    total_chunks: 49,
    chunk_counts: {
        Class: 2,
        Function: 5,
    }
}
