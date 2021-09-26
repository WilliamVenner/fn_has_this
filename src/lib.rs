#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{AttributeArgs, BareFnArg, FnArg, ItemFn, ItemStatic, ItemType, LitStr, Pat, PatIdent, PatType, Type, TypeBareFn, parse_macro_input};
use syn_squash::syn_squash;

fn into_bare_fn_arg(fn_arg: FnArg) -> BareFnArg {
	if let FnArg::Typed(fn_arg) = fn_arg {
		BareFnArg {
			attrs: fn_arg.attrs,
			name: None,
			ty: *fn_arg.ty,
		}
	} else {
		panic!("`self` parameter is only allowed in associated functions");
	}
}

syn_squash! {
	syn_squash_fn => {
		default! => {
			fn add_this_arg(&mut self, arg: FnArg);
		};

		ItemFn => {
			fn add_this_arg(&mut self, arg: FnArg) {
				self.sig.inputs.insert(0, arg);
			}
		};

		TypeBareFn => {
			fn add_this_arg(&mut self, arg: FnArg) {
				self.inputs.insert(0, into_bare_fn_arg(arg));
			}
		};

		ItemStatic => {
			fn add_this_arg(&mut self, arg: FnArg) {
				if let syn::Type::BareFn(ref mut function) = *self.ty {
					function.inputs.insert(0, into_bare_fn_arg(arg));
				} else {
					panic!("Only bare function types are supported, please use the macro on a type alias instead");
				}
			}
		};

		ItemType => {
			fn add_this_arg(&mut self, arg: FnArg) {
				let mut alias = syn::parse::<TypeBareFn>(self.ty.to_token_stream().into()).expect("Only bare function types are supported, please use the macro on a type alias instead");
				alias.inputs.insert(0, into_bare_fn_arg(arg));
				self.ty = Box::new(Type::BareFn(alias));
			}
		}
	}
}

#[proc_macro_attribute]
#[doc = include_str!("../README.md")]
pub fn has_this(args: TokenStream, input: TokenStream) -> TokenStream {
	let mut args = parse_macro_input!(args as AttributeArgs);
	assert!(args.len() <= 1, "Please provide the name and type of your `this` variable, e.g.: `has_this(this: *mut c_void)` or `has_this(*mut c_void)`");

	let mut input = syn_squash_fn(input).expect("fn_has_this is not supported on this item");

	let this_arg: TokenStream = args.remove(0).into_token_stream().into();
	let this_arg = parse_macro_input!(this_arg as LitStr);
	let this_arg = this_arg.value();
	let this_arg = match syn::parse_str::<FnArg>(&this_arg).or_else(|_| {
		syn::parse_str::<BareFnArg>(&this_arg).map(|arg| {
			FnArg::Typed(PatType {
				attrs: arg.attrs,
				pat: Box::new(Pat::Ident(PatIdent {
					attrs: vec![],
					by_ref: None,
					mutability: None,
					ident: syn::parse_str("this").unwrap(),
					subpat: None,
				})),
				ty: Box::new(arg.ty),
				colon_token: Default::default(),
			})
		})
	}) {
		Ok(this_arg) => this_arg,
		Err(err) => return err.into_compile_error().into(),
	};

	input.add_this_arg(this_arg);

	input.into_tokens().into()
}
