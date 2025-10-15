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
        Namespace: 1,
        Class: 1,
        Method: 3,
        CodeBlock: 8,
        File: 1,
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
        Namespace: 1,
        Interface: 2,
        Method: 5,
        CodeBlock: 3,
        File: 1,
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
        Namespace: 1,
        Struct: 1,
        Enum: 2,
        Method: 2,
        Variable: 2,
        CodeBlock: 4,
        File: 1,
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
        Namespace: 1,
        Class: 1,
        Method: 1,
        Variable: 5,
        CodeBlock: 2,
        File: 1,
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
        Namespace: 2,
        Class: 2,
        Method: 1,
        Variable: 2,
        CodeBlock: 4,
        File: 1,
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
        Namespace: 1,
        Class: 2,
        Method: 3,
        Variable: 6,
        Loop: 3,
        Conditional: 7,
        CodeBlock: 19,
        File: 1,
    }
}

test_language_extractor! {
    name: test_csharp_linq_and_lambda,
    language: "csharp",
    extension: "cs",
    source: r#"
using System;
using System.Collections.Generic;
using System.Linq;

namespace DataAnalysis
{
    public class DataAnalyzer
    {
        public List<int> ProcessNumbers(List<int> numbers)
        {
            var filtered = numbers.Where(n => n > 10).ToList();
            var mapped = filtered.Select(n => n * 2).ToList();
            var sorted = mapped.OrderBy(n => n).ToList();
            return sorted;
        }

        public Dictionary<string, int> GroupAndCount<T>(List<T> items, Func<T, string> keySelector)
        {
            return items
                .GroupBy(keySelector)
                .ToDictionary(g => g.Key, g => g.Count());
        }

        public IEnumerable<string> GetFormattedResults(List<int> numbers)
        {
            return from n in numbers
                   where n % 2 == 0
                   orderby n descending
                   select $"Number: {n}";
        }

        public bool AnyMatchesCondition(List<int> numbers, Func<int, bool> predicate)
        {
            return numbers.Any(predicate);
        }

        public int AggregateSum(List<int> numbers)
        {
            return numbers.Aggregate(0, (sum, n) => sum + n);
        }
    }
}
"#,
    total_chunks: 22,
    chunk_counts: {
        Method: 5,
        File: 1,
        FunctionCall: 2,
        Namespace: 1,
        CodeBlock: 12,
        Class: 1,
    }
}

test_language_extractor! {
    name: test_csharp_async_await,
    language: "csharp",
    extension: "cs",
    source: r#"
using System;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;

namespace AsyncOperations
{
    public class AsyncDataFetcher
    {
        private readonly HttpClient _httpClient;

        public AsyncDataFetcher(HttpClient httpClient)
        {
            _httpClient = httpClient;
        }

        public async Task<string> FetchDataAsync(string url)
        {
            try
            {
                var response = await _httpClient.GetAsync(url);
                response.EnsureSuccessStatusCode();
                return await response.Content.ReadAsStringAsync();
            }
            catch (HttpRequestException ex)
            {
                Console.WriteLine($"Request failed: {ex.Message}");
                return null;
            }
        }

        public async Task<List<string>> FetchMultipleAsync(List<string> urls)
        {
            var tasks = urls.Select(url => FetchDataAsync(url)).ToList();
            var results = await Task.WhenAll(tasks);
            return results.Where(r => r != null).ToList();
        }

        public async Task<T> RetryAsync<T>(Func<Task<T>> operation, int maxRetries)
        {
            for (int i = 0; i < maxRetries; i++)
            {
                try
                {
                    return await operation();
                }
                catch (Exception)
                {
                    if (i == maxRetries - 1)
                        throw;
                    await Task.Delay(TimeSpan.FromSeconds(Math.Pow(2, i)));
                }
            }
            throw new InvalidOperationException("Should not reach here");
        }

        public async Task ProcessWithCancellationAsync(CancellationToken cancellationToken)
        {
            while (!cancellationToken.IsCancellationRequested)
            {
                await Task.Delay(1000, cancellationToken);
                Console.WriteLine("Processing...");
            }
        }
    }
}
"#,
    total_chunks: 31,
    chunk_counts: {
        File: 1,
        Namespace: 1,
        Method: 5,
        Loop: 2,
        Class: 1,
        Conditional: 1,
        CodeBlock: 18,
        ErrorHandling: 2,
    }
}

test_language_extractor! {
    name: test_csharp_records_and_pattern_matching,
    language: "csharp",
    extension: "cs",
    source: r#"
using System;

namespace ModernCSharp
{
    public record Person(string FirstName, string LastName, int Age);

    public record Employee(string FirstName, string LastName, int Age, string Department)
        : Person(FirstName, LastName, Age);

    public abstract record Shape;
    public record Circle(double Radius) : Shape;
    public record Rectangle(double Width, double Height) : Shape;
    public record Triangle(double Base, double Height) : Shape;

    public class ShapeAnalyzer
    {
        public double CalculateArea(Shape shape)
        {
            return shape switch
            {
                Circle c => Math.PI * c.Radius * c.Radius,
                Rectangle r => r.Width * r.Height,
                Triangle t => 0.5 * t.Base * t.Height,
                _ => throw new ArgumentException("Unknown shape")
            };
        }

        public string DescribePerson(Person person)
        {
            return person switch
            {
                Employee { Department: "Engineering" } emp => $"{emp.FirstName} is an engineer",
                Employee emp => $"{emp.FirstName} works in {emp.Department}",
                Person { Age: >= 65 } p => $"{p.FirstName} is a senior",
                Person { Age: < 18 } p => $"{p.FirstName} is a minor",
                Person p => $"{p.FirstName} is {p.Age} years old"
            };
        }

        public bool IsValidShape(Shape shape)
        {
            return shape is Circle { Radius: > 0 } or
                   Rectangle { Width: > 0, Height: > 0 } or
                   Triangle { Base: > 0, Height: > 0 };
        }

        public string GetShapeType(Shape shape)
        {
            if (shape is Circle circle)
            {
                return $"Circle with radius {circle.Radius}";
            }
            else if (shape is Rectangle rect)
            {
                return $"Rectangle {rect.Width}x{rect.Height}";
            }
            else if (shape is Triangle triangle)
            {
                return $"Triangle with base {triangle.Base}";
            }
            return "Unknown";
        }
    }
}
"#,
    total_chunks: 29,
    chunk_counts: {
        Conditional: 3,
        Class: 7,
        Method: 4,
        Namespace: 1,
        File: 1,
        CodeBlock: 13,
    }
}
