use crate::integration::languages::extractor::test_language_extractor;

test_language_extractor! {
    name: test_scala_function_extraction,
    language: "scala",
    extension: "scala",
    source: r#"
def hello(): Unit = {
    println("Hello, Scala!")
}

def add(a: Int, b: Int): Int = {
    a + b
}

def multiply(x: Int, y: Int): Int = x * y
"#,
    total_chunks: 3,
    chunk_counts: {
        Function: 3,
    }
}

test_language_extractor! {
    name: test_scala_class_extraction,
    language: "scala",
    extension: "scala",
    source: r#"
class Person(val name: String, val age: Int) {
    def greet(): String = s"Hello, I'm $name"
}

case class Point(x: Double, y: Double) {
    def distance(other: Point): Double = {
        math.sqrt(math.pow(x - other.x, 2) + math.pow(y - other.y, 2))
    }
}

abstract class Animal {
    def speak(): String
}
"#,
    total_chunks: 5,
    chunk_counts: {
        Class: 3,
        Function: 2,
    }
}

test_language_extractor! {
    name: test_scala_object_extraction,
    language: "scala",
    extension: "scala",
    source: r#"
object Main {
    def main(args: Array[String]): Unit = {
        println("Hello, world!")
    }

    val PI = 3.14159
}

object MathUtils {
    def factorial(n: Int): Int = {
        if (n <= 1) 1 else n * factorial(n - 1)
    }
}

case object Singleton {
    def process(): String = "processing"
}
"#,
    total_chunks: 6,
    chunk_counts: {
        Class: 3,
        Function: 3,
    }
}

test_language_extractor! {
    name: test_scala_trait_extraction,
    language: "scala",
    extension: "scala",
    source: r#"
trait Animal {
    def speak(): String
    def move(): Unit = println("Moving...")
}

sealed trait Result[+T] {
    def map[U](f: T => U): Result[U]
}

trait Drawable {
    def draw(): Unit
}
"#,
    total_chunks: 4,
    chunk_counts: {
        Class: 3,
        Function: 1,
    }
}

test_language_extractor! {
    name: test_scala_enum_extraction,
    language: "scala",
    extension: "scala",
    source: r##"
enum Color {
    case Red, Green, Blue
    case RGB(r: Int, g: Int, b: Int)

    def toHex(): String = this match {
        case Red => "#FF0000"
        case Green => "#00FF00"
        case Blue => "#0000FF"
        case RGB(r, g, b) => f"#$r%02X$g%02X$b%02X"
    }
}

enum Direction {
    case North, South, East, West
}
"##,
    total_chunks: 3,
    chunk_counts: {
        Const: 2,
        Function: 1,
    }
}

test_language_extractor! {
    name: test_scala_all_constructs_combined,
    language: "scala",
    extension: "scala",
    source: r#"
// Object definition
object Calculator {
    def add(a: Int, b: Int): Int = a + b
}

// Class definition
class Person(val name: String) {
    def greet(): String = s"Hello, $name"
}

// Case class
case class Point(x: Int, y: Int) {
    def move(dx: Int, dy: Int): Point = Point(x + dx, y + dy)
}

// Trait definition
trait Drawable {
    def draw(): Unit
}

// Enum definition
enum Status {
    case Active, Inactive
}

// Function definition
def factorial(n: Int): Int = {
    if (n <= 1) 1 else n * factorial(n - 1)
}

// Type definition
type UserId = Long
"#,
    total_chunks: 10,
    chunk_counts: {
        Class: 5,
        Const: 1,
        Function: 4,
    }
}

test_language_extractor! {
    name: test_scala_comment_ranges_in_challenge,
    language: "scala",
    extension: "scala",
    source: r#"// Scala function with comments
def calculateSum(a: Int, b: Int): Int = {
    val result = a + b // Add the numbers
    /*
     * Return the result
     */
    result
}
"#,
    total_chunks: 1,
    chunk_counts: {
        Function: 1,
    }
}

test_language_extractor! {
    name: test_scala_no_duplicates,
    language: "scala",
    extension: "scala",
    source: r#"
package com.example.test

object Calculator {
    def add(a: Int, b: Int): Int = {
        val result = a + b
        result
    }

    def processValue(value: Any): String = {
        value match {
            case s: String => s"String: $s"
            case i: Int => s"Int: $i"
            case _ => "Unknown"
        }
    }

    val numbers = List(1, 2, 3)
    val doubled = for {
        n <- numbers
        if n > 1
        result = n * 2
    } yield result

    val filtered = numbers.filter(x => x > 2).map(y => y * y)

    val attempt = Try {
        "123".toInt
    }
}

class Person(val name: String) {
    def greet(): String = s"Hello, $name"

    def isLongName(): Boolean = {
        if (name.length > 5) true else false
    }
}

trait Animal {
    def speak(): String

    def move(): Unit = {
        println("Moving...")
    }
}

extension (s: String) {
    def isPalindrome: Boolean = s == s.reverse
}

@deprecated
def oldFunction(): Unit = {}
"#,
    total_chunks: 14,
    chunk_counts: {
        Class: 3,
        Conditional: 1,
        Function: 8,
        FunctionCall: 1,
        Loop: 1,
    }
}

