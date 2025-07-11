use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, punctuated::Punctuated, ItemFn, Meta, Token};

/// Marks a function as the entry point for a WUPS application.
/// 
/// Optionally, a list of attribute values of available logger channels can be used to set up logging.
/// 
/// As the Wii U needs custom code to run before, in, and after main this macro hides this implementation from the user.
/// 
/// # Example
/// 
/// ```rust
/// #![no_std]
/// #![no_main]
/// 
/// use wut;
/// 
/// #[wut::main(Udp)]
/// fn main() { }
/// ```
#[proc_macro_attribute]
pub fn main(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated)
        .into_iter()
        .collect::<Vec<_>>();

    let func_name = &input.sig.ident;
    let block = &input.block;

    // Parse the attributes and generate the corresponding code
    let mut custom_args = Vec::new();
    for arg in args {
        if let Meta::Path(path) = arg {
            if let Some(ident) = path.get_ident() {
                custom_args.push(quote! { ::wut::logger::#ident });
            }
        }
    }

    let expanded = quote! {
        #[unsafe(no_mangle)]
        pub extern "C" fn #func_name() {
            ::wut::process::init(#(#custom_args)|*);
            #block
            ::wut::process::deinit();
        }
    };

    TokenStream::from(expanded)
}

/// Automatically implements the gx2 `Attributes` trait for a struct.
/// 
/// The struct should only contain named fields of type `wut::gx2::shader::Attribute` and can optionally contain the following (Rust) attributes:
/// * #\[name = `&str`\]: sets the name of the attribute in the shader. Defaults to field name.
/// * #\[index = `u32`\]: sets the internal attribute index. Defaults to 0 for the first field and increaes with *every* field from the last value.
/// * #\[offset = `u32`\]: sets the internal attribute offset. Defaults to 0.
/// 
/// # Example
/// 
/// ```
/// #[derive(ShaderAttributes)]
/// struct MyShader {
///     #[name = "aPosition"]
///     // #[index = 0]
///     // #[offset = 3]
///     a_position: wut::gx2::shader::Attribute<Float4>,
///     #[name = "aColour"]
///     // #[index = 1]
///     // #[offset = 0]
///     a_color: wut::gx2::shader::Attribute<Float4>,
/// }
/// ```
#[proc_macro_derive(ShaderAttributes, attributes(name, index, offset))]
pub fn gx2_attributes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    // Process the struct fields
    let fields = match input.data {
        syn::Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields) => &fields.named,
            _ => panic!("Gx2Shader can only be derived for structs with named fields"),
        },
        _ => panic!("Gx2Shader can only be derived for structs"),
    };

    let mut index = 0;

    let attribute_impl = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().expect("Field must have a name");

            // check if field.ty is Attribute (wut::gx2::render::test::Attribute) else panic
            if !field.ty.to_token_stream().to_string().contains("Attribute") {
                panic!("Field \"{}\" must be of type `wut::gx2::shader::attribute::Attribute`", field_name);
            }
            

            let mut name = field_name.to_string();
            let mut offset = 0;

            for attr in &field.attrs {
                if attr.path().is_ident("name") {
                    if let Ok(meta) = attr.meta.require_name_value() {
                        if let syn::Expr::Lit(expr_lit) = &meta.value {
                            if let syn::Lit::Str(lit) = &expr_lit.lit {
                                name = lit.value();
                            }
                        }
                    }
                } else if attr.path().is_ident("index") {
                    if let Ok(meta) = attr.meta.require_name_value() {
                        if let syn::Expr::Lit(expr_lit) = &meta.value {
                            if let syn::Lit::Int(lit) = &expr_lit.lit {
                                index = lit.base10_parse::<u32>().unwrap();
                            }
                        }
                    }
                } else if attr.path().is_ident("offset") {
                    if let Ok(meta) = attr.meta.require_name_value() {
                        if let syn::Expr::Lit(expr_lit) = &meta.value {
                            if let syn::Lit::Int(lit) = &expr_lit.lit {
                                offset = lit.base10_parse::<u32>().unwrap();
                            }
                        }
                    }
                }
            }

            let out = quote! {
                #field_name: ::wut::gx2::shader::attribute::Attribute::new(group, #name, #index, #offset)?,
            };

            index += 1;

            out
        })
        .collect::<Vec<_>>();

    let struct_name = &input.ident;

    let output = quote! {
        impl ::wut::gx2::shader::attribute::Attributes for #struct_name {
            fn new(group: &mut ::wut::sys::WHBGfxShaderGroup) -> Result<Self, ()> {
                Ok(Self {
                    #(#attribute_impl)*
                })
            }
        }
    };

    output.into()
}

// #[proc_macro]
// pub fn glsl(input: TokenStream) -> TokenStream {

//     let str = input.to_string();

//     let output = quote! {
//         #str
//     };
//     output.into()
// }

// #[proc_macro]
// pub fn assemble(input: TokenStream) -> TokenStream {

//     let input = parse_macro_input!(input as ShaderCode);

//     let vertex = input.vertex;

//     let output = quote! {
//         ""
//     };
//     output.into()
// }

// struct ShaderCode {
//     vertex: String,
//     fragment: String
// }


// impl syn::parse::Parse for ShaderCode {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         // Parse vertex field
//         input.parse::<syn::Ident>()?; // Parse "vertex"
//         input.parse::<syn::Token![:]>()?; // Parse ":"
        
//         let vertex_content = input.parse::<syn::Block>()?;
//         // let vertex = vertex_content.stmts.
        
//         // Parse comma
//         input.parse::<syn::Token![,]>()?;
        
//         // Parse fragment field
//         input.parse::<syn::Ident>()?; // Parse "fragment"
//         input.parse::<syn::Token![:]>()?; // Parse ":"
        
//         // Get fragment shader code
//         let fragment_context = input.parse::<syn::Block>()?;
        
//         Ok(ShaderCode {
//             vertex: vertex_content.to_string(),
//             fragment: fragment_context.to_string(),
//         })
//     }
// }
