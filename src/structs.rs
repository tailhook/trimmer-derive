use syn;
use quote;


pub fn derive(ast: &syn::MacroInput, fields: &[syn::Field]) -> quote::Tokens
{
    let name = &ast.ident;
    let items: Vec<_> = fields.iter().map(|f| {
        let ident = &f.ident;

        quote! {
            stringify!(#ident) => Ok(::trimmer::Var::borrow(&self.#ident))
        }
    }).collect();
    quote! {
        impl<'render> ::trimmer::Variable<'render> for #name {
            fn typename(&self) -> &'static str {
                return stringify!(#name);
            }
            fn attr<'x>(&'x self,  attr: &str)
                -> Result<::trimmer::Var<'x, 'render>, ::trimmer::DataError>
                where 'render: 'x
            {
                match attr {
                    #(#items,)*
                    _ => Err(::trimmer::DataError::AttrNotFound),
                }
            }
        }
    }
}
