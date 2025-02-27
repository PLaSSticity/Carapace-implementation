
#![feature(trace_macros)]

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use std::collections::HashSet;
//use std::io::Write;
use std::{iter::FromIterator, str::FromStr};
use syn::{parse_macro_input, spanned::Spanned, Data, DataStruct, DeriveInput, Expr, Fields, Type, Block, FieldValue, ExprField};
use syn::parse::{Parse, ParseStream};
use syn::token::{Comma, Eq};
//use core::iter::Map;

//trace_macros!(true);

struct InfoFlowLabeledBlockStaticAll {
    t1: Type,
    t2: Type,
    blk: Block
}

impl Parse for InfoFlowLabeledBlockStaticAll {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        //println!("\n\n\n{}\n\n\n", input.to_string());
        let t1: Type = input.parse().unwrap_or_else(|_|{panic!("not a type")});
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let t2: Type = input.parse().unwrap_or_else(|_|{panic!("not a type")});
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let blk: Block = input.parse().unwrap();
        Ok(InfoFlowLabeledBlockStaticAll {t1, t2, blk})
    }
}

struct InfoFlowLabeledBlockDynamicOne {
    t1: Type,
    t2: Type,
    e: Expr,
    blk: Block
}

impl Parse for InfoFlowLabeledBlockDynamicOne {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        //println!("\n\n\n{}\n\n\n", input.to_string());
        let t1: Type = input.parse().unwrap_or_else(|_|{panic!("not a type")});
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let t2: Type = input.parse().unwrap_or_else(|_|{panic!("not a type")});
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let e: Expr = input.parse().unwrap();
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let blk: Block = input.parse().unwrap();
        Ok(InfoFlowLabeledBlockDynamicOne {t1, t2, e, blk})
    }
}

struct InfoFlowLabeledBlockDynamicAll {
    t1: Type,
    t2: Type,
    e1: Expr,
    e2: Expr,
    blk: Block
}

impl Parse for InfoFlowLabeledBlockDynamicAll {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        //println!("\n\n\n{}\n\n\n", input.to_string());
        let t1: Type = input.parse().unwrap_or_else(|_|{panic!("not a type")});
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let t2: Type = input.parse().unwrap_or_else(|_|{panic!("not a type")});
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let e1: Expr = input.parse().unwrap();
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let e2: Expr = input.parse().unwrap();
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let blk: Block = input.parse().unwrap();
        Ok(InfoFlowLabeledBlockDynamicAll {t1, t2, e1, e2, blk})
    }
}

struct InfoFlowLabeledBlockStaticAllDeclassifyStaticAll {
    t1: Type,
    t2: Type,
    t3: Type,
    t4: Type,
    blk: Block
}

impl Parse for InfoFlowLabeledBlockStaticAllDeclassifyStaticAll {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        //println!("\n\n\n{}\n\n\n", input.to_string());
        let t1: Type = input.parse().unwrap_or_else(|_|{panic!("not a type")});
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let t2: Type = input.parse().unwrap_or_else(|_|{panic!("not a type")});
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let t3: Type = input.parse().unwrap_or_else(|_|{panic!("not a type")});
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let t4: Type = input.parse().unwrap_or_else(|_|{panic!("not a type")});
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let blk: Block = input.parse().unwrap();
        Ok(InfoFlowLabeledBlockStaticAllDeclassifyStaticAll {t1, t2, t3, t4, blk})
    }
}

struct InfoFlowLabeledBlockDynamicOneDeclassifyDynamicOne {
    t1: Type,
    t2: Type,
    e1: Expr,
    t3: Type,
    t4: Type,
    e2: Expr,
    blk: Block
}

impl Parse for InfoFlowLabeledBlockDynamicOneDeclassifyDynamicOne {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        //println!("\n\n\n{}\n\n\n", input.to_string());
        let t1: Type = input.parse().unwrap_or_else(|_|{panic!("not a type")});
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let t2: Type = input.parse().unwrap_or_else(|_|{panic!("not a type")});
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let e1: Expr = input.parse().unwrap();
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let t3: Type = input.parse().unwrap_or_else(|_|{panic!("not a type")});
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let t4: Type = input.parse().unwrap_or_else(|_|{panic!("not a type")});
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let e2: Expr = input.parse().unwrap();
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let blk: Block = input.parse().unwrap();
        Ok(InfoFlowLabeledBlockDynamicOneDeclassifyDynamicOne {t1, t2, e1, t3, t4, e2, blk})
    }
}

struct InfoFlowLabeledBlockDynamicAllDeclassifyDynamicAll {
    t1: Type,
    t2: Type,
    e1: Expr,
    e2: Expr,
    t3: Type,
    t4: Type,
    e3: Expr,
    e4: Expr,
    blk: Block
}

impl Parse for InfoFlowLabeledBlockDynamicAllDeclassifyDynamicAll {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        //println!("\n\n\n{}\n\n\n", input.to_string());
        let t1: Type = input.parse().unwrap_or_else(|_|{panic!("not a type")});
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let t2: Type = input.parse().unwrap_or_else(|_|{panic!("not a type")});
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let e1: Expr = input.parse().unwrap();
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let e2: Expr = input.parse().unwrap();
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let t3: Type = input.parse().unwrap_or_else(|_|{panic!("not a type")});
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let t4: Type = input.parse().unwrap_or_else(|_|{panic!("not a type")});
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let e3: Expr = input.parse().unwrap();
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let e4: Expr = input.parse().unwrap();
        let _: Comma = input.parse().unwrap_or_else(|_|{panic!("not a comma")});
        let blk: Block = input.parse().unwrap();
        Ok(InfoFlowLabeledBlockDynamicAllDeclassifyDynamicAll {t1, t2, e1, e2, t3, t4, e3, e4, blk})
    }
}

fn generate_increment(#[allow(unused_variables)] tokens: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    #[cfg(feature = "benchmarking")]
    { return quote::quote!{ unsafe { ::secret_structs::secret::#tokens.increment() } }; }

    return quote::quote!{};
}

#[proc_macro]
pub fn untrusted_secure_block_static_all(tokens: TokenStream) -> TokenStream {
    //println!("\n\nreached first {}\n\n", tokens.to_string());
    let InfoFlowLabeledBlockStaticAll{t1, t2, blk} = parse_macro_input!(tokens as InfoFlowLabeledBlockStaticAll);
    let executed_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), false, false, false, true, true, false
    ).into();
    let checking_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), true, false, false, true, true, false
    ).into();
    let init_code: proc_macro2::TokenStream = block_init_code(false, false, false, None, None, None, None).into();
    let total_increment = generate_increment(quote::quote!{TOTAL_BLOCK_COUNTER});
    let untrusted_increment = generate_increment(quote::quote!{UNTRUSTED_BLOCK_COUNTER});
    quote::quote! {
        {
            /*#[cfg(feature = "benchmarking")] 
            unsafe { secret_structs::secret::TOTAL_BLOCK_COUNTER.increment();
                secret_structs::secret::UNTRUSTED_BLOCK_COUNTER.increment(); }*/
            #total_increment
            #untrusted_increment
            #init_code
            if true {
                ::secret_structs::secret::call_info_flow_closure::<#t1, #t2, _, _>(
                    #executed_code
                )
            } else {
                ::secret_structs::secret::call_info_flow_closure::<#t1, #t2, _, _>(
                    #[allow(unused_mut)]
                    #checking_code
                )
            }
        }
    }.into()
}
#[proc_macro]
pub fn untrusted_secure_block_dynamic_secrecy(tokens: TokenStream) -> TokenStream {
    let InfoFlowLabeledBlockDynamicOne{t1, t2, e, blk} = parse_macro_input!(tokens as InfoFlowLabeledBlockDynamicOne);
    let executed_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), false, true, false, true, true, false
    ).into();
    let checking_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), true, true, false, true, true, false
    ).into();
    let init_code: proc_macro2::TokenStream = block_init_code(true, false, false, Some(e), None, None, None).into();
    let total_increment = generate_increment(quote::quote!{TOTAL_BLOCK_COUNTER});
    let untrusted_increment = generate_increment(quote::quote!{UNTRUSTED_BLOCK_COUNTER});
    let return_check_code: proc_macro2::TokenStream = block_return_check_code(true, false, false).into();
    quote::quote! {
        {
            /*#[cfg(feature = "benchmarking")] 
            unsafe { secret_structs::secret::TOTAL_BLOCK_COUNTER.increment();
                secret_structs::secret::UNTRUSTED_BLOCK_COUNTER.increment(); }*/
            #total_increment
            #untrusted_increment
            #init_code
            let return_value = if true {
                ::secret_structs::secret::call_info_flow_closure::<#t1, #t2, _, _>(
                    #executed_code
                )
            } else {
                ::secret_structs::secret::call_info_flow_closure::<#t1, #t2, _, _>(
                    #[allow(unused_mut)]
                    #checking_code
                )
            };
            #return_check_code
            return_value
        }
    }.into()
}
#[proc_macro]
pub fn untrusted_secure_block_dynamic_integrity(tokens: TokenStream) -> TokenStream {
    let InfoFlowLabeledBlockDynamicOne{t1, t2, e, blk} = parse_macro_input!(tokens as InfoFlowLabeledBlockDynamicOne);
    let executed_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), false, false, true, true, true, false
    ).into();
    let checking_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), true, false, true, true, true, false
    ).into();
    let init_code: proc_macro2::TokenStream = block_init_code(false, true, false, None, Some(e), None, None).into();
    let total_increment = generate_increment(quote::quote!{TOTAL_BLOCK_COUNTER});
    let untrusted_increment = generate_increment(quote::quote!{UNTRUSTED_BLOCK_COUNTER});
    let return_check_code: proc_macro2::TokenStream = block_return_check_code(false, true, false).into();
    quote::quote! {
        {
            /*#[cfg(feature = "benchmarking")] 
            unsafe { secret_structs::secret::TOTAL_BLOCK_COUNTER.increment();
                secret_structs::secret::UNTRUSTED_BLOCK_COUNTER.increment(); }*/
            #total_increment
            #untrusted_increment
            #init_code
            let return_value = if true {
                ::secret_structs::secret::call_info_flow_closure::<#t1, #t2, _, _>(
                    #executed_code
                )
            } else {
                ::secret_structs::secret::call_info_flow_closure::<#t1, #t2, _, _>(
                    #[allow(unused_mut)]
                    #checking_code
                )
            };
            #return_check_code
            return_value
        }
    }.into()
}
#[proc_macro]
pub fn untrusted_secure_block_dynamic_all(tokens: TokenStream) -> TokenStream {
    let InfoFlowLabeledBlockDynamicAll{t1, t2, e1, e2, blk} = parse_macro_input!(tokens as InfoFlowLabeledBlockDynamicAll);
    let executed_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), false, true, true, true, true, false
    ).into();
    let checking_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), true, true, true, true, true, false
    ).into();
    let init_code: proc_macro2::TokenStream = block_init_code(true, true, false, Some(e1), Some(e2), None, None).into();
    let total_increment = generate_increment(quote::quote!{TOTAL_BLOCK_COUNTER});
    let untrusted_increment = generate_increment(quote::quote!{UNTRUSTED_BLOCK_COUNTER});
    let return_check_code: proc_macro2::TokenStream = block_return_check_code(true, true, false).into();
    quote::quote! {
        {
            /*#[cfg(feature = "benchmarking")] 
            unsafe { secret_structs::secret::TOTAL_BLOCK_COUNTER.increment();
                secret_structs::secret::UNTRUSTED_BLOCK_COUNTER.increment(); }*/
            #total_increment
            #untrusted_increment
            #init_code
            let return_value = if true {
                ::secret_structs::secret::call_info_flow_closure::<#t1, #t2, _, _>(
                    #executed_code
                )
            } else {
                ::secret_structs::secret::call_info_flow_closure::<#t1, #t2, _, _>(
                    #[allow(unused_mut)]
                    #checking_code
                )
            };
            #return_check_code
            return_value
        }
    }.into()
}

#[proc_macro] 
pub fn trusted_secure_block_static_all(tokens: TokenStream) -> TokenStream {
    let InfoFlowLabeledBlockStaticAll{t1, t2, blk} = parse_macro_input!(tokens as InfoFlowLabeledBlockStaticAll);
    let executed_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), false, false, false, true, false, false
    ).into();
    let checking_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), true, false, false, true, false, false
    ).into();
    let init_code: proc_macro2::TokenStream = block_init_code(false, false, false, None, None, None, None).into();
    let total_increment = generate_increment(quote::quote!{TOTAL_BLOCK_COUNTER});
    let trusted_increment = generate_increment(quote::quote!{TRUSTED_BLOCK_COUNTER});
    quote::quote! {
        {
            /*#[cfg(feature = "benchmarking")]
            unsafe { secret_structs::secret::TOTAL_BLOCK_COUNTER.increment();
                secret_structs::secret::TRUSTED_BLOCK_COUNTER.increment(); }*/
            #total_increment
            #trusted_increment
            #init_code
            if true {
                ::secret_structs::secret::call_info_flow_closure_declassify::<#t1, #t2, _, _>(
                    #executed_code
                )
            } else {
                ::secret_structs::secret::call_info_flow_closure_declassify::<#t1, #t2, _, _>(
                    #[allow(unused_mut)]
                    #checking_code
                )
            }
        }
    }.into()
}
#[proc_macro] 
pub fn trusted_secure_block_dynamic_secrecy(tokens: TokenStream) -> TokenStream {
    let InfoFlowLabeledBlockDynamicOne{t1, t2, e, blk} = parse_macro_input!(tokens as InfoFlowLabeledBlockDynamicOne);
    let executed_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), false, true, false, true, false, false
    ).into();
    let checking_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), true, true, false, true, false, false
    ).into();
    let init_code: proc_macro2::TokenStream = block_init_code(true, false, false, Some(e), None, None, None).into();
    let total_increment = generate_increment(quote::quote!{TOTAL_BLOCK_COUNTER});
    let trusted_increment = generate_increment(quote::quote!{TRUSTED_BLOCK_COUNTER});
    quote::quote! {
        {
            /*#[cfg(feature = "benchmarking")]
            unsafe { secret_structs::secret::TOTAL_BLOCK_COUNTER.increment();
                secret_structs::secret::TRUSTED_BLOCK_COUNTER.increment(); }*/
            #total_increment
            #trusted_increment
            #init_code
            if true {
                ::secret_structs::secret::call_info_flow_closure_declassify::<#t1, #t2, _, _>(
                    #executed_code
                )
            } else {
                ::secret_structs::secret::call_info_flow_closure_declassify::<#t1, #t2, _, _>(
                    #[allow(unused_mut)]
                    #checking_code
                )
            }
        }
    }.into()
}
#[proc_macro] 
pub fn trusted_secure_block_dynamic_integrity(tokens: TokenStream) -> TokenStream {
    let InfoFlowLabeledBlockDynamicOne{t1, t2, e, blk} = parse_macro_input!(tokens as InfoFlowLabeledBlockDynamicOne);
    let executed_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), false, false, true, true, false, false
    ).into();
    let checking_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), true, false, true, true, false, false
    ).into();
    let init_code: proc_macro2::TokenStream = block_init_code(false, true, false, None, Some(e), None, None).into();
    let total_increment = generate_increment(quote::quote!{TOTAL_BLOCK_COUNTER});
    let trusted_increment = generate_increment(quote::quote!{TRUSTED_BLOCK_COUNTER});
    quote::quote! {
        {
            /*#[cfg(feature = "benchmarking")]
            unsafe { secret_structs::secret::TOTAL_BLOCK_COUNTER.increment();
                secret_structs::secret::TRUSTED_BLOCK_COUNTER.increment(); }*/
            #total_increment
            #trusted_increment
            #init_code
            if true {
                ::secret_structs::secret::call_info_flow_closure_declassify::<#t1, #t2, _, _>(
                    #executed_code
                )
            } else {
                ::secret_structs::secret::call_info_flow_closure_declassify::<#t1, #t2, _, _>(
                    #[allow(unused_mut)]
                    #checking_code
                )
            }
        }
    }.into()
}
#[proc_macro] 
pub fn trusted_secure_block_dynamic_all(tokens: TokenStream) -> TokenStream {
    let InfoFlowLabeledBlockDynamicAll{t1, t2, e1, e2, blk} = parse_macro_input!(tokens as InfoFlowLabeledBlockDynamicAll);
    let executed_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), false, true, true, true, false, false
    ).into();
    let checking_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), true, true, true, true, false, false
    ).into();
    let init_code: proc_macro2::TokenStream = block_init_code(true, true, false, Some(e1), Some(e2), None, None).into();
    let total_increment = generate_increment(quote::quote!{TOTAL_BLOCK_COUNTER});
    let trusted_increment = generate_increment(quote::quote!{TRUSTED_BLOCK_COUNTER});
    return quote::quote! {
        {
            /*#[cfg(feature = "benchmarking")]
            unsafe { secret_structs::secret::TOTAL_BLOCK_COUNTER.increment();
                secret_structs::secret::TRUSTED_BLOCK_COUNTER.increment(); }*/
            #total_increment
            #trusted_increment
            #init_code
            if true {
                ::secret_structs::secret::call_info_flow_closure_declassify::<#t1, #t2, _, _>(
                    #executed_code
                )
            } else {
                ::secret_structs::secret::call_info_flow_closure_declassify::<#t1, #t2, _, _>(
                    #[allow(unused_mut)]
                    #checking_code
                )
            }
        }
    }.into()
}

