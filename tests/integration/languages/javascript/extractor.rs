use crate::integration::languages::extractor::test_language_extractor;

test_language_extractor! {
    name: test_javascript_function_extraction,
    language: "javascript",
    extension: "js",
    source: r#"
function calculateSum(a, b) {
    return a + b;
}

function greetUser(name) {
    return `Hello, ${name}!`;
}

async function fetchUserData(userId) {
    const response = await fetch(`/api/users/${userId}`);
    return response.json();
}
"#,
    total_chunks: 6,
    chunk_counts: {
        Function: 3,
        CodeBlock: 2,
        File: 1,
    }
}

test_language_extractor! {
    name: test_javascript_arrow_function_extraction,
    language: "javascript",
    extension: "js",
    source: r#"
const add = (a, b) => a + b;

const multiply = (x, y) => {
    return x * y;
};

const processData = async (data) => {
    const processed = data.map(item => item.value);
    return processed;
};
"#,
    total_chunks: 4,
    chunk_counts: {
        Function: 3,
        File: 1,
    }
}

test_language_extractor! {
    name: test_javascript_class_extraction,
    language: "javascript",
    extension: "js",
    source: r#"
class UserManager {
    constructor(apiKey) {
        this.apiKey = apiKey;
        this.users = [];
    }

    async loadUsers() {
        try {
            const response = await fetchData('/users', {
                headers: { 'Authorization': `Bearer ${this.apiKey}` }
            });
            this.users = response.data;
            return this.users;
        } catch (error) {
            console.error('Failed to load users:', error);
            throw error;
        }
    }

    findUser = (id) => {
        return this.users.find(user => user.id === id);
    };
}

class EventEmitter {
    constructor() {
        this.events = {};
    }

    on(eventName, callback) {
        if (!this.events[eventName]) {
            this.events[eventName] = [];
        }
        this.events[eventName].push(callback);
    }

    emit(eventName, data) {
        if (this.events[eventName]) {
            this.events[eventName].forEach(callback => callback(data));
        }
    }
}
"#,
    total_chunks: 19,
    chunk_counts: {
        Class: 2,
        Method: 5,
        Conditional: 2,
        ErrorHandling: 1,
        FunctionCall: 1,
        Lambda: 1,
        CodeBlock: 6,
        File: 1,
    }
}

test_language_extractor! {
    name: test_javascript_export_extraction,
    language: "javascript",
    extension: "js",
    source: r#"
export function validateEmail(email) {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email);
}

export class ApiClient {
    constructor(baseUrl) {
        this.baseUrl = baseUrl;
    }

    async get(endpoint) {
        const response = await fetch(`${this.baseUrl}${endpoint}`);
        return response.json();
    }
}

export const config = {
    apiUrl: process.env.API_URL || 'http://localhost:3000',
    timeout: 5000
};

export default class UserService {
    constructor(apiClient) {
        this.apiClient = apiClient;
    }

    async getUser(id) {
        return this.apiClient.get(`/users/${id}`);
    }
}
"#,
    total_chunks: 12,
    chunk_counts: {
        Class: 2,
        Function: 1,
        Method: 4,
        CodeBlock: 4,
        File: 1,
    }
}

test_language_extractor! {
    name: test_javascript_object_method_extraction,
    language: "javascript",
    extension: "js",
    source: r#"
const utils = {
    formatDate: (date) => {
        return date.toLocaleDateString();
    },

    calculateAge: function(birthDate) {
        const today = new Date();
        return today.getFullYear() - birthDate.getFullYear();
    }
};

const eventHandlers = {
    handleClick: (event) => {
        console.log('Button clicked:', event.target);
    },

    handleSubmit: async function(formData) {
        try {
            const response = await fetch('/api/submit', {
                method: 'POST',
                body: formData
            });
            return response.json();
        } catch (error) {
            console.error('Submit failed:', error);
        }
    }
};
"#,
    total_chunks: 1,
    chunk_counts: {
        File: 1,
    }
}

