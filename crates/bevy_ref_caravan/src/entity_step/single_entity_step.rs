use std::str::FromStr;
use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use crate::{
    syntax_in::*,
    syntax_out::*,
    common::collect_until_punct::*,
};

pub(super) fn single_entity_step(
    caravan: TokenIter, 
    package: TokenStream,
    exit_rule: &TokenStream,

    current: TokenTree, 
    //wildcard: EntityBindingKind, 
) ->  Result<(TokenIter, TokenStream), ()> {
    let result = collect_entity_clause(caravan, current);
    let (caravan, mut entity_clause) = match result {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    // Direct. Remove when wildcards are added.
    let Ok(go_method) = TokenStream::from_str(TO_ENTITY_FN) else {
        return Err(())
    };
    entity_clause.extend(go_method);

    todo!(); // Query step goes here (ignoring nesting for now)

    /* 
    // Into query step.
    match wildcard {
        // Add .go() to entity clause, then move to query step.
        EntityBindingKind::Direct => {
            let Ok(go_method) = TokenStream::from_str(TO_ENTITY_FN) else {
                return Err(())
            };
            entity_clause.extend(go_method);
            todo!()
        },
        // Construct lifted binding, add to package, then move to query step.
        EntityBindingKind::Lifted => {
            let entity_clause_iter = TokenStream::from_iter(entity_clause.clone().into_iter()).into_iter(); // Structure conversions...
            let lifted_clause = match create_lifted_clause(entity_clause_iter) { // Create lifted entity clause.
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            let binding = match construct_lifted_binding(lifted_clause, &entity_clause) { // Construct binding (let lifted_clause = entity_clause.go();)
                Ok(ok) => ok,
                Err(err) => return Err(err), 
            };

            caravan.pack(binding); // Add binding to caravan package.
            todo!()
        },
        // Construct overlap binding, add to package, then move to query step.
        EntityBindingKind::Overlap => {
            let binding = match construct_overlap_binding(entity_clause) { // Construct binding (let entity_clause = entity_clause.go();)
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            caravan.pack(binding); // Add binding to caravan package.
            todo!()
        },
        // Can immediately enter query step.
        EntityBindingKind::Literal => {
            todo!()
        },
    }
    */
}

fn collect_entity_clause(
    iter: TokenIter, 
    current: TokenTree
) -> Result<(TokenIter, Vec<TokenTree>), ()> {
    let mut entity_clause = Vec::new();
    entity_clause.push(current);
    return collect_until_clause_end(iter, entity_clause)
}

fn collect_until_clause_end(
    iter: TokenIter, 
    output: Vec<TokenTree>
) -> Result<(TokenIter, Vec<TokenTree>), ()> {
    let (result, iter, output) = until_exact_combo(ENTITY_TO_QUERY_PUNCT.to_vec(), iter, output);
    match result {
        ExactComboFound::WasFound => return Ok((iter, output)),
        ExactComboFound::WasNeverFound => return Err(()),
    }
}

/* 
fn create_lifted_clause(entity_clause: TokenIter) -> Result<TokenStream, ()> {
    // Iterate until the first ident is found, apply lift edits to that part of the token stream, reconstruct token stream and return.

    let (iter, processed, ident) = until_ident(entity_clause); // Get ident.

    let Some(ident) = ident else { // No ident was found; an invalid entity clause.
        return Err(())
    };

    // Apply lift edits
    let ident = ident.to_string();
    let lifted = ident.as_str();
    let lifted = { 
        match ident.strip_prefix(LIFT_PREFIX_REMOVE) {
            Some(prefix_removed) => prefix_removed, // Prefix removed.
            None => &(lifted.to_owned() + LIFT_SUFFIX_ADD) // If no prefix, add suffix,
        }
    };

    // Lifted to token stream.
    let Ok(lifted) = TokenStream::from_str(lifted) else {
        return Err(())
    };

    // Reconstruct complete entity_clause token stream.
    let mut entity_clause = TokenStream::from_iter(processed.into_iter()); // Processed as token stream.
    entity_clause.extend(lifted); // Add lifted ident.
    entity_clause.extend(iter); // Add un-processed tokens.

    return Ok(entity_clause)
}

fn construct_lifted_binding(lifted_clause: TokenStream, entity_clause: &Vec<TokenTree>) -> Result<TokenStream, ()> {
    // Create binding elements
    let Ok(let_token) = TokenStream::from_str("let") else { // let
        return Err(())
    };
    let Ok(eq_token) = TokenStream::from_str("=") else { // =
        return Err(())
    };
    let Ok(go_method) = TokenStream::from_str(&(TO_ENTITY_FN.to_owned() + ";")) else { // .go();
        return Err(())
    };

    // Construct binding
    let mut binding = let_token; // let
    binding.extend(lifted_clause.clone()); // let lifted_clause
    binding.extend(eq_token.clone()); // let lifted_clause =
    binding.extend(entity_clause.clone()); // let lifted_clause = entity_clause
    binding.extend(go_method); // let lifted_clause = entity_clause.go();
    
    // Return
    return Ok(binding)
}

fn construct_overlap_binding(entity_clause: Vec<TokenTree>) -> Result<TokenStream, ()> {
    // Create binding elements
    let Ok(let_token) = TokenStream::from_str("let") else { // let
        return Err(())
    };
    let Ok(eq_token) = TokenStream::from_str("=") else { // =
        return Err(())
    };
    let Ok(go_method) = TokenStream::from_str(&(TO_ENTITY_FN.to_owned() + ";")) else { // .go();
        return Err(())
    };

    // Construct binding
    let mut binding = let_token; // let
    binding.extend(entity_clause.clone()); // let entity_clause
    binding.extend(eq_token); // let entity_clause =
    binding.extend(entity_clause.clone()); // let entity_clause = entity_clause
    binding.extend(go_method); // let entity_clause = entity_clause.go();
    
    // Return
    return Ok(binding)
}
*/