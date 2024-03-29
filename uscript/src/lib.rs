use std::any::{Any, TypeId};
use std::ops::Deref;
use rhai::plugin::CallableFunction;

mod dyonn;
mod rhaii;

pub trait RegisterNativeFunction< A: 'static, const N: usize, const X: bool, R: 'static, const F: bool > {
	/// Convert this function into a [`CallableFunction`].
	#[must_use]
	fn into_callable_function(self, name: Identifier, no_const: bool) -> CallableFunction;

	/// Get the type ID's of this function's parameters.
	#[must_use]
	fn param_types() -> [TypeId; N];

	/// Get the number of parameters for this function.
	#[inline(always)]
	#[must_use]
	fn num_params() -> usize {
		N
	}

	/// Is there a [`NativeCallContext`] parameter for this function?
	#[inline(always)]
	#[must_use]
	fn has_context() -> bool {
		X
	}
}

#[test]
fn testing() {
	fn hello() { println!("h") }

	pub fn register_fn< A: 'static, const N: usize, const C: bool, R: Variant + Clone, const L: bool, F: RegisterNativeFunction<A, N, C, R, L> >( name: impl AsRef<str>, func: F, ) {
		let param_types = F::param_types();

		let mut param_type_names: crate::StaticVec<_> = F::param_names()
			.iter()
			.map( |ty| format!( "_: {}", self.format_type_name( ty ) ) )
			.collect();

		if F::return_type() != TypeId::of::<()>() {
			param_type_names.push( self.format_type_name( F::return_type_name() ).into() );
		}

		let param_type_names: crate::StaticVec<_> = param_type_names.iter().map( String::as_str ).collect();
		let param_type_names = Some( param_type_names.as_ref() );

		let fn_name = name.as_ref();

		let no_const = (
			F::num_params() == 3 && fn_name == "index$set$"
		) || (
			F::num_params() == 2 && fn_name.starts_with( "set$" )
		);

		let func = func.into_callable_function(fn_name.into(), no_const);


	}
}
