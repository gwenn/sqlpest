#![recursion_limit = "400"]

#[macro_use]
extern crate pest;

use pest::prelude::*;

impl_rdp! {
    grammar! {
        cmd_list = { (explain_cmd ~ [";"])* ~ explain_cmd? }
        explain_cmd = { (["EXPLAIN"] ~ (["QUERY"] ~ ["PLAN"])?)? ~ cmd }
        cmd = {
            select
        }

        // Select
        select = { with? ~ select_no_with ~ order_by? ~ limit? }
        select_no_with = {
            one_select ~ (compound_operator ~ one_select)*
        }
        compound_operator = { ["UNION"] | ["UNION"] ~ ["ALL"] | ["EXCEPT"] | ["INTERSECT"] }
        one_select = {
            ["SELECT"] ~ distinct? ~ (select_column ~ ([","] ~ select_column)*) ~ from? ~ where_clause? ~ group_by? |
            values
        }
        distinct = { ["DISTINCT"] | ["ALL"] }
        select_column = { expr ~ as_qualif? | ["*"] | table_name ~ ["."] ~ ["*"] }
        values = {
            ["VALUES"] ~ ["("] ~ (expr ~ ([","] ~ expr)*) ~ [")"] |
            values ~ [","] ~ ["("] ~ (expr ~ ([","] ~ expr)*) ~ [")"]
        }
        as_qualif = {
            ["AS"] ~ name |
            id_string
        }
        from = { ["FROM"] ~ select_table_list }
        select_table_list = {
            select_table |
            select_table_list ~ join_operator ~ select_table ~ join_constraint?
        }
        select_table = {
            qualified_table_name ~ as_qualif? ~ indexed? |
            qualified_table_name ~ ["("] ~ (expr ~ ([","] ~ expr)*)? ~ [")"] ~ as_qualif? |
            ["("] ~ select ~ [")"] ~ as_qualif? |
            ["("] ~ select_table_list ~ [")"] ~ as_qualif?
        }
        join_constraint = {
            ["ON"] ~ expr |
            ["USING"] ~ ["("] ~ (column_name ~ ([","] ~ column_name)*) ~ [")"]
        }
        join_operator = {
            [","] |
            ["JOIN"] |
            ["NATURAL"]? ~ join_type ~ ["JOIN"]
        }
        join_type = { ["LEFT"] ~ ["OUTER"]? | ["INNER"] | ["CROSS"] }

        indexed = {
            ["INDEXED"] ~ ["BY"] ~ index_name |
            ["NOT"] ~ ["INDEXED"]
        }

        where_clause = { ["WHERE"] ~ expr }
        group_by = { ["GROUP"] ~ ["BY"] ~ (expr ~ ([","] ~ expr)*) ~ (["HAVING"] ~ expr)? }
        order_by = { ["ORDER"] ~ ["BY"] ~ (sorted_column ~ ([","] ~ sorted_column)*) }
        limit = {
            ["LIMIT"] ~ expr |
            ["LIMIT"] ~ expr ~ ["OFFSET"] ~ expr |
            ["LIMIT"] ~ expr ~ [","] ~ expr
        }

        // Common Table Expressions
        with = { ["WITH"] ~ ["RECURSIVE"]? ~ (with_query ~ ([","] ~ with_query)*) }
        with_query = { table_name ~ (["("] ~ (indexed_column ~ ([","] ~ indexed_column)*) ~ [")"])? ~ ["AS"] ~ ["("] ~ select ~ [")"] }

        database_name = _{ name }
        table_name = _{ name }
        qualified_table_name = { (database_name ~ ["."])? ~ table_name }
        column_name = _{ name }
        index_name = _{ name }
        name = _{ id } // TODO literal

        id_string = { id | literal }
        collation_name = _{ id_string }
        type_name = _{
            id_string+ |
            id_string+ ~ ["("] ~ signed_number ~ [")"] |
            id_string+ ~ ["("] ~ signed_number ~ [","] ~ signed_number ~ [")"]
        }

        signed_number = {
            (["+"] | ["-"])? ~ number
        }

        sort_order = { ["ASC"] | ["DESC"] }
        indexed_column = { column_name ~ (["COLLATE"] ~ collation_name)? ~ sort_order? }
        sorted_column = { expr ~ sort_order? }

        // Expression
        expr = {
            literal |
            ["("] ~ expr ~ [")"] |
            id |
            name ~ ["."] ~ name |
            name ~ ["."] ~ name ~ ["."] ~ name |
            variable |
            ["CAST"] ~ ["("] ~ expr ~ ["AS"] ~ type_name ~ [")"] |
            id ~ ["("] ~ distinct? ~ (expr ~ ([","] ~ expr)*)? ~ [")"] |
            id ~ ["("] ~ ["*"] ~ [")"] |
            ["("] ~ select ~ [")"] |
            ["EXISTS"] ~ ["("] ~ select ~ [")"] |
            ["CASE"] ~ expr? ~ (["WHEN"] ~ expr ~ ["THEN"] ~ expr)+ ~ (["ELSE"] ~ expr)? ~ ["END"] |
            ["RAISE"] ~ ["("] ~ ["IGNORE"] ~ [")"] |
            ["RAISE"] ~ ["("] ~ raise_type ~ [","] ~ literal ~ [")"] // TODO name versus literal
        }

        raise_type = { ["ROLLBACK"] | ["ABORT"] | ["FAIL"] }

        // A keyword in single quotes is a string literal.
        literal = @{ ["'"] ~ (["''"] | !["'"] ~ any)* ~ ["'"] }
        blob = @{ (["x"] | ["X"]) ~ ["'"] ~ (hex_digit)+ ~ ["'"] } // TODO nb of hex digit must be even.

        id = @{
            id_start ~ (id_cont)* |
            // empty Id ("") is OK
            // A keyword in double-quotes is an identifier.
            ["\""] ~ (["\"\""] | !["\""] ~ any)* ~ ["\""] |
            // A keyword enclosed in grave accents (ASCII code 96) is an identifier. This is not standard SQL.
            ["`"] ~ (["``"] | !["`"] ~ any)* ~ ["`"] |
            // A keyword enclosed in square brackets is an identifier. This is not standard SQL.
            ["["] ~ (!["]"] ~ any)* ~ ["]"]
        }
        id_start = { ['A'..'Z'] | ["_"] | ['a'..'z'] | ['\u{7F}'..'\u{1FFFF}'] }
        id_cont = { ["$"] | ['0'..'9'] | ['A'..'Z'] | ["_"] | ['a'..'z'] | ['\u{7F}'..'\u{1FFFF}'] }

        variable = @{
            ["?"] ~ digit* |
            (["$"] | ["@"] | ["#"] | [":"]) ~ (id_cont)+
        }

        number = @{ int | float }
        int = {
            digit+ |
            ["0"] ~ (["x"] | ["X"]) ~ hex_digit+ // Must not be empty (Ox is invalid)
        }
        float = {
            digit+ ~ ["."] ~ digit* ~ exponent? |
            ["."] ~ digit+ ~ exponent? |
            digit+ ~ exponent
        }
        exponent = { (["e"] | ["E"]) ~ (["+"] | ["-"])? ~ digit+ }
        digit = _{ ['0'..'9'] }
        hex_digit = _{ ['0'..'9'] | ['a'..'f'] | ['A'..'F'] }

        comment = _{
            // line comment
            ["--"] ~ ((!(["\n"]) ~ any)* ~ (["\n"] | eoi)) |
            // block comment
            ["/*"] ~ ((!["*/"] ~ any)*)
        }
        whitespace = _{ [" "] | ["\t"] | ["\r"] | ["\n"] }
    }
}