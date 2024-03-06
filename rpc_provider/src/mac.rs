#[macro_export]
macro_rules! rpc_params {
	($($param:expr),*) => {
		{
            use $crate::primitives::RpcParams;

			let mut params = RpcParams::new();
			$(
				if let Err(err) = params.insert($param) {
					panic!("Parameter `{}` cannot be serialized: {:?}", stringify!($param), err);
				}
			)*
			params
		}
	};
}

#[macro_export]
macro_rules! no_params {
    () => {
        crate::RpcParams::default()
    };
}