test_language_extractor! {
    name: test_javascript_mixed_patterns,
    language: "javascript",
    extension: "js",
    source: r#"
import { fetchData } from './api.js';

class UserManager {
    constructor(apiKey) {
        this.apiKey = apiKey;
        this.users = [];
    }

    async loadUsers() {
        try {
            const response = await fetchData('/users', {
                headers: { 'Authorization': `Bearer ${this.apiKey}` }
            });
            this.users = response.data;
            return this.users;
        } catch (error) {
            console.error('Failed to load users:', error);
            throw error;
        }
    }

    findUser = (id) => {
        return this.users.find(user => user.id === id);
    };
}

function processUsers(users) {
    return users.map(user => ({
        ...user,
        displayName: `${user.firstName} ${user.lastName}`
    }));
}

const filterActiveUsers = (users) => {
    return users.filter(user => user.isActive);
};

const userService = {
    async createUser(userData) {
        const response = await fetch('/api/users', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(userData)
        });
        return response.json();
    }
};

export default UserManager;
"#,
    total_chunks: 18,
    chunk_counts: {
        Class: 1,
        Method: 3,
        Function: 2,
        Lambda: 1,
        ErrorHandling: 1,
        FunctionCall: 3,
        CodeBlock: 6,
        File: 1,
    }
}

test_language_extractor! {
    name: test_javascript_complex_algorithm_extraction,
    language: "javascript",
    extension: "js",
    source: r#"
function complexDataProcessor(input, options = {}) {
    const { threshold = 10, enableCaching = true } = options;
    const cache = new Map();
    const results = [];

    // Main processing algorithm - extractable middle chunk
    for (let i = 0; i < input.length; i++) {
        const item = input[i];
        const cacheKey = `item_${i}_${item.id || i}`;

        if (enableCaching && cache.has(cacheKey)) {
            const cachedResult = cache.get(cacheKey);
            if (cachedResult.valid) {
                results.push(cachedResult.data);
                continue;
            }
        }

        // Complex transformation logic
        let processedItem;
        if (typeof item === 'object' && item !== null) {
            const score = (item.value || 0) * (item.weight || 1);

            if (score > threshold) {
                processedItem = {
                    id: item.id || i,
                    originalValue: item.value,
                    score: score,
                    category: score > threshold * 2 ? 'high' : 'medium',
                    metadata: {
                        processed: true,
                        timestamp: Date.now(),
                        processor: 'complex'
                    }
                };
            } else {
                processedItem = {
                    id: item.id || i,
                    originalValue: item.value,
                    score: score + threshold,
                    category: 'low',
                    metadata: {
                        processed: true,
                        timestamp: Date.now(),
                        processor: 'simple',
                        adjusted: true
                    }
                };
            }
        } else {
            processedItem = {
                id: i,
                originalValue: item,
                score: Number(item) || 0,
                category: 'primitive',
                metadata: {
                    processed: true,
                    timestamp: Date.now(),
                    processor: 'primitive'
                }
            };
        }

        if (enableCaching) {
            cache.set(cacheKey, { data: processedItem, valid: true });
        }

        results.push(processedItem);
    }

    return results;
}

const dataAnalyzer = {
    analyzePatterns(data, patternTypes = ['sequence', 'frequency']) {
        const analysis = {
            patterns: [],
            statistics: {},
            insights: []
        };

        // Pattern analysis algorithm - extractable middle chunk
        if (patternTypes.includes('sequence')) {
            for (let i = 1; i < data.length; i++) {
                const current = data[i];
                const previous = data[i - 1];

                if (typeof current === 'number' && typeof previous === 'number') {
                    const difference = current - previous;
                    const percentChange = previous !== 0 ? (difference / previous) * 100 : 0;

                    const pattern = {
                        type: 'sequence',
                        position: i,
                        change: difference,
                        percentChange: percentChange,
                        trend: difference > 0 ? 'increasing' :
                               difference < 0 ? 'decreasing' : 'stable',
                        magnitude: Math.abs(percentChange) > 50 ? 'significant' : 'minor'
                    };

                    analysis.patterns.push(pattern);
                }
            }
        }

        if (patternTypes.includes('frequency')) {
            const frequency = {};
            data.forEach(item => {
                const key = typeof item === 'object' ?
                    JSON.stringify(item) : String(item);
                frequency[key] = (frequency[key] || 0) + 1;
            });

            analysis.statistics.frequency = frequency;
            analysis.statistics.mostCommon = Object.entries(frequency)
                .sort(([,a], [,b]) => b - a)
                .slice(0, 5);
        }

        return analysis;
    }
};
"#,
    total_chunks: 21,
    chunk_counts: {
        Function: 1,
        Method: 1,
        Loop: 1,
        Conditional: 8,
        FunctionCall: 3,
        CodeBlock: 6,
        File: 1,
    }
}

