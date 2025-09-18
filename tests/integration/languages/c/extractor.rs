use crate::integration::languages::extractor::test_language_extractor;

test_language_extractor! {
    name: test_c_function_extraction,
    language: "c",
    extension: "c",
    source: r#"
int main(void) {
    printf("Hello, world!");
    return 0;
}

int add(int a, int b) {
    return a + b;
}

void print_number(int num) {
    printf("%d\n", num);
}
"#,
    total_chunks: 3,
    chunk_counts: {
        Function: 3,
    }
}

test_language_extractor! {
    name: test_c_struct_extraction,
    language: "c",
    extension: "c",
    source: r#"
struct Point {
    int x;
    int y;
};

struct Person {
    char name[50];
    int age;
    struct Point location;
};

int main() {
    struct Point p = {10, 20};
    return 0;
}
"#,
    total_chunks: 4,
    chunk_counts: {
        Function: 1,
        Struct: 2,
        Variable: 1,
    }
}

test_language_extractor! {
    name: test_c_variable_extraction,
    language: "c",
    extension: "c",
    source: r#"
int global_counter = 0;
char *buffer = NULL;
static int static_var = 42;

int main() {
    int local_var = 10;
    char arr[100];
    float *ptr = malloc(sizeof(float));
    return 0;
}
"#,
    total_chunks: 4,
    chunk_counts: {
        Function: 1,
        Variable: 3,
    }
}

test_language_extractor! {
    name: test_c_function_declarations,
    language: "c",
    extension: "h",
    source: r#"
#ifndef TEST_H
#define TEST_H

// Function declarations
int calculate_sum(int a, int b);
void print_message(const char *msg);
float compute_average(int *values, int count);

// Function definitions
static inline int max(int a, int b) {
    return (a > b) ? a : b;
}

#endif
"#,
    total_chunks: 2,
    chunk_counts: {
        Function: 2,
    }
}

test_language_extractor! {
    name: test_c_complex_types,
    language: "c",
    extension: "c",
    source: r#"
typedef struct {
    int id;
    char name[32];
} User;

typedef union {
    int i;
    float f;
    char c[4];
} Value;

enum Status {
    SUCCESS,
    ERROR,
    PENDING
};

int process_user(User *user, enum Status *status) {
    if (!user || !status) return -1;
    *status = SUCCESS;
    return user->id;
}
"#,
    total_chunks: 5,
    chunk_counts: {
        Function: 1,
        Struct: 4,
    }
}

test_language_extractor! {
    name: test_c_complex_algorithm_extraction,
    language: "c",
    extension: "c",
    source: r#"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    int *data;
    size_t size;
    size_t capacity;
} DynamicArray;

typedef struct {
    int id;
    int value;
    char category[32];
} ProcessedItem;

int complex_data_processor(int *input, size_t input_size, int threshold, ProcessedItem **output, size_t *output_size) {
    if (!input || !output || !output_size) return -1;

    ProcessedItem *results = malloc(sizeof(ProcessedItem) * input_size);
    if (!results) return -1;

    size_t result_count = 0;

    // Main processing algorithm - extractable middle chunk
    for (size_t i = 0; i < input_size; i++) {
        int value = input[i];
        ProcessedItem item;
        item.id = (int)i;

        if (value > threshold) {
            int transformed = value * 2;
            item.value = transformed;

            if (transformed > threshold * 3) {
                strcpy(item.category, "HIGH");
            } else {
                strcpy(item.category, "MEDIUM");
            }

            // Additional processing for high values
            if (transformed > 100) {
                item.value += 10; // bonus
            }
        } else if (value > 0) {
            item.value = value + threshold;
            strcpy(item.category, "LOW");
        } else {
            continue; // skip negative values
        }

        results[result_count++] = item;
    }

    // Finalization logic
    if (result_count > 0) {
        // Calculate average for validation
        int total = 0;
        for (size_t i = 0; i < result_count; i++) {
            total += results[i].value;
        }
        int average = total / (int)result_count;

        // Add average as metadata (simplified approach)
        printf("Average processed value: %d\n", average);
    }

    *output = results;
    *output_size = result_count;
    return 0;
}

void analyze_patterns(ProcessedItem *items, size_t count) {
    if (!items || count == 0) return;

    int category_counts[3] = {0}; // LOW, MEDIUM, HIGH
    int value_sums[3] = {0};

    // Pattern analysis logic - extractable middle chunk
    for (size_t i = 0; i < count; i++) {
        ProcessedItem *item = &items[i];
        int category_index = -1;

        if (strcmp(item->category, "LOW") == 0) {
            category_index = 0;
        } else if (strcmp(item->category, "MEDIUM") == 0) {
            category_index = 1;
        } else if (strcmp(item->category, "HIGH") == 0) {
            category_index = 2;
        }

        if (category_index >= 0) {
            category_counts[category_index]++;
            value_sums[category_index] += item->value;

            // Time-based analysis simulation
            if (item->value > 1000) {
                printf("High value item found: %d\n", item->value);
            }
        }
    }

    // Output analysis results
    const char *categories[] = {"LOW", "MEDIUM", "HIGH"};
    for (int i = 0; i < 3; i++) {
        if (category_counts[i] > 0) {
            int average = value_sums[i] / category_counts[i];
            printf("Category %s: %d items, average value: %d\n",
                   categories[i], category_counts[i], average);
        }
    }
}
"#,
    total_chunks: 38,
    chunk_counts: {
        Function: 2,
        Struct: 2,
    }
}
