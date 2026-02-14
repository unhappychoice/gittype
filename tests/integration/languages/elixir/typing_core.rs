use crate::typing_core_test_with_parser;
use gittype::domain::models::typing::ProcessingOptions;

typing_core_test_with_parser!(
    elixir_function_with_comment,
    "elixir",
    r#"def hello(name) do
  # Greet the user
  "Hello, #{name}!"
end"#
);

typing_core_test_with_parser!(
    elixir_module_with_comments,
    "elixir",
    r#"defmodule Greeter do
  # Public greeting function
  def greet(name) do
    "Hello, #{name}!"
  end

  # Private helper
  defp format(text) do
    String.trim(text)
  end
end"#
);

typing_core_test_with_parser!(
    elixir_pattern_matching_with_comments,
    "elixir",
    r#"def process(value) do
  # Handle different input types
  case value do
    {:ok, result} -> result
    {:error, reason} -> raise reason
    _ -> nil
  end
end"#
);

typing_core_test_with_parser!(
    elixir_pipe_operator_with_comments,
    "elixir",
    r#"def transform(data) do
  # Pipeline processing
  data
  |> Enum.map(&String.upcase/1)
  |> Enum.filter(&valid?/1)
  |> Enum.join(", ")
end"#
);

typing_core_test_with_parser!(
    elixir_empty_line_preservation_enabled,
    "elixir",
    r#"def process(x) do
  y = x + 1

  z = y * 2

  z
end"#,
    ProcessingOptions {
        preserve_empty_lines: true,
        ..ProcessingOptions::default()
    }
);

typing_core_test_with_parser!(
    elixir_empty_line_preservation_disabled,
    "elixir",
    r#"def process(x) do
  y = x + 1

  z = y * 2

  z
end"#,
    ProcessingOptions {
        preserve_empty_lines: false,
        ..ProcessingOptions::default()
    }
);

typing_core_test_with_parser!(
    elixir_doc_comment_with_heredoc,
    "elixir",
    r#"defmodule Math do
  @doc """
  Adds two numbers together.
  """
  def add(a, b) do
    a + b
  end
end"#
);

typing_core_test_with_parser!(
    elixir_nested_with_expression_comments,
    "elixir",
    r#"def create_user(params) do
  # Validate and create user with chained operations
  with {:ok, name} <- validate_name(params),
       # Check email format
       {:ok, email} <- validate_email(params),
       # Persist to database
       {:ok, user} <- insert_user(name, email) do
    {:ok, user}
  else
    {:error, reason} -> {:error, reason}
  end
end"#
);

typing_core_test_with_parser!(
    elixir_genserver_with_comments,
    "elixir",
    r#"defmodule Counter do
  use GenServer

  # Client API
  def start_link(opts) do
    GenServer.start_link(__MODULE__, opts)
  end

  # Server callbacks
  def init(initial_value) do
    {:ok, initial_value}
  end

  def handle_call(:get, _from, state) do
    # Return current count
    {:reply, state, state}
  end
end"#
);

typing_core_test_with_parser!(
    elixir_multi_clause_with_comments,
    "elixir",
    r#"def fibonacci(0), do: 0
# Base case for 1
def fibonacci(1), do: 1
# Recursive case
def fibonacci(n) when n > 1 do
  fibonacci(n - 1) + fibonacci(n - 2)
end"#
);
