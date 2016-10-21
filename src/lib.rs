#![recursion_limit = "500"]

#[macro_use]
extern crate pest;

use pest::prelude::*;

#[cfg(test)]
mod test;

impl_rdp! {
    grammar! {
        cmd_list = { (explain_cmd ~ [";"])* ~ explain_cmd? }
        explain_cmd = { ([i"explain"] ~ ([i"query"] ~ [i"plan"])?)? ~ cmd }
        cmd = {
            alter_table |
            create_table |
            select
        }

        // Alter Table
        alter_table = { [i"alter"] ~ [i"table"] ~ qualified_table_name ~ alter_table_body }
        alter_table_body = {
            [i"rename"] ~ [i"to"] ~ table_name |
            [i"add"] ~ [i"column"]? ~ column_def
        }

        // Create Table
        create_table = {
            [i"create"] ~ [i"temp"]? ~ [i"table"] ~ if_not_exists? ~ qualified_table_name ~ create_table_body
        }
        if_not_exists = { [i"if"] ~ [i"not"] ~ [i"exists"] }
        create_table_body = {
            ["("] ~ column_def ~ ([","] ~ !(named_table_constraint) ~ column_def)* ~ ([","] ~ named_table_constraint)* ~ [")"] ~ ([i"without"] ~ name)? |
            [i"as"] ~ select
        }
        column_def = {
            column_name ~ type_name? ~ (([i"constraint"] ~ constraint_name)? ~ column_constraint)*
        }
        column_constraint = {
            [i"primary"] ~ [i"key"] ~ sort_order? ~ conflict_clause? ~ [i"autoincrement"]? |
            [i"not"]? ~ [i"null"] ~ conflict_clause? |
            [i"unique"] ~ conflict_clause? |
            [i"check"] ~ ["("] ~ expr ~ [")"] |
            [i"default"] ~ default_value |
            [i"collate"] ~ collation_name |
            [i"references"] ~ table_name ~ (["("] ~ indexed_column ~ ([","] ~ indexed_column)* ~ [")"])? ~ ref_arg*
        }
        conflict_clause = {
            [i"on"] ~ [i"conflict"] ~ resolve_type
        }
        resolve_type = {
            raise_type |
            [i"ignore"] |
            [i"replace"]
        }
        default_value = {
            literal |
            ["("] ~ expr ~ [")"] |
            ["+"] ~ number |
            ["-"] ~ number |
            id
        }
        ref_arg = {
            [i"on"] ~ [i"delete"] ~ ref_act |
            [i"on"] ~ [i"update"] ~ ref_act |
            [i"match"] ~ name
        }
        ref_act = {
            [i"set"] ~ [i"null"] |
            [i"set"] ~ [i"default"] |
            [i"cascade"] |
            [i"restrict"] |
            [i"no"] ~ [i"action"]
        }
        named_table_constraint = { ([i"constraint"] ~ constraint_name)? ~ table_constraint }
        table_constraint = {
            [i"primary"] ~ [i"key"] ~ ["("] ~ sorted_column ~ ([","] ~ sorted_column)* ~ [i"autoincrement"]? ~ [")"] ~ conflict_clause? |
            [i"unique"] ~ ["("] ~ sorted_column ~ ([","] ~ sorted_column)* ~ [")"] ~ conflict_clause? |
            [i"check"] ~ ["("] ~ expr ~ [")"] |
            [i"foreign"] ~ [i"key"] ~ ["("] ~ indexed_column ~ ([","] ~ indexed_column)* ~ [")"] ~ [i"references"] ~ table_name ~ (["("] ~ indexed_column ~ ([","] ~ indexed_column)* ~ [")"])? ~ ref_arg* ~ defer_sub_clause?

        }
        defer_sub_clause = {
            [i"not"]? ~ [i"deferrable"] ~ ([i"initially"] ~ ([i"deferred"] | [i"immediate"]))?
        }

        // Select
        select = { with? ~ select_no_with ~ order_by? ~ limit? }
        select_no_with = {
            one_select ~ (compound_operator ~ one_select)*
        }
        compound_operator = { [i"union"] | [i"union"] ~ [i"all"] | [i"except"] | [i"intersect"] }
        one_select = {
            [i"select"] ~ distinct? ~ select_column ~ ([","] ~ select_column)* ~ from? ~ where_clause? ~ group_by? |
            values
        }
        distinct = { [i"distinct"] | [i"all"] }
        select_column = { expr ~ as_qualif? | ["*"] | table_name ~ ["."] ~ ["*"] }
        values = {
            [i"values"] ~ ["("] ~ (expr ~ ([","] ~ expr)*) ~ [")"] ~ values_tail?
        }
        values_tail = {
            [","] ~ ["("] ~ (expr ~ ([","] ~ expr)*) ~ [")"] ~ values_tail?
        }
        as_qualif = {
            [i"as"] ~ name |
            !([i"limit"] | [i"order"] | [i"where"]) ~ id_string
        }
        from = { [i"from"] ~ select_table_list }
        select_table_list = {
            select_table ~ select_table_list_tail?
        }
        select_table_list_tail = {
            join_operator ~ select_table ~ join_constraint? ~ select_table_list_tail?
        }
        select_table = {
            qualified_table_name ~ as_qualif? ~ indexed? |
            qualified_table_name ~ ["("] ~ (expr ~ ([","] ~ expr)*)? ~ [")"] ~ as_qualif? |
            ["("] ~ select ~ [")"] ~ as_qualif? |
            ["("] ~ select_table_list ~ [")"] ~ as_qualif?
        }
        join_constraint = {
            [i"on"] ~ expr |
            [i"using"] ~ ["("] ~ (column_name ~ ([","] ~ column_name)*) ~ [")"]
        }
        join_operator = {
            [","] |
            [i"join"] |
            [i"natural"]? ~ join_type ~ [i"join"]
        }
        join_type = { [i"left"] ~ [i"outer"]? | [i"inner"] | [i"cross"] }

        indexed = {
            [i"indexed"] ~ [i"by"] ~ index_name |
            [i"not"] ~ [i"indexed"]
        }

        where_clause = { [i"where"] ~ expr }
        group_by = { [i"group"] ~ [i"by"] ~ (expr ~ ([","] ~ expr)*) ~ ([i"having"] ~ expr)? }
        order_by = { [i"order"] ~ [i"by"] ~ (sorted_column ~ ([","] ~ sorted_column)*) }
        limit = {
            [i"limit"] ~ expr ~ (([i"offset"] | [","]) ~ expr)?
        }

        // Common Table Expressions
        with = { [i"with"] ~ [i"recursive"]? ~ (with_query ~ ([","] ~ with_query)*) }
        with_query = { table_name ~ (["("] ~ (indexed_column ~ ([","] ~ indexed_column)*) ~ [")"])? ~ [i"as"] ~ ["("] ~ select ~ [")"] }

        database_name = _{ name }
        table_name = _{ name }
        qualified_table_name = @{ (database_name ~ ["."])? ~ table_name }
        column_name = _{ name }
        index_name = _{ name }
        constraint_name = _{ name }
        name = _{ id } // TODO string_literal

        id_string = { id | string_literal }
        collation_name = _{ id_string }
        type_name = _{
            id_string+ ~ (["("] ~ signed_number ~ ([","] ~ signed_number)? ~ [")"])?
        }

        signed_number = {
            (["+"] | ["-"])? ~ number
        }

        sort_order = { [i"asc"] | [i"desc"] }
        indexed_column = { column_name ~ ([i"collate"] ~ collation_name)? ~ sort_order? }
        sorted_column = { expr ~ sort_order? }

        // Expression
        expr = {
            literal |
            ["("] ~ expr ~ [")"] |
            id |
            name ~ ["."] ~ name ~ (["."] ~ name)? |
            variable |
            [i"cast"] ~ ["("] ~ expr ~ [i"as"] ~ type_name ~ [")"] |
            id ~ ["("] ~ distinct? ~ (expr ~ ([","] ~ expr)*)? ~ [")"] |
            id ~ ["("] ~ ["*"] ~ [")"] |
            ["("] ~ select ~ [")"] |
            [i"exists"] ~ ["("] ~ select ~ [")"] |
            [i"case"] ~ expr? ~ ([i"when"] ~ expr ~ [i"then"] ~ expr)+ ~ ([i"else"] ~ expr)? ~ [i"end"] |
            [i"raise"] ~ ["("] ~ [i"ignore"] ~ [")"] |
            [i"raise"] ~ ["("] ~ raise_type ~ [","] ~ string_literal ~ [")"] // TODO name versus string_literal
        }

        raise_type = { [i"rollback"] | [i"abort"] | [i"fail"] }

        literal = {
            number |
            string_literal |
            blob |
            [i"null"] |
            [i"current_date"] |
            [i"current_time"] |
            [i"current_timestamp"]
        }
        // A keyword in single quotes is a string literal.
        string_literal = @{ ["'"] ~ (["''"] | !["'"] ~ any)* ~ ["'"] }
        blob = @{ (["x"] | ["X"]) ~ ["'"] ~ (hex_digit)+ ~ ["'"] } // TODO nb of hex digit must be even.

        id = @{
            (['A'..'Z'] | ["_"] | ['a'..'z']) ~ (["$"] | ['0'..'9'] | ['A'..'Z'] | ["_"] | ['a'..'z'])* |
            // empty Id ("") is OK
            // A keyword in double-quotes is an identifier.
            ["\""] ~ (["\"\""] | !["\""] ~ any)* ~ ["\""] |
            // A keyword enclosed in grave accents (ASCII code 96) is an identifier. This is not standard SQL.
            ["`"] ~ (["``"] | !["`"] ~ any)* ~ ["`"] |
            // A keyword enclosed in square brackets is an identifier. This is not standard SQL.
            ["["] ~ (!["]"] ~ any)* ~ ["]"]
        }
        // FIXME ranges should have same-sized UTF-8 limits
        //id_start = { ['A'..'Z'] | ["_"] | ['a'..'z'] | ['\u{7F}'..'\u{1FFFF}'] }
        //id_cont = { ["$"] | ['0'..'9'] | ['A'..'Z'] | ["_"] | ['a'..'z'] | ['\u{7F}'..'\u{1FFFF}'] }

        keyword = @{
            [i"add"] | [i"all"] | [i"alter"] | [i"and"] | [i"as"] | [i"autoincrement"] |
            [i"between"] |
            [i"check"] | [i"collate"] | [i"commit"] | [i"constraint"] | [i"create"] | [i"cross"] |
            [i"default"] | [i"deferrable"] | [i"delete"] | [i"distinct"] | [i"drop"] |
            [i"escape"] | [i"exists"] |
            [i"foreign"] | [i"from"] | [i"full"] |
            [i"group"] |
            [i"having"] |
            [i"in"] | [i"index"] | [i"inner"] | [i"insert"] | [i"intersect"] | [i"into"] | [i"is"] | [i"isnull"] |
            [i"join"] |
            [i"left"] | [i"limit"] |
            [i"natural"] | [i"not"] | [i"notnull"] | [i"null"] |
            [i"on"] | [i"or"] | [i"order"] | [i"outer"] |
            [i"primary"] |
            [i"references"] | [i"right"] |
            [i"select"] | [i"set"] |
            [i"table"] | [i"temporary"] | [i"then"] | [i"to"] | [i"transaction"] |
            [i"unique"] | [i"update"] | [i"using"] |
            [i"values"] |
            [i"when"] | [i"where"]
        }

        variable = @{
            ["?"] ~ digit* |
            (["$"] | ["@"] | ["#"] | [":"]) ~ (["$"] | ['0'..'9'] | ['A'..'Z'] | ["_"] | ['a'..'z'])+
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
