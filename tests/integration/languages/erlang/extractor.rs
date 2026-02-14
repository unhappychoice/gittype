use crate::integration::languages::extractor::test_language_extractor;

test_language_extractor! {
    name: test_erlang_function_extraction,
    language: "erlang",
    extension: "erl",
    source: r#"
-module(greeter).
-export([hello/1]).

hello(Name) ->
    io:format("Hello, ~s!~n", [Name]).
"#,
    total_chunks: 4,
    chunk_counts: {
        File: 1,
        Module: 1,
        CodeBlock: 1,
        Function: 1,
    }
}

test_language_extractor! {
    name: test_erlang_multiple_functions_extraction,
    language: "erlang",
    extension: "erl",
    source: r#"
-module(math).
-export([add/2, subtract/2]).

add(A, B) ->
    A + B.

subtract(A, B) ->
    A - B.
"#,
    total_chunks: 5,
    chunk_counts: {
        File: 1,
        Module: 1,
        CodeBlock: 1,
        Function: 2,
    }
}

test_language_extractor! {
    name: test_erlang_record_extraction,
    language: "erlang",
    extension: "erl",
    source: r#"
-module(user).
-export([new/2]).

-record(user, {name, email, age = 0}).

new(Name, Email) ->
    #user{name = Name, email = Email}.
"#,
    total_chunks: 5,
    chunk_counts: {
        File: 1,
        Module: 1,
        CodeBlock: 1,
        Struct: 1,
        Function: 1,
    }
}

test_language_extractor! {
    name: test_erlang_type_spec_extraction,
    language: "erlang",
    extension: "erl",
    source: r#"
-module(calculator).
-export([add/2]).

-type number_result() :: {ok, number()} | {error, term()}.

-spec add(number(), number()) -> number().
add(A, B) ->
    A + B.
"#,
    total_chunks: 6,
    chunk_counts: {
        File: 1,
        Module: 1,
        CodeBlock: 2,
        TypeAlias: 1,
        Function: 1,
    }
}

test_language_extractor! {
    name: test_erlang_behaviour_extraction,
    language: "erlang",
    extension: "erl",
    source: r#"
-module(my_server).
-behaviour(gen_server).
-export([init/1, handle_call/3]).

init(State) ->
    {ok, State}.

handle_call(get, _From, State) ->
    {reply, State, State}.
"#,
    total_chunks: 6,
    chunk_counts: {
        File: 1,
        Module: 1,
        Interface: 1,
        CodeBlock: 1,
        Function: 2,
    }
}

test_language_extractor! {
    name: test_erlang_case_expression,
    language: "erlang",
    extension: "erl",
    source: r#"
-module(handler).
-export([process/1]).

process(Value) ->
    case Value of
        {ok, Result} -> Result;
        {error, Reason} -> throw(Reason);
        _ -> undefined
    end.
"#,
    total_chunks: 5,
    chunk_counts: {
        File: 1,
        Module: 1,
        CodeBlock: 1,
        Function: 1,
        Conditional: 1,
    }
}

test_language_extractor! {
    name: test_erlang_gen_server_full,
    language: "erlang",
    extension: "erl",
    source: r#"
-module(counter).
-behaviour(gen_server).
-export([start_link/0, increment/0, get_count/0]).
-export([init/1, handle_call/3, handle_cast/2]).

start_link() ->
    gen_server:start_link({local, ?MODULE}, ?MODULE, 0, []).

increment() ->
    gen_server:cast(?MODULE, increment).

get_count() ->
    gen_server:call(?MODULE, get_count).

init(InitialCount) ->
    {ok, InitialCount}.

handle_call(get_count, _From, Count) ->
    {reply, Count, Count}.

handle_cast(increment, Count) ->
    {noreply, Count + 1}.
"#,
    total_chunks: 11,
    chunk_counts: {
        File: 1,
        Module: 1,
        Interface: 1,
        CodeBlock: 2,
        Function: 6,
    }
}

test_language_extractor! {
    name: test_erlang_fun_expression,
    language: "erlang",
    extension: "erl",
    source: r#"
-module(processor).
-export([transform/1]).

transform(List) ->
    lists:map(fun(X) -> X * 2 end, List).
"#,
    total_chunks: 4,
    chunk_counts: {
        File: 1,
        Module: 1,
        CodeBlock: 1,
        Function: 1,
    }
}

test_language_extractor! {
    name: test_erlang_receive_expression,
    language: "erlang",
    extension: "erl",
    source: r#"
-module(listener).
-export([loop/0]).

loop() ->
    receive
        {msg, Text} ->
            io:format("Got: ~s~n", [Text]),
            loop();
        stop ->
            ok
    after 5000 ->
        io:format("Timeout~n"),
        loop()
    end.
"#,
    total_chunks: 5,
    chunk_counts: {
        File: 1,
        Module: 1,
        CodeBlock: 1,
        Function: 1,
        Conditional: 1,
    }
}
