use autocxx::prelude::*;

include_cpp! {
	#include "angelscript.h"
	safety!(unsafe)
	generate!("asITypeInfo")
}

#[cfg(test)]
mod tests {
	#[test]
    fn it_works() {
		
    }
}
