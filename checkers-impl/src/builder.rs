use quote::{format_ident, quote, ToTokens};
use syn::spanned::Spanned;
use syn::{parse_quote, DeriveInput};

pub fn builder(
    input: DeriveInput,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
        ..
    }) = input.data
    {
        let (name, vis) = (input.ident, input.vis);
        let builder_name = format_ident!("{}Builder", name);

        let mut builder_fields: Vec<syn::Field> = vec![];
        let mut builder_new: Vec<syn::FieldValue> = vec![];
        let mut build_values: Vec<syn::FieldValue> = vec![];
        let mut setters: Vec<syn::ImplItemFn> = vec![];

        for field in named.clone().iter() {
            let ident = field.ident.as_ref().expect("Field name not found");
            let ty = &field.ty;
            let attrs = &field.attrs;
            // Builder fields block
            {
                if get_inner("Option", ty).is_some()
                    || get_inner("Vec", ty).is_some()
                {
                    builder_fields.push(parse_quote!(
                        #ident: #ty
                    ));
                }
                else {
                    builder_fields.push(parse_quote!(
                        #ident: std::option::Option<#ty>
                    ));
                }
            }
            // Builder new block
            {
                if attrs.iter().any(|attr| {
                    attr.path()
                        .segments
                        .iter()
                        .any(|segment| segment.ident == "builder")
                }) {
                    builder_new.push(parse_quote! {
                        #ident: std::vec![]
                    })
                }
                else {
                    builder_new.push(parse_quote! {
                        #ident: std::option::Option::None
                    });
                }
            }
            // Builder values block
            {
                let inner_option = get_inner("Option", ty);
                let inner_vec = get_inner("Vec", ty);
                if inner_option.is_some() || inner_vec.is_some() {
                    build_values.push(parse_quote! {
                        #ident: self.#ident.clone()
                    });
                }
                else {
                    build_values.push(parse_quote! {
                    #ident: self.#ident.take().ok_or_else(|| format!("{} is not set", stringify!(#ident)))?
                });
                }
            }
            // Setters block
            {
                if let Some(attr) = attrs.iter().find(|attr| {
                    attr.path()
                        .segments
                        .iter()
                        .any(|segment| segment.ident == "builder")
                }) {
                    match attr.parse_nested_meta(|meta| {
                    if meta.path.segments.len() > 1
                        || meta
                            .path
                            .segments
                            .iter()
                            .any(|path_segment| path_segment.ident != "each")
                    {
                        return Err(syn::Error::new_spanned(
                            attr.meta.to_token_stream(),
                            "expected `builder(each = \"...\")`",
                        ));
                    }
                    let lit_str: syn::LitStr = meta.value()?.parse()?;
                    let inner_ty = get_inner("Vec", ty).ok_or(meta.error("Field does not contain a Vec type"))?;
                    let lit_ident = format_ident!("{}", lit_str.value());
                    if let syn::Type::Path(syn::TypePath { path, .. }) = inner_ty {
                        if path.is_ident("String") {
                            setters.push(parse_quote! {
                                #vis fn #lit_ident(&mut self, val: impl AsRef<str>) -> &mut Self {
                                    self.#ident.push(val.as_ref().to_string());
                                    self
                                }
                            });
                            if lit_ident != *ident {
                                setters.push(parse_quote! {
                                    #vis fn #ident(&mut self, #ident:Vec<impl AsRef<str>>) -> &mut Self {
                                        self.#ident = #ident.iter().map(|val| val.as_ref().to_string()).collect();
                                        self
                                    }
                                });
                            }
                        }
                        else {
                            setters.push(parse_quote! {
                                #vis fn #lit_ident(&mut self, val: #inner_ty) -> &mut Self {
                                    self.#ident.push(val);
                                    self
                                }
                            });
                            if lit_ident != *ident {
                                setters.push(parse_quote! {
                                    #vis fn #ident(&mut self, #ident:#ty) -> &mut Self {
                                        self.#ident = #ident.clone();
                                        self
                                    }
                                });
                            }
                        }
                    }
                    else {
                        setters.push(parse_quote! {
                            #vis fn #lit_ident(&mut self, val: #inner_ty) -> &mut Self {
                                self.#ident.push(val);
                                self
                            }
                        });
                        if lit_ident != *ident {
                            setters.push(parse_quote! {
                                #vis fn #ident(&mut self, #ident:#ty) -> &mut Self {
                                    self.#ident = #ident.clone();
                                    self
                                }
                            });
                        }
                    }

                    Ok(())
                }) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
                }
                else {
                    let mut ty_option = ty;
                    if let Some(inner) = get_inner("Option", ty) {
                        ty_option = inner;
                    }
                    if let syn::Type::Path(syn::TypePath { path, .. }) =
                        ty_option
                    {
                        if path.is_ident("String") {
                            setters.push(parse_quote! {
                            #vis fn #ident(&mut self, #ident:impl AsRef<str>) -> &mut Self {
                                self.#ident = std::option::Option::Some(#ident.as_ref().to_string());
                                self
                            }
                        });
                        }
                        else {
                            setters.push(parse_quote! {
                            #vis fn #ident(&mut self, #ident:#ty_option) -> &mut Self {
                                self.#ident = std::option::Option::Some(#ident);
                                self
                            }
                        });
                        }
                    }
                    else {
                        setters.push(parse_quote! {
                        #vis fn #ident(&mut self, #ident:#ty_option) -> &mut Self {
                            self.#ident = std::option::Option::Some(#ident);
                            self
                        }
                    });
                    }
                }
            }
        }

        Ok(quote! {
            impl #name {
                #vis fn builder() -> #builder_name {
                    #builder_name {
                        #(#builder_new),*
                    }
                }
            }

            #vis struct #builder_name {
                #(#builder_fields),*
            }

            impl #builder_name {
                #(#setters)*
                #vis fn build(&mut self) -> std::result::Result<#name, std::boxed::Box<dyn std::error::Error>> {
                    std::result::Result::Ok(#name {
                        #(#build_values),*
                    })
                }
            }
        })
    }
    else {
        Err(syn::Error::new(input.span(), "Named structs are required"))
    }
}

fn get_inner(wrapper: impl AsRef<str>, ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(syn::TypePath {
        path: syn::Path { segments, .. },
        ..
    }) = ty
    {
        let segment_idents: Vec<_> = segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect();
        if segment_idents.last()? == wrapper.as_ref() {
            if let syn::PathSegment {
                arguments:
                    syn::PathArguments::AngleBracketed(
                        syn::AngleBracketedGenericArguments { args, .. },
                    ),
                ..
            } = segments.last()?
            {
                if let syn::GenericArgument::Type(tp) = args.first()? {
                    return Some(tp);
                }
            }
        }
    }
    None
}
