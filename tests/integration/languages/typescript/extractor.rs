use crate::integration::languages::extractor::test_language_extractor;

test_language_extractor! {
    name: test_typescript_interface_extraction,
    language: "typescript",
    extension: "ts",
    source: r#"
interface User {
    id: number;
    name: string;
    email?: string;
}

interface Admin extends User {
    permissions: string[];
}
"#,
    total_chunks: 2,
    chunk_counts: {
        Interface: 2,
    }
}

test_language_extractor! {
    name: test_typescript_type_alias_extraction,
    language: "typescript",
    extension: "ts",
    source: r#"
type Status = 'pending' | 'completed' | 'failed';
type UserId = number;
type ApiResponse<T> = {
    data: T;
    error?: string;
};
"#,
    total_chunks: 3,
    chunk_counts: {
        TypeAlias: 3,
    }
}

test_language_extractor! {
    name: test_typescript_enum_extraction,
    language: "typescript",
    extension: "ts",
    source: r#"
enum Color {
    Red = '#ff0000',
    Green = '#00ff00',
    Blue = '#0000ff'
}

enum Status {
    Pending,
    Completed,
    Failed
}
"#,
    total_chunks: 2,
    chunk_counts: {
        Enum: 2,
    }
}

test_language_extractor! {
    name: test_typescript_namespace_extraction,
    language: "typescript",
    extension: "ts",
    source: r#"
namespace Utils {
    export function formatDate(date: Date): string {
        return date.toISOString();
    }

    export function calculateAge(birthDate: Date): number {
        const today = new Date();
        return today.getFullYear() - birthDate.getFullYear();
    }
}

namespace Api {
    export interface Response<T> {
        data: T;
        status: number;
    }
}
"#,
    total_chunks: 5,
    chunk_counts: {
        Function: 2,
        Interface: 1,
        Module: 2,
    }
}

test_language_extractor! {
    name: test_typescript_all_new_constructs_combined,
    language: "typescript",
    extension: "ts",
    source: r#"
// Interface declaration
interface User {
    id: number;
    name: string;
    email?: string;
}

// Type alias
type Status = 'pending' | 'completed' | 'failed';

// Enum declaration
enum Color {
    Red = '#ff0000',
    Green = '#00ff00',
    Blue = '#0000ff'
}

// Namespace declaration
namespace Utils {
    export function formatDate(date: Date): string {
        return date.toISOString();
    }
}

// Existing constructs (should still work)
class UserService {
    private users: User[] = [];

    addUser(user: User): void {
        this.users.push(user);
    }

    getUserById(id: number): User | undefined {
        return this.users.find(u => u.id === id);
    }
}

function processUser(user: User): Status {
    return 'pending';
}

const calculateTotal = (items: number[]): number => {
    return items.reduce((sum, item) => sum + item, 0);
};
"#,
    total_chunks: 10,
    chunk_counts: {
        Class: 1,
        Enum: 1,
        Function: 3,
        Interface: 1,
        Method: 2,
        Module: 1,
        TypeAlias: 1,
    }
}

