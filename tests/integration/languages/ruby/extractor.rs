use crate::integration::languages::extractor::test_language_extractor;

test_language_extractor! {
    name: test_ruby_function_extraction,
    language: "ruby",
    extension: "rb",
    source: r#"
def hello_world
  puts "Hello, world!"
end

def calculate_sum(a, b)
  a + b
end
"#,
    total_chunks: 5,
    chunk_counts: {
        File: 1,
        Method: 2,
        CodeBlock: 2,
    }
}

test_language_extractor! {
    name: test_ruby_class_extraction,
    language: "ruby",
    extension: "rb",
    source: r#"
class Person
  attr_accessor :name, :age

  def initialize(name, age)
    @name = name
    @age = age
  end

  def greet
    puts "Hello, I'm #{@name}!"
  end
end
"#,
    total_chunks: 6,
    chunk_counts: {
        File: 1,
        Class: 1,
        Method: 3,
        CodeBlock: 1,
    }
}

test_language_extractor! {
    name: test_ruby_module_extraction,
    language: "ruby",
    extension: "rb",
    source: r#"
module Authentication
  def login(username, password)
    puts "Logging in #{username}"
    true
  end

  def logout
    puts "Logged out"
  end
end
"#,
    total_chunks: 5,
    chunk_counts: {
        File: 1,
        Method: 2,
        Module: 1,
        CodeBlock: 1,
    }
}

test_language_extractor! {
    name: test_ruby_class_method_extraction,
    language: "ruby",
    extension: "rb",
    source: r#"
class User
  def self.find_by_email(email)
    puts "Finding user by email: #{email}"
  end

  def instance_method
    "instance"
  end
end
"#,
    total_chunks: 6,
    chunk_counts: {
        File: 1,
        Class: 1,
        CodeBlock: 2,
        Method: 2,
    }
}

test_language_extractor! {
    name: test_ruby_attr_accessor_extraction,
    language: "ruby",
    extension: "rb",
    source: r#"
class Product
  attr_accessor :name, :price
  attr_reader :id
  attr_writer :description
end
"#,
    total_chunks: 5,
    chunk_counts: {
        File: 1,
        Method: 3,
        Class: 1,
    }
}

test_language_extractor! {
    name: test_ruby_complex_algorithm_extraction,
    language: "ruby",
    extension: "rb",
    source: r#"
class ProcessedItem
  attr_accessor :id, :original_value, :transformed_value, :category, :timestamp, :metadata

  def initialize(id, original_value, transformed_value, category, metadata = {})
    @id = id
    @original_value = original_value
    @transformed_value = transformed_value
    @category = category
    @timestamp = Time.now
    @metadata = metadata
  end
end

class DataProcessor
  def initialize(threshold)
    @threshold = threshold
    @cache = {}
    @processing_log = []
  end

  def process_complex_data(input)
    results = []
    processed_count = 0

    # Main processing algorithm - extractable middle chunk
    input.each_with_index do |value, index|
      cache_key = "item_#{index}_#{value}"

      if @cache.key?(cache_key)
        results << @cache[cache_key]
        next
      end

      processed_item = if value > @threshold
                         transformed_value = value * 2
                         category = transformed_value > @threshold * 3 ? 'HIGH' : 'MEDIUM'
                         bonus_value = transformed_value > 100 ? transformed_value + 10 : transformed_value

                         ProcessedItem.new(
                           index,
                           value,
                           bonus_value,
                           category,
                           {
                             processed: true,
                             multiplier: 0,
                             processor: 'enhanced'
                           }
                         ).tap { processed_count += 1 }
                       elsif value > 0
                         ProcessedItem.new(
                           index,
                           value,
                           value + @threshold,
                           'LOW',
                           {
                             processed: true,
                             adjusted: true,
                             processor: 'basic'
                           }
                         )
                       else
                         next # skip negative values
                       end

      @cache[cache_key] = processed_item
      @processing_log << processed_item
      results << processed_item
    end

    # Finalization logic
    if processed_count > 0
      average = results.sum(&:transformed_value).to_f / results.size
      puts "Processing complete. Average: #{format('%.2f', average)}"

      # Add processing statistics
      results.each { |item| item.metadata[:processing_average] = average }
    end

    results
  end

  def analyze_patterns(items)
    analysis = {}
    category_groups = items.group_by(&:category)

    # Pattern analysis logic - extractable middle chunk
    category_groups.each do |category, category_items|
      values = category_items.map(&:transformed_value)
      category_analysis = {
        count: category_items.size,
        percentage: (category_items.size.to_f / items.size * 100),
        avg_value: values.sum.to_f / values.size,
        min_value: values.min,
        max_value: values.max
      }

      # Time-based analysis
      current_time = Time.now
      recent_items = category_items.select { |item| current_time - item.timestamp < 60 } # last minute
      unless recent_items.empty?
        recent_values = recent_items.map(&:transformed_value)
        category_analysis[:recent_count] = recent_items.size
        category_analysis[:recent_avg] = recent_values.sum.to_f / recent_values.size
      end

      # High-value analysis
      high_value_items = category_items.select { |item| item.transformed_value > 1000 }
      category_analysis[:high_value_count] = high_value_items.size unless high_value_items.empty?

      analysis[category] = category_analysis
    end

    analysis.merge(
      total_items: items.size,
      processing_time: Time.now.to_i
    )
  end
end
"#,
    total_chunks: 23,
    chunk_counts: {
        File: 1,
        Method: 5,
        FunctionCall: 5,
        Conditional: 4,
        Class: 2,
        CodeBlock: 6,
    }
}
