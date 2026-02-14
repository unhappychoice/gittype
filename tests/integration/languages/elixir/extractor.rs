use crate::integration::languages::extractor::test_language_extractor;

test_language_extractor! {
    name: test_elixir_defmodule_and_def_extraction,
    language: "elixir",
    extension: "ex",
    source: r#"
defmodule Greeter do
  def hello(name) do
    "Hello, #{name}!"
  end

  defp format_name(name) do
    String.trim(name)
  end
end
"#,
    total_chunks: 2,
    chunk_counts: {
        File: 1,
        Module: 1,
    }
}

test_language_extractor! {
    name: test_elixir_defmacro_extraction,
    language: "elixir",
    extension: "ex",
    source: r#"
defmodule MyMacros do
  defmacro unless(condition, do: block) do
    quote do
      if !unquote(condition), do: unquote(block)
    end
  end
end
"#,
    total_chunks: 2,
    chunk_counts: {
        File: 1,
        Module: 1,
    }
}

test_language_extractor! {
    name: test_elixir_defprotocol_extraction,
    language: "elixir",
    extension: "ex",
    source: r#"
defprotocol Stringify do
  def to_string(value)
end

defimpl Stringify, for: Integer do
  def to_string(value) do
    Integer.to_string(value)
  end
end
"#,
    total_chunks: 3,
    chunk_counts: {
        File: 1,
        Interface: 1,
        Class: 1,
    }
}

test_language_extractor! {
    name: test_elixir_defstruct_extraction,
    language: "elixir",
    extension: "ex",
    source: r#"
defmodule User do
  defstruct [:name, :email, :age]

  def new(name, email) do
    %User{name: name, email: email, age: 0}
  end
end
"#,
    total_chunks: 3,
    chunk_counts: {
        File: 1,
        Module: 1,
        Struct: 1,
    }
}

test_language_extractor! {
    name: test_elixir_mixed_definitions,
    language: "elixir",
    extension: "ex",
    source: r#"
defmodule Calculator do
  def add(a, b) do
    a + b
  end

  def subtract(a, b) do
    a - b
  end

  defp validate(n) do
    is_number(n)
  end
end
"#,
    total_chunks: 3,
    chunk_counts: {
        File: 1,
        Module: 1,
        CodeBlock: 1,
    }
}

test_language_extractor! {
    name: test_elixir_genserver_extraction,
    language: "elixir",
    extension: "ex",
    source: r#"
defmodule MyApp.Counter do
  use GenServer

  def start_link(initial_value) do
    GenServer.start_link(__MODULE__, initial_value, name: __MODULE__)
  end

  def increment do
    GenServer.call(__MODULE__, :increment)
  end

  def init(initial_value) do
    {:ok, initial_value}
  end

  def handle_call(:increment, _from, state) do
    {:reply, state + 1, state + 1}
  end

  def handle_cast(:reset, _state) do
    {:noreply, 0}
  end
end
"#,
    total_chunks: 4,
    chunk_counts: {
        File: 1,
        Module: 1,
        Function: 1,
        CodeBlock: 1,
    }
}

test_language_extractor! {
    name: test_elixir_defguard_extraction,
    language: "elixir",
    extension: "ex",
    source: r#"
defmodule Validators do
  defguard is_positive(value) when is_integer(value) and value > 0

  defguardp is_even(value) when rem(value, 2) == 0

  def validate(n) when is_positive(n) do
    {:ok, n}
  end

  def validate(_n) do
    {:error, :invalid}
  end
end
"#,
    total_chunks: 3,
    chunk_counts: {
        File: 1,
        Module: 1,
        CodeBlock: 1,
    }
}

test_language_extractor! {
    name: test_elixir_multiple_modules_extraction,
    language: "elixir",
    extension: "ex",
    source: r#"
defmodule MyApp.Schema do
  defstruct [:id, :name]
end

defmodule MyApp.Repo do
  def get(id) do
    %MyApp.Schema{id: id, name: "item"}
  end

  def all do
    []
  end
end

defprotocol Renderable do
  def render(data)
end
"#,
    total_chunks: 9,
    chunk_counts: {
        File: 1,
        Module: 2,
        Struct: 1,
        Interface: 1,
        Function: 1,
        CodeBlock: 3,
    }
}

test_language_extractor! {
    name: test_elixir_complex_algorithm_extraction,
    language: "elixir",
    extension: "ex",
    source: r#"
defmodule MyApp.DataProcessor do
  def process_batch(items) do
    items
    |> Enum.filter(fn item -> item.active end)
    |> Enum.map(fn item ->
      case validate(item) do
        {:ok, valid} -> transform(valid)
        {:error, _reason} -> nil
      end
    end)
    |> Enum.reject(&is_nil/1)
    |> Enum.group_by(fn item -> item.category end)
  end

  defp validate(item) do
    cond do
      is_nil(item.name) -> {:error, :missing_name}
      String.length(item.name) < 3 -> {:error, :name_too_short}
      true -> {:ok, item}
    end
  end

  defp transform(item) do
    with {:ok, normalized} <- normalize(item),
         {:ok, enriched} <- enrich(normalized) do
      enriched
    else
      {:error, reason} -> raise "Transform failed: #{reason}"
    end
  end

  defp normalize(item) do
    {:ok, %{item | name: String.trim(item.name)}}
  end

  defp enrich(item) do
    if item.category do
      {:ok, Map.put(item, :processed, true)}
    else
      {:error, :no_category}
    end
  end
end
"#,
    total_chunks: 8,
    chunk_counts: {
        File: 1,
        Module: 1,
        FunctionCall: 1,
        Conditional: 4,
        CodeBlock: 1,
    }
}
