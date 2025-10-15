use crate::integration::languages::extractor::test_language_extractor;

test_language_extractor! {
    name: test_go_function_extraction,
    language: "go",
    extension: "go",
    source: r#"package main

import "fmt"

func main() {
    fmt.Println("Hello, world!")
}

func add(a, b int) int {
    return a + b
}

func multiply(x int, y int) int {
    return x * y
}
"#,
    total_chunks: 4,
    chunk_counts: {
        File: 1,
        Function: 3,
    }
}

test_language_extractor! {
    name: test_go_struct_extraction,
    language: "go",
    extension: "go",
    source: r#"package main

type Person struct {
    Name string
    Age  int
}

type Address struct {
    Street string
    City   string
    Zip    string
}

func (p Person) GetName() string {
    return p.Name
}

func (a *Address) GetFullAddress() string {
    return a.Street + ", " + a.City + " " + a.Zip
}
"#,
    total_chunks: 6,
    chunk_counts: {
        Struct: 2,
        Method: 2,
        CodeBlock: 1,
        File: 1,
    }
}

test_language_extractor! {
    name: test_go_interface_extraction,
    language: "go",
    extension: "go",
    source: r#"package main

type Writer interface {
    Write([]byte) (int, error)
}

type Reader interface {
    Read([]byte) (int, error)
}

type ReadWriter interface {
    Reader
    Writer
}

func process(rw ReadWriter) {
    // Implementation here
}
"#,
    total_chunks: 6,
    chunk_counts: {
        Interface: 3,
        Function: 1,
        CodeBlock: 1,
        File: 1,
    }
}

test_language_extractor! {
    name: test_go_const_var_type_alias_extraction,
    language: "go",
    extension: "go",
    source: r#"package main

import "errors"

// Const block test
const (
    StatusOK = 200
    StatusNotFound = 404
    StatusError = 500
)

// Single const
const MaxRetries = 3

// Var block test
var (
    ErrNotFound = errors.New("not found")
    ErrTimeout = errors.New("timeout")
)

// Single var
var GlobalCounter int

// Type alias tests
type UserID int64
type Handler func(string, string)
type Point struct {
    X, Y int
}

func main() {}
"#,
    total_chunks: 9,
    chunk_counts: {
        Struct: 1,
        Function: 1,
        Const: 2,
        Variable: 2,
        TypeAlias: 2,
        File: 1,
    }
}

test_language_extractor! {
    name: test_go_complex_algorithm_extraction,
    language: "go",
    extension: "go",
    source: r#"
package main

import (
    "fmt"
    "sync"
    "time"
)

type DataProcessor struct {
    cache      map[string]interface{}
    mutex      sync.RWMutex
    stats      ProcessingStats
    threshold  int
}

type ProcessingStats struct {
    Processed   int
    CacheHits   int
    Errors      int
    StartTime   time.Time
}

type ProcessedItem struct {
    ID             string
    OriginalValue  int
    TransformedValue int
    Category       string
    Timestamp      time.Time
    Metadata       map[string]interface{}
}

func NewDataProcessor(threshold int) *DataProcessor {
    return &DataProcessor{
        cache:     make(map[string]interface{}),
        stats:     ProcessingStats{StartTime: time.Now()},
        threshold: threshold,
    }
}

func (dp *DataProcessor) ProcessComplexData(items []int) ([]ProcessedItem, error) {
    var results []ProcessedItem

    // Main processing algorithm - extractable middle chunk
    for i, value := range items {
        cacheKey := fmt.Sprintf("item_%d_%d", i, value)

        dp.mutex.RLock()
        if cached, exists := dp.cache[cacheKey]; exists {
            dp.mutex.RUnlock()

            if processedItem, ok := cached.(ProcessedItem); ok {
                results = append(results, processedItem)
                dp.stats.CacheHits++
                continue
            }
        } else {
            dp.mutex.RUnlock()
        }

        // Complex transformation logic
        var processedItem ProcessedItem

        if value > dp.threshold {
            transformedValue := value * 2
            category := "MEDIUM"
            if transformedValue > dp.threshold*3 {
                category = "HIGH"
            }

            processedItem = ProcessedItem{
                ID:              fmt.Sprintf("item_%d", i),
                OriginalValue:   value,
                TransformedValue: transformedValue,
                Category:        category,
                Timestamp:       time.Now(),
                Metadata: map[string]interface{}{
                    "processed":    true,
                    "multiplier":   2,
                    "threshold":    dp.threshold,
                    "processor":    "complex",
                },
            }
        } else {
            adjustedValue := value + dp.threshold
            processedItem = ProcessedItem{
                ID:              fmt.Sprintf("item_%d", i),
                OriginalValue:   value,
                TransformedValue: adjustedValue,
                Category:        "LOW",
                Timestamp:       time.Now(),
                Metadata: map[string]interface{}{
                    "processed":  true,
                    "adjusted":   true,
                    "threshold":  dp.threshold,
                    "processor":  "simple",
                },
            }
        }

        // Cache the result
        dp.mutex.Lock()
        dp.cache[cacheKey] = processedItem
        dp.mutex.Unlock()

        results = append(results, processedItem)
        dp.stats.Processed++
    }

    return results, nil
}

func (dp *DataProcessor) AnalyzePatterns(items []ProcessedItem) map[string]interface{} {
    analysis := make(map[string]interface{})
    categoryCount := make(map[string]int)
    valueSum := make(map[string]int)

    // Pattern analysis logic - extractable middle chunk
    for _, item := range items {
        category := item.Category
        categoryCount[category]++
        valueSum[category] += item.TransformedValue

        // Time-based analysis
        timeDiff := time.Since(item.Timestamp)
        if timeDiff < time.Minute {
            recentKey := fmt.Sprintf("%s_recent", category)
            if count, exists := categoryCount[recentKey]; exists {
                categoryCount[recentKey] = count + 1
            } else {
                categoryCount[recentKey] = 1
            }
        }

        // Value distribution analysis
        if item.TransformedValue > 1000 {
            highValueKey := fmt.Sprintf("%s_high_value", category)
            categoryCount[highValueKey]++
        }
    }

    // Calculate averages
    averages := make(map[string]float64)
    for category, sum := range valueSum {
        if count := categoryCount[category]; count > 0 {
            averages[category] = float64(sum) / float64(count)
        }
    }

    analysis["category_counts"] = categoryCount
    analysis["averages"] = averages
    analysis["total_items"] = len(items)
    analysis["processing_time"] = time.Since(dp.stats.StartTime)

    return analysis
}
"#,
    total_chunks: 31,
    chunk_counts: {
        Struct: 3,
        Function: 1,
        Method: 2,
        Variable: 2,
        Loop: 2,
        Conditional: 8,
        CodeBlock: 12,
        File: 1,
    }
}
