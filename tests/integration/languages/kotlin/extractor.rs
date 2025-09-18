use crate::integration::languages::extractor::test_language_extractor;

test_language_extractor! {
    name: test_kotlin_function_extraction,
    language: "kotlin",
    extension: "kt",
    source: r#"
fun greet(name: String): String {
    return "Hello, $name!"
}

fun calculateSum(a: Int, b: Int): Int {
    return a + b
}

fun processData(data: List<String>) {
    data.forEach { println(it) }
}
"#,
    total_chunks: 3,
    chunk_counts: {
        Function: 3,
    }
}

test_language_extractor! {
    name: test_kotlin_class_extraction,
    language: "kotlin",
    extension: "kt",
    source: r#"
class Person(val name: String, val age: Int) {
    fun greet(): String {
        return "Hello, I'm $name and I'm $age years old"
    }

    fun isAdult(): Boolean {
        return age >= 18
    }
}

data class User(
    val id: Long,
    val name: String,
    val email: String
) {
    fun getDisplayName(): String = "$name ($email)"
}
"#,
    total_chunks: 5,
    chunk_counts: {
        Class: 2,
        Function: 3,
    }
}

test_language_extractor! {
    name: test_kotlin_object_extraction,
    language: "kotlin",
    extension: "kt",
    source: r#"
object DatabaseHelper {
    const val DB_NAME = "app.db"

    fun connect(): String {
        return "Connected to $DB_NAME"
    }

    fun disconnect() {
        println("Disconnected from database")
    }
}

object Utils {
    fun formatName(name: String): String {
        return name.trim().lowercase()
    }
}
"#,
    total_chunks: 6,
    chunk_counts: {
        Class: 2,
        Function: 3,
        Variable: 1,
    }
}

test_language_extractor! {
    name: test_kotlin_comprehensive_extraction,
    language: "kotlin",
    extension: "kt",
    source: r#"
// Line comment
/* Block comment */

package com.example.test

// Type alias
typealias StringList = List<String>

// Interface declaration
interface TestInterface {
    fun interfaceMethod(): String
}

// Regular function
fun regularFunction(param: String): String {
    return "Hello $param"
}

// Anonymous function
val anonymousFunc = fun(x: Int): Int { return x * 2 }

// Regular class
class RegularClass(private val name: String) {
    fun method(): String = name
}

// Data class
data class DataClass(val id: Int, val name: String)

// Enum class
enum class Color {
    RED,
    GREEN,
    BLUE
}

// Object declaration
object SingletonObject {
    const val CONSTANT = "constant_value"

    fun objectMethod(): String = "object method"
}

// Class with companion object
class ClassWithCompanion {
    companion object {
        const val COMPANION_CONSTANT = "companion_constant"

        fun companionMethod(): String = "companion method"
    }
}

// Properties
val globalVal: String = "global val"
var globalVar: String = "global var"
"#,
    total_chunks: 21,
    chunk_counts: {
        Class: 8,
        Const: 3,
        Function: 6,
        Variable: 4,
    }
}

test_language_extractor! {
    name: test_kotlin_complex_algorithm_extraction,
    language: "kotlin",
    extension: "kt",
    source: r#"
data class ProcessedItem(
    val id: Int,
    val originalValue: Int,
    val transformedValue: Int,
    val category: String,
    val timestamp: Long = System.currentTimeMillis(),
    val metadata: MutableMap<String, Any> = mutableMapOf()
)

class DataProcessor(private val threshold: Int) {
    private val cache = mutableMapOf<String, ProcessedItem>()
    private val processingLog = mutableListOf<ProcessedItem>()

    fun processComplexData(input: List<Int>): List<ProcessedItem> {
        val results = mutableListOf<ProcessedItem>()
        var processedCount = 0

        // Main processing algorithm - extractable middle chunk
        input.forEachIndexed { index, value ->
            val cacheKey = "item_${index}_$value"

            cache[cacheKey]?.let { cachedItem ->
                results.add(cachedItem)
                return@forEachIndexed
            }

            val processedItem = when {
                value > threshold -> {
                    val transformedValue = value * 2
                    val category = if (transformedValue > threshold * 3) "HIGH" else "MEDIUM"
                    val bonusValue = if (transformedValue > 100) transformedValue + 10 else transformedValue

                    ProcessedItem(
                        id = index,
                        originalValue = value,
                        transformedValue = bonusValue,
                        category = category,
                        metadata = mutableMapOf(
                            "processed" to true,
                            "multiplier" to 2,
                            "processor" to "enhanced"
                        )
                    ).also { processedCount++ }
                }
                value > 0 -> ProcessedItem(
                    id = index,
                    originalValue = value,
                    transformedValue = value + threshold,
                    category = "LOW",
                    metadata = mutableMapOf(
                        "processed" to true,
                        "adjusted" to true,
                        "processor" to "basic"
                    )
                )
                else -> return@forEachIndexed // skip negative values
            }

            cache[cacheKey] = processedItem
            processingLog.add(processedItem)
            results.add(processedItem)
        }

        // Finalization logic
        if (processedCount > 0) {
            val average = results.map { it.transformedValue }.average()
            println("Processing complete. Average: %.2f".format(average))

            results.forEach { item ->
                item.metadata["processing_average"] = average
            }
        }

        return results
    }

    fun analyzePatterns(items: List<ProcessedItem>): Map<String, Map<String, Any>> {
        val analysis = mutableMapOf<String, Map<String, Any>>()
        val categoryGroups = items.groupBy { it.category }

        // Pattern analysis logic - extractable middle chunk
        categoryGroups.forEach { (category, categoryItems) ->
            val values = categoryItems.map { it.transformedValue.toDouble() }
            val categoryAnalysis = mapOf(
                "count" to categoryItems.size,
                "percentage" to (categoryItems.size.toDouble() / items.size * 100),
                "avg_value" to values.average(),
                "min_value" to values.minOrNull(),
                "max_value" to values.maxOrNull()
            ).toMutableMap()

            // Time-based analysis
            val currentTime = System.currentTimeMillis()
            val recentItems = categoryItems.filter { currentTime - it.timestamp < 60000 } // last minute
            if (recentItems.isNotEmpty()) {
                categoryAnalysis["recent_count"] = recentItems.size
                categoryAnalysis["recent_avg"] = recentItems.map { it.transformedValue.toDouble() }.average()
            }

            // High-value analysis
            val highValueItems = categoryItems.filter { it.transformedValue > 1000 }
            if (highValueItems.isNotEmpty()) {
                categoryAnalysis["high_value_count"] = highValueItems.size
            }

            analysis[category] = categoryAnalysis
        }

        return analysis + mapOf(
            "total_items" to items.size,
            "processing_time" to System.currentTimeMillis()
        )
    }
}
"#,
    total_chunks: 33,
    chunk_counts: {
        Class: 2,
        Function: 2,
    }
}
