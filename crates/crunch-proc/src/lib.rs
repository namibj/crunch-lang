use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, AttributeArgs, DeriveInput, Error, ItemFn,
    Lit, Meta, MetaNameValue, NestedMeta, Result, Variant,
};

/// A macro for use in the parser, inserts a stack frame recording into the function
///
/// ```rust
/// # use crunch_proc::recursion_guard;
/// # struct Dummy;
/// # impl Dummy {
/// #     fn add_stack_frame(&self) -> Result<(), ()> { Ok(()) }
/// #
/// // Counting a the function as a single frame
/// #[recursion_guard]
/// # fn a(&self) -> Result<(), ()> { Ok(()) }
/// #[recursion_guard()]
/// # fn b(&self) -> Result<(), ()> { Ok(()) }
///
/// // Counting the function as more than one frame, `1` can be any integer
/// #[recursion_guard(1)]
/// # fn c(&self) -> Result<(), ()> { Ok(()) }
/// #[recursion_guard(frames = 1)]
/// # fn d(&self) -> Result<(), ()> { Ok(()) }
/// # }
/// ```
#[proc_macro_attribute]
pub fn recursion_guard(attrs: TokenStream1, input: TokenStream1) -> TokenStream1 {
    recursion_guard_inner(
        parse_macro_input!(attrs as _),
        parse_macro_input!(input as _),
    )
    .unwrap_or_else(|err| err.to_compile_error())
    .into()
}

fn recursion_guard_inner(mut meta: AttributeArgs, mut input: ItemFn) -> Result<TokenStream> {
    let frames_added = if meta.is_empty() {
        Ok(1)
    } else if meta.len() > 1 {
        Err(Error::new(
            Span::join(
                &meta.first().expect("There's more than 1").span(),
                meta.last().expect("There's more than 1").span(),
            )
            .expect("An attribute can only be from one file"),
            "Only one item is allowed for declaring the number of frames to add",
        ))
    } else {
        let meta = meta.pop().expect("There is exactly 1");

        match meta {
            NestedMeta::Lit(Lit::Int(int)) => int.base10_parse(),

            NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                path,
                lit: Lit::Int(int),
                ..
            })) if path.is_ident("frames") => int.base10_parse(),

            NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                lit: Lit::Int(..), ..
            })) => Err(Error::new(meta.span(), "Only integer literals are allowed")),

            meta => Err(Error::new(meta.span(), "Unrecognized attribute")),
        }
    }?;

    let block = &mut input.block;
    let add_stack_frames = (0..frames_added).map(|i| {
        let frame_name = format_ident!("__frame_{}", i.to_string());

        quote! {
            let #frame_name = self.add_stack_frame()?;
        }
    });

    *block = parse_quote!({
        #(#add_stack_frames)*

        #block
    });

    Ok(quote! {
        #input
    })
}

#[proc_macro_attribute]
pub fn instrument(attrs: TokenStream1, input: TokenStream1) -> TokenStream1 {
    instrument_inner(
        parse_macro_input!(attrs as _),
        parse_macro_input!(input as _),
    )
    .unwrap_or_else(|err| err.to_compile_error())
    .into()
}

fn instrument_inner(mut meta: AttributeArgs, mut input: ItemFn) -> Result<TokenStream> {
    let function_name = if meta.is_empty() {
        Ok(input.sig.ident.to_string())
    } else if meta.len() > 1 {
        Err(Error::new(
            Span::join(
                &meta.first().expect("There's more than 1").span(),
                meta.last().expect("There's more than 1").span(),
            )
            .expect("An attribute can only be from one file"),
            "Only one item is allowed for declaring the number of frames to add",
        ))
    } else {
        let meta = meta.pop().expect("There is exactly 1");

        match meta {
            NestedMeta::Lit(Lit::Str(name)) => Ok(name.value()),
            NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                path,
                lit: Lit::Str(name),
                ..
            })) if path.is_ident("name") => Ok(name.value()),

            NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                lit: Lit::Str(..), ..
            })) => Err(Error::new(meta.span(), "Only string literals are allowed")),

            meta => Err(Error::new(meta.span(), "Unrecognized attribute")),
        }
    }?;

    let block = &mut input.block;

    let instrument_name = format_ident!("__instrument_{}", function_name.replace(" ", "_"),);
    *block = parse_quote!({
        let #instrument_name = crunch_shared::utils::Timer::start(#function_name);

        #block
    });

    Ok(quote! {
        #input
    })
}