#[proc_macro] 
pub fn partial_trusted_secure_block_static_all(tokens: TokenStream) -> TokenStream {
    let InfoFlowLabeledBlockStaticAllDeclassifyStaticAll{t1, t2, t3, t4, blk} = parse_macro_input!(tokens as InfoFlowLabeledBlockStaticAllDeclassifyStaticAll);
    let executed_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2, #t3, #t4) { #blk }
        }.into(), false, false, false, true, true, true
    ).into();
    let checking_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2, #t3, #t4) { #blk }
        }.into(), true, false, false, true, true, true
    ).into();
    let init_code: proc_macro2::TokenStream = block_init_code(false, false, true, None, None, None, None).into();
    let total_increment = generate_increment(quote::quote!{TOTAL_BLOCK_COUNTER});
    let trusted_increment = generate_increment(quote::quote!{TRUSTED_BLOCK_COUNTER});
    quote::quote! {
        {
            /*#[cfg(feature = "benchmarking")]
            unsafe { secret_structs::secret::TOTAL_BLOCK_COUNTER.increment();
                secret_structs::secret::TRUSTED_BLOCK_COUNTER.increment(); }*/
            #total_increment
            #trusted_increment
            #init_code
            if true {
                ::secret_structs::secret::call_info_flow_closure::<#t3, #t4, _, _>(
                    #executed_code
                )
            } else {
                ::secret_structs::secret::(
                    #[allow(unused_mut)]
                    #checking_code
                )
            }
        }
    }.into()
}
#[proc_macro] 
pub fn partial_trusted_secure_block_dynamic_secrecy(tokens: TokenStream) -> TokenStream {
    let InfoFlowLabeledBlockDynamicOneDeclassifyDynamicOne{t1, t2, e1, t3, t4, e2, blk} = parse_macro_input!(tokens as InfoFlowLabeledBlockDynamicOneDeclassifyDynamicOne);
    let executed_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2, #t3, #t4) { #blk }
        }.into(), false, true, false, true, true, true
    ).into();
    let checking_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2, #t3, #t4) { #blk }
        }.into(), true, true, false, true, true, true
    ).into();
    let init_code: proc_macro2::TokenStream = block_init_code(true, false, true, Some(e1), None, Some(e2), None).into();
    let total_increment = generate_increment(quote::quote!{TOTAL_BLOCK_COUNTER});
    let trusted_increment = generate_increment(quote::quote!{TRUSTED_BLOCK_COUNTER});
    let return_check_code: proc_macro2::TokenStream = block_return_check_code(true, false, true).into();
    quote::quote! {
        {
            /*#[cfg(feature = "benchmarking")]
            unsafe { secret_structs::secret::TOTAL_BLOCK_COUNTER.increment();
                secret_structs::secret::TRUSTED_BLOCK_COUNTER.increment(); }*/
            #total_increment
            #trusted_increment
            #init_code
            let return_value = if true {
                ::secret_structs::secret::call_info_flow_closure::<#t3, #t4, _, _>(
                    #executed_code
                )
            } else {
                ::secret_structs::secret::call_info_flow_closure::<#t3, #t4, _, _>(
                    #[allow(unused_mut)]
                    #checking_code
                )
            };
            #return_check_code
            return_value
        }
    }.into()
}
#[proc_macro] 
pub fn partial_trusted_secure_block_dynamic_integrity(tokens: TokenStream) -> TokenStream {
    let InfoFlowLabeledBlockDynamicOneDeclassifyDynamicOne{t1, t2, e1, t3, t4, e2, blk} = parse_macro_input!(tokens as InfoFlowLabeledBlockDynamicOneDeclassifyDynamicOne);
    let executed_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2, #t3, #t4) { #blk }
        }.into(), false, false, true, true, true, true
    ).into();
    let checking_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2, #t3, #t4) { #blk }
        }.into(), true, false, true, true, true, true
    ).into();
    let init_code: proc_macro2::TokenStream = block_init_code(false, true, true, None, Some(e1), None, Some(e2)).into();
    let total_increment = generate_increment(quote::quote!{TOTAL_BLOCK_COUNTER});
    let trusted_increment = generate_increment(quote::quote!{TRUSTED_BLOCK_COUNTER});
    let return_check_code: proc_macro2::TokenStream = block_return_check_code(false, true, true).into();
    quote::quote! {
        {
            /*#[cfg(feature = "benchmarking")]
            unsafe { secret_structs::secret::TOTAL_BLOCK_COUNTER.increment();
                secret_structs::secret::TRUSTED_BLOCK_COUNTER.increment(); }*/
            #total_increment
            #trusted_increment
            #init_code
            let return_value = if true {
                ::secret_structs::secret::call_info_flow_closure::<#t3, #t4, _, _>(
                    #executed_code
                )
            } else {
                ::secret_structs::secret::call_info_flow_closure::<#t3, #t4, _, _>(
                    #[allow(unused_mut)]
                    #checking_code
                )
            };
            #return_check_code
            return_value
        }
    }.into()
}
#[proc_macro] 
pub fn partial_trusted_secure_block_dynamic_all(tokens: TokenStream) -> TokenStream {
    let InfoFlowLabeledBlockDynamicAllDeclassifyDynamicAll{t1, t2, e1, e2, t3, t4, e3, e4, blk} = parse_macro_input!(tokens as InfoFlowLabeledBlockDynamicAllDeclassifyDynamicAll);
    let executed_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2, #t3, #t4) { #blk }
        }.into(), false, true, true, true, true, true
    ).into();
    let checking_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2, #t3, #t4) { #blk }
        }.into(), true, true, true, true, true, true
    ).into();
    let init_code: proc_macro2::TokenStream = block_init_code(true, true, true, Some(e1), Some(e2), Some(e3), Some(e4)).into();
    let total_increment = generate_increment(quote::quote!{TOTAL_BLOCK_COUNTER});
    let trusted_increment = generate_increment(quote::quote!{TRUSTED_BLOCK_COUNTER});
    let return_check_code: proc_macro2::TokenStream = block_return_check_code(true, true, true).into();
    quote::quote! {
        {
            /*#[cfg(feature = "benchmarking")]
            unsafe { secret_structs::secret::TOTAL_BLOCK_COUNTER.increment();
                secret_structs::secret::TRUSTED_BLOCK_COUNTER.increment(); }*/
            #total_increment
            #trusted_increment
            #init_code
            let return_value = if true {
                ::secret_structs::secret::call_info_flow_closure::<#t3, #t4, _, _>(
                    #executed_code
                )
            } else {
                ::secret_structs::secret::call_info_flow_closure::<#t3, #t4, _, _>(
                    #[allow(unused_mut)]
                    #checking_code
                )
            };
            #return_check_code
            return_value
        }
    }.into()
}

#[proc_macro]
pub fn untrusted_secure_block_no_return_static_all(tokens: TokenStream) -> TokenStream {
    let InfoFlowLabeledBlockStaticAll{t1, t2, blk} = parse_macro_input!(tokens as InfoFlowLabeledBlockStaticAll);
    let executed_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), false, false, false, false, true, false
    ).into();
    let checking_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), true, false, false, false, true, false
    ).into();
    let init_code: proc_macro2::TokenStream = block_init_code(false, false, false, None, None, None, None).into();
    let total_increment = generate_increment(quote::quote!{TOTAL_BLOCK_COUNTER});
    let untrusted_increment = generate_increment(quote::quote!{UNTRUSTED_BLOCK_COUNTER});
    quote::quote! {
        {
            /*#[cfg(feature = "benchmarking")]
            unsafe { secret_structs::secret::TOTAL_BLOCK_COUNTER.increment();
                secret_structs::secret::UNTRUSTED_BLOCK_COUNTER.increment(); }*/
            #total_increment
            #untrusted_increment
            #init_code
            if true {
                ::secret_structs::secret::call_info_flow_closure_no_return::<#t1, #t2, _>(
                    #executed_code
                )
            } else {
                ::secret_structs::secret::call_info_flow_closure_no_return::<#t1, #t2, _>(
                    #[allow(unused_mut)]
                    #checking_code
                )
            }
        }
    }.into()
}
#[proc_macro]
pub fn untrusted_secure_block_no_return_dynamic_secrecy(tokens: TokenStream) -> TokenStream {
    let InfoFlowLabeledBlockDynamicOne{t1, t2, e, blk} = parse_macro_input!(tokens as InfoFlowLabeledBlockDynamicOne);
    let executed_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), false, true, false, false, true, false
    ).into();
    let checking_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), true, true, false, false, true, false
    ).into();
    let init_code: proc_macro2::TokenStream = block_init_code(true, false, false, Some(e), None, None, None).into();
    let total_increment = generate_increment(quote::quote!{TOTAL_BLOCK_COUNTER});
    let untrusted_increment = generate_increment(quote::quote!{UNTRUSTED_BLOCK_COUNTER});
    quote::quote! {
        {
            /*#[cfg(feature = "benchmarking")]
            unsafe { secret_structs::secret::TOTAL_BLOCK_COUNTER.increment();
                secret_structs::secret::UNTRUSTED_BLOCK_COUNTER.increment(); }*/
            #total_increment
            #untrusted_increment
            #init_code
            if true {
                ::secret_structs::secret::call_info_flow_closure_no_return::<#t1, #t2, _>(
                    #executed_code
                )
            } else {
                ::secret_structs::secret::call_info_flow_closure_no_return::<#t1, #t2, _>(
                    #[allow(unused_mut)]
                    #checking_code
                )
            }
        }
    }.into()
}
#[proc_macro]
pub fn untrusted_secure_block_no_return_dynamic_integrity(tokens: TokenStream) -> TokenStream {
    let InfoFlowLabeledBlockDynamicOne{t1, t2, e, blk} = parse_macro_input!(tokens as InfoFlowLabeledBlockDynamicOne);
    let executed_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), false, false, true, false, true, false
    ).into();
    let checking_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), true, false, true, false, true, false
    ).into();
    let init_code: proc_macro2::TokenStream = block_init_code(false, true, false, None, Some(e), None, None).into();
    let total_increment = generate_increment(quote::quote!{TOTAL_BLOCK_COUNTER});
    let untrusted_increment = generate_increment(quote::quote!{UNTRUSTED_BLOCK_COUNTER});
    quote::quote! {
        {
            /*#[cfg(feature = "benchmarking")]
            unsafe { secret_structs::secret::TOTAL_BLOCK_COUNTER.increment();
                secret_structs::secret::UNTRUSTED_BLOCK_COUNTER.increment(); }*/
            #total_increment
            #untrusted_increment
            #init_code
            if true {
                ::secret_structs::secret::call_info_flow_closure_no_return::<#t1, #t2, _>(
                    #executed_code
                )
            } else {
                ::secret_structs::secret::call_info_flow_closure_no_return::<#t1, #t2, _>(
                    #[allow(unused_mut)]
                    #checking_code
                )
            }
        }
    }.into()
}
#[proc_macro]
pub fn untrusted_secure_block_no_return_dynamic_all(tokens: TokenStream) -> TokenStream {
    let InfoFlowLabeledBlockDynamicAll{t1, t2, e1, e2, blk} = parse_macro_input!(tokens as InfoFlowLabeledBlockDynamicAll);
    let executed_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), false, true, true, false, true, false
    ).into();
    let checking_code: proc_macro2::TokenStream = info_flow_block_backend_helper(
        quote::quote! {
            || -> (#t1, #t2) { #blk }
        }.into(), true, true, true, false, true, false
    ).into();
    let init_code: proc_macro2::TokenStream = block_init_code(true, true, false, Some(e1), Some(e2), None, None).into();
    let total_increment = generate_increment(quote::quote!{TOTAL_BLOCK_COUNTER});
    let untrusted_increment = generate_increment(quote::quote!{UNTRUSTED_BLOCK_COUNTER});
    quote::quote! {
        {
            /*#[cfg(feature = "benchmarking")]
            unsafe { secret_structs::secret::TOTAL_BLOCK_COUNTER.increment();
                secret_structs::secret::UNTRUSTED_BLOCK_COUNTER.increment(); }*/
            #total_increment
            #untrusted_increment
            #init_code
            if true {
                ::secret_structs::secret::call_info_flow_closure_no_return::<#t1, #t2, _>(
                    #executed_code
                )
            } else {
                ::secret_structs::secret::call_info_flow_closure_no_return::<#t1, #t2, _>(
                    #[allow(unused_mut)]
                    #checking_code
                )
            }
        }
    }.into()
}

fn block_init_code(dynamic_secrecy: bool, dynamic_integrity: bool, partial_declassify: bool, expr1: Option<Expr>, expr2: Option<Expr>, expr3: Option<Expr>, expr4: Option<Expr>) -> TokenStream {
    let init_code: proc_macro2::TokenStream;
    let return_init_code: proc_macro2::TokenStream;
    if dynamic_secrecy && dynamic_integrity {
        init_code = quote::quote! {
            let block_secrecy: &::secret_structs::secret::DynLabel<Sec> = #expr1;
            let block_secrecy: &::secret_structs::secret::DynLabel<Sec> = unsafe { std::mem::transmute(block_secrecy) };
            let block_integrity: &::secret_structs::secret::DynLabel<Int> = #expr2;
            let block_integrity: &::secret_structs::secret::DynLabel<Int> = unsafe { std::mem::transmute(block_integrity) };
        };
        if partial_declassify {
            return_init_code = quote::quote! {
                let return_secrecy: &::secret_structs::secret::DynLabel<Sec> = #expr3;
                let return_secrecy: &::secret_structs::secret::DynLabel<Sec> = unsafe { std::mem::transmute(return_secrecy) };
                let return_integrity: &::secret_structs::secret::DynLabel<Int> = #expr4;
                let return_integrity: &::secret_structs::secret::DynLabel<Int> = unsafe { std::mem::transmute(return_integrity) };
            };
        } else {
            return_init_code = quote::quote! {};
        } 
    } else if dynamic_secrecy {
        init_code = quote::quote! {
            let block_secrecy: &::secret_structs::secret::DynLabel<Sec> = #expr1;
            let block_secrecy: &::secret_structs::secret::DynLabel<Sec> = unsafe { std::mem::transmute(block_secrecy) };
        };
        if partial_declassify {
            return_init_code = quote::quote! {
                let return_secrecy: &::secret_structs::secret::DynLabel<Sec> = #expr3;
                let return_secrecy: &::secret_structs::secret::DynLabel<Sec> = unsafe { std::mem::transmute(return_secrecy) };
            };
        } else {
            return_init_code = quote::quote! {};
        }
    } else if dynamic_integrity {
        init_code = quote::quote! {
            let block_integrity: &::secret_structs::secret::DynLabel<Int> = #expr2;
            let block_integrity: &::secret_structs::secret::DynLabel<Int> = unsafe { std::mem::transmute(block_integrity) };
        };
        if partial_declassify {
            return_init_code = quote::quote!{
                let return_integrity: &::secret_structs::secret::DynLabel<Int> = #expr4;
                let return_integrity: &::secret_structs::secret::DynLabel<Int> = unsafe { std::mem::transmute(return_integrity) };
            };
        } else {
            return_init_code = quote::quote! {};
        }
    } else {
        init_code = quote::quote!{};
        return_init_code = quote::quote!{};
    }
    quote::quote!{
        #init_code
        #return_init_code
    }.into()
}

