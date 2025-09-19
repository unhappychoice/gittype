use crate::integration::languages::extractor::test_language_extractor;

test_language_extractor! {
    name: test_php_function_extraction,
    language: "php",
    extension: "php",
    source: r#"<?php

function hello_world() {
    echo "Hello, world!";
}

function calculate_sum($a, $b) {
    return $a + $b;
}

function fibonacci($n) {
    if ($n <= 1) {
        return $n;
    }
    return fibonacci($n - 1) + fibonacci($n - 2);
}

?>"#,
    total_chunks: 6,
    chunk_counts: {
        CodeBlock: 2,
        File: 1,
        Function: 3,
    }
}

test_language_extractor! {
    name: test_php_class_extraction,
    language: "php",
    extension: "php",
    source: r#"<?php

class Person {
    private $name;
    private $age;

    public function __construct($name, $age) {
        $this->name = $name;
        $this->age = $age;
    }

    public function greet() {
        return "Hello, I'm " . $this->name . "!";
    }

    public function getAge() {
        return $this->age;
    }
}

class Calculator {
    public function add($a, $b) {
        return $a + $b;
    }

    public function multiply($a, $b) {
        return $a * $b;
    }
}

?>"#,
    total_chunks: 10,
    chunk_counts: {
        CodeBlock: 2,
        File: 1,
    }
}

test_language_extractor! {
    name: test_php_namespace_and_use_statements,
    language: "php",
    extension: "php",
    source: r#"<?php

namespace App\Services;

use App\Models\User;
use Exception;

class UserService {
    private $database;

    public function __construct(DatabaseConnection $database) {
        $this->database = $database;
    }

    public function createUser(array $userData): User {
        try {
            $user = new User([
                'name' => $userData['name'],
                'email' => $userData['email'],
                'password' => password_hash($userData['password'], PASSWORD_DEFAULT)
            ]);

            $userId = $this->database->insert('users', $user->toArray());
            $user->setId($userId);

            return $user;
        } catch (Exception $e) {
            throw new UserCreationException("Failed to create user: " . $e->getMessage());
        }
    }

    public function findUsersByRole(string $role): array {
        $query = "SELECT * FROM users WHERE role = ?";
        return $this->database->query($query, [$role]);
    }
}

?>"#,
    total_chunks: 10,
    chunk_counts: {
        File: 1,
        CodeBlock: 5,
        ErrorHandling: 0,
    }
}

test_language_extractor! {
    name: test_php_traits_and_interfaces,
    language: "php",
    extension: "php",
    source: r#"<?php

interface Drawable {
    public function draw();
}

trait Timestampable {
    private $created_at;
    private $updated_at;

    public function touch() {
        $this->updated_at = time();
    }
}

class Shape implements Drawable {
    use Timestampable;

    protected $color;

    public function __construct($color) {
        $this->color = $color;
        $this->created_at = time();
        $this->updated_at = time();
    }

    public function draw() {
        return "Drawing a shape in " . $this->color;
    }
}

?>"#,
    total_chunks: 10,
    chunk_counts: {
        CodeBlock: 4,
        File: 1,
    }
}

test_language_extractor! {
    name: test_php_complex_algorithm_extraction,
    language: "php",
    extension: "php",
    source: r#"
<?php

class ProcessedItem {
    public $id;
    public $originalValue;
    public $transformedValue;
    public $category;
    public $timestamp;
    public $metadata;

    public function __construct($id, $originalValue, $transformedValue, $category, $metadata = []) {
        $this->id = $id;
        $this->originalValue = $originalValue;
        $this->transformedValue = $transformedValue;
        $this->category = $category;
        $this->timestamp = time();
        $this->metadata = $metadata;
    }
}

class DataProcessor {
    private $cache = [];
    private $processingLog = [];
    private $threshold;

    public function __construct($threshold) {
        $this->threshold = $threshold;
    }

    public function processComplexData($input) {
        $results = [];
        $processedCount = 0;

        // Main processing algorithm - extractable middle chunk
        foreach ($input as $index => $value) {
            $cacheKey = "item_{$index}_{$value}";

            if (isset($this->cache[$cacheKey])) {
                $results[] = $this->cache[$cacheKey];
                continue;
            }

            $processedItem = null;
            if ($value > $this->threshold) {
                $transformedValue = $value * 2;
                $category = $transformedValue > $this->threshold * 3 ? 'HIGH' : 'MEDIUM';
                $bonusValue = $transformedValue > 100 ? $transformedValue + 10 : $transformedValue;

                $processedItem = new ProcessedItem(
                    $index,
                    $value,
                    $bonusValue,
                    $category,
                    [
                        'processed' => true,
                        'multiplier' => 2,
                        'processor' => 'enhanced'
                    ]
                );
                $processedCount++;
            } elseif ($value > 0) {
                $processedItem = new ProcessedItem(
                    $index,
                    $value,
                    $value + $this->threshold,
                    'LOW',
                    [
                        'processed' => true,
                        'adjusted' => true,
                        'processor' => 'basic'
                    ]
                );
            } else {
                continue; // skip negative values
            }

            $this->cache[$cacheKey] = $processedItem;
            $this->processingLog[] = $processedItem;
            $results[] = $processedItem;
        }

        // Finalization logic
        if ($processedCount > 0) {
            $total = array_sum(array_map(function($item) {
                return $item->transformedValue;
            }, $results));
            $average = $total / count($results);

            echo "Processing complete. Average: " . number_format($average, 2) . "\n";

            // Add processing statistics
            foreach ($results as $item) {
                $item->metadata['processing_average'] = $average;
            }
        }

        return $results;
    }

    public function analyzePatterns($items) {
        $analysis = [];
        $categoryGroups = [];

        // Group items by category
        foreach ($items as $item) {
            $categoryGroups[$item->category][] = $item;
        }

        // Pattern analysis logic - extractable middle chunk
        foreach ($categoryGroups as $category => $categoryItems) {
            $values = array_map(function($item) {
                return $item->transformedValue;
            }, $categoryItems);

            $categoryAnalysis = [
                'count' => count($categoryItems),
                'percentage' => (count($categoryItems) / count($items)) * 100,
                'avg_value' => array_sum($values) / count($values),
                'min_value' => min($values),
                'max_value' => max($values)
            ];

            // Time-based analysis
            $currentTime = time();
            $recentItems = array_filter($categoryItems, function($item) use ($currentTime) {
                return ($currentTime - $item->timestamp) < 60; // last minute
            });

            if (count($recentItems) > 0) {
                $recentValues = array_map(function($item) {
                    return $item->transformedValue;
                }, $recentItems);
                $categoryAnalysis['recent_count'] = count($recentItems);
                $categoryAnalysis['recent_avg'] = array_sum($recentValues) / count($recentValues);
            }

            // High-value analysis
            $highValueItems = array_filter($categoryItems, function($item) {
                return $item->transformedValue > 1000;
            });

            if (count($highValueItems) > 0) {
                $categoryAnalysis['high_value_count'] = count($highValueItems);
            }

            $analysis[$category] = $categoryAnalysis;
        }

        $analysis['total_items'] = count($items);
        $analysis['processing_time'] = time();

        return $analysis;
    }
}

?>
"#,
    total_chunks: 13,
    chunk_counts: {
        CodeBlock: 6,
        File: 1,
    }
}