#[proc_macro_derive(Nanopass)]
pub fn nanopass_derive(input: TokenStream1) -> TokenStream1 {
    nanopass_derive_inner(parse_macro_input!(input as _))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn nanopass_derive_inner(_input: DeriveInput) -> Result<TokenStream> {
    todo!()
}

#[proc_macro_attribute]
pub fn nanopass(attrs: TokenStream1, input: TokenStream1) -> TokenStream1 {
    nanopass_attr_inner(
        parse_macro_input!(attrs as _),
        parse_macro_input!(input as _),
    )
    .unwrap_or_else(|err| err.to_compile_error())
    .into()
}

fn nanopass_attr_inner(mut args: AttributeArgs, input: ItemFn) -> Result<TokenStream> {
    let _accepts: Variant = match attr_arg("accepts", &mut args)? {
        Lit::Str(accepts) => accepts.parse()?,
        err => return Err(Error::new(err.span(), "expected a string literal")),
    };
    let _output: Variant = match attr_arg("outputs", &mut args)? {
        Lit::Str(outputs) => outputs.parse()?,
        err => return Err(Error::new(err.span(), "expected a string literal")),
    };

    Ok(TokenStream::from(quote! { #input }))
}

fn attr_arg(name: &str, args: &mut AttributeArgs) -> Result<Lit> {
    let idx = args.iter().position(|arg| {
        matches!(arg, NestedMeta::Meta(Meta::NameValue(value)) if value.path.is_ident(name))
    })
    // TODO: Return error
    .unwrap();

    if let NestedMeta::Meta(Meta::NameValue(MetaNameValue { lit, .. })) = args.remove(idx) {
        Ok(lit)
    } else {
        unreachable!()
    }
}

/*
enum Tree {
    A(u32, u32, u32),
    B(bool),
    C(Box<Tree>),
}

enum FinalTree {
    WasA(u64),
    WasB(Box<FinalTree>),
}

enum ATree {
    WasA(u64),
    B(bool),
    C(Box<ATree>),
}

fn a(t: Tree) -> ATree {
    match t {
        Tree::A(a, b, c) => ATree::WasA((a + b + c) as u64),
        Tree::B(b) => ATree::B(b),
        Tree::C(t) => ATree::C(Box::new(a(*t))),
    }
}

enum BTree {
    WasA(u64),
    WasB(Box<BTree>),
    C(Box<BTree>),
}

fn b(t: ATree) -> BTree {
    match t {
        ATree::WasA(a) => BTree::WasA(a),
        ATree::B(b) => BTree::WasB(Box::new(BTree::WasA(b as u64))),
        ATree::C(t) => BTree::C(Box::new(b(*t))),
    }
}

enum CTree {
    WasA(u64),
    WasB(Box<CTree>),
}

fn c(t: BTree) -> CTree {
    match t {
        BTree::WasA(a) => CTree::WasA(a),
        BTree::WasB(t) => CTree::WasB(Box::new(c(*t))),
        BTree::C(t) => CTree::WasB(Box::new(c(*t))),
    }
}

fn fin(t: CTree) -> FinalTree {
    match t {
        CTree::WasA(a) => FinalTree::WasA(a),
        CTree::WasB(b) => FinalTree::WasB(Box::new(fin(*b))),
    }
}
*/