fn block_return_check_code(dynamic_secrecy: bool, dynamic_integrity: bool, partial_declassify: bool) -> TokenStream {
    let return_check_code: proc_macro2::TokenStream;
    if dynamic_secrecy && dynamic_integrity {
        if partial_declassify {
            return_check_code = quote::quote! {
                if unsafe { !(return_value.get_dyn_sec_label_ref().is_equal_to(return_secrecy)) } {
                    panic!("Returning a value of different secrecy than the block.");
                }
                if unsafe { !(return_value.get_dyn_int_label_ref().is_equal_to(return_integrity)) } {
                    panic!("Returning a value of different integrity than the block.");
                }
            };
        } else {
            return_check_code = quote::quote! {
                if unsafe { !(return_value.get_dyn_sec_label_ref().is_equal_to(block_secrecy)) } {
                    panic!("Returning a value of different secrecy than the block.");
                }
                if unsafe { !(return_value.get_dyn_int_label_ref().is_equal_to(block_integrity)) } {
                    panic!("Returning a value of different integrity than the block.");
                }
            };
        }
    } else if dynamic_secrecy {
        if partial_declassify {
            return_check_code = quote::quote! {
                if unsafe { !(return_value.get_dyn_sec_label_ref().is_equal_to(return_secrecy)) } {
                    panic!("Returning a value of different secrecy than the block.");
                }
            };
        } else {
            return_check_code = quote::quote! {
                if unsafe { !(return_value.get_dyn_sec_label_ref().is_equal_to(block_secrecy)) } {
                    panic!("Returning a value of different secrecy than the block.");
                }
            };
        }
    } else if dynamic_integrity {
        if partial_declassify {
            return_check_code = quote::quote! {
                if unsafe { !(return_value.get_dyn_int_label_ref().is_equal_to(return_integrity)) } {
                    panic!("Returning a value of different integrity than the block.");
                }
            };
        } else {
            return_check_code = quote::quote! {
                if unsafe { !(return_value.get_dyn_int_label_ref().is_equal_to(block_integrity)) } {
                    panic!("Returning a value of different integrity than the block.");
                }
            };
        }
    } else {
        return_check_code = quote::quote!{};
    }
    return_check_code.into()
}

