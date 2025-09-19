use crate::integration::languages::extractor::test_language_extractor;

test_language_extractor! {
    name: test_csharp_class_extraction,
    language: "csharp",
    extension: "cs",
    source: r#"
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
"#,
    total_chunks: 14,
    chunk_counts: {
        CodeBlock: 9,
        File: 1,
        CodeBlock: 9,
        CodeBlock: 9,
        File: 1,
        Namespace: 0,
    }
}

test_language_extractor! {
    name: test_csharp_interface_extraction,
    language: "csharp",
    extension: "cs",
    source: r#"
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
"#,
    total_chunks: 12,
    chunk_counts: {
        CodeBlock: 6,
        File: 1,
        Method: 5,
    }
}

test_language_extractor! {
    name: test_csharp_struct_and_enum_extraction,
    language: "csharp",
    extension: "cs",
    source: r#"
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
"#,
    total_chunks: 13,
    chunk_counts: {
        CodeBlock: 10,
        File: 1,
        Method: 2,
    }
}

test_language_extractor! {
    name: test_csharp_properties_and_fields,
    language: "csharp",
    extension: "cs",
    source: r#"
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
"#,
    total_chunks: 11,
    chunk_counts: {
        CodeBlock: 8,
        File: 1,
        Method: 1,
        Class: 1,
    }
}

test_language_extractor! {
    name: test_csharp_namespace_extraction,
    language: "csharp",
    extension: "cs",
    source: r#"
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
"#,
    total_chunks: 12,
    chunk_counts: {
        CodeBlock: 8,
        File: 1,
        Class: 2,
        Method: 1,
    }
}

test_language_extractor! {
    name: test_csharp_complex_algorithm_extraction,
    language: "csharp",
    extension: "cs",
    source: r#"
using System;
using System.Collections.Generic;
using System.Linq;

namespace DataProcessing
{
    public class ProcessedItem
    {
        public int Id { get; set; }
        public int OriginalValue { get; set; }
        public int TransformedValue { get; set; }
        public string Category { get; set; }
        public DateTime Timestamp { get; set; }
        public Dictionary<string, object> Metadata { get; set; }
    }

    public class DataProcessor<T> where T : IComparable<T>
    {
        private readonly Dictionary<string, ProcessedItem> _cache;
        private readonly List<ProcessedItem> _processingLog;
        private readonly T _threshold;

        public DataProcessor(T threshold)
        {
            _cache = new Dictionary<string, ProcessedItem>();
            _processingLog = new List<ProcessedItem>();
            _threshold = threshold;
        }

        public List<ProcessedItem> ProcessComplexData(List<T> input)
        {
            var results = new List<ProcessedItem>();
            var processedCount = 0;

            // Main processing algorithm - extractable middle chunk
            for (int i = 0; i < input.Count; i++)
            {
                var value = input[i];
                var cacheKey = $"item_{i}_{value}";

                if (_cache.TryGetValue(cacheKey, out var cachedItem))
                {
                    results.Add(cachedItem);
                    continue;
                }

                var processedItem = new ProcessedItem
                {
                    Id = i,
                    OriginalValue = Convert.ToInt32(value),
                    Timestamp = DateTime.Now,
                    Metadata = new Dictionary<string, object>()
                };

                if (value.CompareTo(_threshold) > 0)
                {
                    processedItem.TransformedValue = processedItem.OriginalValue * 2;
                    processedItem.Category = processedItem.TransformedValue > Convert.ToInt32(_threshold) * 3 ? "HIGH" : "MEDIUM";
                    processedCount++;

                    // Additional processing for high values
                    if (processedItem.TransformedValue > 100)
                    {
                        processedItem.Metadata["bonus"] = true;
                        processedItem.TransformedValue += 10;
                    }
                }
                else if (value.CompareTo(default(T)) > 0)
                {
                    processedItem.TransformedValue = processedItem.OriginalValue + Convert.ToInt32(_threshold);
                    processedItem.Category = "LOW";
                }
                else
                {
                    continue; // skip invalid values
                }

                _cache[cacheKey] = processedItem;
                _processingLog.Add(processedItem);
                results.Add(processedItem);
            }

            // Finalization logic
            if (processedCount > 0)
            {
                var average = results.Average(r => r.TransformedValue);
                Console.WriteLine($"Processing complete. Average: {average:F2}");

                // Add summary metadata
                foreach (var item in results)
                {
                    item.Metadata["processing_average"] = average;
                }
            }

            return results;
        }

        public Dictionary<string, object> AnalyzePatterns(List<ProcessedItem> items)
        {
            var analysis = new Dictionary<string, object>();
            var categoryGroups = items.GroupBy(i => i.Category).ToDictionary(g => g.Key, g => g.ToList());

            // Pattern analysis logic - extractable middle chunk
            foreach (var categoryGroup in categoryGroups)
            {
                var category = categoryGroup.Key;
                var categoryItems = categoryGroup.Value;

                var categoryAnalysis = new Dictionary<string, object>
                {
                    ["count"] = categoryItems.Count,
                    ["percentage"] = (double)categoryItems.Count / items.Count * 100,
                    ["avg_value"] = categoryItems.Average(i => i.TransformedValue),
                    ["min_value"] = categoryItems.Min(i => i.TransformedValue),
                    ["max_value"] = categoryItems.Max(i => i.TransformedValue)
                };

                // Time-based analysis
                var recentItems = categoryItems.Where(i => (DateTime.Now - i.Timestamp).TotalMinutes < 1).ToList();
                if (recentItems.Any())
                {
                    categoryAnalysis["recent_count"] = recentItems.Count;
                    categoryAnalysis["recent_avg"] = recentItems.Average(i => i.TransformedValue);
                }

                // High-value analysis
                var highValueItems = categoryItems.Where(i => i.TransformedValue > 1000).ToList();
                if (highValueItems.Any())
                {
                    categoryAnalysis["high_value_count"] = highValueItems.Count;
                }

                analysis[category] = categoryAnalysis;
            }

            analysis["total_items"] = items.Count;
            analysis["processing_time"] = DateTime.Now;

            return analysis;
        }
    }
}
"#,
    total_chunks: 42,
    chunk_counts: {
        CodeBlock: 26,
        File: 1,
        Class: 2,
        Method: 3,
    }
}
