use std::collections::HashMap;

use darling::{ast::NestedMeta, util::PathList, FromMeta};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::ItemTrait;

#[derive(Debug, FromMeta)]
struct MacroMeta {
    modules: PathList,
}

/// behaviorのinner関数
pub fn behavior_inner(args: TokenStream, input_trait: &ItemTrait) -> syn::Result<TokenStream> {
    let MacroMeta { modules } = MacroMeta::from_list(&NestedMeta::parse_meta_list(args)?)?;

    // 各モジュールに対応した関数ブロックのリスト
    let mut modules_fn_impls_map = modules
        .iter()
        .map(|module_path| (module_path, Vec::<TokenStream>::new()))
        .collect::<HashMap<_, _>>();

    let ItemTrait {
        attrs: trait_attrs,
        vis: trait_vis,
        items: trait_items,
        ident: trait_name,
        ..
    } = input_trait;

    for trait_item in trait_items.iter() {
        // 関数(スタティクメソッド)の時のみ
        if let syn::TraitItem::Fn(trait_fn) = trait_item {
            let syn::TraitItemFn {
                attrs: fn_attrs,
                sig: fn_sig,
                ..
            } = trait_fn;

            // 各モジュールの関数ブロックののリストに実装を挿入
            for (module, fn_impls) in modules_fn_impls_map.iter_mut() {
                let arg_names = fn_sig.inputs.iter().filter_map(|fn_arg| {
                    if let syn::FnArg::Typed(typed_fn_arg) = fn_arg {
                        Some(&typed_fn_arg.pat)
                    } else {
                        None
                    }
                });
                let fn_name = &fn_sig.ident;

                if fn_sig.asyncness.is_some() {
                    // 非同期の場合
                    fn_impls.push(quote! {
                        #(#fn_attrs)*
                        #fn_sig
                        {
                            #module :: #fn_name ( #(#arg_names),* ).await
                        }
                    });
                } else {
                    // 同期の場合
                    fn_impls.push(quote! {
                        #(#fn_attrs)*
                        #fn_sig
                        {
                            #module :: #fn_name ( #(#arg_names),* )
                        }
                    });
                }
            }
        }
    }

    // モジュールに対応した型を定義してトレイトを実装する
    let struct_trait_impls = modules_fn_impls_map.iter().map(|(module, fn_impls)| {
        let type_name = {
            let mut type_name_vec = module
                .segments
                .iter()
                .map(|path_segment| path_segment.ident.to_string())
                .collect::<Vec<String>>();
            type_name_vec.push("Module".to_string());
            type_name_vec.push(trait_name.to_string());

            format_ident!(
                "{}",
                heck::AsPascalCase(&type_name_vec.join(" ")).to_string()
            )
        };

        quote! {
            #trait_vis struct #type_name;

            #(#trait_attrs)*
            impl #trait_name for #type_name {
                #(#fn_impls)*
            }
        }
    });

    Ok(quote! {
        #input_trait

        #(#struct_trait_impls)*
    })
}
