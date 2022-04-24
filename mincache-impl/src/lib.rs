use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{AttributeArgs, parse_macro_input, parse_quote, ItemFn, ReturnType};
use darling::FromMeta;

#[derive(Debug, FromMeta)]
struct MacroArgs {
	t: u64,
	fmt: String,

	#[darling(default)]
	reference: Option<bool>,
}

/// A timed cache.
/// The function will be called once, and then wait the specified amount of time until it is able to be called again.
/// In the mean time it returns the cached value, being cloned each time as an owned value.
#[proc_macro_attribute]
pub fn timed(args: TokenStream, item: TokenStream) -> TokenStream {
	let attr_args = parse_macro_input!(args as AttributeArgs);
	let mut func = parse_macro_input!(item as ItemFn);

	let args = match MacroArgs::from_list(&attr_args) {
		Ok(v) => v,
		Err(e) => { return TokenStream::from(e.write_errors()); }
	};

	let innerfn_name = format_ident!("inner_{}", func.sig.ident);
	let time_const_var = format_ident!("__TIME_SEC_{}", func.sig.ident);
	let time_var = format_ident!("__LAST_TIME_{}", func.sig.ident);
	let last_val_var = format_ident!("__LAST_VAL_{}", func.sig.ident);

	let ret_type = if let ReturnType::Type(_, ret_type) = &func.sig.output {
		ret_type
	} else {
		panic!("mincache: function must have a return type (for now)");
	};

	let stmts = func.block.stmts.clone();
	let inputs = &func.sig.inputs;

	let function_args = func.sig.inputs
		.iter()
		.map(|arg| match arg {
			syn::FnArg::Typed(arg) => &arg.pat,
			_ => panic!("mincache: function arguments must be named")
		})
		.collect::<Vec<_>>();

	func.block.stmts.clear();

	// Cooldown hasn't passed. Return last value.
	let no_cooldown = if args.reference.unwrap_or(false) {
		quote! {
			{
				#[allow(unused_unsafe)]
				return unsafe { (*#last_val_var).as_ref().unwrap_unchecked() };
			}
		}
	} else {
		quote! {
			{
				#[allow(unused_unsafe)]
				return unsafe { (*#last_val_var).clone().unwrap_unchecked() }
			}
		}
	};

	let initialize = if args.reference.unwrap_or(false) {
		quote! {
			{
				// First initialization OR time has passed
				let __ret = #innerfn_name( #(#function_args),* );
				unsafe {
					*#time_var.get_mut() = Some(__now);
					*#last_val_var.get_mut() = Some(__ret);
				}
				return __ret;
			}
		}
	} else {
		quote! {
			{
				// First initialization OR time has passed
				let __ret = #innerfn_name( #(#function_args),* );
				unsafe {
					*#time_var.get_mut() = Some(__now);
					*#last_val_var.get_mut() = Some(__ret.clone());
				}
				return __ret;
			}
		}
	};


	func.block.stmts.push(parse_quote! {
		{
			#[inline(always)]
			fn #innerfn_name( #inputs ) -> #ret_type {
				#(#stmts)*
			}

			let __now = std::time::Instant::now();
			match *#time_var {
				Some(last_time) if __now.duration_since(last_time) < #time_const_var  => #no_cooldown,
				_ => #initialize
			}
		}
	});


	let timefn = format_ident!("from_{}", args.fmt);
	let time = args.t;

	TokenStream::from(quote! {
		#[allow(non_upper_case_globals)]
		const #time_const_var: core::time::Duration = core::time::Duration::#timefn(#time);
		#[allow(non_upper_case_globals)]
		static #time_var: mincache::SyncUnsafeCell<Option<std::time::Instant>> = mincache::SyncUnsafeCell::new(None);
		#[allow(non_upper_case_globals)]
		static #last_val_var: mincache::SyncUnsafeCell<Option<#ret_type>> = mincache::SyncUnsafeCell::new(None);

		#func
	})
}