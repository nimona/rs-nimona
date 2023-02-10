use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(my_trait))]
struct Opts {
    answer: Option<i32>,
}

#[proc_macro_derive(ToDocument, attributes(my_trait))]
pub fn to_document(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    // let opts = Opts::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, .. } = input;

    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => panic!("Can only be used with structs"),
    };

    let field_into = fields.iter().map(|field| {
        let field_name = &field.ident.as_ref().unwrap();
        quote! {
            map.insert(stringify!(#field_name).to_owned(), self.#field_name.into());
        }
    });

    let output = quote! {
        impl ToDocument for #ident {
            fn to_document(self) -> document::Field {
                let mut map = std::collections::BTreeMap::new();
                #(#field_into);*
                document::Field::Map(map)
            }
        }
    };
    output.into()
}

#[proc_macro_derive(FromDocument, attributes(my_trait))]
pub fn from_document(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let fields = match ast.data {
        Data::Struct(st) => st.fields,
        _ => panic!("Not a struct"),
    };
    let idents: Vec<&Ident> = fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect::<Vec<&Ident>>();

    let keys: Vec<String> = idents
        .clone()
        .iter()
        .map(|ident| ident.to_string())
        .collect::<Vec<String>>();

    let name: &Ident = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let tokens = quote! {
        use document::Field;
        impl #impl_generics FromDocument for #name #ty_generics #where_clause {
            fn from_document(doc: document::Field) -> Result<#name, String> {
                let mut settings: #name = Default::default();
                let map = match doc {
                    document::Field::Map(map) => map,
                    _ => return Err("Not a map".to_owned()),
                };
                #(
                    let value = map.get(#keys).ok_or("Missing key")?;
                    settings.#idents = (value.clone()).into();
                )*
                Ok(settings)
            }
        }
    };
    TokenStream::from(tokens)
}
