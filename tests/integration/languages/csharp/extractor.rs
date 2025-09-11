use crate::integration::{extract_chunks_for_test, test_extraction_options};
use gittype::extractor::CodeChunkExtractor;
use gittype::models::ChunkType;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_csharp_class_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.cs");

    let csharp_code = r#"
using System;
using System.Collections.Generic;

namespace MyApplication.Services
{
    public class UserService
    {
        private readonly IUserRepository _repository;

        public UserService(IUserRepository repository)
        {
            _repository = repository ?? throw new ArgumentNullException(nameof(repository));
        }

        public async Task<User> GetUserAsync(int id)
        {
            var user = await _repository.GetByIdAsync(id);
            return user;
        }

        public IEnumerable<User> GetActiveUsers()
        {
            return _repository.GetAll().Where(u => u.IsActive);
        }
    }
}
"#;
    fs::write(&file_path, csharp_code).unwrap();

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
            .unwrap();

    // Should find namespace and class
    assert!(chunks.len() >= 2);

    let class_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class))
        .collect();
    assert_eq!(class_chunks.len(), 1);

    let class_names: Vec<&String> = class_chunks.iter().map(|c| &c.name).collect();
    assert!(class_names.contains(&&"UserService".to_string()));

    // Check if we extracted methods
    let method_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Method))
        .collect();
    assert!(method_chunks.len() >= 2);
}

#[test]
fn test_csharp_interface_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.cs");

    let csharp_code = r#"
using System;
using System.Threading.Tasks;

namespace MyApplication.Contracts
{
    public interface IUserRepository
    {
        Task<User> GetByIdAsync(int id);
        Task<IEnumerable<User>> GetAllAsync();
        Task<bool> ExistsAsync(int id);
    }

    public interface IEmailService
    {
        Task SendEmailAsync(string to, string subject, string body);
        Task<bool> ValidateEmailAsync(string email);
    }
}
"#;
    fs::write(&file_path, csharp_code).unwrap();

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
            .unwrap();

    let interface_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Interface))
        .collect();
    assert_eq!(interface_chunks.len(), 2);

    let interface_names: Vec<&String> = interface_chunks.iter().map(|c| &c.name).collect();
    assert!(interface_names.contains(&&"IUserRepository".to_string()));
    assert!(interface_names.contains(&&"IEmailService".to_string()));
}

#[test]
fn test_csharp_struct_and_enum_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.cs");

    let csharp_code = r#"
namespace MyApplication.Models
{
    public struct Point
    {
        public int X { get; set; }
        public int Y { get; set; }

        public Point(int x, int y)
        {
            X = x;
            Y = y;
        }

        public double DistanceToOrigin()
        {
            return Math.Sqrt(X * X + Y * Y);
        }
    }

    public enum UserRole
    {
        Guest,
        User,
        Moderator,
        Administrator
    }

    public enum Status
    {
        Active = 1,
        Inactive = 0,
        Pending = 2
    }
}
"#;
    fs::write(&file_path, csharp_code).unwrap();

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
            .unwrap();

    let struct_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Struct))
        .collect();
    assert_eq!(struct_chunks.len(), 1);

    let enum_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Enum))
        .collect();
    assert_eq!(enum_chunks.len(), 2);

    let struct_names: Vec<&String> = struct_chunks.iter().map(|c| &c.name).collect();
    assert!(struct_names.contains(&&"Point".to_string()));

    let enum_names: Vec<&String> = enum_chunks.iter().map(|c| &c.name).collect();
    assert!(enum_names.contains(&&"UserRole".to_string()));
    assert!(enum_names.contains(&&"Status".to_string()));
}

#[test]
fn test_csharp_properties_and_fields() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.cs");

    let csharp_code = r#"
namespace MyApplication.Models
{
    public class User
    {
        private int _id;
        private string _email;

        public string Name { get; set; }
        public DateTime CreatedAt { get; private set; }
        
        public string FullName => $"{FirstName} {LastName}";
        
        public string FirstName { get; set; }
        public string LastName { get; set; }

        public User(int id, string email)
        {
            _id = id;
            _email = email;
            CreatedAt = DateTime.Now;
        }
    }
}
"#;
    fs::write(&file_path, csharp_code).unwrap();

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
            .unwrap();

    // Should find class and properties
    let class_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Class))
        .collect();
    assert_eq!(class_chunks.len(), 1);

    let property_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Variable))
        .collect();
    assert!(property_chunks.len() >= 5); // Properties and fields

    let class_names: Vec<&String> = class_chunks.iter().map(|c| &c.name).collect();
    assert!(class_names.contains(&&"User".to_string()));
}

#[test]
fn test_csharp_namespace_extraction() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.cs");

    let csharp_code = r#"
using System;

namespace MyApplication.Core.Services
{
    public class EmailService
    {
        public void SendEmail(string to, string subject, string body)
        {
            Console.WriteLine($"Sending email to {to}");
        }
    }
}

namespace MyApplication.Core.Models
{
    public class Message
    {
        public string Content { get; set; }
        public DateTime Timestamp { get; set; }
    }
}
"#;
    fs::write(&file_path, csharp_code).unwrap();

    let mut extractor = CodeChunkExtractor::new().unwrap();
    let chunks =
        extract_chunks_for_test(&mut extractor, temp_dir.path(), test_extraction_options())
            .unwrap();

    let namespace_chunks: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c.chunk_type, ChunkType::Namespace))
        .collect();
    assert_eq!(namespace_chunks.len(), 2);

    let namespace_names: Vec<&String> = namespace_chunks.iter().map(|c| &c.name).collect();
    assert!(namespace_names.contains(&&"MyApplication.Core.Services".to_string()));
    assert!(namespace_names.contains(&&"MyApplication.Core.Models".to_string()));
}