test_language_extractor! {
    name: test_typescript_complex_algorithm_extraction,
    language: "typescript",
    extension: "ts",
    source: r#"
interface DataItem {
    id: string;
    value: number;
    metadata?: Record<string, unknown>;
}

interface ProcessedItem extends DataItem {
    transformedValue: number;
    category: 'LOW' | 'MEDIUM' | 'HIGH';
    timestamp: number;
    processingInfo: {
        cached: boolean;
        processor: string;
        duration?: number;
    };
}

class DataProcessor<T extends DataItem> {
    private cache = new Map<string, ProcessedItem>();
    private stats = {
        processed: 0,
        cacheHits: 0,
        errors: 0
    };

    constructor(private threshold: number) {}

    async processComplexData(items: T[]): Promise<ProcessedItem[]> {
        const results: ProcessedItem[] = [];
        const startTime = Date.now();

        // Main processing algorithm - extractable middle chunk
        for (let i = 0; i < items.length; i++) {
            const item = items[i];
            const cacheKey = `item_${i}_${item.id}`;

            if (this.cache.has(cacheKey)) {
                const cachedResult = this.cache.get(cacheKey)!;
                results.push({
                    ...cachedResult,
                    processingInfo: {
                        ...cachedResult.processingInfo,
                        cached: true
                    }
                });
                this.stats.cacheHits++;
                continue;
            }

            try {
                // Complex transformation logic
                let transformedValue: number;
                let category: 'LOW' | 'MEDIUM' | 'HIGH';
                let processor: string;

                if (item.value > this.threshold) {
                    transformedValue = item.value * 2;
                    category = transformedValue > this.threshold * 3 ? 'HIGH' : 'MEDIUM';
                    processor = 'enhanced';

                    // Additional processing for high values
                    if (category === 'HIGH' && item.metadata?.boost) {
                        transformedValue *= 1.5;
                        processor = 'boosted';
                    }
                } else {
                    transformedValue = item.value + this.threshold;
                    category = 'LOW';
                    processor = 'basic';
                }

                const processedItem: ProcessedItem = {
                    ...item,
                    transformedValue,
                    category,
                    timestamp: Date.now(),
                    processingInfo: {
                        cached: false,
                        processor,
                        duration: Date.now() - startTime
                    }
                };

                this.cache.set(cacheKey, processedItem);
                results.push(processedItem);
                this.stats.processed++;

            } catch (error) {
                this.stats.errors++;
                console.error(`Error processing item ${item.id}:`, error);
            }
        }

        return results;
    }

    analyzePatterns(items: ProcessedItem[]): Record<string, unknown> {
        const analysis: Record<string, unknown> = {
            totalItems: items.length,
            categoryDistribution: {},
            averageValues: {},
            processingStats: this.stats
        };

        // Pattern analysis logic - extractable middle chunk
        const categoryGroups = items.reduce((groups, item) => {
            const { category } = item;
            if (!groups[category]) {
                groups[category] = [];
            }
            groups[category].push(item);
            return groups;
        }, {} as Record<string, ProcessedItem[]>);

        for (const [category, categoryItems] of Object.entries(categoryGroups)) {
            const values = categoryItems.map(item => item.transformedValue);
            const average = values.reduce((sum, val) => sum + val, 0) / values.length;
            const max = Math.max(...values);
            const min = Math.min(...values);

            analysis.categoryDistribution = {
                ...analysis.categoryDistribution as object,
                [category]: {
                    count: categoryItems.length,
                    percentage: (categoryItems.length / items.length) * 100,
                    avgValue: average,
                    minValue: min,
                    maxValue: max
                }
            };

            // Time-based analysis
            const recentItems = categoryItems.filter(
                item => Date.now() - item.timestamp < 60000 // last minute
            );

            if (recentItems.length > 0) {
                analysis.averageValues = {
                    ...analysis.averageValues as object,
                    [`${category}_recent`]: recentItems.length
                };
            }
        }

        return analysis;
    }
}

// Utility functions with complex logic
function createDataValidator<T>(schema: Record<keyof T, (value: unknown) => boolean>) {
    return (data: unknown): data is T => {
        if (!data || typeof data !== 'object') return false;

        // Validation logic - extractable middle chunk
        for (const [key, validator] of Object.entries(schema)) {
            const value = (data as Record<string, unknown>)[key];

            if (!validator(value)) {
                console.warn(`Validation failed for field ${key}:`, value);
                return false;
            }

            // Additional type-specific validations
            if (typeof value === 'string' && value.length === 0) {
                console.warn(`Empty string not allowed for field ${key}`);
                return false;
            }

            if (typeof value === 'number' && !Number.isFinite(value)) {
                console.warn(`Invalid number for field ${key}:`, value);
                return false;
            }
        }

        return true;
    };
}
"#,
    total_chunks: 26,
    chunk_counts: {
        Class: 1,
        Function: 1,
        Interface: 2,
    }
}
