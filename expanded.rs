#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;
extern crate clap;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
use clap::{Arg, App};
use std::io::prelude::*;
use pest::Parser;
use std::collections::HashSet;

#[grammar = "enums.pest"]
struct EnumsParser;
#[allow(non_upper_case_globals)]
const _PEST_GRAMMAR_EnumsParser: &'static str =
    "//string = @{ \"\\\"\" ~ inner ~ \"\\\"\" }\r\n//inner = @{ (!(\"\\\"\" | \"\\\\\" | \"\\u{0000}\" | \"\\u{001F}\") ~ ANY)* ~ (escape ~ inner)? }\r\n\r\n//string = {\"\\\"\" ~ (\"\\\\\\\"\" | ^\"\\\"\")* ~ \"\\\"\"}\r\n//string2 = {\"<\" ~ (^\">\")* ~ \">\"}\r\n//include = { \"#include\" ~ (string | string2) }\r\n\r\nshort_comment = @{ \"//\" ~ (!NEWLINE ~ ANY)* ~ NEWLINE }\r\nlong_comment = @{ \"/*\" ~ (!\"*/\" ~ ANY)* ~ \"*/\" }\r\nCOMMENT = { short_comment | long_comment }\r\n\r\nalpha = { \'a\'..\'z\' | \'A\'..\'Z\' }\r\nalpha_ = { alpha | \"_\" }\r\ndigit = { \'0\'..\'9\' }\r\n\r\nidentifier = @{ alpha_ ~ (alpha_ | digit)* }\r\ntype_name = @{ identifier }\r\nnamespace_name = @{ identifier }\r\n\r\ntype_reference = { (\"const\")? ~ (namespace_name ~ \"::\")* ~ type_name ~ (\"<\" ~ type_reference ~ (\",\" ~ type_reference)* ~ \">\")? ~ ((\"const\")? ~ \"*\")* ~ (\"const\"? ~ \"&\")? }\r\n\r\nsimple_sumtype_item = @{ identifier }\r\ntyped_sumtype_item = { identifier ~ \"(\" ~ type_reference ~ \")\" }\r\n\r\nsemicolon = @{ \";\" }\r\nsumtype_item = { (typed_sumtype_item | simple_sumtype_item) ~ semicolon }\r\n\r\ncurly_open = @{ \"{\" }\r\ncurly_close = @{ \"}\" }\r\n\r\nsumtype_definition = { \"sumtype\" ~ type_name ~ curly_open ~ (sumtype_item)* ~ curly_close ~ (semicolon)? }\r\n\r\nfill = @{ (!\"sumtype\" ~ ANY)* }\r\ntail = @{ (ANY)* }\r\n\r\nfile = _{ SOI ~ (fill ~ sumtype_definition)* ~ tail ~ EOI }\r\n\r\nWHITESPACE = { \" \" | \"\\t\" | NEWLINE }\r\n";
