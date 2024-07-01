use proc_macro::TokenStream;
use syn::Fields;
//use colored::Colorize;
use quote::quote;

fn strip_underscore(input: &str) -> String {
    let mut result = String::new();
    let mut prev_underscore = false;

    for c in input.chars() {
        if c == '_' {
            prev_underscore = true;
        } else if prev_underscore {
            result.push(c.to_ascii_uppercase());
            prev_underscore = false;
        } else {
            result.push(c);
        }
    }
    result
}

#[proc_macro_derive(DBData)]
pub fn dbdata_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_dbdata_macro(&ast)
}

fn impl_dbdata_macro(ast: &syn::DeriveInput) -> TokenStream {
    let table_name = &ast.ident;
    //let vec_name = syn::Ident::new(&format!("{}Vec", table_name), table_name.span());
    let fields: Fields = match ast.data {
        syn::Data::Struct(ref data_struct) => data_struct.fields.clone(),
        _ => unreachable!(),
    };

    let mut table_values = String::new();
    let field_names: Vec<String> = fields
        .clone()
        .into_iter()
        .map(|x| x.ident.unwrap().to_string())
        .collect();
    for f in &field_names {
        table_values.push_str(&strip_underscore(f).to_lowercase());
        table_values.push(',');
    }
    table_values = table_values.trim_end_matches(',').to_string();

    let fields_ammount = fields.len();
    let empty_fields = "?,"
        .repeat(fields_ammount)
        .trim_end_matches(',')
        .to_string();

    let mut field_accessors = quote! {};
    for f in &fields {
        let f = f.ident.clone().unwrap();
        field_accessors = quote! {
            #field_accessors
            .bind(&self.#f)
        }
    }

    let mut vec_field_accessors = quote! {};
    for f in &fields {
        let f = f.ident.clone().unwrap();
        vec_field_accessors = quote! {
            #vec_field_accessors
            .push_bind(t.#f)
        }
    }

    let insert_query = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table_name, table_values, empty_fields
    );

    let gen = quote! {
        impl DBData for #table_name {
            async fn insertar_en_db(&self, pool: &Pool<MySql>) -> Result<(), Box<dyn Error>> {
                match sqlx::query(#insert_query)
                #field_accessors
                .execute(pool)
                .await
                {
                    Ok(_) => incrementar_contador(INFO).await,
                    Err(err) => {
                        notificar_carga(WARN, &err.to_string());
                        incrementar_contador(WARN).await;
                    }
                };
                Ok(())
            }
        }
    };
    gen.into()
}
