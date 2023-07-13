use super::*;
use syn::Ident;

pub fn impl_mul(struct_name: Ident) -> TokenStream {

    let methods = quote! {
        impl #struct_name{

            pub fn mul(&self) -> i32 {
                self.op1 * self.op2
            }
            pub fn run(&self) -> i32{
                self.mul()
            }
        }
    };
methods.into()
}