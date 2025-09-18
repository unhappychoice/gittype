use crate::integration::languages::extractor::test_language_extractor;

test_language_extractor! {
    name: test_java_class_method_extraction,
    language: "java",
    extension: "java",
    source: r#"public class HelloWorld {
    private String message;

    public HelloWorld(String message) {
        this.message = message;
    }

    public void printMessage() {
        System.out.println(this.message);
    }

    public static void main(String[] args) {
        HelloWorld hello = new HelloWorld("Hello, World!");
        hello.printMessage();
    }

    private int calculateLength() {
        return this.message.length();
    }
}"#,
    total_chunks: 6,
    chunk_counts: {
        Class: 1,
        Method: 4,
        Variable: 1,
    }
}

test_language_extractor! {
    name: test_java_interface_extraction,
    language: "java",
    extension: "java",
    source: r#"public interface Drawable {
    void draw();
    void setColor(String color);
    String getColor();
}

public interface Resizable {
    void resize(int width, int height);
    int getWidth();
    int getHeight();
}

public class Circle implements Drawable, Resizable {
    private String color;
    private int radius;

    @Override
    public void draw() {
        System.out.println("Drawing a " + color + " circle");
    }

    @Override
    public void setColor(String color) {
        this.color = color;
    }

    @Override
    public String getColor() {
        return this.color;
    }

    @Override
    public void resize(int width, int height) {
        this.radius = Math.min(width, height) / 2;
    }

    @Override
    public int getWidth() {
        return radius * 2;
    }

    @Override
    public int getHeight() {
        return radius * 2;
    }
}"#,
    total_chunks: 23,
    chunk_counts: {
        Class: 1,
        CodeBlock: 6,
        Interface: 2,
        Method: 12,
        Variable: 2,
    }
}

test_language_extractor! {
    name: test_java_enum_extraction,
    language: "java",
    extension: "java",
    source: r##"public enum Color {
    RED("red", "#FF0000"),
    GREEN("green", "#00FF00"),
    BLUE("blue", "#0000FF");

    private final String name;
    private final String hexCode;

    Color(String name, String hexCode) {
        this.name = name;
        this.hexCode = hexCode;
    }

    public String getName() {
        return name;
    }

    public String getHexCode() {
        return hexCode;
    }
}

public class ColorTest {
    public void testColor() {
        Color color = Color.RED;
        System.out.println(color.getName());
    }
}"##,
    total_chunks: 8,
    chunk_counts: {
        Class: 1,
        Enum: 1,
        Method: 4,
        Variable: 2,
    }
}

test_language_extractor! {
    name: test_java_field_extraction,
    language: "java",
    extension: "java",
    source: r#"public class Person {
    private String name;
    private int age;
    private static final String DEFAULT_COUNTRY = "Unknown";
    public boolean isActive;

    public Person(String name, int age) {
        this.name = name;
        this.age = age;
        this.isActive = true;
    }

    public String getName() {
        return name;
    }

    public int getAge() {
        return age;
    }
}"#,
    total_chunks: 8,
    chunk_counts: {
        Class: 1,
        Method: 3,
        Variable: 4,
    }
}

test_language_extractor! {
    name: test_java_complex_algorithm_extraction,
    language: "java",
    extension: "java",
    source: r#"
import java.util.*;
import java.util.stream.Collectors;

public class DataProcessor {
    private Map<String, Object> cache;
    private List<String> processingLog;

    public DataProcessor() {
        this.cache = new HashMap<>();
        this.processingLog = new ArrayList<>();
    }

    public List<ProcessedItem> processComplexData(List<DataItem> items, int threshold) {
        List<ProcessedItem> results = new ArrayList<>();
        int processedCount = 0;

        // Main processing algorithm - extractable middle chunk
        for (int i = 0; i < items.size(); i++) {
            DataItem item = items.get(i);
            String cacheKey = "item_" + i + "_" + item.getId();

            ProcessedItem processedItem;
            if (cache.containsKey(cacheKey)) {
                processedItem = (ProcessedItem) cache.get(cacheKey);
                processingLog.add("Cache hit for: " + cacheKey);
            } else {
                // Complex transformation logic
                if (item.getValue() > threshold) {
                    int transformedValue = item.getValue() * 2;
                    String category = transformedValue > threshold * 3 ? "HIGH" : "MEDIUM";

                    processedItem = new ProcessedItem(
                        item.getId(),
                        item.getValue(),
                        transformedValue,
                        category,
                        System.currentTimeMillis()
                    );

                    processedCount++;
                } else {
                    int adjustedValue = item.getValue() + threshold;
                    processedItem = new ProcessedItem(
                        item.getId(),
                        item.getValue(),
                        adjustedValue,
                        "LOW",
                        System.currentTimeMillis()
                    );
                }

                cache.put(cacheKey, processedItem);
                processingLog.add("Processed new item: " + cacheKey);
            }

            results.add(processedItem);
        }

        // Finalization logic
        if (processedCount > 0) {
            double average = results.stream()
                .mapToDouble(ProcessedItem::getTransformedValue)
                .average()
                .orElse(0.0);

            processingLog.add("Processing complete. Average: " + average);
        }

        return results;
    }

    public Map<String, List<ProcessedItem>> analyzeAndGroup(List<ProcessedItem> items) {
        Map<String, List<ProcessedItem>> grouped = new HashMap<>();

        // Analysis and grouping logic - extractable middle chunk
        for (ProcessedItem item : items) {
            String category = item.getCategory();

            grouped.computeIfAbsent(category, k -> new ArrayList<>()).add(item);

            // Additional analysis for high-value items
            if ("HIGH".equals(category)) {
                String subCategory = item.getTransformedValue() > 1000 ? "PREMIUM" : "STANDARD";
                String key = category + "_" + subCategory;
                grouped.computeIfAbsent(key, k -> new ArrayList<>()).add(item);
            }
        }

        // Sort each group by transformed value
        grouped.forEach((category, itemList) -> {
            itemList.sort(Comparator.comparingInt(ProcessedItem::getTransformedValue).reversed());
        });

        return grouped;
    }
}

class DataItem {
    private String id;
    private int value;

    public DataItem(String id, int value) {
        this.id = id;
        this.value = value;
    }

    public String getId() { return id; }
    public int getValue() { return value; }
}

class ProcessedItem {
    private String id;
    private int originalValue;
    private int transformedValue;
    private String category;
    private long timestamp;

    public ProcessedItem(String id, int originalValue, int transformedValue,
                        String category, long timestamp) {
        this.id = id;
        this.originalValue = originalValue;
        this.transformedValue = transformedValue;
        this.category = category;
        this.timestamp = timestamp;
    }

    public String getId() { return id; }
    public int getOriginalValue() { return originalValue; }
    public int getTransformedValue() { return transformedValue; }
    public String getCategory() { return category; }
    public long getTimestamp() { return timestamp; }
}
"#,
    total_chunks: 38,
    chunk_counts: {
        Class: 3,
        Method: 12,
    }
}