test_language_extractor! {
    name: test_javascript_promises_callbacks,
    language: "javascript",
    extension: "js",
    source: r#"
function fetchWithCallback(url, callback) {
    setTimeout(() => {
        const data = { status: 'success', url: url };
        callback(null, data);
    }, 1000);
}

function promiseExample() {
    return new Promise((resolve, reject) => {
        const success = Math.random() > 0.5;
        if (success) {
            resolve({ message: 'Success!' });
        } else {
            reject(new Error('Failed'));
        }
    });
}

async function chainedPromises() {
    try {
        const result1 = await promiseExample();
        const result2 = await promiseExample();
        return [result1, result2];
    } catch (error) {
        console.error('Promise chain failed:', error);
        throw error;
    }
}

const promiseAllExample = async (urls) => {
    const promises = urls.map(url => fetch(url));
    const results = await Promise.all(promises);
    return results.map(r => r.json());
};

Promise.race([
    new Promise((resolve) => setTimeout(() => resolve('fast'), 100)),
    new Promise((resolve) => setTimeout(() => resolve('slow'), 500))
]).then(result => {
    console.log('Winner:', result);
});
"#,
    total_chunks: 16,
    chunk_counts: {
        CodeBlock: 7,
        Function: 4,
        File: 1,
        Conditional: 1,
        ErrorHandling: 1,
        Lambda: 1,
        FunctionCall: 1,
    }
}

test_language_extractor! {
    name: test_javascript_destructuring_spread,
    language: "javascript",
    extension: "js",
    source: r#"
const user = {
    name: 'John Doe',
    age: 30,
    email: 'john@example.com',
    address: {
        city: 'New York',
        country: 'USA'
    }
};

const { name, age, address: { city } } = user;

function processUser({ name, email, ...rest }) {
    return {
        displayName: name,
        contact: email,
        metadata: rest
    };
}

const numbers = [1, 2, 3, 4, 5];
const [first, second, ...remaining] = numbers;

function mergeArrays(arr1, arr2) {
    return [...arr1, ...arr2];
}

const defaults = { theme: 'dark', language: 'en' };
const userPrefs = { language: 'ja', fontSize: 14 };
const config = { ...defaults, ...userPrefs };

function sumAll(...numbers) {
    return numbers.reduce((sum, n) => sum + n, 0);
}

const createUser = ({ name = 'Anonymous', age = 0 } = {}) => {
    return { name, age, createdAt: Date.now() };
};

const [, , third] = [1, 2, 3, 4];
const { a: renamed, b = 10 } = { a: 5 };
"#,
    total_chunks: 7,
    chunk_counts: {
        CodeBlock: 2,
        Function: 4,
        File: 1,
    }
}

test_language_extractor! {
    name: test_javascript_generators_iterators,
    language: "javascript",
    extension: "js",
    source: r#"
function* simpleGenerator() {
    yield 1;
    yield 2;
    yield 3;
}

function* infiniteSequence() {
    let i = 0;
    while (true) {
        yield i++;
    }
}

function* fibonacci() {
    let [prev, curr] = [0, 1];
    while (true) {
        yield curr;
        [prev, curr] = [curr, prev + curr];
    }
}

function* rangeGenerator(start, end, step = 1) {
    for (let i = start; i < end; i += step) {
        yield i;
    }
}

async function* asyncGenerator() {
    for (let i = 0; i < 5; i++) {
        await new Promise(resolve => setTimeout(resolve, 100));
        yield i;
    }
}

class CustomIterable {
    constructor(data) {
        this.data = data;
    }

    *[Symbol.iterator]() {
        for (const item of this.data) {
            yield item * 2;
        }
    }

    async *asyncIterator() {
        for (const item of this.data) {
            await new Promise(resolve => setTimeout(resolve, 10));
            yield item;
        }
    }
}

function* delegatingGenerator() {
    yield* [1, 2, 3];
    yield* simpleGenerator();
}

const iterator = {
    data: [1, 2, 3],
    index: 0,
    next() {
        if (this.index < this.data.length) {
            return { value: this.data[this.index++], done: false };
        }
        return { done: true };
    }
};
"#,
    total_chunks: 12,
    chunk_counts: {
        Conditional: 1,
        CodeBlock: 4,
        File: 1,
        Method: 3,
        Loop: 2,
        Class: 1,
    }
}