test_language_extractor! {
    name: test_scala_complex_algorithm_extraction,
    language: "scala",
    extension: "scala",
    source: r#"
import scala.collection.mutable
import scala.util.{Success, Failure, Try}
import java.time.Instant

case class ProcessedItem(
  id: Int,
  originalValue: Int,
  transformedValue: Int,
  category: String,
  timestamp: Instant = Instant.now(),
  metadata: mutable.Map[String, Any] = mutable.Map.empty
)

class DataProcessor(threshold: Int) {
  private val cache = mutable.Map[String, ProcessedItem]()
  private val processingLog = mutable.ListBuffer[ProcessedItem]()

  def processComplexData(input: List[Int]): List[ProcessedItem] = {
    val results = mutable.ListBuffer[ProcessedItem]()
    var processedCount = 0

    // Main processing algorithm - extractable middle chunk
    input.zipWithIndex.foreach { case (value, index) =>
      val cacheKey = s"item_${index}_$value"

      cache.get(cacheKey) match {
        case Some(cachedItem) =>
          results += cachedItem
        case None =>
          val processedItem = value match {
            case v if v > threshold =>
              val transformedValue = v * 2
              val category = if (transformedValue > threshold * 3) "HIGH" else "MEDIUM"
              val bonusValue = if (transformedValue > 100) transformedValue + 10 else transformedValue

              processedCount += 1
              ProcessedItem(
                id = index,
                originalValue = v,
                transformedValue = bonusValue,
                category = category,
                metadata = mutable.Map(
                  "processed" -> true,
                  "multiplier" -> 2,
                  "processor" -> "enhanced"
                )
              )

            case v if v > 0 =>
              ProcessedItem(
                id = index,
                originalValue = v,
                transformedValue = v + threshold,
                category = "LOW",
                metadata = mutable.Map(
                  "processed" -> true,
                  "adjusted" -> true,
                  "processor" -> "basic"
                )
              )

            case _ => // skip negative values
              null
          }

          if (processedItem != null) {
            cache(cacheKey) = processedItem
            processingLog += processedItem
            results += processedItem
          }
      }
    }

    // Finalization logic
    if (processedCount > 0) {
      val average = results.map(_.transformedValue).sum.toDouble / results.size
      println(f"Processing complete. Average: $average%.2f")

      // Add processing statistics
      results.foreach(_.metadata("processing_average") = average)
    }

    results.toList
  }

  def analyzePatterns(items: List[ProcessedItem]): Map[String, Map[String, Any]] = {
    val categoryGroups = items.groupBy(_.category)

    // Pattern analysis logic - extractable middle chunk
    val analysis = categoryGroups.map { case (category, categoryItems) =>
      val values = categoryItems.map(_.transformedValue)
      val categoryAnalysis = Map(
        "count" -> categoryItems.size,
        "percentage" -> (categoryItems.size.toDouble / items.size * 100),
        "avg_value" -> (values.sum.toDouble / values.size),
        "min_value" -> values.min,
        "max_value" -> values.max
      ) ++ {
        // Time-based analysis
        val currentTime = Instant.now()
        val recentItems = categoryItems.filter { item =>
          java.time.Duration.between(item.timestamp, currentTime).getSeconds < 60
        }

        if (recentItems.nonEmpty) {
          val recentValues = recentItems.map(_.transformedValue)
          Map(
            "recent_count" -> recentItems.size,
            "recent_avg" -> (recentValues.sum.toDouble / recentValues.size)
          )
        } else Map.empty
      } ++ {
        // High-value analysis
        val highValueItems = categoryItems.filter(_.transformedValue > 1000)
        if (highValueItems.nonEmpty) {
          Map("high_value_count" -> highValueItems.size)
        } else Map.empty
      }

      category -> categoryAnalysis
    }

    analysis ++ Map(
      "total_items" -> items.size,
      "processing_time" -> Instant.now().toString
    )
  }
}

// Companion object with utility functions
object DataProcessor {
  def createProcessor(threshold: Int): DataProcessor = new DataProcessor(threshold)

  // Complex transformation function with pattern matching
  def complexTransform(item: ProcessedItem): ProcessedItem = {
    val newValue = item.category match {
      case "HIGH" => item.transformedValue * 2
      case "MEDIUM" => item.transformedValue + 50
      case "LOW" => item.transformedValue + 10
      case _ => item.transformedValue
    }

    item.copy(transformedValue = newValue)
  }

  // Functional approach to filtering and sorting
  def filterAndSort(items: List[ProcessedItem], targetCategory: String): List[ProcessedItem] = {
    items
      .filter(_.category == targetCategory)
      .sortBy(_.transformedValue)(Ordering[Int].reverse)
  }

  // Batch processing with error handling
  def batchProcess(batches: List[List[Int]], threshold: Int): List[Try[List[ProcessedItem]]] = {
    batches.map { batch =>
      Try {
        val processor = new DataProcessor(threshold)
        processor.processComplexData(batch)
      }
    }
  }
}
"#,
    total_chunks: 32,
    chunk_counts: {
        Class: 3,
        Function: 6,
    }
}