fn info_flow_block_backend_helper(input: TokenStream, is_verbose: bool, dynamic_secrecy: bool, dynamic_integrity: bool, does_return: bool, wrap_return: bool, partial_declassify: bool) -> TokenStream {
    //let ast_returned: syn::ExprClosure = syn::parse(input.clone()).unwrap();
    let ast: syn::ExprClosure = syn::parse(input).expect("Input not parsed ERROR");
    let params = ast.inputs;
    assert!(params.is_empty());
    let ret = ast.output;
    let info_flow_label = match ret {
        syn::ReturnType::Default => panic!("Unsupported"), // Even if the secret block has no return type, the input closure still must have a return type to indicate the secrecy label
        syn::ReturnType::Type(_, t) => *t,
    };
    let label_elems = match info_flow_label {
        syn::Type::Tuple(t) => t.elems,
        _ => panic!("Unsupported") // Even if the secret block has no return type for either integrity or secrecy, the input closure still must have both return types to indicate the secrecy and integrity labels
    };
    //let label_punctuated = (info_flow_label as syn::TypeTuple).elems;
    let mut iterator = label_elems.iter();
    let secrecy_label_option = iterator.next();
    let integrity_label_option = iterator.next();
    let secrecy_label = match secrecy_label_option {
        Option::Some(t) => Option::Some((*t).clone()),
        Option::None => panic!("Unsupported")
    };
    let integrity_label = match integrity_label_option {
        Option::Some(t) => Option::Some((*t).clone()),
        Option::None => panic!("Unsupported")
    };
    assert!(secrecy_label != integrity_label);
    //let secrecy_label_returned = secrecy_label.clone();
    //let secrecy_label = Option::Some(ret_simple);
    let body: proc_macro2::TokenStream = if dynamic_secrecy && dynamic_integrity {
        if partial_declassify {
            let declassify_check_code: proc_macro2::TokenStream = quote::quote! {
                if unsafe { !(block_secrecy.set_minus(return_secrecy).subset_of(&::secret_structs::secret::PRINCIPAL_REMOVE_SECRECY)) } {
                    panic!("Principal doesn't have the capability to declassify that much.");
                }
                if unsafe { !(block_integrity.set_minus(return_integrity).subset_of(&::secret_structs::secret::PRINCIPAL_REMOVE_INTEGRITY)) } {
                    panic!("Principal doesn't have the capability to endorse that much.");
                }
            };
            let new_secrecy_label_option = iterator.next();
            let new_integrity_label_option = iterator.next();
            let new_secrecy_label = match new_secrecy_label_option {
                Option::Some(t) => Option::Some((*t).clone()),
                Option::None => panic!("Unsupported")
            };
            let new_integrity_label = match new_integrity_label_option {
                Option::Some(t) => Option::Some((*t).clone()),
                Option::None => panic!("Unsupported")
            };
            match &*ast.body {
                syn::Expr::Block(b) => {
                    let block_body: proc_macro2::TokenStream = expand_and_check_block(&b.block, &secrecy_label, &integrity_label, &new_secrecy_label, &new_integrity_label, true, is_verbose, dynamic_secrecy, dynamic_integrity, does_return, wrap_return, false, false).into();
                    quote::quote! {
                        #declassify_check_code
                        #block_body
                    }
                }
                _ => {
                    let expr_body: proc_macro2::TokenStream = expand_and_check_expr(&*ast.body, &secrecy_label, &integrity_label, &new_secrecy_label, &new_integrity_label, true, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, false, false);
                    quote::quote! { 
                        #declassify_check_code
                        #expr_body
                    }
                }
            }
        } else {
            let declassify_check_code: proc_macro2::TokenStream = if does_return && !wrap_return {
                quote::quote! {
                    if unsafe { !block_secrecy.subset_of(&::secret_structs::secret::PRINCIPAL_REMOVE_SECRECY) } {
                        panic!("Principal doesn't have the capability to declassify that much.");
                    }
                    if unsafe { !block_integrity.subset_of(&::secret_structs::secret::PRINCIPAL_REMOVE_INTEGRITY) } {
                        panic!("Principal doesn't have the capability to endorse that much.");
                    }
                    //TODO:Figure out if any integrity checks can work here
                }
            } else {
                quote::quote! {

                }
            };
            match &*ast.body {
                syn::Expr::Block(b) => {
                    let block_body: proc_macro2::TokenStream = expand_and_check_block(&b.block, &secrecy_label, &integrity_label, &None, &None, false, is_verbose, dynamic_secrecy, dynamic_integrity, does_return, wrap_return, false, false).into();
                    quote::quote! {
                        #declassify_check_code
                        #block_body
                    }
                }
                _ => {
                    let expr_body: proc_macro2::TokenStream = expand_and_check_expr(&*ast.body, &secrecy_label, &integrity_label, &None, &None, false, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, false, false);
                    quote::quote! { 
                        #declassify_check_code
                        #expr_body
                    }
                }
            }
        }
    } else if dynamic_secrecy {
        if partial_declassify {
            let declassify_check_code: proc_macro2::TokenStream = quote::quote! {
                if unsafe { !(block_secrecy.set_minus(return_secrecy).subset_of(&::secret_structs::secret::PRINCIPAL_REMOVE_SECRECY)) } {
                    panic!("Principal doesn't have the capability to declassify that much.");
                }
            };
            let new_secrecy_label_option = iterator.next();
            let new_integrity_label_option = iterator.next();
            let new_secrecy_label = match new_secrecy_label_option {
                Option::Some(t) => Option::Some((*t).clone()),
                Option::None => panic!("Unsupported")
            };
            let new_integrity_label = match new_integrity_label_option {
                Option::Some(t) => Option::Some((*t).clone()),
                Option::None => panic!("Unsupported")
            };
            match &*ast.body {
                syn::Expr::Block(b) => {
                    let block_body: proc_macro2::TokenStream = expand_and_check_block(&b.block, &secrecy_label, &integrity_label, &new_secrecy_label, &new_integrity_label, true, is_verbose, dynamic_secrecy, dynamic_integrity, does_return, wrap_return, false, false).into();
                    quote::quote! {
                        #declassify_check_code
                        #block_body
                    }
                }
                _ => {
                    let expr_body: proc_macro2::TokenStream = expand_and_check_expr(&*ast.body, &secrecy_label, &integrity_label, &new_secrecy_label, &new_integrity_label, true, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, false, false);
                    quote::quote! { 
                        #declassify_check_code
                        #expr_body
                    }
                }
            }
        } else {
            let declassify_check_code: proc_macro2::TokenStream = if does_return && !wrap_return {
                quote::quote! {
                    if unsafe { !block_secrecy.subset_of(&::secret_structs::secret::PRINCIPAL_REMOVE_SECRECY) } {
                        panic!("Principal doesn't have the capability to declassify that much.");
                    }
                }
            } else {
                quote::quote! {

                }
            };
            match &*ast.body {
                syn::Expr::Block(b) => {
                    let block_body: proc_macro2::TokenStream = expand_and_check_block(&b.block, &secrecy_label, &integrity_label, &None, &None, false, is_verbose, dynamic_secrecy, dynamic_integrity, does_return, wrap_return, false, false).into();
                    quote::quote! {
                        #declassify_check_code
                        #block_body
                    }
                }
                _ => {
                    let expr_body: proc_macro2::TokenStream = expand_and_check_expr(&*ast.body, &secrecy_label, &integrity_label, &None, &None, false, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, false, false);
                    quote::quote! { 
                        #declassify_check_code
                        #expr_body
                    }
                }
            }
        }
    } else if dynamic_integrity {
        if partial_declassify {
            let declassify_check_code: proc_macro2::TokenStream = quote::quote! {
                if unsafe { !(block_integrity.set_minus(return_integrity).subset_of(&::secret_structs::secret::PRINCIPAL_REMOVE_INTEGRITY)) } {
                    panic!("Principal doesn't have the capability to endorse that much.");
                }
            };
            let new_secrecy_label_option = iterator.next();
            let new_integrity_label_option = iterator.next();
            let new_secrecy_label = match new_secrecy_label_option {
                Option::Some(t) => Option::Some((*t).clone()),
                Option::None => panic!("Unsupported")
            };
            let new_integrity_label = match new_integrity_label_option {
                Option::Some(t) => Option::Some((*t).clone()),
                Option::None => panic!("Unsupported")
            };
            match &*ast.body {
                syn::Expr::Block(b) => {
                    let block_body: proc_macro2::TokenStream = expand_and_check_block(&b.block, &secrecy_label, &integrity_label, &new_secrecy_label, &new_integrity_label, true, is_verbose, dynamic_secrecy, dynamic_integrity, does_return, wrap_return, false, false).into();
                    quote::quote! {
                        #declassify_check_code
                        #block_body
                    }
                }
                _ => {
                    let expr_body: proc_macro2::TokenStream = expand_and_check_expr(&*ast.body, &secrecy_label, &integrity_label, &new_secrecy_label, &new_integrity_label, true, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, false, false);
                    quote::quote! { 
                        #declassify_check_code
                        #expr_body
                    }
                }
            }
        } else {
            let declassify_check_code: proc_macro2::TokenStream = if does_return && !wrap_return {
                quote::quote! {
                    if unsafe { !block_integrity.subset_of(&::secret_structs::secret::PRINCIPAL_REMOVE_INTEGRITY) } {
                        panic!("Principal doesn't have the capability to endorse that much.");
                    }
                }
            } else {
                quote::quote! {

                }
            };
            match &*ast.body {
                syn::Expr::Block(b) => {
                    let block_body: proc_macro2::TokenStream = expand_and_check_block(&b.block, &secrecy_label, &integrity_label, &None, &None, false, is_verbose, dynamic_secrecy, dynamic_integrity, does_return, wrap_return, false, false).into();
                    quote::quote! {
                        #declassify_check_code
                        #block_body
                    }
                }
                _ => {
                    let expr_body: proc_macro2::TokenStream = expand_and_check_expr(&*ast.body, &secrecy_label, &integrity_label, &None, &None, false, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, false, false);
                    quote::quote! { 
                        #declassify_check_code
                        #expr_body
                    }
                }
            }
        }
    } else {
        if partial_declassify {
            let new_secrecy_label_option = iterator.next();
            let new_integrity_label_option = iterator.next();
            let new_secrecy_label = match new_secrecy_label_option {
                Option::Some(t) => Option::Some((*t).clone()),
                Option::None => panic!("Unsupported")
            };
            let new_integrity_label = match new_integrity_label_option {
                Option::Some(t) => Option::Some((*t).clone()),
                Option::None => panic!("Unsupported")
            };
            match &*ast.body {
                syn::Expr::Block(b) => {
                    let block_body: proc_macro2::TokenStream = expand_and_check_block(&b.block, &secrecy_label, &integrity_label, &new_secrecy_label, &new_integrity_label, true, is_verbose, dynamic_secrecy, dynamic_integrity, does_return, wrap_return, false, false).into();
                    quote::quote! {
                        #block_body
                    }
                }
                _ => {
                    let expr_body: proc_macro2::TokenStream = expand_and_check_expr(&*ast.body, &secrecy_label, &integrity_label, &new_secrecy_label, &new_integrity_label, true, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, false, false);
                    quote::quote! {
                        #expr_body
                    }
                }
            }
        } else {
            match &*ast.body {
                syn::Expr::Block(b) => {
                    let block_body: proc_macro2::TokenStream = expand_and_check_block(&b.block, &secrecy_label, &integrity_label, &None, &None, false, is_verbose, dynamic_secrecy, dynamic_integrity, does_return, wrap_return, false, false).into();
                    quote::quote! {
                        #block_body
                    }
                }
                _ => {
                    let expr_body: proc_macro2::TokenStream = expand_and_check_expr(&*ast.body, &secrecy_label, &integrity_label, &None, &None, false, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, false, false);
                    quote::quote! { 
                        #expr_body
                    }
                }
            }
        }
    };

    let gen = quote::quote! {
        (|| -> _ { #body })
    };
    gen.into()
}

// Returns if a string is an allow-listed function.
fn is_allowlisted_function(string: &String) -> bool {
    let allowed_functions = HashSet::from([
        "<[_]>::copy_from_slice".to_string(),
        "<[_]>::iter".to_string(),
        "<[_]>::len".to_string(),
        "<[_]>::into_vec".to_string(),
        "<[u8]>::sort".to_string(),
        "<[u8]>::to_owned".to_string(),
        "<[u8;16]>::clone".to_string(),

        "core::iter::Cycle::<std::iter::Copied<std::slice::Iter<u8>>>::next".to_string(),

        "core::primitive::char::is_alphabetic".to_string(),
        "core::primitive::char::is_ascii_whitespace".to_string(),
        "core::primitive::char::is_digit".to_string(),
        "core::primitive::char::is_numeric".to_string(),
        "core::primitive::char::len_utf8".to_string(),
        "core::primitive::char::len_utf16".to_string(),

        "core::primitive::f32::clamp".to_string(),
        "core::primitive::f32::clamp_length_max".to_string(),
        "core::primitive::f32::is_finite".to_string(),
        "core::primitive::f32::powi".to_string(),

        "core::primitive::f64::ceil".to_string(),
        "core::primitive::f64::fract".to_string(),
        "core::primitive::f64::is_infinite".to_string(),
        "core::primitive::f64::is_nan".to_string(),
        "core::primitive::f64::round".to_string(),
        "core::primitive::f64::sqrt".to_string(),
        "core::primitive::f64::to_string".to_string(),

        "core::primitive::i32::clone".to_string(),
        "core::primitive::i32::div_euclid".to_string(),
        "core::primitive::i32::rem_euclid".to_string(),

        "core::primitive::i64::div_euclid".to_string(),
        "core::primitive::i64::rem_euclid".to_string(),

        "core::primitive::str::as_bytes".to_string(),
        "core::primitive::str::char_indices".to_string(),
        "core::primitive::str::chars".to_string(),
        "core::primitive::str::contains".to_string(),
        "core::primitive::str::ends_with".to_string(),
        "core::primitive::str::find".to_string(),
        "core::primitive::str::len".to_string(),
        "core::primitive::str::make_ascii_lowercase".to_string(),
        "core::primitive::str::parse::<i32>".to_string(),
        "core::primitive::str::parse::<u32>".to_string(),
        "core::primitive::str::parse::<f64>".to_string(),
        "core::primitive::str::replace".to_string(),
        "core::primitive::str::split".to_string(),
        "core::primitive::str::split_at".to_string(),
        "core::primitive::str::split_word_bounds".to_string(),
        "core::primitive::str::starts_with".to_string(),
        "core::primitive::str::to_string".to_string(),
        "core::primitive::str::trim".to_string(),
        "core::primitive::str::trim_end_matches".to_string(),

        "core::primitive::u16::min".to_string(),

        "core::primitive::u32::is_power_of_two".to_string(),
        
        "core::primitive::u128::min".to_string(),

        "core::primitive::usize::min".to_string(),
        "core::primitive::usize::to_string".to_string(),

        "secret_structs::secret::SecureValue::get_dyn_int_label".to_string(),
        "secret_structs::secret::SecureValue::get_dyn_sec_label".to_string(),

        /*"secret_structs::secret::SafeAdd::safe_add".to_string(),
        "secret_structs::secret::SafeAddAssign::safe_add_assign".to_string(),
        "secret_structs::secret::SafeDiv::safe_div".to_string(),
        "secret_structs::secret::SafeDivAssign::safe_div_assign".to_string(),
        "secret_structs::secret::SafeIndex::safe_index".to_string(),
        "secret_structs::secret::SafeIndexMut::safe_index_mut".to_string(),
        "secret_structs::secret::SafeMul::safe_mul".to_string(),
        "secret_structs::secret::SafeMulAssign::safe_mul_assign".to_string(),
        "secret_structs::secret::SafeNeg::safe_neg".to_string(),
        "secret_structs::secret::SafeNot::safe_not".to_string(),
        "secret_structs::secret::SafePartialEq::safe_eq".to_string(),
        "secret_structs::secret::SafePartialEq::safe_ne".to_string(),
        "secret_structs::secret::SafePartialOrd::safe_ge".to_string(),
        "secret_structs::secret::SafePartialOrd::safe_gt".to_string(),
        "secret_structs::secret::SafePartialOrd::safe_le".to_string(),
        "secret_structs::secret::SafePartialOrd::safe_lt".to_string(),
        "secret_structs::secret::SafeSub::safe_sub".to_string(),
        "secret_structs::secret::SafeSubAssign::safe_sub_assign".to_string(),*/
        
        "std::arch::x86_64::_mm_rsqrt_ps".to_string(),
        "std::arch::x86_64::_mm256_add_pd".to_string(),
        "std::arch::x86_64::_mm256_cvtpd_ps".to_string(),
        "std::arch::x86_64::_mm256_cvtps_pd".to_string(),
        "std::arch::x86_64::_mm256_hadd_pd".to_string(),
        "std::arch::x86_64::_mm256_mul_pd".to_string(),
        "std::arch::x86_64::_mm256_permute2f128_pd".to_string(),
        "std::arch::x86_64::_mm256_set1_pd".to_string(),
        "std::arch::x86_64::_mm256_setr_pd".to_string(),
        "std::arch::x86_64::_mm256_store_pd".to_string(),
        "std::arch::x86_64::_mm256_sub_pd".to_string(),

        "std::boxed::Box::new".to_string(),

        //"std::collections::HashMap::get".to_string(),
        "std::collections::HashMap::insert".to_string(),
        "std::collections::HashMap::contains_key".to_string(),

        "std::fs::File::open".to_string(),

        "std::iter::Copied::cycle".to_string(),

        "std::mem::MaybeUninit::assume_init".to_string(), //TODO: Remove this?
        "std::mem::MaybeUninit::uninit".to_string(), //TODO: Remove this?

        "std::mem::transmute".to_string(),

        "std::num::NonZeroI32::get".to_string(),

        "std::option::Option::Some".to_string(),
        "std::option::Option::expect".to_string(),
        "std::option::Option::unwrap".to_string(),
        "std::option::Option::unwrap_or".to_string(),
        "std::option::Option::ok_or".to_string(),

        "std::result::Result::Ok".to_string(),
        "std::result::Result::unwrap".to_string(),
        "std::result::Result::Err".to_string(),
        "std::result::Result::is_err".to_string(),
        "std::result::Result::map_err".to_string(),

        "std::slice::Iter::by_ref".to_string(),
        "std::slice::Iter::copied".to_string(),
        "std::slice::Iter::take".to_string(),
        "<&mut std::slice::Iter::<u8>>::take".to_string(),

        "std::str::Chars::all".to_string(),
        "std::str::Chars::any".to_string(),
        "std::str::Chars::count".to_string(),
        "std::str::Chars::fold".to_string(),
        "std::str::Chars::map".to_string(),
        "std::str::Chars::next".to_string(),

        "str::Split::nth".to_string(),
        "str::Split::next".to_string(),

        "std::string::String::as_str".to_string(),
        "std::string::String::clear".to_string(),
        "std::string::String::clone".to_string(),
        "std::string::String::from".to_string(),
        "std::string::String::from_utf8_unchecked".to_string(),
        "std::string::String::into_bytes".to_string(),
        "std::string::String::is_empty".to_string(),
        "std::string::String::len".to_string(),
        "std::string::String::new".to_string(),
        "std::string::String::push".to_string(),
        "std::string::String::push_str".to_string(),
        "std::string::String::replace_range".to_string(),
        "std::string::String::retain".to_string(),
        "std::string::String::split".to_string(),
        "std::string::String::truncate".to_string(),

        "std::vec::from_elem".to_string(),
        "std::vec::Vec::append".to_string(),
        "std::vec::Vec::clear".to_string(),
        "std::vec::Vec::clone".to_string(),
        "std::vec::Vec::extend_from_slice".to_string(),
        "std::vec::Vec::insert".to_string(),
        "std::vec::Vec::iter".to_string(),
        "std::vec::Vec::len".to_string(),
        "std::vec::Vec::new".to_string(),
        "std::vec::Vec::pop".to_string(),
        "std::vec::Vec::push".to_string(),
        "std::vec::Vec::with_capacity".to_string(),

        // Add other allowed functions here.
    ]);
    allowed_functions.contains(string)
}

// Returns if the path is to an allow-listed function.
fn is_path_to_allowlisted_function(path_expr: &syn::ExprPath) -> bool {
    let mut path_str = quote::quote! {#path_expr}.to_string();
    path_str.retain(|c| !c.is_whitespace());
    is_allowlisted_function(&path_str)
}

// Returns if the function call is allow-listed.
fn is_call_to_allowlisted_function(call: &syn::ExprCall) -> bool {
    //println!("\n\nSPECIAL STRING {}", call.to_token_stream());
    if let syn::Expr::Path(path_expr) = &*call.func {
        is_path_to_allowlisted_function(&path_expr)
    } else {
        false
    }
}

// Returns whether the function call is a specific function.
fn is_call_to(call: &syn::ExprCall, path: &str) -> bool {
    if let syn::Expr::Path(path_expr) = &*call.func {
        let mut path_str = quote::quote! {#path_expr}.to_string();
        path_str.retain(|c| !c.is_whitespace());
        return path_str == path;
    } else {
        false
    }
}

fn is_macro_call_to(call: &syn::ExprMacro, path_comparison: &str) -> bool {
    let syn::Macro{path, bang_token: _, delimiter: _, tokens: _} = &call.mac;
    let mut path_str = quote::quote! {#path}.to_string();
    path_str.retain(|c| !c.is_whitespace());
    return path_str == path_comparison;
}

fn expand_and_check_expr(expr: &syn::Expr, secrecy_label: &Option<syn::Type>, integrity_label: &Option<syn::Type>, sec_label_2: &Option<syn::Type>, int_label_2: &Option<syn::Type>, partial_declassify: bool, is_verbose: bool, dynamic_secrecy: bool, dynamic_integrity: bool, do_sbs_check: bool, does_return: bool, wrap_return: bool, is_func: bool, strip_unchecked_only: bool) -> proc_macro2::TokenStream {
    match expr {
        syn::Expr::Array(array_exp) => {
            let elements = comma_separate(
                array_exp.elems.iter().map(|expr| expand_and_check_expr(expr, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only))
            );
            quote::quote!{
                [#elements]
            }
        }
        syn::Expr::Break(expr_break) => expr_break.into_token_stream(),
        syn::Expr::Call(expr_call) => {
            let args = comma_separate(expr_call.args.iter().map(
                |arg: &syn::Expr| -> proc_macro2::TokenStream { expand_and_check_expr(arg, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only) },
            ));
            if strip_unchecked_only {
                if is_call_to(expr_call, "unchecked_operation") {
                    if expr_call.args.len() != 1 {
                        quote::quote!{compile_error!("unchecked_operation takes a single expression as an argument.");}
                    } else {
                        //let unchecked_increment = generate_increment(quote::quote!{UNCHECKED_COUNTER});
                        let expr = expr_call.args.iter().nth(0).unwrap();
                        //TODO: Look at adding braces around this expression: example: let a = unchecked_operation(b);
                        quote::quote! {
                            //#unchecked_increment
                            #expr
                        }
                    }
                } else if is_call_to(expr_call, "explicit_allowlisted") {
                    if expr_call.args.len() != 1 {
                        quote::quote! {compile_error!("explicit_allowlisted takes a single function call as an argument.");}
                    } else {
                        let call = expr_call.args.iter().nth(0).unwrap();
                        match call {
                            syn::Expr::Call(call) => {
                                let args = comma_separate(call.args.iter().map(
                                    |arg: &syn::Expr| -> proc_macro2::TokenStream { expand_and_check_expr(arg, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only) },
                                ));
                                let func = *call.func.clone();
                                quote::quote! { #func(#args) }
                            },
                            _ => {
                                quote::quote! {compile_error!("explicit_allowlisted only takes a single function call.");}
                            }
                        }
                    }
                } else {
                    let call = *expr_call.func.clone();
                    quote::quote! { #call(#args) }
                }
            } else {
                if is_call_to(expr_call, "unwrap_ref") && secrecy_label.is_some() /* OPTIMIZE */ {
                    if !is_func {
                        let sec_label = Some(secrecy_label).expect("ErrA");
                        let int_label = Some(integrity_label).expect("ErrB");
                        if is_verbose {
                            let unwrap_increment = generate_increment(quote::quote!{UNWRAP_COUNTER});
                            quote::quote! {
                                {
                                    #unwrap_increment
                                    unsafe { ::secret_structs::secret::SecureValue::unwrap_unsafe2::<#sec_label, #int_label>(#args) } 
                                }
                            }
                        } else {
                            if dynamic_secrecy && dynamic_integrity {
                                let unwrap_increment = generate_increment(quote::quote!{UNWRAP_COUNTER});
                                quote::quote! {
                                    {
                                        #unwrap_increment
                                        fn __unwrap_fn<T: ::secret_structs::secret::SecureValueSafe, L1, L2, D1: ::secret_structs::secret::DynField<::secret_structs::secret::Sec>, D2: ::secret_structs::secret::DynField<::secret_structs::secret::Int>>(secure: &::secret_structs::secret::SecureValue<T, L1, L2, D1, D2>, block_secrecy: &::secret_structs::secret::DynLabel<::secret_structs::secret::Sec>, block_integrity: &::secret_structs::secret::DynLabel<::secret_structs::secret::Int>) {
                                            if !(unsafe{secure.get_dyn_sec_label_ref().subset_of(block_secrecy)}) {
                                                panic!("Principal tried to unwrap high secrecy in a low secrecy block.");
                                            }
                                            if !(unsafe{secure.get_dyn_int_label_ref().subset_of(block_integrity)}) {
                                                panic!("A Principal tried to unwrap low integrity in a high integrity block.");
                                            }
                                        }
                                        let __closure = |secure| {
                                            __unwrap_fn(secure, block_secrecy, block_integrity);
                                            unsafe { ::secret_structs::secret::SecureValue::unwrap_unsafe2::<#sec_label, #int_label>(secure) }
                                        };
                                        __closure(#args)
                                    }
                                }
                            } else if dynamic_secrecy {
                                let unwrap_increment = generate_increment(quote::quote!{UNWRAP_COUNTER});
                                quote::quote! {
                                    {
                                        #unwrap_increment
                                        fn __unwrap_fn<T: ::secret_structs::secret::SecureValueSafe, L1, L2, D1: ::secret_structs::secret::DynField<::secret_structs::secret::Sec>, D2: ::secret_structs::secret::DynField<::secret_structs::secret::Int>>(secure: &::secret_structs::secret::SecureValue<T, L1, L2, D1, D2>, block_secrecy: &::secret_structs::secret::DynLabel<::secret_structs::secret::Sec>) {
                                            if !(unsafe{secure.get_dyn_sec_label_ref().subset_of(block_secrecy)}) {
                                                panic!("Principal tried to unwrap high secrecy in a low secrecy block.");
                                            }
                                        }
                                        let __closure = |secure| {
                                            __unwrap_fn(secure, block_secrecy);
                                            unsafe { ::secret_structs::secret::SecureValue::unwrap_unsafe2::<#sec_label, #int_label>(secure) }
                                        };
                                        __closure(#args)
                                    }
                                }
                            } else if dynamic_integrity {
                                let unwrap_increment = generate_increment(quote::quote!{UNWRAP_COUNTER});
                                quote::quote! {
                                    {
                                        #unwrap_increment
                                        fn __unwrap_fn<T: ::secret_structs::secret::SecureValueSafe, L1, L2, D1: ::secret_structs::secret::DynField<::secret_structs::secret::Sec>, D2: ::secret_structs::secret::DynField<::secret_structs::secret::Int>>(secure: &::secret_structs::secret::SecureValue<T, L1, L2, D1, D2>, block_integrity: &::secret_structs::secret::DynLabel<::secret_structs::secret::Int>) {
                                            if !(unsafe{secure.get_dyn_int_label_ref().subset_of(block_integrity)}) {
                                                panic!("A Principal tried to unwrap low integrity in a high integrity block.");
                                            }
                                        }
                                        let __closure = |secure| {
                                            __unwrap_fn(secure, block_integrity);
                                            unsafe { ::secret_structs::secret::SecureValue::unwrap_unsafe2::<#sec_label, #int_label>(secure) }
                                        };
                                        __closure(#args)
                                    }
                                }
                            } else {
                                let unwrap_increment = generate_increment(quote::quote!{UNWRAP_COUNTER});
                                quote::quote! {
                                    {
                                        #unwrap_increment
                                        let __closure = |secure| {
                                            unsafe { ::secret_structs::secret::SecureValue::unwrap_unsafe2::<#sec_label, #int_label>(secure) }
                                        };
                                        __closure(#args)
                                    }
                                }
                            }
                        }
                    } else {
                        let unwrap_increment = generate_increment(quote::quote!{UNWRAP_COUNTER});
                        quote::quote! {
                            {
                                #unwrap_increment
                                unwrap_ref(#args)
                            }
                        }
                    }
                } else if is_call_to(expr_call, "unwrap_mut_ref") && secrecy_label.is_some() /* OPTIMIZE */ { 
                    if !is_func {
                        let sec_label = Some(secrecy_label).expect("ErrC");
                        let int_label = Some(integrity_label).expect("ErrD");
                        if is_verbose {
                            let unwrap_increment = generate_increment(quote::quote!{UNWRAP_COUNTER});
                            quote::quote! {
                                {
                                    #unwrap_increment
                                    unsafe { ::secret_structs::secret::SecureValue::unwrap_mut_unsafe2::<#sec_label, #int_label>(#args) } 
                                }
                            }
                        } else {
                            if dynamic_secrecy && dynamic_integrity {
                                let unwrap_increment = generate_increment(quote::quote!{UNWRAP_COUNTER});
                                quote::quote! {
                                    {
                                        #unwrap_increment
                                        fn __unwrap_mut_fn<'__a, T: ::secret_structs::secret::SecureValueSafe, L1, L2, D1: ::secret_structs::secret::DynField<::secret_structs::secret::Sec>, D2: ::secret_structs::secret::DynField<::secret_structs::secret::Int>>(secure: &'__a mut ::secret_structs::secret::SecureValue<T, L1, L2, D1, D2>, block_secrecy: &::secret_structs::secret::DynLabel<::secret_structs::secret::Sec>, block_integrity: &::secret_structs::secret::DynLabel<::secret_structs::secret::Int>) -> &'__a mut ::secret_structs::secret::SecureValue<T, L1, L2, D1, D2>{
                                            if !(unsafe{secure.get_dyn_sec_label_ref().is_equal_to(block_secrecy)}) {
                                                panic!("Principal tried to unwrap high secrecy in a low secrecy block.");
                                            }
                                            if !(unsafe{secure.get_dyn_int_label_ref().is_equal_to(block_integrity)}) {
                                                panic!("A Principal tried to unwrap low integrity in a high integrity block.");
                                            }
                                            secure
                                        }
                                        let __closure = |secure| {
                                            let secure = __unwrap_mut_fn(secure, block_secrecy, block_integrity);
                                            unsafe { ::secret_structs::secret::SecureValue::unwrap_mut_unsafe2::<#sec_label, #int_label>(secure) }
                                        };
                                        __closure(#args)
                                    }
                                }
                            } else if dynamic_secrecy {
                                let unwrap_increment = generate_increment(quote::quote!{UNWRAP_COUNTER});
                                quote::quote! {
                                    {
                                        #unwrap_increment
                                        fn __unwrap_mut_fn<'__a, T: ::secret_structs::secret::SecureValueSafe, L1, L2, D1: ::secret_structs::secret::DynField<::secret_structs::secret::Sec>, D2: ::secret_structs::secret::DynField<::secret_structs::secret::Int>>(secure: &'__a mut ::secret_structs::secret::SecureValue<T, L1, L2, D1, D2>, block_secrecy: &::secret_structs::secret::DynLabel<::secret_structs::secret::Sec>) -> &'__a mut ::secret_structs::secret::SecureValue<T, L1, L2, D1, D2>{
                                            if !(unsafe{secure.get_dyn_sec_label_ref().is_equal_to(block_secrecy)}) {
                                                panic!("Principal tried to unwrap high secrecy in a low secrecy block.");
                                            }
                                            secure
                                        }
                                        let __closure = |secure| {
                                            let secure = __unwrap_mut_fn(secure, block_secrecy);
                                            unsafe { ::secret_structs::secret::SecureValue::unwrap_mut_unsafe2::<#sec_label, #int_label>(secure) }
                                        };
                                        __closure(#args)
                                    }
                                }
                            } else if dynamic_integrity {
                                let unwrap_increment = generate_increment(quote::quote!{UNWRAP_COUNTER});
                                quote::quote! {
                                    {
                                        #unwrap_increment
                                        fn __unwrap_mut_fn<'__a, T: ::secret_structs::secret::SecureValueSafe, L1, L2, D1: ::secret_structs::secret::DynField<::secret_structs::secret::Sec>, D2: ::secret_structs::secret::DynField<::secret_structs::secret::Int>>(secure: &'__a mut ::secret_structs::secret::SecureValue<T, L1, L2, D1, D2>, block_integrity: &::secret_structs::secret::DynLabel<::secret_structs::secret::Int>) -> &'__a mut ::secret_structs::secret::SecureValue<T, L1, L2, D1, D2>{
                                            if !(unsafe{secure.get_dyn_int_label_ref().is_equal_to(block_integrity)}) {
                                                panic!("A Principal tried to unwrap low integrity in a high integrity block.");
                                            }
                                            secure
                                        }
                                        let __closure = |secure| {
                                            let secure = __unwrap_mut_fn(secure, block_integrity);
                                            unsafe { ::secret_structs::secret::SecureValue::unwrap_mut_unsafe2::<#sec_label, #int_label>(secure) }
                                        };
                                        __closure(#args)
                                    }
                                }
                            } else {
                                let unwrap_increment = generate_increment(quote::quote!{UNWRAP_COUNTER});
                                quote::quote! {
                                    {
                                        #unwrap_increment
                                        let __closure = |secure| {
                                            unsafe { ::secret_structs::secret::SecureValue::unwrap_mut_unsafe2::<#sec_label, #int_label>(secure) }
                                        };
                                        __closure(#args)
                                    }
                                }
                            }
                        }
                    } else {
                        let unwrap_increment = generate_increment(quote::quote!{UNWRAP_COUNTER});
                        return quote::quote! {
                            {
                                #unwrap_increment
                                unwrap_mut_ref(#args) 
                            }
                        }
                    }
                } else if is_call_to(expr_call, "unwrap") && secrecy_label.is_some() /* OPTIMIZE */ {
                    if !is_func {
                        let sec_label = Some(secrecy_label).expect("ErrE");
                        let int_label = Some(integrity_label).expect("ErrF");
                        if is_verbose {
                            let unwrap_increment = generate_increment(quote::quote!{UNWRAP_COUNTER});
                            quote::quote! {
                                {
                                    #unwrap_increment
                                    unsafe { ::secret_structs::secret::SecureValue::unwrap_consume_unsafe2::<#sec_label, #int_label>(#args) } 
                                }
                            }
                        } else {
                            if dynamic_secrecy && dynamic_integrity {
                                let unwrap_increment = generate_increment(quote::quote!{UNWRAP_COUNTER});
                                quote::quote! {
                                    {
                                        #unwrap_increment
                                        fn __unwrap_consume_fn<T: ::secret_structs::secret::SecureValueSafe, L1, L2, D1: ::secret_structs::secret::DynField<::secret_structs::secret::Sec>, D2: ::secret_structs::secret::DynField<::secret_structs::secret::Int>>(secure: ::secret_structs::secret::SecureValue<T, L1, L2, D1, D2>, block_secrecy: &::secret_structs::secret::DynLabel<::secret_structs::secret::Sec>, block_integrity: &::secret_structs::secret::DynLabel<::secret_structs::secret::Int>) -> ::secret_structs::secret::SecureValue<T, L1, L2, D1, D2>{
                                            if !(unsafe{secure.get_dyn_sec_label_ref().subset_of(block_secrecy)}) {
                                                panic!("Principal tried to unwrap high secrecy in a low secrecy block.");
                                            }
                                            if !(unsafe{secure.get_dyn_int_label_ref().subset_of(block_integrity)}) {
                                                panic!("C Principal tried to unwrap low integrity in a high integrity block.");
                                            }
                                            secure
                                        }
                                        let __closure = |secure| {
                                            let secure = __unwrap_consume_fn(secure, block_secrecy, block_integrity);
                                            unsafe { ::secret_structs::secret::SecureValue::unwrap_consume_unsafe2::<#sec_label, #int_label>(secure) }
                                        };
                                        __closure(#args)
                                    }
                                }
                            } else if dynamic_secrecy {
                                let unwrap_increment = generate_increment(quote::quote!{UNWRAP_COUNTER});
                                quote::quote! {
                                    {
                                        #unwrap_increment
                                        fn __unwrap_consume_fn<T: ::secret_structs::secret::SecureValueSafe, L1, L2, D1: ::secret_structs::secret::DynField<::secret_structs::secret::Sec>, D2: ::secret_structs::secret::DynField<::secret_structs::secret::Int>>(secure: ::secret_structs::secret::SecureValue<T, L1, L2, D1, D2>, block_secrecy: &::secret_structs::secret::DynLabel<::secret_structs::secret::Sec>) -> ::secret_structs::secret::SecureValue<T, L1, L2, D1, D2> {
                                            if !(unsafe{secure.get_dyn_sec_label_ref().subset_of(block_secrecy)}) {
                                                panic!("Principal tried to unwrap high secrecy in a low secrecy block.");
                                            }
                                            secure
                                        }
                                        let __closure = |secure| {
                                            let secure = __unwrap_consume_fn(secure, block_secrecy);
                                            unsafe { ::secret_structs::secret::SecureValue::unwrap_consume_unsafe2::<#sec_label, #int_label>(secure) }
                                        };
                                        __closure(#args)
                                    }
                                }
                            } else if dynamic_integrity {
                                let unwrap_increment = generate_increment(quote::quote!{UNWRAP_COUNTER});
                                quote::quote! {
                                    {
                                        #unwrap_increment
                                        fn __unwrap_consume_fn<T: ::secret_structs::secret::SecureValueSafe, L1, L2, D1: ::secret_structs::secret::DynField<::secret_structs::secret::Sec>, D2: ::secret_structs::secret::DynField<::secret_structs::secret::Int>>(secure: ::secret_structs::secret::SecureValue<T, L1, L2, D1, D2>, block_integrity: &::secret_structs::secret::DynLabel<::secret_structs::secret::Int>) -> ::secret_structs::secret::SecureValue<T, L1, L2, D1, D2>{
                                            if !(unsafe{secure.get_dyn_int_label_ref().subset_of(block_integrity)}) {
                                                panic!("C Principal tried to unwrap low integrity in a high integrity block.");
                                            }
                                            secure
                                        }
                                        let __closure = |secure| {
                                            let secure = __unwrap_consume_fn(secure, block_integrity);
                                            unsafe { ::secret_structs::secret::SecureValue::unwrap_consume_unsafe2::<#sec_label, #int_label>(secure) }
                                        };
                                        __closure(#args)
                                    }
                                }
                            } else {
                                let unwrap_increment = generate_increment(quote::quote!{UNWRAP_COUNTER});
                                quote::quote! {
                                    {
                                        #unwrap_increment
                                        let __closure = |secure| {
                                            unsafe { ::secret_structs::secret::SecureValue::unwrap_consume_unsafe2::<#sec_label, #int_label>(secure) }
                                        };
                                        __closure(#args)
                                    }
                                }
                            }
                        }
                    } else {
                        let unwrap_increment = generate_increment(quote::quote!{UNWRAP_COUNTER});
                        quote::quote! {
                            {
                                #unwrap_increment
                                unwrap(#args) 
                            }
                        }
                    }
                } else if is_call_to(expr_call, "wrap") && secrecy_label.is_some() /* OPTIMIZE */ {
                    if !is_func {
                        let sec_label = if partial_declassify {
                            Some(sec_label_2).expect("ErrG")
                        } else {
                            Some(secrecy_label).expect("ErrG")
                        };
                        let int_label = if partial_declassify {
                            Some(int_label_2).expect("ErrH")
                        } else {
                            Some(integrity_label).expect("ErrH")
                        };
                        if is_verbose {
                            if dynamic_secrecy && dynamic_integrity {
                                let wrap_increment = generate_increment(quote::quote!{WRAP_COUNTER});
                                let ret_sec_expr = if partial_declassify {
                                    quote::quote!{return_secrecy.clone()}
                                } else {
                                    quote::quote!{block_secrecy.clone()}
                                };
                                let ret_int_expr = if partial_declassify {
                                    quote::quote!{return_integrity.clone()}
                                } else {
                                    quote::quote!{block_integrity.clone()}
                                };
                                quote::quote! {
                                    {
                                        #wrap_increment
                                        unsafe { ::secret_structs::secret::SecureValue::<_,#sec_label,#int_label, ::secret_structs::secret::DynLabel<::secret_structs::secret::Sec>, ::secret_structs::secret::DynLabel<::secret_structs::secret::Int>>::new_info_flow_struct(#args, #ret_sec_expr, #ret_int_expr) }
                                    }
                                }
                            } else if dynamic_secrecy {
                                let wrap_increment = generate_increment(quote::quote!{WRAP_COUNTER});
                                let ret_sec_expr = if partial_declassify {
                                    quote::quote!{return_secrecy.clone()}
                                } else {
                                    quote::quote!{block_secrecy.clone()}
                                };
                                quote::quote! {
                                    {
                                        #wrap_increment
                                        unsafe { ::secret_structs::secret::StaticDynamicSecretStaticIntegrity::<_,#sec_label,#int_label, ::secret_structs::secret::DynLabel<::secret_structs::secret::Sec>>::new_static_dynamic_secret_static_integrity(#args, #ret_sec_expr) }
                                    }
                                }
                            } else if dynamic_integrity {
                                let wrap_increment = generate_increment(quote::quote!{WRAP_COUNTER});
                                let ret_int_expr = if partial_declassify {
                                    quote::quote!{return_integrity.clone()}
                                } else {
                                    quote::quote!{block_integrity.clone()}
                                };
                                quote::quote! {
                                    {
                                        #wrap_increment
                                        unsafe { ::secret_structs::secret::StaticSecretStaticDynamicIntegrity::<_,#sec_label,#int_label, ::secret_structs::secret::DynLabel<::secret_structs::secret::Int>>::new_static_secret_static_dynamic_integrity(#args, #ret_int_expr) }
                                    }
                                }
                            } else {
                                let wrap_increment = generate_increment(quote::quote!{WRAP_COUNTER});
                                quote::quote! {
                                    {
                                        #wrap_increment
                                        unsafe { ::secret_structs::secret::StaticAll::<_,#sec_label,#int_label>::new_static_all(#args) }
                                    }
                                }
                            }
                        } else {
                            if dynamic_secrecy && dynamic_integrity {
                                let wrap_increment = generate_increment(quote::quote!{WRAP_COUNTER});
                                let ret_sec_expr = if partial_declassify {
                                    quote::quote!{return_secrecy.clone()}
                                } else {
                                    quote::quote!{block_secrecy.clone()}
                                };
                                let ret_int_expr = if partial_declassify {
                                    quote::quote!{return_integrity.clone()}
                                } else {
                                    quote::quote!{block_integrity.clone()}
                                };
                                quote::quote! {
                                    {
                                        #wrap_increment
                                        let __closure = |val| {
                                            unsafe { ::secret_structs::secret::SecureValue::<_, #sec_label, #int_label, ::secret_structs::secret::DynLabel<::secret_structs::secret::Sec>, ::secret_structs::secret::DynLabel<::secret_structs::secret::Int>>::new_info_flow_struct(val, #ret_sec_expr, #ret_int_expr) }
                                        };
                                        __closure(#args)
                                    }
                                }
                            } else if dynamic_secrecy {
                                let wrap_increment = generate_increment(quote::quote!{WRAP_COUNTER});
                                let ret_sec_expr = if partial_declassify {
                                    quote::quote!{return_secrecy.clone()}
                                } else {
                                    quote::quote!{block_secrecy.clone()}
                                };
                                quote::quote! {
                                    {
                                        #wrap_increment
                                        let __closure = |val| {
                                            unsafe { ::secret_structs::secret::StaticDynamicSecretStaticIntegrity::<_, #sec_label, #int_label, ::secret_structs::secret::DynLabel<::secret_structs::secret::Sec>>::new_static_dynamic_secret_static_integrity(val, #ret_sec_expr) }
                                        };
                                        __closure(#args)
                                    }
                                }
                            } else if dynamic_integrity {
                                let wrap_increment = generate_increment(quote::quote!{WRAP_COUNTER});
                                let ret_int_expr = if partial_declassify {
                                    quote::quote!{return_integrity.clone()}
                                } else {
                                    quote::quote!{block_integrity.clone()}
                                };
                                quote::quote! {
                                    {
                                        #wrap_increment
                                        let __closure = |val| {
                                            unsafe { ::secret_structs::secret::StaticSecretStaticDynamicIntegrity::<_, #sec_label, #int_label, ::secret_structs::secret::DynLabel<::secret_structs::secret::Int>>::new_static_secret_static_dynamic_integrity(val, #ret_int_expr) }
                                        };
                                        __closure(#args)
                                    }
                                }
                            } else {
                                let wrap_increment = generate_increment(quote::quote!{WRAP_COUNTER});
                                quote::quote! {
                                    {
                                        #wrap_increment
                                        let __closure = |val| {
                                            unsafe { ::secret_structs::secret::StaticAll::<_, #sec_label, #int_label>::new_static_all(val) }
                                        };
                                        __closure(#args)
                                    }
                                }
                            }
                        }
                    } else {
                        let wrap_increment = generate_increment(quote::quote!{WRAP_COUNTER});
                        quote::quote! {
                            {
                                #wrap_increment
                                wrap(#args)
                            }
                        }
                    }
                } else if is_call_to(expr_call, "unchecked_operation") {
                    if expr_call.args.len() != 1 {
                        quote::quote!{compile_error!("unchecked_operation takes a single expression as an argument.");}
                    } else {
                        //let unchecked_increment = generate_increment(quote::quote!{UNCHECKED_COUNTER});
                        let expr = expr_call.args.iter().nth(0).unwrap();
                        quote::quote! {
                            //#unchecked_increment
                            #expr
                        }
                    }
                } else if is_call_to(expr_call, "explicit_allowlisted") {
                    if expr_call.args.len() != 1 {
                        quote::quote! {compile_error!("explicit_allowlisted takes a single function call as an argument.");}
                    } else {
                        let call = expr_call.args.iter().nth(0).unwrap();
                        match call {
                            syn::Expr::Call(call) => {
                                let args = comma_separate(call.args.iter().map(
                                    |arg: &syn::Expr| -> proc_macro2::TokenStream { expand_and_check_expr(arg, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only) },
                                ));
                                let func = *call.func.clone();
                                quote::quote! { #func(#args) }
                            },
                            _ => {
                                quote::quote! {compile_error!("explicit_allowlisted only takes a single function call.");}
                            }
                        }
                    }
                } else if is_call_to_allowlisted_function(expr_call) {
                    let func = &*expr_call.func;
                    if is_verbose {
                        make_check_secret_block_safe(quote::quote! { #func(#args) }, do_sbs_check)
                    } else {
                        quote::quote! {
                            #func(#args)
                        }
                    }
                } else {
                    let func = &*expr_call.func;
                    let mut func_path;
                    let func_attrs;
                    let func_qself;
                    match func {
                        syn::Expr::Path(path_access) => {
                            func_path = path_access.path.clone();
                            func_attrs = path_access.attrs.clone();
                            func_qself = path_access.qself.clone();
                            let last_segment = func_path.segments.pop().unwrap();
                            func_path.segments.push(syn::PathSegment {
                                ident: Ident::new(&("__".to_owned() + &last_segment.value().ident.to_string() + "_secret_trampoline_wrapper"), Span::call_site()),
                                arguments: last_segment.value().arguments.clone(),
                            });
                            let path_func = syn::ExprPath{attrs: func_attrs, qself: func_qself, path: func_path};
                            if is_verbose {
                                quote::quote! {
                                    unsafe { (::secret_structs::secret::VettedTrait::<_>::unwrap(#path_func(#args))) }
                                }
                            } else {
                                let args_num = expr_call.args.len();
                                let mut args2 = quote::quote! {};
                                if args_num > 0 {
                                    args2 = quote::quote! { a0 };
                                }
                                for i in 1..args_num {
                                    let ai = proc_macro2::TokenStream::from_str(("a".to_owned() + &i.to_string()).as_str()).unwrap();
                                    args2 = quote::quote! { #args2, #ai };
                                }
                                quote::quote! {
                                    {
                                        let __closure = |#args2| {
                                            unsafe { (::secret_structs::secret::VettedTrait::<_>::unwrap(#path_func(#args2))) }
                                        };
                                        __closure(#args)
                                    }
                                }
                            }
                        },
                        _ => {
                            quote::quote! {
                                compile_error!("Unaccepted Call Type")
                            }
                        }
                    }
                }
            }
        }
        syn::Expr::Continue(continue_stmt) => continue_stmt.into_token_stream(),
        // TODO: Handle macros better. I think you can look at their token stream to get their expansion?
        syn::Expr::Macro(expr_macro) => {
            if strip_unchecked_only {
                expr_macro.into_token_stream()
            } else {
                if is_macro_call_to(expr_macro, "secret_structs::untrusted_secure_block_dynamic_all") {
                    let tokens = expr_macro.mac.tokens.clone();
                    quote::quote! {
                        ::secret_structs::untrusted_secure_block_dynamic_all!(#tokens)
                    }
                } else if is_macro_call_to(expr_macro, "secret_structs::untrusted_secure_block_dynamic_secrecy") {
                    let tokens = expr_macro.mac.tokens.clone();
                    quote::quote! {
                        ::secret_structs::untrusted_secure_block_dynamic_secrecy!(#tokens)
                    }
                } else if is_macro_call_to(expr_macro, "secret_structs::untrusted_secure_block_dynamic_integrity") {
                    let tokens = expr_macro.mac.tokens.clone();
                    quote::quote! {
                        ::secret_structs::untrusted_secure_block_dynamic_integrity!(#tokens)
                    }
                } else if is_macro_call_to(expr_macro, "secret_structs::untrusted_secure_block_static_all") {
                    let tokens = expr_macro.mac.tokens.clone();
                    quote::quote! {
                        ::secret_structs::untrusted_secure_block_static_all!(#tokens)
                    }
                } else if is_macro_call_to(expr_macro, "secret_structs::trusted_secure_block_static_all") {
                    let tokens = expr_macro.mac.tokens.clone();
                    quote::quote! {
                        ::secret_structs::trusted_secure_block_static_all!(#tokens)
                    }
                } else if is_macro_call_to(expr_macro, "secret_structs::trusted_secure_block_dynamic_secrecy") {
                    let tokens = expr_macro.mac.tokens.clone();
                    quote::quote! {
                        ::secret_structs::trusted_secure_block_dynamic_secrecy!(#tokens)
                    }
                } else if is_macro_call_to(expr_macro, "secret_structs::trusted_secure_block_dynamic_integrity") {
                    let tokens = expr_macro.mac.tokens.clone();
                    quote::quote! {
                        ::secret_structs::trusted_secure_block_dynamic_integrity!(#tokens)
                    }
                } else if is_macro_call_to(expr_macro, "secret_structs::trusted_secure_block_dynamic_all") {
                    let tokens = expr_macro.mac.tokens.clone();
                    quote::quote! {
                        ::secret_structs::trusted_secure_block_dynamic_all!(#tokens)
                    }
                } else if is_macro_call_to(expr_macro, "secret_structs::untrusted_secure_block_no_return_static_all") {
                    let tokens = expr_macro.mac.tokens.clone();
                    quote::quote! {
                        ::secret_structs::untrusted_secure_block_no_return_static_all!(#tokens)
                    }
                } else if is_macro_call_to(expr_macro, "secret_structs::untrusted_secure_block_no_return_dynamic_secrecy") {
                    let tokens = expr_macro.mac.tokens.clone();
                    quote::quote! {
                        ::secret_structs::untrusted_secure_block_no_return_dynamic_secrecy!(#tokens)
                    }
                } else if is_macro_call_to(expr_macro, "secret_structs::untrusted_secure_block_no_return_dynamic_integrity") {
                    let tokens = expr_macro.mac.tokens.clone();
                    quote::quote! {
                        ::secret_structs::untrusted_secure_block_no_return_dynamic_integrity!(#tokens)
                    }
                } else if is_macro_call_to(expr_macro, "secret_structs::untrusted_secure_block_no_return_dynamic_all") {
                    let tokens = expr_macro.mac.tokens.clone();
                    quote::quote! {
                        ::secret_structs::untrusted_secure_block_no_return_dynamic_all!(#tokens)
                    }
                /*} else if is_macro_call_to(expr_macro, "debug_assert") {
                    let e_tokens = expr_macro.mac.tokens.clone();
                    let mut e: syn::Expr = syn::parse(e_tokens.into()).expect("Err60");
                    let mut new_e = expand_and_check_expr(&e, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, do_sbs_check, does_return, wrap_return, is_func, strip_unchecked_only);
                    quote::quote! {
                        debug_assert!(#new_e)
                    }
                }*/ 
                } else { 
                    let syn::Macro{path, bang_token: _, delimiter: _, tokens: _} = &expr_macro.mac;
                    let mut path_str = quote::quote! {#path}.to_string();
                    path_str.retain(|c| !c.is_whitespace());
                    quote::quote! {
                        compile_error!("Function calls & macros are not allowed in secret blocks.")
                    }
                }
            }
        }
        syn::Expr::Binary(expr_binary) => {
            if strip_unchecked_only {
                let lhs = expand_and_check_expr(&*expr_binary.left, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, do_sbs_check, does_return, wrap_return, is_func, strip_unchecked_only);
                let rhs = expand_and_check_expr(&*expr_binary.right, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, do_sbs_check, does_return, wrap_return, is_func, strip_unchecked_only);
                let op = expr_binary.op;
                quote::quote! {
                    (#lhs) #op (#rhs)
                }
            } else {
                if is_verbose {
                    let op = expr_binary.op;
                    // Outer SBS checks not needed because expressions have built-in types
                    let new_expr_left: proc_macro2::TokenStream = expand_and_check_expr(&expr_binary.left, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, false, does_return, wrap_return, is_func, strip_unchecked_only);
                    let new_expr_right = expand_and_check_expr(&expr_binary.right, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, false, does_return, wrap_return, is_func, strip_unchecked_only);
                    match op {
                        syn::BinOp::Add(_) => {
                            quote::quote! { ::secret_structs::secret::SafeAdd::safe_add(#new_expr_left, #new_expr_right) }
                        }
                        syn::BinOp::Sub(_) => {
                            quote::quote! { ::secret_structs::secret::SafeSub::safe_sub(#new_expr_left, #new_expr_right) }
                        }
                        syn::BinOp::Mul(_) => {
                            quote::quote! { ::secret_structs::secret::SafeMul::safe_mul(#new_expr_left, #new_expr_right) }
                        }
                        syn::BinOp::Div(_) => {
                            quote::quote! { ::secret_structs::secret::SafeDiv::safe_div(#new_expr_left, #new_expr_right) }
                        }
                        syn::BinOp::Rem(_) => {
                            quote::quote! { ::secret_structs::secret::SafeRem::safe_rem(#new_expr_left, #new_expr_right) }
                        }
                        syn::BinOp::Ne(_) => {
                            quote::quote! { ::secret_structs::secret::SafePartialEq::safe_ne(&(#new_expr_left), &(#new_expr_right)) }
                        }
                        syn::BinOp::Eq(_) => {
                            quote::quote! { ::secret_structs::secret::SafePartialEq::safe_eq(&(#new_expr_left), &(#new_expr_right)) }
                        }
                        syn::BinOp::Le(_) => {
                            quote::quote! { ::secret_structs::secret::SafePartialOrd::safe_le(&(#new_expr_left), &(#new_expr_right)) }
                        }
                        syn::BinOp::Ge(_) => {
                            quote::quote! { ::secret_structs::secret::SafePartialOrd::safe_ge(&(#new_expr_left), &(#new_expr_right)) }
                        }
                        syn::BinOp::Gt(_) => {
                            quote::quote! { ::secret_structs::secret::SafePartialOrd::safe_gt(&(#new_expr_left), &(#new_expr_right)) }
                        }
                        syn::BinOp::Lt(_) => {
                            quote::quote! { ::secret_structs::secret::SafePartialOrd::safe_lt(&(#new_expr_left), &(#new_expr_right)) }
                        }
                        // these are not overloadable (shortcircuiting logical or/and)
                        // https://doc.rust-lang.org/book/appendix-02-operators.html
                        syn::BinOp::Or(_) | syn::BinOp::And(_) => {
                            // Outer SBS checks not needed because expressions have built-in types
                            let lhs = expand_and_check_expr(&*expr_binary.left, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, false, does_return, wrap_return, is_func, strip_unchecked_only);
                            let rhs = expand_and_check_expr(&*expr_binary.right, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, false, does_return, wrap_return, is_func, strip_unchecked_only);
                            quote::quote! { (#lhs) #op (#rhs) }
                        }
                        syn::BinOp::BitXor(_) => {
                            // TODO: Fix me!
                            // I know, I know. This is incorrect. But the
                            // implementation is identical to the code above. It's a
                            // bad use of time to do this correctly right now.
                            // Outer SBS checks not needed because expressions have built-in types (well, once we disallow overloading)
                            let lhs = expand_and_check_expr(&*expr_binary.left, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, false, does_return, wrap_return, is_func, strip_unchecked_only);
                            let rhs = expand_and_check_expr(&*expr_binary.right, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, false, does_return, wrap_return, is_func, strip_unchecked_only);
                            quote::quote! { ((#lhs) #op (#rhs)) }
                        }
                        syn::BinOp::BitAnd(_) => {
                            quote::quote! { ::secret_structs::secret::SafeBitAnd::safe_bitand(&(#new_expr_left), &(#new_expr_right)) }
                        }
                        syn::BinOp::BitOr(_) => {
                            quote::quote! { ::secret_structs::secret::SafeBitOr::safe_bitor(&(#new_expr_left), &(#new_expr_right)) }
                        }
                        syn::BinOp::Shl(_) => {
                            quote::quote! { ::secret_structs::secret::SafeShl::safe_shl(&(#new_expr_left), &(#new_expr_right)) }
                        }
                        syn::BinOp::Shr(_) => {
                            quote::quote! { ::secret_structs::secret::SafeShr::safe_shr(&(#new_expr_left), &(#new_expr_right)) }
                        }
                        // Disallows all other binary operators
                        _op => {
                            let expr_display = proc_macro2::TokenStream::to_string(&quote! {#expr_binary});
                            let ts = proc_macro2::TokenStream::from_str(&format!(
                                "r#\"secret_macros: Unsupported binary operator: {:?}, called in {:?}.\"#", _op,
                                expr_display
                            ))
                            .unwrap_or(
                                proc_macro2::TokenStream::from_str(
                                    "\"secret_macros: Unsupported binary operator.\"",
                                )
                                .expect("ErrM"),
                            );
                            // Note: in some contexts you need a semicolon to terminate the compile_error! macro.
                            // In other contexts you don't.
                            // If you see a compiler error like `error: custom attribute panicked` then try adding a semicolon to the compile_error! macro here to get a better message.
                            quote::quote! {
                                { compile_error!(#ts); }
                            }
                        }
                    }
                } else {
                    let lhs = expand_and_check_expr(&*expr_binary.left, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, do_sbs_check, does_return, wrap_return, is_func, strip_unchecked_only);
                    let rhs = expand_and_check_expr(&*expr_binary.right, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, do_sbs_check, does_return, wrap_return, is_func, strip_unchecked_only);
                    let op = expr_binary.op;
                    quote::quote! {
                        (#lhs) #op (#rhs)
                    }
                }
            }
        }
        syn::Expr::If(expr_if) => {
            let condition = expand_and_check_expr(&*expr_if.cond, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only);
            let then_block: proc_macro2::TokenStream =
                expand_and_check_block(&expr_if.then_branch, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, does_return, wrap_return, is_func, strip_unchecked_only).into();
            let else_branch = match &expr_if.else_branch {
                Some(block) => expand_and_check_expr(&*block.1, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only),
                None => quote::quote! {},
            };
            quote::quote! {
                if #condition {
                    #then_block
                } else {
                    #else_branch
                }
            }
        }
        syn::Expr::Block(expr_block) => expand_and_check_block(&expr_block.block, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, does_return, wrap_return, is_func, strip_unchecked_only).into(),
        syn::Expr::Closure(closure_expr) => {
            let mut new_closure = closure_expr.clone();
            new_closure.body =
                Box::new(syn::parse2(expand_and_check_expr(&new_closure.body, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only)).expect("ErrN"));
            new_closure.into_token_stream()
        }
        syn::Expr::Assign(assign_expr) => {
            // Set do_sbs_check for LHS of assignments, since it's an lvalue, not an rvalue
            let lhs: proc_macro2::TokenStream =
                expand_and_check_expr(&assign_expr.left, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, false, does_return, wrap_return, is_func, strip_unchecked_only).into();
            let rhs: proc_macro2::TokenStream =
                expand_and_check_expr(&assign_expr.right, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only).into();
            if !strip_unchecked_only && is_verbose {
                make_check_secret_block_safe(quote::quote! { (*::secret_structs::secret::not_mut_secret(&mut #lhs)) = (#rhs) }, do_sbs_check) 
            } else {
                quote::quote! {
                    (#lhs) = (#rhs)
                }
            }
        }
        syn::Expr::AssignOp(assign_op_expr) => {
            if !strip_unchecked_only && is_verbose {
                let op = assign_op_expr.op;
                // Outer SBS checks not needed because expressions have built-in types
                let new_expr_left: proc_macro2::TokenStream = expand_and_check_expr(&make_mut_ref(*assign_op_expr.left.clone()), secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, false, does_return, wrap_return, is_func, strip_unchecked_only);
                let new_expr_right = expand_and_check_expr(&assign_op_expr.right, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, false, does_return, wrap_return, is_func, strip_unchecked_only);
                match op {
                    // Don't need not_mut_secret here since the functions guarantee the types are not Secure Values.
                    syn::BinOp::AddEq(_) => {
                        quote::quote! { ::secret_structs::secret::SafeAddAssign::safe_add_assign(#new_expr_left, #new_expr_right) }
                    }
                    syn::BinOp::SubEq(_) => {
                        quote::quote! { ::secret_structs::secret::SafeSubAssign::safe_sub_assign(#new_expr_left, #new_expr_right) }
                    }
                    syn::BinOp::MulEq(_) => {
                        quote::quote! { ::secret_structs::secret::SafeMulAssign::safe_mul_assign(#new_expr_left, #new_expr_right) }
                    }
                    syn::BinOp::DivEq(_) => {
                        quote::quote! { ::secret_structs::secret::SafeDivAssign::safe_div_assign(#new_expr_left, #new_expr_right) }
                    }
                    syn::BinOp::RemEq(_) => {
                        quote::quote! { ::secret_structs::secret::SafeRemAssign::safe_rem_assign(#new_expr_left, #new_expr_right) }
                    }
                    // Disallows all other assignment operators
                    _op => {
                        let expr_display =
                            proc_macro2::TokenStream::to_string(&quote! {#assign_op_expr});
                        let ts = proc_macro2::TokenStream::from_str(&format!(
                            "r#\"secret_macros: Unsupported binary operator: {:?}, called in {:?}.\"#", _op,
                            expr_display
                        ))
                        .unwrap_or(
                            proc_macro2::TokenStream::from_str(
                                "\"secret_macros: Unsupported binary operator.\"",
                            )
                            .expect("ErrO"),
                        );
                        // Note: in some contexts you need a semicolon to terminate the compile_error! macro.
                        // In other contexts you don't.
                        // If you see a compiler error like `error: custom attribute panicked` then try adding a semicolon to the compile_error! macro here to get a better message.
                        quote::quote! {
                            { compile_error!(#ts); }
                        }
                    }
                }
            } else {
                let lhs: proc_macro2::TokenStream =
                    expand_and_check_expr(&assign_op_expr.left, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, do_sbs_check, does_return, wrap_return, is_func, strip_unchecked_only).into();
                let rhs: proc_macro2::TokenStream =
                    expand_and_check_expr(&assign_op_expr.right, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, do_sbs_check, does_return, wrap_return, is_func, strip_unchecked_only).into();
                let op = assign_op_expr.op;
                quote::quote! {
                    (#lhs) #op (#rhs)
                }
            }
        }
        syn::Expr::MethodCall(method_call_expr) => {
            let receiver: proc_macro2::TokenStream =
                expand_and_check_expr(&method_call_expr.receiver, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only).into();
            let args = comma_separate(method_call_expr.args.iter().map(
                |arg: &syn::Expr| -> proc_macro2::TokenStream { expand_and_check_expr(arg, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only) },
            ));
            let method = &method_call_expr.method;
            let turbofish = &method_call_expr.turbofish;
            if strip_unchecked_only {
                quote::quote! {
                    (#receiver.#method #turbofish(#args))
                }
            } else {
                let new_fn_name_wrapper = get_trampoline_fn_name(&method.to_string(), &"_wrapper".to_string());
                let method = Ident::new(&new_fn_name_wrapper, method.span());
                if is_verbose {
                    // Don't need an outer check since side_effect_free_attr methods are guaranteed to be InvisibleSideEffectFree
                    // TODO: Shouldn't evaluate #args inside of unsafe block
                    quote::quote! {
                        unsafe { use ::secret_structs::allowlisted::*; (::secret_structs::secret::VettedTrait::<_>::unwrap(#receiver.#method #turbofish(#args))) }
                        //((&#receiver).#method#turbofish(#args) as secret_structs::secret::VettedTrait<_>).unwrap()
                    }
                } else {
                    let args_num = method_call_expr.args.len();
                    let mut args2 = quote::quote! {};
                    if args_num > 0 {
                        args2 = quote::quote! { a0 };
                    }
                    for i in 1..args_num {
                        let ai = proc_macro2::TokenStream::from_str(("a".to_owned() + &i.to_string()).as_str()).unwrap();
                        args2 = quote::quote! { #args2, #ai };
                    }
                    quote::quote! {
                        {
                            let mut __closure = |#args2| {
                                unsafe { use ::secret_structs::allowlisted::*; (::secret_structs::secret::VettedTrait::<_>::unwrap(#receiver.#method #turbofish(#args2))) }
                            };
                            __closure(#args)
                        }
                    }
                }
            }
        }
        syn::Expr::Lit(expr_lit) => expr_lit.into_token_stream(),
        syn::Expr::Field(field_access) => {
            let e: Expr  = syn::parse2(expand_and_check_expr(&*(field_access.base), secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only)).expect("ErrS");
            let e2: Expr = syn::parse2(quote::quote!{ (#e) }).expect("ErrS");
            let e3: Box<Expr> = Box::new(e2);
            let f_new = ExprField {
                attrs: field_access.attrs.clone(),
                base: e3,
                dot_token: field_access.dot_token.clone(),
                member: field_access.member.clone()
            };
            // Don't need check around whole expression because e.f is InvisibleSideEffectFree if e is
            f_new.into_token_stream() 
        }
        // fix_sbs_checking: Path (e.g., an identifier) needs a check because VisibleSideEffectFree doesn't exclude all non-InvisibleSideEffectFree types from being captured
        syn::Expr::Path(path_access) => {
            if strip_unchecked_only || is_path_to_allowlisted_function(path_access) || !is_verbose {
                path_access.into_token_stream()
            } else {
                let p = path_access.into_token_stream();
                make_check_secret_block_safe_ptr_read(p, do_sbs_check)
            }
        }
        syn::Expr::Paren(paren_expr) => {
            let interal_expr = expand_and_check_expr(&paren_expr.expr, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, do_sbs_check, does_return, wrap_return, is_func, strip_unchecked_only);
            let mut new_paren_expr = paren_expr.clone();
            new_paren_expr.expr = Box::new(syn::parse2(interal_expr).expect("ErrR"));
            new_paren_expr.into_token_stream()
        }
        syn::Expr::Struct(struct_literal) => {
            // Struct initializer expressions need checks
            let fields: syn::punctuated::Punctuated<FieldValue, Comma> = {
                let mut f = syn::punctuated::Punctuated::<FieldValue, Comma>::new();
                for field in struct_literal.clone().fields.iter() {
                    // The inner types don't need an SBS check because they are guaranteed to be InvisibleSideEffectFree because the outer type is guaranteed to be (because of check inserted below)
                    let e = syn::parse2(expand_and_check_expr(&field.expr, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, false, does_return, wrap_return, is_func, strip_unchecked_only)).expect("ErrS");
                    let fv = syn::FieldValue {
                        attrs: field.attrs.clone(), 
                        member: field.member.clone(),
                        colon_token: field.colon_token.clone(),
                        expr: e
                    };
                    f.push(fv);
                }
                f
            };
            let struct_new = syn::ExprStruct {
                attrs: struct_literal.attrs.clone(),
                path: struct_literal.path.clone(),
                brace_token: struct_literal.brace_token.clone(),
                fields: fields,
                dot2_token: struct_literal.dot2_token.clone(),
                rest: struct_literal.rest.clone(),
            };
            if !strip_unchecked_only && is_verbose {
                let s = struct_new.into_token_stream();
                make_check_secret_block_safe(s, do_sbs_check)
            } else {
                struct_new.into_token_stream()
            }
        }
        syn::Expr::ForLoop(for_loop) => {
            // TODO: Does #pat need expansion?
            let pat = for_loop.pat.clone().into_token_stream();
            let expr: proc_macro2::TokenStream = expand_and_check_expr(&*for_loop.expr, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only).into();
            let body: proc_macro2::TokenStream = expand_and_check_block(&for_loop.body, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, does_return, wrap_return, is_func, strip_unchecked_only).into();
            quote::quote! {
                for #pat in #expr {
                    #body
                }
            }
        }
        syn::Expr::While(while_loop) => {
            let cond: proc_macro2::TokenStream =
                expand_and_check_expr(&*while_loop.cond, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only).into();
            let body: proc_macro2::TokenStream =
                expand_and_check_block(&while_loop.body, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, does_return, wrap_return, is_func, strip_unchecked_only).into();
            quote::quote! {
                while #cond {
                    #body
                }
            }
        }
        syn::Expr::Loop(loop_expr) => {
            let body: proc_macro2::TokenStream = expand_and_check_block(&loop_expr.body, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, does_return, wrap_return, is_func, strip_unchecked_only).into();
            quote::quote! {
                loop {
                    #body
                }
            }
        }
        // TODO: Need to implement check for match -- wait, what still needs to be implemented?
        syn::Expr::Match(expr_match) => {
            let mut expr_match_copy = expr_match.clone();
            expr_match_copy.expr =
                Box::new(syn::parse2(expand_and_check_expr(&*expr_match_copy.expr, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only)).expect("ErrS"));
            for arm in &mut expr_match_copy.arms {
                match &arm.guard {
                    Some((if_token, guard_expr_boxed)) => {
                        arm.guard = Some((
                            *if_token,
                            Box::new(
                                syn::parse2(expand_and_check_expr(&*guard_expr_boxed, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only)).expect("ErrT"),
                            ),
                        ))
                    }
                    _ => {}
                }
                arm.body = Box::new(syn::parse2(expand_and_check_expr(&*arm.body, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only)).expect("ErrU"));
            }
            expr_match_copy.into_token_stream()
        }
        syn::Expr::Range(range) => {
            let mut range_copy = range.clone();
            match range_copy.from {
                Some(from) => range_copy.from = Some(syn::parse2(expand_and_check_expr(&*from, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, does_return, wrap_return, true, is_func, strip_unchecked_only)).unwrap()),
                _ => {},
            };
            match range_copy.to {
                Some(to) => range_copy.to = Some(syn::parse2(expand_and_check_expr(&*to, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, does_return, wrap_return, true, is_func, strip_unchecked_only)).unwrap()),
                _ => {},
            };
            if !strip_unchecked_only && is_verbose {
                quote::quote! {
                    ::secret_structs::secret::check_safe_range_bounds(#range)
                }
            } else {
                quote::quote!{(#range_copy)}
            } 
        }
        syn::Expr::Repeat(repeat_expr) => {
            let expr = expand_and_check_expr(&repeat_expr.expr, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only);
            let mut new_repeat_expr = repeat_expr.clone();
            new_repeat_expr.expr = Box::new(syn::parse2(expr).expect("ErrY"));
            if is_verbose {
                new_repeat_expr.len = repeat_expr.len.clone();
            } else {
                let len_expr = expand_and_check_expr(&repeat_expr.len, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only);
                new_repeat_expr.len = Box::new(syn::parse2(len_expr).expect("ErrZ"));
            }
            new_repeat_expr.into_token_stream()
        }
        syn::Expr::Return(return_expr) => {
            if let None = return_expr.expr {
                return return_expr.into_token_stream();
            }
            let mut new_return_expr = return_expr.clone();
            let expr = expand_and_check_expr(&new_return_expr.expr.expect("Err1"), secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only);
            new_return_expr.expr = Some(Box::new(syn::parse2(expr).expect("Err2")));
            new_return_expr.into_token_stream()
        }
        syn::Expr::Index(idx) => {
            // Outer expressions don't need checks since the arguments of safe_index must be built-in types
            let new_idx_expr: proc_macro2::TokenStream = expand_and_check_expr(&idx.expr, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, false, does_return, wrap_return, is_func, strip_unchecked_only);
            let new_idx_index = expand_and_check_expr(&idx.index, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, false, does_return, wrap_return, is_func, strip_unchecked_only);
            if !strip_unchecked_only && is_verbose {
                quote::quote! { ::secret_structs::secret::check_safe_index_expr(#new_idx_expr)[::secret_structs::secret::check_safe_index(#new_idx_index)] }
            } else {
                quote::quote! {
                    #new_idx_expr[#new_idx_index]
                }
            }
        }
        syn::Expr::Tuple(tuple) => {
            let args = comma_separate(tuple.elems.iter().map(
                |arg: &syn::Expr| -> proc_macro2::TokenStream { expand_and_check_expr(arg, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only) },
            ));
            quote::quote! {
                (#args)
            }
        }
        syn::Expr::Try(exprtry) => {
            let new_expr = expand_and_check_expr(&exprtry.expr, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only);
            let expanded_expr_try = syn::ExprTry {
                attrs: exprtry.attrs.clone(),
                expr: Box::new(syn::parse2(new_expr).expect("Err2")),
                question_token: exprtry.question_token.clone()
            };
            expanded_expr_try.into_token_stream()
        }
        syn::Expr::Unary(unary) => {
            let operator = unary.op;
            if !strip_unchecked_only && is_verbose {
                match operator {
                    syn::UnOp::Deref(_) => {
                        let new_operand_expr = expand_and_check_expr(&*unary.expr, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only);
                        // Outer expression doesn't need a check since operand must be InvisibleSideEffectFree
                        quote::quote! { #operator(#new_operand_expr) }
                    }
                    syn::UnOp::Not(_) => {
                        // Expressions don't need InvisibleSideEffectFree checks because they're built-in types
                        let new_operand_expr = expand_and_check_expr(&*unary.expr, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, false, does_return, wrap_return, is_func, strip_unchecked_only);
                        quote::quote! { ::secret_structs::secret::SafeNot::safe_not(#new_operand_expr) }
                    }
                    syn::UnOp::Neg(_) => {
                        // Expressions don't need InvisibleSideEffectFree checks because they're built-in types
                        let new_operand_expr = expand_and_check_expr(&*unary.expr, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, false, does_return, wrap_return, is_func, strip_unchecked_only);
                        quote::quote! { ::secret_structs::secret::SafeNeg::safe_neg(#new_operand_expr) }
                    }
                }
            } else {
                let operand = expand_and_check_expr(&*unary.expr, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only);
                quote::quote! {
                    #operator #operand
                }
            }
        }
        syn::Expr::Unsafe(unsafe_expr) => quote::quote! {#unsafe_expr},
        syn::Expr::Reference(reference) => {
            // TODO: Why put the check around &e instead of putting it around e?
            let operand = expand_and_check_expr(&*reference.expr, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, false, does_return, wrap_return, is_func, strip_unchecked_only);
            if !strip_unchecked_only && is_verbose {
                match reference.mutability {
                    Some(_) => {
                        // fix_sbs_checking: We need a check around this expression because VisibleSideEffectFree doesn't exclude all non-SBS types from being captured
                        make_check_secret_block_safe_mut_ref(quote::quote! { &mut #operand }, do_sbs_check)
                    }
                    _ => {
                        // fix_sbs_checking: We need a check around this expression because VisibleSideEffectFree doesn't exclude all non-SBS types from being captured
                        make_check_secret_block_safe_ref(quote::quote! { &#operand }, do_sbs_check)
                    }
                }
            } else {
                match reference.mutability {
                    Some(_) => {
                        quote::quote! {
                            &mut #operand
                        }
                    }
                    _ => {
                        quote::quote! {
                            &#operand
                        }
                    }
                }
            }
        }
        syn::Expr::Cast(cast) => {
            let expr = expand_and_check_expr(&*cast.expr, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_secrecy, true, does_return, wrap_return, is_func, strip_unchecked_only);
            let ty = &cast.ty;
            if !strip_unchecked_only && is_verbose {
                make_check_secret_block_safe(quote::quote! { #expr as #ty }, do_sbs_check)
            } else {
                quote::quote! { #expr as #ty }
            }
        }
        // TODO: Handle the other kinds of expressions.
        expr => {
            let expr_display = proc_macro2::TokenStream::to_string(&quote! {#expr});
            let ts = proc_macro2::TokenStream::from_str(&format!(
                "r#\"secret_macros: Unrecognized syntax: {:?}.\"#",
                expr_display
            ))
            .unwrap_or(
                proc_macro2::TokenStream::from_str("\"secret_macros: Unrecognized syntax.\"")
                    .expect("Err3"),
            );

            // Note: in some contexts you need a semicolon to terminate the compile_error! macro.
            // In other contexts you don't.
            // If you see a compiler error like `error: custom attribute panicked` then try adding a semicolon to the compile_error! macro here to get a better message.
            quote::quote! {
                { compile_error!(#ts) }
            }
        }
    }
}

fn make_check_secret_block_safe(e: proc_macro2::TokenStream, do_check: bool) -> proc_macro2::TokenStream {
    if do_check {
        // TODO: The outer { } are needed or there's an error in millionaires
        quote::quote! {
            { ::secret_structs::secret::check_ISEF(#e) }
        }
    } else {
        e
    }
}

fn make_check_secret_block_safe_ptr_read(e: proc_macro2::TokenStream, do_check: bool) -> proc_macro2::TokenStream {
    if do_check {
        quote::quote! {
            { let tmp = &(#e); unsafe { ::secret_structs::secret::check_ISEF_unsafe(tmp) } }
        }
    } else {
        e
    }
}

fn make_check_secret_block_safe_ref(e: proc_macro2::TokenStream, do_check: bool) -> proc_macro2::TokenStream {
    if do_check {
        quote::quote! {
            { ::secret_structs::secret::check_expr_secret_block_safe_ref((#e)) }
        }
    } else {
        e
    }
}

fn make_check_secret_block_safe_mut_ref(e: proc_macro2::TokenStream, do_check: bool) -> proc_macro2::TokenStream {
    if do_check {
        quote::quote! {
            { ::secret_structs::secret::check_ISEF_mut_ref((#e)) }
        }
    } else {
        e
    }
}

// Helper function to get "&e" from "e"
fn _make_ref(e: Expr) -> Expr {
    syn::parse((quote::quote! {& #e } as proc_macro2::TokenStream).into()).unwrap()
}

// Helper function to get "&mut e" from "e"
fn make_mut_ref(e: Expr) -> Expr {
    syn::parse((quote::quote! {&mut #e } as proc_macro2::TokenStream).into()).expect("Err5")
}

fn expand_and_check_block(input: &syn::Block, secrecy_label: &Option<syn::Type>, integrity_label: &Option<syn::Type>, sec_label_2: &Option<syn::Type>, int_label_2: &Option<syn::Type>, partial_declassify: bool, is_verbose: bool, dynamic_secrecy: bool, dynamic_integrity: bool, does_return: bool, wrap_return: bool, is_func: bool, strip_unchecked_only: bool) -> TokenStream {
    // We have to use proc_macro2::TokenStream here because it has an implementation
    // for ToTokens, but TokenStream does not implement.
    let token_streams: Vec<proc_macro2::TokenStream> = input
        .stmts
        .iter()
        .map(|stmt: &syn::Stmt| -> proc_macro2::TokenStream {
            match stmt {
                syn::Stmt::Local(local_expr) => match &local_expr.init {
                    // Check the right-hand side of a store.
                    Some((_, expr)) => {
                        let mut new_expr = local_expr.clone();
                        //eprintln!("msg7: {}", quote::quote!{#expr}.to_string());
                        let new_init: Expr =
                            syn::parse(expand_and_check_expr(expr, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only).into()).expect("Err7");
                        new_expr.init = Some((Eq(expr.span()), Box::new(new_init)));
                        quote! {
                            #new_expr
                        }
                    }
                    None => local_expr.into_token_stream().into(),
                }
                syn::Stmt::Item(item) => {
                    if is_verbose {
                        match item {
                            // Const items can never have side-effects, so leave them alone.
                            syn::Item::Const(const_item) => const_item.into_token_stream(),
                            _ => {
                                let i = item.into_token_stream();
                                make_check_secret_block_safe(i, true)
                            }
                        }
                    } else {
                        // Looking at the definition of Item, any Item should be fine.
                        item.into_token_stream().into()
                    }
                }
                syn::Stmt::Expr(expr) => expand_and_check_expr(expr, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only),
                syn::Stmt::Semi(expr, _) => {
                    let expr_tokens = expand_and_check_expr(expr, secrecy_label, integrity_label, sec_label_2, int_label_2, partial_declassify, is_verbose, dynamic_secrecy, dynamic_integrity, true, does_return, wrap_return, is_func, strip_unchecked_only);
                    quote::quote! {
                        #expr_tokens;
                    }
                }
            }
        })
        .collect();
    let stream: proc_macro2::TokenStream = proc_macro2::TokenStream::from_iter(token_streams);
    let gen = quote::quote! {
        {
            #stream
        }
    };
    gen.into()
}

// Comma separates each of the tokens in the iterator of TokenStreams ts.
fn comma_separate<T: Iterator<Item = proc_macro2::TokenStream>>(ts: T) -> proc_macro2::TokenStream {
    ts.fold(
        proc_macro2::TokenStream::new(),
        |acc: proc_macro2::TokenStream,
         token: proc_macro2::TokenStream|
         -> proc_macro2::TokenStream {
            if acc.is_empty() {
                token
            } else {
                let ba: proc_macro2::TokenStream = acc.into();
                let bt: proc_macro2::TokenStream = token.into();
                quote! {#ba, #bt}
            }
        },
    )
}

fn get_trampoline_fn_name(fn_name: &str, special: &str) -> String {
    "__".to_owned() + fn_name + "_secret_trampoline"  + special
}
#[proc_macro_attribute]
pub fn side_effect_free_attr_trait(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut fn_definition: syn::TraitItemMethod = syn::parse(item).expect("Err8");
    let new_fn_name_checked = get_trampoline_fn_name(&fn_definition.sig.ident.to_string(), &"_checked".to_string());
    let new_fn_name_unchecked = get_trampoline_fn_name(&fn_definition.sig.ident.to_string(), &"_unchecked".to_string());
    let new_fn_name_wrapper = get_trampoline_fn_name(&fn_definition.sig.ident.to_string(), &"_wrapper".to_string());
    let new_fn_name_wrapper = Ident::new(&new_fn_name_wrapper, fn_definition.span());

    //Get the names of the formal parameters of the function.
    let is_method;
    match attr.to_string().as_str() {
        "method" => is_method = true,
        _ => is_method = false,
    }
    let param_names_vec: Vec<_> = fn_definition
        .sig
        .inputs
        .clone()
        .iter()
        .map(|arg| -> proc_macro2::TokenStream {
            match arg {
                syn::FnArg::Receiver(_r) => {
                    quote! {self}
                }
                syn::FnArg::Typed(t) => {
                    let name = &t.pat;
                    quote! {#name}
                }
            }
        })
        .collect();

    let param_names = comma_separate(
        param_names_vec
        .iter()
        .map(|x| x.clone()),
    );

    // Make a new unsafe function with the same name as the function the user defined.
    let fn_sig = fn_definition.sig.clone();
    let fn_const = fn_definition.sig.constness;
    let is_const = match fn_const {
        Some(_) => true,
        None => false
    };
    let _fn_name = fn_sig.ident;
    let fn_args = fn_sig.inputs;
    let fn_return_type = match fn_sig.output {
        syn::ReturnType::Default => quote! {()},
        syn::ReturnType::Type(_, t) => {
            let t = *t;
            quote! { #t }
        }
    };
    let generic_params = fn_sig.generics.params;
    let where_clause = match fn_sig.generics.where_clause {
        Some(clause) => quote::quote! {#clause},
        _ => quote::quote! {}
    };

    let mut new_fn_definition_unchecked = fn_definition.clone();
    let new_fn_name_unchecked = Ident::new(&new_fn_name_unchecked, fn_definition.span());
    new_fn_definition_unchecked.sig.ident = new_fn_name_unchecked.clone();
    let new_block = match new_fn_definition_unchecked.default {
        Some(b) => {
            let a = expand_and_check_block(&(b.clone()), &None, &None, &None, &None, false, false, false, false, true, false, true, false);
            let s = a.to_string() + "Err9b";
            Some(syn::parse(expand_and_check_block(&b, &None, &None, &None, &None, false, false, false, false, true, false, true, false)).expect(&s))
        },
        None => None
    };
    new_fn_definition_unchecked.default = new_block;

    let mut new_fn_definition_checked = fn_definition.clone();
    let new_fn_name_checked = Ident::new(&new_fn_name_checked, fn_definition.span());
    new_fn_definition_checked.sig.ident = new_fn_name_checked.clone();
    let new_block = match new_fn_definition_checked.default {
        Some(b) => {
            Some(syn::parse(expand_and_check_block(&b, &None, &None, &None, &None, false, true, false, false, true, false, true, false)).expect("Err0"))
        },
        None => None
    };
    new_fn_definition_checked.default = new_block;

    let new_block = match fn_definition.default {
        Some(b) => {
            Some(syn::parse(expand_and_check_block(&b, &None, &None, &None, &None, false, false, false, false, false, false, false, true)).expect("Err0"))
        },
        None => None
    };
    fn_definition.default = new_block;

    let self_block = if is_method {
        quote! {Self::}
    } else {
        quote! {}
    };

    let vetted_type = if is_const {
        quote::quote! {
            /*::secret_structs::secret::Vetted*/impl ::secret_structs::secret::VettedTrait
        }
    } else {
        quote::quote! {
            impl ::secret_structs::secret::VettedTrait
        }
    };
    

    let gen1 = quote! {
        #fn_definition

        #[allow(unused_parens)]
        #new_fn_definition_unchecked
        
        #[allow(unused_parens)]
        #[allow(unused_mut)]
        #new_fn_definition_checked
    };
    let gen = match fn_definition.default {
        Some(b) => {
            quote! {
                #gen1

                #[inline(always)]
                #fn_const unsafe fn #new_fn_name_wrapper<#generic_params>(#fn_args) -> #vetted_type<#fn_return_type> #where_clause {
                        ::secret_structs::secret::vetted_wrap::<#fn_return_type>(#self_block #new_fn_name_unchecked(#param_names))
                }
            }
        },
        None => {
            quote! {
                #gen1

                #[inline(always)]
                #fn_const unsafe fn #new_fn_name_wrapper<#generic_params>(#fn_args) -> #vetted_type<#fn_return_type> #where_clause;
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn side_effect_free_attr(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut fn_definition: syn::ItemFn = syn::parse(item).expect("Err8");
    let new_fn_name_checked = get_trampoline_fn_name(&fn_definition.sig.ident.to_string(), &"_checked".to_string());
    let new_fn_name_unchecked = get_trampoline_fn_name(&fn_definition.sig.ident.to_string(), &"_unchecked".to_string());
    let new_fn_name_wrapper = get_trampoline_fn_name(&fn_definition.sig.ident.to_string(), &"_wrapper".to_string());
    let new_fn_name_wrapper = Ident::new(&new_fn_name_wrapper, fn_definition.span());

    //Get the names of the formal parameters of the function.
    let is_method;
    match attr.to_string().as_str() {
        "method" => is_method = true,
        _ => is_method = false,
    }
    let param_names_vec: Vec<_> = fn_definition
        .sig
        .inputs
        .clone()
        .iter()
        .map(|arg| -> proc_macro2::TokenStream {
            match arg {
                syn::FnArg::Receiver(_r) => {
                    quote! {self}
                }
                syn::FnArg::Typed(t) => {
                    let name = &t.pat;
                    quote! {#name}
                }
            }
        })
        .collect();

    let param_names = comma_separate(
        param_names_vec
        .iter()
        .map(|x| x.clone()),
    );

    // Make a new unsafe function with the same name as the function the user defined.
    let fn_access = fn_definition.vis.clone();
    let fn_sig = fn_definition.sig.clone();
    let fn_const = fn_definition.sig.constness;
    let is_const = match fn_const {
        Some(_) => true,
        None => false
    };
    let _fn_name = fn_sig.ident;
    let fn_args = fn_sig.inputs;
    let fn_return_type = match fn_sig.output {
        syn::ReturnType::Default => quote! {()},
        syn::ReturnType::Type(_, t) => {
            let t = *t;
            quote! { #t }
        }
    };
    let generic_params = fn_sig.generics.params;
    let where_clause = match fn_sig.generics.where_clause {
        Some(clause) => quote::quote! {#clause},
        _ => quote::quote! {}
    };

    let mut new_fn_definition_unchecked = fn_definition.clone();
    let new_fn_name_unchecked = Ident::new(&new_fn_name_unchecked, fn_definition.span());
    new_fn_definition_unchecked.sig.ident = new_fn_name_unchecked.clone();
    let a = expand_and_check_block(&*(new_fn_definition_unchecked.block.clone()), &None, &None, &None, &None, false, false, false, false, true, false, true, false);
    let s = a.to_string() + "Err9";
    new_fn_definition_unchecked.block =
        Box::new(syn::parse(expand_and_check_block(&*(new_fn_definition_unchecked.block), &None, &None, &None, &None, false, false, false, false, true, false, true, false)).expect(&s));

    let mut new_fn_definition_checked = fn_definition.clone();
    let new_fn_name_checked = Ident::new(&new_fn_name_checked, fn_definition.span());
    new_fn_definition_checked.sig.ident = new_fn_name_checked.clone();
    new_fn_definition_checked.block = Box::new(syn::parse(expand_and_check_block(&*(new_fn_definition_checked.block), &None, &None, &None, &None, false, true, false, false, true, false, true, false)).expect("Err0"));

    fn_definition.block = Box::new(syn::parse(expand_and_check_block(&*(fn_definition.block), &None, &None, &None, &None, false, false, false, false, false, false, false, true)).expect("Err0"));
    let self_block = if is_method {
        quote! {Self::}
    } else {
        quote! {}
    };

    let vetted_type = if is_const {
        quote::quote! {
            /*::secret_structs::secret::Vetted*/impl ::secret_structs::secret::VettedTrait
        }
    } else {
        quote::quote! {
            impl ::secret_structs::secret::VettedTrait
        }
    };
    
    let gen = quote! {
        #fn_definition

        #[allow(unused_parens)]
        #new_fn_definition_unchecked
        
        #[allow(unused_parens)]
        #[allow(unused_mut)]
        #new_fn_definition_checked

        #[inline(always)]
        #fn_access #fn_const unsafe fn #new_fn_name_wrapper<#generic_params>(#fn_args) -> #vetted_type<#fn_return_type> #where_clause {
                ::secret_structs::secret::vetted_wrap::<#fn_return_type>(#self_block #new_fn_name_unchecked(#param_names))
        }
    };
    gen.into()
}

// Based on https://blog.turbo.fish/proc-macro-simple-derive/
#[proc_macro_derive(InvisibleSideEffectFreeDerive)]
pub fn secret_block_safe_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => syn::punctuated::Punctuated::new(),
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(fields),
            ..
        }) => fields.unnamed,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let st_name = input.ident;
    let st_generics = input.generics;
    let st_generics_params = st_generics.clone().params;
    let st_generics_names = st_generics_params
        .into_iter()
        .map(|p: syn::GenericParam| match p {
            syn::GenericParam::Type(y) => y.ident,
            _ => panic!("can only support GenericParam::Type, not other GenericParam::*"),
        });
    let st_generics_names: proc_macro2::TokenStream = quote! {
        <#(#st_generics_names,)*>
    };
    let st_where_clause = st_generics.clone().where_clause;

    let getters = fields.into_iter().map(|f| {
            // Interpolation only works for variables, not arbitrary expressions.
            // That's why we need to move these fields into local variables first
            // (borrowing would also work though).
            let field_ty = f.ty;
            quote! {
                ::secret_structs::secret::check_type_is_secret_block_safe::<#field_ty>();
            }
        });

    let st_n = st_name.clone();
    let st_string = quote::quote! { #st_n };
    let wrapped_trait_name = proc_macro2::TokenStream::from_str(("WrappedDrop___".to_owned() + &st_string.to_string()).as_str()).unwrap();

    // Build the output, possibly using quasi-quotation
    let expanded: proc_macro2::TokenStream = quote! {
        #[automatically_derived]
        unsafe impl #st_generics ::secret_structs::secret::InvisibleSideEffectFree for #st_name #st_generics_names #st_where_clause {
            unsafe fn check_all_types() {
                #(#getters)*
            }
        }
        //::secret_structs::assert_not_impl_all!(#st_name #st_generics_names #st_where_clause: ::std::ops::Drop);
        //#[automatically_derived]
        //impl #st_generics ::std::ops::Drop for #st_name #st_generics_names #st_where_clause { fn drop(&mut self) {} }
        pub unsafe trait #wrapped_trait_name {}
        unsafe impl<T: Drop> #wrapped_trait_name for ::secret_structs::secret::Wrapped<T> {}
        #[automatically_derived]
        impl #st_generics !#wrapped_trait_name for ::secret_structs::secret::Wrapped<#st_name #st_generics_names> #st_where_clause {}
        #[automatically_derived]
        impl #st_generics !::std::ops::Deref for #st_name #st_generics_names #st_where_clause {}
        #[automatically_derived]
        impl #st_generics !::std::ops::DerefMut for #st_name #st_generics_names #st_where_clause {}
        //#[automatically_derived]
        //impl <T> !::std::ops::Add<T> for #st_name #st_generics_names #st_where_clause {}
        /*impl Add<i32> {
            type = i32
            impl Add {
                SafeAdd(other)
            }
        }*/

        //impl_for_sbs!(Add, #code, Sub, #code) -> #[side_effect_free_attr] fn SafeAdd(), fn SafeSub()
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

// Based on https://blog.turbo.fish/proc-macro-simple-derive/
#[proc_macro_derive(InvisibleSideEffectFreeDeriveEnum)]
pub fn secret_block_safe_macro_enum(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let variants = match input.data {
        Data::Enum(e) => e.variants,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let fields = variants.into_iter().flat_map(|v| {
        match v.fields {
            Fields::Named(fields) => fields.named.into_iter(),
            Fields::Unit => syn::punctuated::Punctuated::<syn::Field, Comma>::new().into_iter(),
            Fields::Unnamed(fields) => fields.unnamed.into_iter(),
        }
    });

    let getters = fields.map(|f| {
            // Interpolation only works for variables, not arbitrary expressions.
            // That's why we need to move these fields into local variables first
            // (borrowing would also work though).
            let field_ty = f.ty;
            quote! {
                ::secret_structs::secret::check_type_is_secret_block_safe::<#field_ty>();
            }
        });

    let st_name = input.ident;
    let st_generics = input.generics;
    let st_generics_params = st_generics.clone().params;
    let st_generics_names = st_generics_params
        .into_iter()
        .map(|p: syn::GenericParam| match p {
            syn::GenericParam::Type(y) => y.ident,
            _ => panic!("can only support GenericParam::Type, not other GenericParam::*"),
        });
    let st_generics_names: proc_macro2::TokenStream = quote! {
        <#(#st_generics_names,)*>
    };
    let st_where_clause = st_generics.clone().where_clause;

    let st_n = st_name.clone();
    let st_string = quote::quote! { #st_n };
    let wrapped_trait_name = proc_macro2::TokenStream::from_str(("WrappedDrop___".to_owned() + &st_string.to_string()).as_str()).unwrap();

    // Build the output, possibly using quasi-quotation
    let expanded: proc_macro2::TokenStream = quote! {
        #[automatically_derived]
        unsafe impl #st_generics ::secret_structs::secret::InvisibleSideEffectFree for #st_name #st_generics_names #st_where_clause {
            unsafe fn check_all_types() {
                #(#getters)*
            }
        }
        //::secret_structs::assert_not_impl_all!(#st_name #st_generics_names #st_where_clause: ::std::ops::Drop);
        //#[automatically_derived]
        //impl #st_generics ::std::ops::Drop for #st_name #st_generics_names #st_where_clause { fn drop(&mut self) {} }
        pub unsafe trait #wrapped_trait_name {}
        unsafe impl<T: Drop> #wrapped_trait_name for ::secret_structs::secret::Wrapped<T> {}
        #[automatically_derived]
        impl #st_generics !#wrapped_trait_name for ::secret_structs::secret::Wrapped<#st_name #st_generics_names> #st_where_clause {}
        #[automatically_derived]
        impl #st_generics !::std::ops::Deref for #st_name #st_generics_names #st_where_clause {}
        #[automatically_derived]
        impl #st_generics !::std::ops::DerefMut for #st_name #st_generics_names #st_where_clause {}
        //#[automatically_derived]
        //impl <T> !::std::ops::Add<T> for #st_name #st_generics_names #st_where_clause {}
        /*impl Add<i32> {
            type = i32
            impl Add {
                SafeAdd(other)
            }
        }*/

        //impl_for_sbs!(Add, #code, Sub, #code) -> #[side_effect_free_attr] fn SafeAdd(), fn SafeSub()
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}