#[allow(dead_code, non_camel_case_types)]
#[structural_match]
#[rustc_copy_clone_marker]
pub enum Rule {
    EOI,
    short_comment,
    long_comment,
    COMMENT,
    alpha,
    alpha_,
    digit,
    identifier,
    type_name,
    namespace_name,
    type_reference,
    simple_sumtype_item,
    typed_sumtype_item,
    semicolon,
    sumtype_item,
    curly_open,
    curly_close,
    sumtype_definition,
    fill,
    tail,
    file,
    WHITESPACE,
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl $crate::clone::Clone for Rule {
    #[inline]
    fn clone(&self) -> Rule { { *self } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl $crate::marker::Copy for Rule { }
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl $crate::fmt::Debug for Rule {
    fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
        match (&*self,) {
            (&Rule::EOI,) => {
                let mut debug_trait_builder = f.debug_tuple("EOI");
                debug_trait_builder.finish()
            }
            (&Rule::short_comment,) => {
                let mut debug_trait_builder = f.debug_tuple("short_comment");
                debug_trait_builder.finish()
            }
            (&Rule::long_comment,) => {
                let mut debug_trait_builder = f.debug_tuple("long_comment");
                debug_trait_builder.finish()
            }
            (&Rule::COMMENT,) => {
                let mut debug_trait_builder = f.debug_tuple("COMMENT");
                debug_trait_builder.finish()
            }
            (&Rule::alpha,) => {
                let mut debug_trait_builder = f.debug_tuple("alpha");
                debug_trait_builder.finish()
            }
            (&Rule::alpha_,) => {
                let mut debug_trait_builder = f.debug_tuple("alpha_");
                debug_trait_builder.finish()
            }
            (&Rule::digit,) => {
                let mut debug_trait_builder = f.debug_tuple("digit");
                debug_trait_builder.finish()
            }
            (&Rule::identifier,) => {
                let mut debug_trait_builder = f.debug_tuple("identifier");
                debug_trait_builder.finish()
            }
            (&Rule::type_name,) => {
                let mut debug_trait_builder = f.debug_tuple("type_name");
                debug_trait_builder.finish()
            }
            (&Rule::namespace_name,) => {
                let mut debug_trait_builder = f.debug_tuple("namespace_name");
                debug_trait_builder.finish()
            }
            (&Rule::type_reference,) => {
                let mut debug_trait_builder = f.debug_tuple("type_reference");
                debug_trait_builder.finish()
            }
            (&Rule::simple_sumtype_item,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("simple_sumtype_item");
                debug_trait_builder.finish()
            }
            (&Rule::typed_sumtype_item,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("typed_sumtype_item");
                debug_trait_builder.finish()
            }
            (&Rule::semicolon,) => {
                let mut debug_trait_builder = f.debug_tuple("semicolon");
                debug_trait_builder.finish()
            }
            (&Rule::sumtype_item,) => {
                let mut debug_trait_builder = f.debug_tuple("sumtype_item");
                debug_trait_builder.finish()
            }
            (&Rule::curly_open,) => {
                let mut debug_trait_builder = f.debug_tuple("curly_open");
                debug_trait_builder.finish()
            }
            (&Rule::curly_close,) => {
                let mut debug_trait_builder = f.debug_tuple("curly_close");
                debug_trait_builder.finish()
            }
            (&Rule::sumtype_definition,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("sumtype_definition");
                debug_trait_builder.finish()
            }
            (&Rule::fill,) => {
                let mut debug_trait_builder = f.debug_tuple("fill");
                debug_trait_builder.finish()
            }
            (&Rule::tail,) => {
                let mut debug_trait_builder = f.debug_tuple("tail");
                debug_trait_builder.finish()
            }
            (&Rule::file,) => {
                let mut debug_trait_builder = f.debug_tuple("file");
                debug_trait_builder.finish()
            }
            (&Rule::WHITESPACE,) => {
                let mut debug_trait_builder = f.debug_tuple("WHITESPACE");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl $crate::cmp::Eq for Rule {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl $crate::hash::Hash for Rule {
    fn hash<__H: $crate::hash::Hasher>(&self, state: &mut __H) -> () {
        match (&*self,) {
            _ => {
                $crate::hash::Hash::hash(&unsafe {
                                              $crate::intrinsics::discriminant_value(self)
                                          }, state)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl $crate::cmp::Ord for Rule {
    #[inline]
    fn cmp(&self, other: &Rule) -> $crate::cmp::Ordering {
        {
            let __self_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) { _ => $crate::cmp::Ordering::Equal, }
            } else { __self_vi.cmp(&__arg_1_vi) }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl $crate::cmp::PartialEq for Rule {
    #[inline]
    fn eq(&self, other: &Rule) -> bool {
        {
            let __self_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) { _ => true, }
            } else { false }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl $crate::cmp::PartialOrd for Rule {
    #[inline]
    fn partial_cmp(&self, other: &Rule)
     -> $crate::option::Option<$crate::cmp::Ordering> {
        {
            let __self_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { $crate::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    _ =>
                    $crate::option::Option::Some($crate::cmp::Ordering::Equal),
                }
            } else { __self_vi.partial_cmp(&__arg_1_vi) }
        }
    }
}
#[allow(clippy::all)]
impl ::pest::Parser<Rule> for EnumsParser {
    fn parse<'i>(rule: Rule, input: &'i str)
     ->
         ::std::result::Result<::pest::iterators::Pairs<'i, Rule>,
                               ::pest::error::Error<Rule>> {
        mod rules {
            pub mod hidden {
                use super::super::Rule;
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn skip(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    if state.atomicity() == ::pest::Atomicity::NonAtomic {
                        state.sequence(|state|
                                           {
                                               state.repeat(|state|
                                                                super::visible::WHITESPACE(state)).and_then(|state|
                                                                                                                {
                                                                                                                    state.repeat(|state|
                                                                                                                                     {
                                                                                                                                         state.sequence(|state|
                                                                                                                                                            {
                                                                                                                                                                super::visible::COMMENT(state).and_then(|state|
                                                                                                                                                                                                            {
                                                                                                                                                                                                                state.repeat(|state|
                                                                                                                                                                                                                                 super::visible::WHITESPACE(state))
                                                                                                                                                                                                            })
                                                                                                                                                            })
                                                                                                                                     })
                                                                                                                })
                                           })
                    } else { Ok(state) }
                }
            }
            pub mod visible {
                use super::super::Rule;
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn short_comment(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::short_comment,
                               |state|
                                   {
                                       state.atomic(::pest::Atomicity::Atomic,
                                                    |state|
                                                        {
                                                            state.sequence(|state|
                                                                               {
                                                                                   state.match_string("//").and_then(|state|
                                                                                                                         {
                                                                                                                             state.repeat(|state|
                                                                                                                                              {
                                                                                                                                                  state.sequence(|state|
                                                                                                                                                                     {
                                                                                                                                                                         state.lookahead(false,
                                                                                                                                                                                         |state|
                                                                                                                                                                                             {
                                                                                                                                                                                                 self::NEWLINE(state)
                                                                                                                                                                                             }).and_then(|state|
                                                                                                                                                                                                             {
                                                                                                                                                                                                                 self::ANY(state)
                                                                                                                                                                                                             })
                                                                                                                                                                     })
                                                                                                                                              })
                                                                                                                         }).and_then(|state|
                                                                                                                                         {
                                                                                                                                             self::NEWLINE(state)
                                                                                                                                         })
                                                                               })
                                                        })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn long_comment(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::long_comment,
                               |state|
                                   {
                                       state.atomic(::pest::Atomicity::Atomic,
                                                    |state|
                                                        {
                                                            state.sequence(|state|
                                                                               {
                                                                                   state.match_string("/*").and_then(|state|
                                                                                                                         {
                                                                                                                             let strings =
                                                                                                                                 ["*/"];
                                                                                                                             state.skip_until(&strings)
                                                                                                                         }).and_then(|state|
                                                                                                                                         {
                                                                                                                                             state.match_string("*/")
                                                                                                                                         })
                                                                               })
                                                        })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn COMMENT(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::COMMENT,
                               |state|
                                   {
                                       state.atomic(::pest::Atomicity::Atomic,
                                                    |state|
                                                        {
                                                            self::short_comment(state).or_else(|state|
                                                                                                   {
                                                                                                       self::long_comment(state)
                                                                                                   })
                                                        })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn alpha(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::alpha,
                               |state|
                                   {
                                       state.match_range('a'..'z').or_else(|state|
                                                                               {
                                                                                   state.match_range('A'..'Z')
                                                                               })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn alpha_(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::alpha_,
                               |state|
                                   {
                                       self::alpha(state).or_else(|state|
                                                                      {
                                                                          state.match_string("_")
                                                                      })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn digit(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::digit,
                               |state| { state.match_range('0'..'9') })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn identifier(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::identifier,
                               |state|
                                   {
                                       state.atomic(::pest::Atomicity::Atomic,
                                                    |state|
                                                        {
                                                            state.sequence(|state|
                                                                               {
                                                                                   self::alpha_(state).and_then(|state|
                                                                                                                    {
                                                                                                                        state.repeat(|state|
                                                                                                                                         {
                                                                                                                                             self::alpha_(state).or_else(|state|
                                                                                                                                                                             {
                                                                                                                                                                                 self::digit(state)
                                                                                                                                                                             })
                                                                                                                                         })
                                                                                                                    })
                                                                               })
                                                        })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn type_name(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::type_name,
                               |state|
                                   {
                                       state.atomic(::pest::Atomicity::Atomic,
                                                    |state|
                                                        {
                                                            self::identifier(state)
                                                        })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn namespace_name(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::namespace_name,
                               |state|
                                   {
                                       state.atomic(::pest::Atomicity::Atomic,
                                                    |state|
                                                        {
                                                            self::identifier(state)
                                                        })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn type_reference(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::type_reference,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              state.optional(|state|
                                                                                 {
                                                                                     state.match_string("const")
                                                                                 }).and_then(|state|
                                                                                                 {
                                                                                                     super::hidden::skip(state)
                                                                                                 }).and_then(|state|
                                                                                                                 {
                                                                                                                     state.sequence(|state|
                                                                                                                                        {
                                                                                                                                            state.optional(|state|
                                                                                                                                                               {
                                                                                                                                                                   state.sequence(|state|
                                                                                                                                                                                      {
                                                                                                                                                                                          self::namespace_name(state).and_then(|state|
                                                                                                                                                                                                                                   {
                                                                                                                                                                                                                                       super::hidden::skip(state)
                                                                                                                                                                                                                                   }).and_then(|state|
                                                                                                                                                                                                                                                   {
                                                                                                                                                                                                                                                       state.match_string("::")
                                                                                                                                                                                                                                                   })
                                                                                                                                                                                      }).and_then(|state|
                                                                                                                                                                                                      {
                                                                                                                                                                                                          state.repeat(|state|
                                                                                                                                                                                                                           {
                                                                                                                                                                                                                               state.sequence(|state|
                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                      super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                                                                              {
                                                                                                                                                                                                                                                                                                  state.sequence(|state|
                                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                                         self::namespace_name(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                                                      super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                                                                  }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                                                                      state.match_string("::")
                                                                                                                                                                                                                                                                                                                                                                                  })
                                                                                                                                                                                                                                                                                                                     })
                                                                                                                                                                                                                                                                                              })
                                                                                                                                                                                                                                                  })
                                                                                                                                                                                                                           })
                                                                                                                                                                                                      })
                                                                                                                                                               })
                                                                                                                                        })
                                                                                                                 }).and_then(|state|
                                                                                                                                 {
                                                                                                                                     super::hidden::skip(state)
                                                                                                                                 }).and_then(|state|
                                                                                                                                                 {
                                                                                                                                                     self::type_name(state)
                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                 {
                                                                                                                                                                     super::hidden::skip(state)
                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                 {
                                                                                                                                                                                     state.optional(|state|
                                                                                                                                                                                                        {
                                                                                                                                                                                                            state.sequence(|state|
                                                                                                                                                                                                                               {
                                                                                                                                                                                                                                   state.match_string("<").and_then(|state|
                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                            super::hidden::skip(state)
                                                                                                                                                                                                                                                                        }).and_then(|state|
                                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                                            self::type_reference(state)
                                                                                                                                                                                                                                                                                        }).and_then(|state|
                                                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                                                            super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                        }).and_then(|state|
                                                                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                                                                            state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                               {
                                                                                                                                                                                                                                                                                                                                                   state.optional(|state|
                                                                                                                                                                                                                                                                                                                                                                      {
                                                                                                                                                                                                                                                                                                                                                                          state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                                                                             {
                                                                                                                                                                                                                                                                                                                                                                                                 state.match_string(",").and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                      {
                                                                                                                                                                                                                                                                                                                                                                                                                                          super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                                                                                                                                      }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                      {
                                                                                                                                                                                                                                                                                                                                                                                                                                                          self::type_reference(state)
                                                                                                                                                                                                                                                                                                                                                                                                                                                      })
                                                                                                                                                                                                                                                                                                                                                                                             }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                             {
                                                                                                                                                                                                                                                                                                                                                                                                                 state.repeat(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                                                                                                                      state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                         {
                                                                                                                                                                                                                                                                                                                                                                                                                                                             super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                state.match_string(",").and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         self::type_reference(state)
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     })
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            })
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     })
                                                                                                                                                                                                                                                                                                                                                                                                                                                         })
                                                                                                                                                                                                                                                                                                                                                                                                                                  })
                                                                                                                                                                                                                                                                                                                                                                                                             })
                                                                                                                                                                                                                                                                                                                                                                      })
                                                                                                                                                                                                                                                                                                                                               })
                                                                                                                                                                                                                                                                                                                        }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                                                                                            super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                                        }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                                                                                                            state.match_string(">")
                                                                                                                                                                                                                                                                                                                                                        })
                                                                                                                                                                                                                               })
                                                                                                                                                                                                        })
                                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                                 {
                                                                                                                                                                                                     super::hidden::skip(state)
                                                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                                                 {
                                                                                                                                                                                                                     state.sequence(|state|
                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                            state.optional(|state|
                                                                                                                                                                                                                                                               {
                                                                                                                                                                                                                                                                   state.sequence(|state|
                                                                                                                                                                                                                                                                                      {
                                                                                                                                                                                                                                                                                          state.optional(|state|
                                                                                                                                                                                                                                                                                                             {
                                                                                                                                                                                                                                                                                                                 state.match_string("const")
                                                                                                                                                                                                                                                                                                             }).and_then(|state|
                                                                                                                                                                                                                                                                                                                             {
                                                                                                                                                                                                                                                                                                                                 super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                             }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                             {
                                                                                                                                                                                                                                                                                                                                                 state.match_string("*")
                                                                                                                                                                                                                                                                                                                                             })
                                                                                                                                                                                                                                                                                      }).and_then(|state|
                                                                                                                                                                                                                                                                                                      {
                                                                                                                                                                                                                                                                                                          state.repeat(|state|
                                                                                                                                                                                                                                                                                                                           {
                                                                                                                                                                                                                                                                                                                               state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                                      super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                              {
                                                                                                                                                                                                                                                                                                                                                                                                  state.sequence(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                                                                                                                                         state.optional(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                                                                                                                                                                                state.match_string("const")
                                                                                                                                                                                                                                                                                                                                                                                                                                            }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                                                                                                                                                            }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                state.match_string("*")
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            })
                                                                                                                                                                                                                                                                                                                                                                                                                     })
                                                                                                                                                                                                                                                                                                                                                                                              })
                                                                                                                                                                                                                                                                                                                                                  })
                                                                                                                                                                                                                                                                                                                           })
                                                                                                                                                                                                                                                                                                      })
                                                                                                                                                                                                                                                               })
                                                                                                                                                                                                                                        })
                                                                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                                                                 {
                                                                                                                                                                                                                                     super::hidden::skip(state)
                                                                                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                                                                                 {
                                                                                                                                                                                                                                                     state.optional(|state|
                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                            state.sequence(|state|
                                                                                                                                                                                                                                                                                               {
                                                                                                                                                                                                                                                                                                   state.optional(|state|
                                                                                                                                                                                                                                                                                                                      {
                                                                                                                                                                                                                                                                                                                          state.match_string("const")
                                                                                                                                                                                                                                                                                                                      }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                      {
                                                                                                                                                                                                                                                                                                                                          super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                                      }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                      {
                                                                                                                                                                                                                                                                                                                                                          state.match_string("&")
                                                                                                                                                                                                                                                                                                                                                      })
                                                                                                                                                                                                                                                                                               })
                                                                                                                                                                                                                                                                        })
                                                                                                                                                                                                                                                 })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn simple_sumtype_item(state:
                                               Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::simple_sumtype_item,
                               |state|
                                   {
                                       state.atomic(::pest::Atomicity::Atomic,
                                                    |state|
                                                        {
                                                            self::identifier(state)
                                                        })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn typed_sumtype_item(state:
                                              Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::typed_sumtype_item,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::identifier(state).and_then(|state|
                                                                                                   {
                                                                                                       super::hidden::skip(state)
                                                                                                   }).and_then(|state|
                                                                                                                   {
                                                                                                                       state.match_string("(")
                                                                                                                   }).and_then(|state|
                                                                                                                                   {
                                                                                                                                       super::hidden::skip(state)
                                                                                                                                   }).and_then(|state|
                                                                                                                                                   {
                                                                                                                                                       self::type_reference(state)
                                                                                                                                                   }).and_then(|state|
                                                                                                                                                                   {
                                                                                                                                                                       super::hidden::skip(state)
                                                                                                                                                                   }).and_then(|state|
                                                                                                                                                                                   {
                                                                                                                                                                                       state.match_string(")")
                                                                                                                                                                                   })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn semicolon(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::semicolon,
                               |state|
                                   {
                                       state.atomic(::pest::Atomicity::Atomic,
                                                    |state|
                                                        {
                                                            state.match_string(";")
                                                        })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn sumtype_item(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::sumtype_item,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              self::typed_sumtype_item(state).or_else(|state|
                                                                                                          {
                                                                                                              self::simple_sumtype_item(state)
                                                                                                          }).and_then(|state|
                                                                                                                          {
                                                                                                                              super::hidden::skip(state)
                                                                                                                          }).and_then(|state|
                                                                                                                                          {
                                                                                                                                              self::semicolon(state)
                                                                                                                                          })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn curly_open(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::curly_open,
                               |state|
                                   {
                                       state.atomic(::pest::Atomicity::Atomic,
                                                    |state|
                                                        {
                                                            state.match_string("{")
                                                        })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn curly_close(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::curly_close,
                               |state|
                                   {
                                       state.atomic(::pest::Atomicity::Atomic,
                                                    |state|
                                                        {
                                                            state.match_string("}")
                                                        })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn sumtype_definition(state:
                                              Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::sumtype_definition,
                               |state|
                                   {
                                       state.sequence(|state|
                                                          {
                                                              state.match_string("sumtype").and_then(|state|
                                                                                                         {
                                                                                                             super::hidden::skip(state)
                                                                                                         }).and_then(|state|
                                                                                                                         {
                                                                                                                             self::type_name(state)
                                                                                                                         }).and_then(|state|
                                                                                                                                         {
                                                                                                                                             super::hidden::skip(state)
                                                                                                                                         }).and_then(|state|
                                                                                                                                                         {
                                                                                                                                                             self::curly_open(state)
                                                                                                                                                         }).and_then(|state|
                                                                                                                                                                         {
                                                                                                                                                                             super::hidden::skip(state)
                                                                                                                                                                         }).and_then(|state|
                                                                                                                                                                                         {
                                                                                                                                                                                             state.sequence(|state|
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    state.optional(|state|
                                                                                                                                                                                                                                       {
                                                                                                                                                                                                                                           self::sumtype_item(state).and_then(|state|
                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                      state.repeat(|state|
                                                                                                                                                                                                                                                                                                       {
                                                                                                                                                                                                                                                                                                           state.sequence(|state|
                                                                                                                                                                                                                                                                                                                              {
                                                                                                                                                                                                                                                                                                                                  super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                                                                                              self::sumtype_item(state)
                                                                                                                                                                                                                                                                                                                                                                          })
                                                                                                                                                                                                                                                                                                                              })
                                                                                                                                                                                                                                                                                                       })
                                                                                                                                                                                                                                                                                  })
                                                                                                                                                                                                                                       })
                                                                                                                                                                                                                })
                                                                                                                                                                                         }).and_then(|state|
                                                                                                                                                                                                         {
                                                                                                                                                                                                             super::hidden::skip(state)
                                                                                                                                                                                                         }).and_then(|state|
                                                                                                                                                                                                                         {
                                                                                                                                                                                                                             self::curly_close(state)
                                                                                                                                                                                                                         }).and_then(|state|
                                                                                                                                                                                                                                         {
                                                                                                                                                                                                                                             super::hidden::skip(state)
                                                                                                                                                                                                                                         }).and_then(|state|
                                                                                                                                                                                                                                                         {
                                                                                                                                                                                                                                                             state.optional(|state|
                                                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                                                    self::semicolon(state)
                                                                                                                                                                                                                                                                                })
                                                                                                                                                                                                                                                         })
                                                          })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn fill(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::fill,
                               |state|
                                   {
                                       state.atomic(::pest::Atomicity::Atomic,
                                                    |state|
                                                        {
                                                            let strings =
                                                                ["sumtype"];
                                                            state.skip_until(&strings)
                                                        })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn tail(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::tail,
                               |state|
                                   {
                                       state.atomic(::pest::Atomicity::Atomic,
                                                    |state|
                                                        {
                                                            state.repeat(|state|
                                                                             {
                                                                                 self::ANY(state)
                                                                             })
                                                        })
                                   })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn file(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state|
                                       {
                                           self::SOI(state).and_then(|state|
                                                                         {
                                                                             super::hidden::skip(state)
                                                                         }).and_then(|state|
                                                                                         {
                                                                                             state.sequence(|state|
                                                                                                                {
                                                                                                                    state.optional(|state|
                                                                                                                                       {
                                                                                                                                           state.sequence(|state|
                                                                                                                                                              {
                                                                                                                                                                  self::fill(state).and_then(|state|
                                                                                                                                                                                                 {
                                                                                                                                                                                                     super::hidden::skip(state)
                                                                                                                                                                                                 }).and_then(|state|
                                                                                                                                                                                                                 {
                                                                                                                                                                                                                     self::sumtype_definition(state)
                                                                                                                                                                                                                 })
                                                                                                                                                              }).and_then(|state|
                                                                                                                                                                              {
                                                                                                                                                                                  state.repeat(|state|
                                                                                                                                                                                                   {
                                                                                                                                                                                                       state.sequence(|state|
                                                                                                                                                                                                                          {
                                                                                                                                                                                                                              super::hidden::skip(state).and_then(|state|
                                                                                                                                                                                                                                                                      {
                                                                                                                                                                                                                                                                          state.sequence(|state|
                                                                                                                                                                                                                                                                                             {
                                                                                                                                                                                                                                                                                                 self::fill(state).and_then(|state|
                                                                                                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                                                                                                    super::hidden::skip(state)
                                                                                                                                                                                                                                                                                                                                }).and_then(|state|
                                                                                                                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                                                                                                                    self::sumtype_definition(state)
                                                                                                                                                                                                                                                                                                                                                })
                                                                                                                                                                                                                                                                                             })
                                                                                                                                                                                                                                                                      })
                                                                                                                                                                                                                          })
                                                                                                                                                                                                   })
                                                                                                                                                                              })
                                                                                                                                       })
                                                                                                                })
                                                                                         }).and_then(|state|
                                                                                                         {
                                                                                                             super::hidden::skip(state)
                                                                                                         }).and_then(|state|
                                                                                                                         {
                                                                                                                             self::tail(state)
                                                                                                                         }).and_then(|state|
                                                                                                                                         {
                                                                                                                                             super::hidden::skip(state)
                                                                                                                                         }).and_then(|state|
                                                                                                                                                         {
                                                                                                                                                             self::EOI(state)
                                                                                                                                                         })
                                       })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WHITESPACE(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::WHITESPACE,
                               |state|
                                   {
                                       state.atomic(::pest::Atomicity::Atomic,
                                                    |state|
                                                        {
                                                            state.match_string(" ").or_else(|state|
                                                                                                {
                                                                                                    state.match_string("\t")
                                                                                                }).or_else(|state|
                                                                                                               {
                                                                                                                   self::NEWLINE(state)
                                                                                                               })
                                                        })
                                   })
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn NEWLINE(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("\n").or_else(|state|
                                                         state.match_string("\r\n")).or_else(|state|
                                                                                                 state.match_string("\r"))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.skip(1)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(state: Box<::pest::ParserState<Rule>>)
                 -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
            }
            pub use self::visible::*;
        }
        ::pest::state(input,
                      |state|
                          {
                              match rule {
                                  Rule::short_comment =>
                                  rules::short_comment(state),
                                  Rule::long_comment =>
                                  rules::long_comment(state),
                                  Rule::COMMENT => rules::COMMENT(state),
                                  Rule::alpha => rules::alpha(state),
                                  Rule::alpha_ => rules::alpha_(state),
                                  Rule::digit => rules::digit(state),
                                  Rule::identifier =>
                                  rules::identifier(state),
                                  Rule::type_name => rules::type_name(state),
                                  Rule::namespace_name =>
                                  rules::namespace_name(state),
                                  Rule::type_reference =>
                                  rules::type_reference(state),
                                  Rule::simple_sumtype_item =>
                                  rules::simple_sumtype_item(state),
                                  Rule::typed_sumtype_item =>
                                  rules::typed_sumtype_item(state),
                                  Rule::semicolon => rules::semicolon(state),
                                  Rule::sumtype_item =>
                                  rules::sumtype_item(state),
                                  Rule::curly_open =>
                                  rules::curly_open(state),
                                  Rule::curly_close =>
                                  rules::curly_close(state),
                                  Rule::sumtype_definition =>
                                  rules::sumtype_definition(state),
                                  Rule::fill => rules::fill(state),
                                  Rule::tail => rules::tail(state),
                                  Rule::file => rules::file(state),
                                  Rule::WHITESPACE =>
                                  rules::WHITESPACE(state),
                                  Rule::EOI => rules::EOI(state),
                              }
                          })
    }
}

fn process_sumtype_item(sumtype_item: pest::iterators::Pairs<Rule>)
 -> (String, String, String) {
    let mut output = String::default();
    let mut item_name = String::default();
    let mut item_type = String::default();

    for item in sumtype_item {
        //println!("***{:?}***\n {}", item.as_rule(), item.as_str());
        match item.as_rule() {
            Rule::WHITESPACE => { output += item.as_str(); }
            Rule::COMMENT => { output += item.as_str(); }
            Rule::simple_sumtype_item => {
                item_name = item.as_str().to_owned();
                output = output + "_" + item.as_str();
            }
            Rule::typed_sumtype_item => {
                for item in item.into_inner() {
                    //println!("****{:?}****\n {}", item.as_rule(), item.as_str());
                    match item.as_rule() {
                        Rule::identifier => {
                            item_name = item.as_str().to_owned();
                            output = output + "_" + item.as_str();
                        }
                        Rule::type_reference => {
                            item_type = item.as_str().to_owned();
                            output =
                                output + " /* (" + item.as_str() + ") */";
                        }
                        _ =>


                        //println!("**{:?}**\n {}", item.as_rule(), item.as_str());






                        //...




                        //println!("*{:?}*\n {}", item.as_rule(), item.as_str());






                        {
                            {
                                $crate::rt::begin_panic("internal error: entered unreachable code",
                                                        &("src\\main.rs",
                                                          48u32, 30u32))
                            }
                        }
                    }
                }
            }
            Rule::semicolon => { output += ","; }
            _ => {
                {
                    $crate::rt::begin_panic("internal error: entered unreachable code",
                                            &("src\\main.rs", 55u32, 18u32))
                }
            }
        }
    }
    ("\t".to_owned() + &output, item_name, item_type)
}
fn process_sumtype_definition(sumtype: pest::iterators::Pairs<Rule>)
 -> String {
    let mut output = String::default();
    let mut sumtype_name = String::default();
    let mut items = Vec::<(String, String)>::new();
    let mut done = HashSet::<String>::new();
    for item in sumtype {
        match item.as_rule() {
            Rule::WHITESPACE => { output += item.as_str(); }
            Rule::COMMENT => { output += item.as_str(); }
            Rule::curly_open => {
                output += r#"{
public:
    enum Tags
    {"#;
            }
            Rule::type_name => {
                output = output + "class " + item.as_str();
                sumtype_name = item.as_str().to_owned();
            }
            Rule::sumtype_item => {
                let (o, n, t) = process_sumtype_item(item.into_inner());
                output += &o;
                items.push((n, t));
            }
            Rule::curly_close => {
                output +=
                    r#"    };

private:
    Tags _tag;

    union
    {
"#;
                for pair in items.iter() {
                    if pair.1.is_empty() { continue ; }
                    output =
                        output + "\t\t" + &pair.1 + " " + &pair.0 + ";\n";
                }
                output =
                    output + r#"    };

    "# + &sumtype_name +
                        r#"(Tags tag) : 
        _tag(tag)
    {
    }
"#;
                for pair in items.iter() {
                    if pair.1.is_empty() { continue ; }
                    let conflict = done.contains(&pair.1);
                    done.insert(pair.1.to_owned());
                    if conflict {
                        output +=
                            r#"
    //This overload already exist above.
    /*"#;
                    }
                    output =
                        output + r#"
    "# + &sumtype_name + r#"(Tags tag, "#
                            + &pair.1 +
                            r#" value) : 
        _tag(tag),
        _"# +
                            &pair.0 + r#"(value)
    {
    }
"#;
                    if conflict { output += "    */\n"; }
                }
                output =
                    output + r#"
public:
    ~"# + &sumtype_name +
                        r#"()
    {
        //call only the required destructor explicitly
        switch (_tag)
        {
"#;
                for pair in items.iter() {
                    if pair.1.is_empty() { continue ; }
                    output =
                        output + "        case " + &pair.0 + ": _" + &pair.0 +
                            ".~" + &pair.1 + "(); break;\n";
                }
                output =
                    output +
                        r#"        }
    }

    Tags tag() { return _tag; };
    
"#;
                for pair in items.iter() {
                    output =
                        output + "    static " + &sumtype_name + " " + &pair.0
                            +
                            &if pair.1.is_empty() {
                                 r#"() 
    {
        return "#.to_owned() +
                                     &sumtype_name + "(" + &pair.0
                             } else {
                                 "(".to_owned() + &pair.1 +
                                     r#" value) 
    {
        return "# +
                                     &sumtype_name + "(" + &pair.0 + ", value"
                             } + r#");
    }
    
"#
                }
                output += "\n}";
            }
            Rule::semicolon => { output += ";"; }
            _ => {
                {
                    $crate::rt::begin_panic("internal error: entered unreachable code",
                                            &("src\\main.rs", 193u32, 18u32))
                }
            }
        }
    }
    output
}
fn process_file(file: pest::iterators::Pairs<Rule>) -> String {
    let mut output = String::default();
    for item in file {
        match item.as_rule() {
            Rule::fill => { output += item.as_str(); }
            Rule::sumtype_definition => {
                output += &process_sumtype_definition(item.into_inner());
            }
            Rule::COMMENT => { output += item.as_str(); }
            Rule::WHITESPACE => { output += item.as_str(); }
            Rule::tail => { output += item.as_str(); }
            Rule::EOI => { }
            _ => {
                {
                    $crate::rt::begin_panic("internal error: entered unreachable code",
                                            &("src\\main.rs", 225u32, 18u32))
                }
            }
        }
    }
    output
}
fn parse(input: &str) -> Result<String, String> {
    match EnumsParser::parse(Rule::file, input) {
        Ok(result) => Ok(process_file(result)),
        Err(err) =>
        Err($crate::fmt::Arguments::new_v1_formatted(&[""],
                                                     &match (&err,) {
                                                          (arg0,) =>
                                                          [$crate::fmt::ArgumentV1::new(arg0,
                                                                                        $crate::fmt::Debug::fmt)],
                                                      },
                                                     &[$crate::fmt::rt::v1::Argument{position:
                                                                                         $crate::fmt::rt::v1::Position::At(0usize),
                                                                                     format:
                                                                                         $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                             ' ',
                                                                                                                         align:
                                                                                                                             $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                         flags:
                                                                                                                             0u32,
                                                                                                                         precision:
                                                                                                                             $crate::fmt::rt::v1::Count::Implied,
                                                                                                                         width:
                                                                                                                             $crate::fmt::rt::v1::Count::Implied,},}]).to_string()),
    }
}
fn run(input_filename: &str, output_filename: &str) -> Result<(), String> {
    let input =
        fs::read_to_string(input_filename).expect("cannot read input file");
    let mut output =
        fs::File::create(output_filename).expect("cannot open output file");
    let result = parse(&input)?;
    output.write(result.as_bytes()).expect("cannot write output file");
    Ok(())
}
fn main() {
    let matches =
        App::new("C++ sumtype").version("0.1").about("Generates C++ code to represent sum types for the poor fellows not allowed to use Rust.").author("Tibor P").arg(Arg::with_name("INPUT").help("Sets the input file to use").required(true).index(1)).arg(Arg::with_name("OUTPUT").help("Sets the output file to use").required(true).index(2)).get_matches();
    if let Some(input_filename) = matches.value_of("INPUT") {
        if let Some(output_filename) = matches.value_of("OUTPUT") {
            if let Err(error) = run(input_filename, output_filename) {
                {
                    $crate::io::_print($crate::fmt::Arguments::new_v1_formatted(&["ERROR: ",
                                                                                  "\n"],
                                                                                &match (&error,)
                                                                                     {
                                                                                     (arg0,)
                                                                                     =>
                                                                                     [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                   $crate::fmt::Debug::fmt)],
                                                                                 },
                                                                                &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                    $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                format:
                                                                                                                    $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                        ' ',
                                                                                                                                                    align:
                                                                                                                                                        $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                    flags:
                                                                                                                                                        0u32,
                                                                                                                                                    precision:
                                                                                                                                                        $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                    width:
                                                                                                                                                        $crate::fmt::rt::v1::Count::Implied,},}]));
                };
            } else {
                {
                    $crate::io::_print($crate::fmt::Arguments::new_v1_formatted(&["Successfully written: ",
                                                                                  "\n"],
                                                                                &match (&output_filename,)
                                                                                     {
                                                                                     (arg0,)
                                                                                     =>
                                                                                     [$crate::fmt::ArgumentV1::new(arg0,
                                                                                                                   $crate::fmt::Display::fmt)],
                                                                                 },
                                                                                &[$crate::fmt::rt::v1::Argument{position:
                                                                                                                    $crate::fmt::rt::v1::Position::At(0usize),
                                                                                                                format:
                                                                                                                    $crate::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                        ' ',
                                                                                                                                                    align:
                                                                                                                                                        $crate::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                    flags:
                                                                                                                                                        0u32,
                                                                                                                                                    precision:
                                                                                                                                                        $crate::fmt::rt::v1::Count::Implied,
                                                                                                                                                    width:
                                                                                                                                                        $crate::fmt::rt::v1::Count::Implied,},}]));
                };
            }
        }
    }
}
