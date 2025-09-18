use crate::integration::languages::extractor::test_language_extractor;

test_language_extractor! {
    name: test_python_function_extraction,
    language: "python",
    extension: "py",
    source: r#"
def hello_world():
    print("Hello, world!")

def calculate_sum(a, b):
    return a + b

def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)
"#,
    total_chunks: 3,
    chunk_counts: {
        Function: 3,
    }
}

test_language_extractor! {
    name: test_python_class_extraction,
    language: "python",
    extension: "py",
    source: r#"
class Person:
    def __init__(self, name, age):
        self.name = name
        self.age = age

    def greet(self):
        return f"Hello, I'm {self.name}!"

    def get_age(self):
        return self.age

class Calculator:
    def add(self, a, b):
        return a + b

    def multiply(self, a, b):
        return a * b
"#,
    total_chunks: 7,
    chunk_counts: {
        Class: 2,
        Function: 5,
    }
}

test_language_extractor! {
    name: test_python_combined_extraction,
    language: "python",
    extension: "py",
    source: r#"
# Module-level function
def calculate_total(items):
    return sum(items)

# Class definition
class User:
    def __init__(self, name, email):
        self.name = name
        self.email = email

    def get_display_name(self):
        return f"{self.name} ({self.email})"

# Another class
class Database:
    def __init__(self):
        self.connections = []

    def connect(self):
        return "Connected"

    def disconnect(self):
        print("Disconnected")

# Another function
def process_data(data):
    return [item.upper() for item in data]
"#,
    total_chunks: 9,
    chunk_counts: {
        Class: 2,
        Function: 7,
    }
}

test_language_extractor! {
    name: test_python_function_extraction_new,
    language: "python",
    extension: "py",
    source: r#"
def complex_algorithm(data, threshold=10):
    """Complex function with middle implementation segments"""
    if not data:
        return []

    # Initialize variables
    result = []
    processed_count = 0

    # Main processing loop - this should be extractable as middle chunk
    for item in data:
        if isinstance(item, dict):
            value = item.get('value', 0)
        else:
            value = item

        # Apply transformations
        if value > threshold:
            transformed = value * 2
            result.append({
                'original': value,
                'transformed': transformed,
                'ratio': transformed / value
            })
            processed_count += 1
        elif value > 0:
            result.append({
                'original': value,
                'transformed': value + threshold,
                'ratio': (value + threshold) / value
            })

    # Final processing
    if processed_count > 0:
        avg_ratio = sum(item['ratio'] for item in result) / len(result)
        for item in result:
            item['normalized_ratio'] = item['ratio'] / avg_ratio

    return result

def data_processor(input_data):
    """Function with complex logic patterns"""
    cache = {}
    results = []

    for idx, item in enumerate(input_data):
        key = f"item_{idx}"

        # Complex conditional logic - extractable middle chunk
        if key in cache:
            cached_result = cache[key]
            if cached_result['valid']:
                results.append(cached_result['data'])
            else:
                # Recompute
                new_result = item * 2 + 1
                cache[key] = {'data': new_result, 'valid': True}
                results.append(new_result)
        else:
            # First computation
            computed = item ** 2 if item > 5 else item * 3
            cache[key] = {'data': computed, 'valid': True}
            results.append(computed)

    return results
"#,
    total_chunks: 13,
    chunk_counts: {
        Function: 2,
    }
}

