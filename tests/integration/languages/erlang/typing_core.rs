use crate::typing_core_test_with_parser;
use gittype::domain::models::typing::ProcessingOptions;

typing_core_test_with_parser!(
    erlang_function_with_comment,
    "erlang",
    r#"hello(Name) ->
    % Greet the user
    io:format("Hello, ~s!~n", [Name])."#
);

typing_core_test_with_parser!(
    erlang_module_with_comments,
    "erlang",
    r#"-module(greeter).
-export([hello/1]).

% Public greeting function
hello(Name) ->
    io:format("Hello, ~s!~n", [Name])."#
);

typing_core_test_with_parser!(
    erlang_case_expression_with_comments,
    "erlang",
    r#"process(Value) ->
    % Handle different input types
    case Value of
        {ok, Result} -> Result;
        {error, Reason} -> throw(Reason);
        _ -> undefined
    end."#
);

typing_core_test_with_parser!(
    erlang_pattern_matching_with_comments,
    "erlang",
    r#"factorial(0) -> 1;
% Recursive case
factorial(N) when N > 0 ->
    N * factorial(N - 1)."#
);

typing_core_test_with_parser!(
    erlang_empty_line_preservation_enabled,
    "erlang",
    r#"process(X) ->
    Y = X + 1,

    Z = Y * 2,

    Z."#,
    ProcessingOptions {
        preserve_empty_lines: true,
        ..ProcessingOptions::default()
    }
);

typing_core_test_with_parser!(
    erlang_empty_line_preservation_disabled,
    "erlang",
    r#"process(X) ->
    Y = X + 1,

    Z = Y * 2,

    Z."#,
    ProcessingOptions {
        preserve_empty_lines: false,
        ..ProcessingOptions::default()
    }
);

typing_core_test_with_parser!(
    erlang_list_comprehension_with_comments,
    "erlang",
    r#"transform(List) ->
    % Double all even numbers
    [X * 2 || X <- List, X rem 2 =:= 0]."#
);

typing_core_test_with_parser!(
    erlang_gen_server_with_comments,
    "erlang",
    r#"-module(counter).
-behaviour(gen_server).

% Client API
start_link() ->
    gen_server:start_link({local, ?MODULE}, ?MODULE, 0, []).

% Server callbacks
init(Count) ->
    {ok, Count}.

handle_call(get, _From, Count) ->
    % Return current count
    {reply, Count, Count}."#
);

typing_core_test_with_parser!(
    erlang_receive_with_comments,
    "erlang",
    r#"loop() ->
    receive
        % Handle incoming messages
        {msg, Text} ->
            io:format("Got: ~s~n", [Text]),
            loop();
        stop ->
            ok
    end."#
);

typing_core_test_with_parser!(
    erlang_record_with_comments,
    "erlang",
    r#"-record(person, {
    % Person's name
    name,
    % Person's age
    age = 0,
    email
})."#
);
