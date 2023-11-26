use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, punctuated::Punctuated, token::Comma, BareFnArg};

const X64_ARG_REGS: [&str; 6] = ["rdi", "rsi", "rdx", "r10", "r8", "r9"];

#[proc_macro_attribute]
pub fn syscall(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_type = parse_macro_input!(item as syn::ItemType);

    let bare_fn = match *item_type.ty.clone() {
        syn::Type::BareFn(bare_fn) => bare_fn,
        _ => {
            panic!("Must be a fn type eg. fn(input: usize) -> usize");
        }
    };

    let inputs = bare_fn.inputs;
    if inputs.len() > 6 {
        panic!("A syscall has a maximum of six arguments")
    }

    let vis = item_type.vis;
    let fn_name = format_ident!("{}", item_type.ident.to_string().to_case(Case::Snake));
    let return_type = bare_fn.output;

    let x86_64_asm_tokens = x86_64_asm_tokens(&inputs);
    //let x86_asm_tokens = x86_asm_tokens(&inputs);
    let sys_num = proc_macro2::TokenStream::from(attr);

    let tokens = quote! {
        #[inline(always)]
        #[cfg(target_arch = "x86_64")]
        #vis unsafe fn #fn_name(#inputs) #return_type {
            let mut rax = #sys_num as _;
            #x86_64_asm_tokens
            rax
        }
        /*#[inline(always)]
        #[cfg(target_arch = "x86")]
        #vis unsafe fn #fn_name(#inputs) #return_type {
            let mut rax = #sys_num as _;
            #x86_asm_tokens
            rax
        }*/
    };

    TokenStream::from(tokens)
}

fn x86_64_asm_tokens(inputs: &Punctuated<BareFnArg, Comma>) -> proc_macro2::TokenStream {
    let map_fnargs_to_reg_tokens = inputs.iter().enumerate().map(|(i, e)| {
        let register_ident = X64_ARG_REGS[i];
        if let Some(variable_str) = &e.name {
            let variable_ident = variable_str.0.clone();
            quote!(in(#register_ident) #variable_ident)
        } else {
            panic!("BareFnArg must have a name")
        }
    });

    quote! {
        core::arch::asm!(
            "syscall",
            inout("rax") rax,
            #(#map_fnargs_to_reg_tokens),*,
            clobber_abi("system"),
            options(nostack)
        );
    }
}
