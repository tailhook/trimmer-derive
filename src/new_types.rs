use syn;
use quote;


pub fn derive(ast: &syn::MacroInput, _field: &syn::Field) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl<'render> ::trimmer::Variable<'render> for #name {
            fn attr<'x>(&'x self,  attr: &str)
                -> Result<::trimmer::Var<'x, 'render>, ::trimmer::DataError>
                where 'render: 'x
            {
                self.0.attr(attr)
            }
            fn index<'x>(&'x self,
                key: &(::trimmer::Variable<'render> + 'render))
                -> Result<::trimmer::Var<'x, 'render>, ::trimmer::DataError>
                where 'render: 'x
            {
                self.0.index(key)
            }
            fn output(&self)
                -> Result<::trimmer::Output, ::trimmer::DataError>
            {
                self.0.output()
            }
            fn typename(&self) -> &'static str {
                return stringify!(#name);
            }
            fn as_str_key<'x>(&'x self)
                -> Result<&'x str, ::trimmer::DataError>
            {
                self.0.as_str_key()
            }
            fn as_int_key(&self) -> Result<usize, ::trimmer::DataError> {
                self.0.as_int_key()
            }
            fn as_bool(&self) -> Result<bool, ::trimmer::DataError> {
                self.0.as_bool()
            }
            fn as_number(&self)
                -> Result<::trimmer::Number, ::trimmer::DataError>
            {
                self.0.as_number()
            }
            fn as_comparable(&self)
                -> Result<::trimmer::Comparable, ::trimmer::DataError>
            {
                self.0.as_comparable()
            }

            fn iterate<'x>(&'x self)
                -> Result<Box<Iterator<Item=
                    ::trimmer::Var<'x, 'render>>+'x>,
                    ::trimmer::DataError>
                where 'render: 'x
            {
                self.0.iterate()
            }

            fn iterate_pairs<'x>(&'x self)
                -> Result<Box<Iterator<Item=(
                    ::trimmer::Var<'x, 'render>,
                    ::trimmer::Var<'x, 'render>
                    )>+'x>,
                    ::trimmer::DataError>
                where 'render: 'x
            {
                self.0.iterate_pairs()
            }
        }
    }
}
