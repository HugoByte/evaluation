use super::*;
use syn::Ident;

pub fn impl_sub(struct_name: Ident) -> TokenStream {

    let methods = quote! {
        impl #struct_name{

            pub fn sub(&self) -> i32 {
                self.op1 - self.op2
            }
            pub fn run(&self) -> i32{
                self.sub()
            }
        }
    };
methods.into()
}