test_language_extractor! {
    name: test_python_class_extraction_new,
    language: "python",
    extension: "py",
    source: r#"
class DataAnalyzer:
    def __init__(self, dataset):
        self.dataset = dataset
        self.cache = {}
        self.stats = {
            'processed': 0,
            'errors': 0,
            'cache_hits': 0
        }

    def analyze_patterns(self, pattern_type='default'):
        """Method with complex middle implementation"""
        patterns = []

        # Pattern detection logic - extractable middle chunk
        for i, data_point in enumerate(self.dataset):
            try:
                if pattern_type == 'numeric':
                    if isinstance(data_point, (int, float)):
                        pattern = {
                            'type': 'number',
                            'value': data_point,
                            'position': i,
                            'quartile': self._get_quartile(data_point)
                        }
                        patterns.append(pattern)
                elif pattern_type == 'sequence':
                    if i > 0:
                        prev_value = self.dataset[i-1]
                        if data_point > prev_value:
                            trend = 'increasing'
                        elif data_point < prev_value:
                            trend = 'decreasing'
                        else:
                            trend = 'stable'

                        pattern = {
                            'type': 'sequence',
                            'trend': trend,
                            'change': data_point - prev_value,
                            'position': i
                        }
                        patterns.append(pattern)

                self.stats['processed'] += 1
            except Exception as e:
                self.stats['errors'] += 1
                continue

        return patterns

    def _get_quartile(self, value):
        sorted_data = sorted([x for x in self.dataset if isinstance(x, (int, float))])
        if not sorted_data:
            return 0

        q1 = sorted_data[len(sorted_data) // 4]
        q3 = sorted_data[3 * len(sorted_data) // 4]

        if value <= q1:
            return 1
        elif value <= q3:
            return 2
        else:
            return 3
"#,
    total_chunks: 12,
    chunk_counts: {
        Class: 1,
        Function: 3,
    }
}

test_language_extractor! {
    name: test_python_combined_extraction_new,
    language: "python",
    extension: "py",
    source: r#"
import json
from typing import List, Dict, Optional

def process_json_data(file_path: str) -> Dict:
    """Function with error handling and complex logic"""
    try:
        with open(file_path, 'r') as file:
            data = json.load(file)
    except FileNotFoundError:
        return {'error': 'File not found', 'data': None}
    except json.JSONDecodeError:
        return {'error': 'Invalid JSON', 'data': None}

    # Data validation and transformation - extractable middle chunk
    if not isinstance(data, dict):
        return {'error': 'Expected dictionary format', 'data': None}

    processed_items = []
    for key, value in data.items():
        if isinstance(value, list):
            # Process list items
            processed_list = []
            for item in value:
                if isinstance(item, dict) and 'id' in item:
                    processed_item = {
                        'id': item['id'],
                        'processed': True,
                        'original_keys': list(item.keys())
                    }
                    processed_list.append(processed_item)

            processed_items.append({
                'key': key,
                'type': 'list',
                'items': processed_list,
                'count': len(processed_list)
            })
        elif isinstance(value, dict):
            processed_items.append({
                'key': key,
                'type': 'dict',
                'keys': list(value.keys()),
                'count': len(value)
            })

    return {
        'error': None,
        'data': processed_items,
        'total_processed': len(processed_items)
    }

class ConfigManager:
    def __init__(self, config_path: str):
        self.config_path = config_path
        self.config = {}
        self.load_config()

    def load_config(self) -> bool:
        """Load configuration with fallback logic"""
        try:
            with open(self.config_path, 'r') as file:
                self.config = json.load(file)
            return True
        except Exception:
            # Default configuration - extractable middle chunk
            self.config = {
                'debug': False,
                'log_level': 'INFO',
                'database': {
                    'host': 'localhost',
                    'port': 5432,
                    'name': 'default_db'
                },
                'cache': {
                    'enabled': True,
                    'ttl': 3600,
                    'max_size': 1000
                },
                'features': {
                    'analytics': True,
                    'notifications': False,
                    'beta_features': False
                }
            }
            return False

    def get_setting(self, key: str, default=None):
        """Get setting with dot notation support"""
        keys = key.split('.')
        value = self.config

        # Navigate through nested keys - extractable middle chunk
        for k in keys:
            if isinstance(value, dict) and k in value:
                value = value[k]
            else:
                return default

        return value
"#,
    total_chunks: 18,
    chunk_counts: {
        Function: 4,
        Class: 1,
    }
}
