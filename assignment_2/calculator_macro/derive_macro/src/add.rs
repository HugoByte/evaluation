use super::*;
use syn::Ident;

pub fn impl_add(struct_name: Ident) -> TokenStream {

    let methods = quote! {
        impl #struct_name{

            pub fn add(&self) -> i32 {
                self.op1 + self.op2
            }
            pub fn run(&self) -> i32{
               self.add()
            }

        }
    };
methods.into()
}