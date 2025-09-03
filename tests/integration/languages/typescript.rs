use crate::integration::test_extraction_options;
use gittype::extractor::{ChunkType, CodeExtractor};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_typescript_interface_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.ts");

    let ts_code = r#"
interface User {
    id: number;
    name: string;
    email?: string;
}

interface Admin extends User {
    permissions: string[];
}
"#;
    fs::write(&file_path, ts_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    assert_eq!(chunks.len(), 2);

    let interface_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Interface))
        .collect();
    assert_eq!(interface_chunks.len(), 2);

    let interface_names: Vec<&String> = interface_chunks.iter().map(|c| &c.name).collect();
    assert!(interface_names.contains(&&"User".to_string()));
    assert!(interface_names.contains(&&"Admin".to_string()));
}

#[test]
fn test_typescript_type_alias_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.ts");

    let ts_code = r#"
type Status = 'pending' | 'completed' | 'failed';
type UserId = number;
type ApiResponse<T> = {
    data: T;
    error?: string;
};
"#;
    fs::write(&file_path, ts_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    assert_eq!(chunks.len(), 3);

    let type_alias_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::TypeAlias))
        .collect();
    assert_eq!(type_alias_chunks.len(), 3);

    let type_names: Vec<&String> = type_alias_chunks.iter().map(|c| &c.name).collect();
    assert!(type_names.contains(&&"Status".to_string()));
    assert!(type_names.contains(&&"UserId".to_string()));
    assert!(type_names.contains(&&"ApiResponse".to_string()));
}

#[test]
fn test_typescript_enum_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.ts");

    let ts_code = r#"
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
"#;
    fs::write(&file_path, ts_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    assert_eq!(chunks.len(), 2);

    let enum_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Enum))
        .collect();
    assert_eq!(enum_chunks.len(), 2);

    let enum_names: Vec<&String> = enum_chunks.iter().map(|c| &c.name).collect();
    assert!(enum_names.contains(&&"Color".to_string()));
    assert!(enum_names.contains(&&"Status".to_string()));
}

#[test]
fn test_typescript_namespace_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.ts");

    let ts_code = r#"
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
"#;
    fs::write(&file_path, ts_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    // Should find 2 namespaces + 2 functions + 1 interface = 5 total
    assert!(chunks.len() >= 2, "Should find at least 2 namespace chunks");

    let namespace_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Module))
        .collect();
    assert!(
        namespace_chunks.len() >= 2,
        "Should find at least 2 namespaces"
    );

    let namespace_names: Vec<&String> = namespace_chunks.iter().map(|c| &c.name).collect();
    assert!(namespace_names.contains(&&"Utils".to_string()));
    assert!(namespace_names.contains(&&"Api".to_string()));
}

#[test]
fn test_typescript_all_new_constructs_combined() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.ts");

    let ts_code = r#"
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
"#;
    fs::write(&file_path, ts_code).unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let chunks = extractor
        .extract_chunks(temp_dir.path(), test_extraction_options())
        .unwrap();

    // Should find: 1 interface + 1 type alias + 1 enum + 1 namespace + 1 class + 4 functions = 9 total minimum
    assert!(
        chunks.len() >= 9,
        "Should find at least 9 chunks, got {}",
        chunks.len()
    );

    // Verify each new chunk type
    let interface_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Interface))
        .count();
    let type_alias_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::TypeAlias))
        .count();
    let enum_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Enum))
        .count();
    let namespace_count = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Module))
        .count();

    assert_eq!(interface_count, 1, "Should find 1 interface");
    assert_eq!(type_alias_count, 1, "Should find 1 type alias");
    assert_eq!(enum_count, 1, "Should find 1 enum");
    assert_eq!(namespace_count, 1, "Should find 1 namespace");

    // Verify names of all new constructs
    let chunk_names: Vec<&String> = chunks.iter().map(|c| &c.name).collect();
    assert!(chunk_names.contains(&&"User".to_string()));
    assert!(chunk_names.contains(&&"Status".to_string()));
    assert!(chunk_names.contains(&&"Color".to_string()));
    assert!(chunk_names.contains(&&"Utils".to_string()));
}
