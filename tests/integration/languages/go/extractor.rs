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

test_language_extractor! {
    name: test_go_concurrency_patterns,
    language: "go",
    extension: "go",
    source: r#"package main

import (
    "fmt"
    "sync"
    "time"
)

type Worker struct {
    id       int
    taskChan chan Task
    wg       *sync.WaitGroup
}

type Task struct {
    ID   int
    Data string
}

func NewWorker(id int, taskChan chan Task, wg *sync.WaitGroup) *Worker {
    return &Worker{
        id:       id,
        taskChan: taskChan,
        wg:       wg,
    }
}

func (w *Worker) Start() {
    go func() {
        defer w.wg.Done()

        for task := range w.taskChan {
            fmt.Printf("Worker %d processing task %d: %s\n", w.id, task.ID, task.Data)
            time.Sleep(100 * time.Millisecond)
        }
    }()
}

func main() {
    var wg sync.WaitGroup
    taskChan := make(chan Task, 10)

    // Start workers
    for i := 1; i <= 3; i++ {
        worker := NewWorker(i, taskChan, &wg)
        wg.Add(1)
        worker.Start()
    }

    // Send tasks
    for i := 1; i <= 5; i++ {
        taskChan <- Task{ID: i, Data: fmt.Sprintf("data-%d", i)}
    }

    close(taskChan)
    wg.Wait()
}
"#,
    total_chunks: 11,
    chunk_counts: {
        Variable: 1,
        File: 1,
        Struct: 2,
        Method: 1,
        Loop: 3,
        FunctionCall: 1,
        Function: 2,
    }
}

test_language_extractor! {
    name: test_go_defer_and_error_handling,
    language: "go",
    extension: "go",
    source: r#"package main

import (
    "errors"
    "fmt"
    "io"
    "os"
)

var (
    ErrInvalidInput = errors.New("invalid input")
    ErrNotFound     = errors.New("not found")
)

type FileProcessor struct {
    filename string
}

func NewFileProcessor(filename string) *FileProcessor {
    return &FileProcessor{filename: filename}
}

func (fp *FileProcessor) Process() error {
    file, err := os.Open(fp.filename)
    if err != nil {
        return fmt.Errorf("failed to open file: %w", err)
    }
    defer file.Close()

    data := make([]byte, 1024)
    n, err := file.Read(data)
    if err != nil && err != io.EOF {
        return fmt.Errorf("failed to read file: %w", err)
    }

    defer func() {
        fmt.Println("Processing completed")
    }()

    if n == 0 {
        return ErrInvalidInput
    }

    return nil
}

func cleanup() {
    defer func() {
        if r := recover(); r != nil {
            fmt.Printf("Recovered from panic: %v\n", r)
        }
    }()

    panic("something went wrong")
}
"#,
    total_chunks: 14,
    chunk_counts: {
        CodeBlock: 2,
        File: 1,
        Function: 2,
        FunctionCall: 2,
        Variable: 1,
        Method: 1,
        Struct: 1,
        Conditional: 4,
    }
}

test_language_extractor! {
    name: test_go_select_and_channels,
    language: "go",
    extension: "go",
    source: r#"package main

import (
    "fmt"
    "time"
)

type Message struct {
    Content string
    Sender  string
}

func sender(ch chan<- Message, name string) {
    for i := 0; i < 3; i++ {
        msg := Message{
            Content: fmt.Sprintf("Message %d", i),
            Sender:  name,
        }
        ch <- msg
        time.Sleep(100 * time.Millisecond)
    }
    close(ch)
}

func receiver(ch1, ch2 <-chan Message, done chan<- bool) {
    for {
        select {
        case msg, ok := <-ch1:
            if !ok {
                ch1 = nil
                continue
            }
            fmt.Printf("Received from ch1: %s from %s\n", msg.Content, msg.Sender)

        case msg, ok := <-ch2:
            if !ok {
                ch2 = nil
                continue
            }
            fmt.Printf("Received from ch2: %s from %s\n", msg.Content, msg.Sender)

        case <-time.After(500 * time.Millisecond):
            if ch1 == nil && ch2 == nil {
                done <- true
                return
            }
            fmt.Println("Timeout waiting for messages")
        }
    }
}
"#,
    total_chunks: 9,
    chunk_counts: {
        Struct: 1,
        Conditional: 3,
        File: 1,
        Function: 2,
        Loop: 2,
    }
}
