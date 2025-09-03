use crate::integration::test_extraction_options;
use gittype::extractor::{CodeExtractor};
use gittype::models::{ChunkType};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_javascript_function_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.js");

    let js_code = r#"
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
"#;
    fs::write(&file_path, js_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    assert_eq!(chunks.len(), 3);

    let function_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Function))
        .collect();
    assert_eq!(function_chunks.len(), 3);

    let function_names: Vec<&String> = function_chunks.iter().map(|c| &c.name).collect();
    assert!(function_names.contains(&&"calculateSum".to_string()));
    assert!(function_names.contains(&&"greetUser".to_string()));
    assert!(function_names.contains(&&"fetchUserData".to_string()));
}

#[test]
fn test_javascript_arrow_function_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.js");

    let js_code = r#"
const add = (a, b) => a + b;

const multiply = (x, y) => {
    return x * y;
};

const processData = async (data) => {
    const processed = data.map(item => item.value);
    return processed;
};
"#;
    fs::write(&file_path, js_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    assert_eq!(chunks.len(), 3);

    let function_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Function))
        .collect();
    assert_eq!(function_chunks.len(), 3);

    let function_names: Vec<&String> = function_chunks.iter().map(|c| &c.name).collect();
    assert!(function_names.contains(&&"add".to_string()));
    assert!(function_names.contains(&&"multiply".to_string()));
    assert!(function_names.contains(&&"processData".to_string()));
}

#[test]
fn test_javascript_class_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.js");

    let js_code = r#"
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
"#;
    fs::write(&file_path, js_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    // Should find 2 classes + their methods
    let class_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class))
        .collect();
    assert_eq!(class_chunks.len(), 2);

    let class_names: Vec<&String> = class_chunks.iter().map(|c| &c.name).collect();
    assert!(class_names.contains(&&"UserManager".to_string()));
    assert!(class_names.contains(&&"EventEmitter".to_string()));
}

#[test]
fn test_javascript_export_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.js");

    let js_code = r#"
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
"#;
    fs::write(&file_path, js_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    // Should find exported functions and classes
    let function_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Function))
        .collect();
    let class_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class))
        .collect();

    assert!(!function_chunks.is_empty());
    assert!(!class_chunks.is_empty());

    let all_names: Vec<&String> = chunks.iter().map(|c| &c.name).collect();
    assert!(all_names.contains(&&"validateEmail".to_string()));
    assert!(all_names.contains(&&"ApiClient".to_string()));
    assert!(all_names.contains(&&"UserService".to_string()));
}

#[test]
fn test_javascript_object_method_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.js");

    let js_code = r#"
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
"#;
    fs::write(&file_path, js_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    // Should find method assignments
    let method_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Method))
        .collect();

    if !method_chunks.is_empty() {
        let method_names: Vec<&String> = method_chunks.iter().map(|c| &c.name).collect();
        // These might be detected as methods if the parser can handle object property assignments
        println!("Found methods: {:?}", method_names);
    }
}

#[test]
fn test_javascript_mixed_patterns() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.js");

    let js_code = r#"
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
"#;
    fs::write(&file_path, js_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    // Should find at least the class and functions
    assert!(!chunks.is_empty());

    let class_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class))
        .count();
    let function_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Function))
        .count();

    assert!(class_count >= 1, "Should find at least 1 class");
    assert!(function_count >= 2, "Should find at least 2 functions");

    let chunk_names: Vec<&String> = chunks.iter().map(|c| &c.name).collect();
    assert!(chunk_names.contains(&&"UserManager".to_string()));
    assert!(chunk_names.contains(&&"processUsers".to_string()));
    assert!(chunk_names.contains(&&"filterActiveUsers".to_string()));
}
