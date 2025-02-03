use quote::{quote, ToTokens};
use proc_macro2::TokenStream;
use syn::{parse::{Parse, ParseStream}, Ident, Lit, LitStr, Result as ParseResult, Token, Type};

#[derive(Debug)]
pub struct ParsedMapEntry(String, TokenStream);

impl ToTokens for ParsedMapEntry {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let key = &self.0;
    let value = &self.1;

    tokens.extend::<TokenStream>(quote! {
      #key.to_string(), #value
    });
  }
}

pub struct HashMapify {
  value_type: Type,
  entries: Vec<ParsedMapEntry>,
}

impl Parse for HashMapify {
  // input, which is of type ParsedStream in this case, works similar to an iterator. You need to parse tokens out of
  // the input using the method parse on it, which will also advance the stream to the beginning of the next token.
  // For example, if you have a stream of tokens representing [a, b, c], as soon as you parse [ out of this stream, the
  // stream will be mutated to only contain a, b, c] . This is very similar to iterators, where as soon as you take a value
  // out, the iterator is advanced by one position and only holds the remaining items.
  fn parse(input: ParseStream) -> ParseResult<Self> {
    let mut entries = Vec::<ParsedMapEntry>::new();

    if input.is_empty() {
      panic!("At least a type must be specified for an empty hashmap");
    }

    // Parse takes a single type argument which represents what to parse.
    // If the first argument cannot be parsed as a valid type, an error will be returned
    let value_type = input.parse::<Type>()?;

    // Next, parse the `,` token, which you expect to be used to separate the arguments.
    input.parse::<Token![,]>()?;

    // tokens are taken out of the stream and it's advanced each time you parse something. This means that when
    // all of the tokens are parsed, the stream will be empty. 
    while !input.is_empty() {
      let key = if let Ok(key) = input.parse::<Ident>() {
        key.to_string()
      } else if let Ok(key) = input.parse::<LitStr>() {
        key.value()
      } else {
        panic!("Key must be either a identifier literal or a string literal!");
      };

        // Parse the `=` sign, which should be the next token after  a key.
      input.parse::<Token![=]>()?;

      // Next, try to parse the value as an identifier. If it is, it means that it's a variable, so we should convert
      // it to token stream directly
      let value = if let Ok(value) = input.parse::<Ident>() {
        value.to_token_stream()

        // If the input isn't an identifier, try to parse it as a literal value such as `"string"` for strings, `42`
        // for numbers `false` for boolean value, etc.
      } else if let Ok(value) = input.parse::<Lit>() {
        value.to_token_stream()
      } else {
        panic!("Value must be either a literal or an identifier!");
      };

      entries.push(ParsedMapEntry(key, value));

      // Check if next token is a comma, without advancing the stream
      if input.peek(Token![,]) {
        // If it is, then parse it out and advance the stream before moving on to the next key-value pair
        input.parse::<Token![,]>()?;
      }
    }

    Ok(HashMapify {value_type, entries})
  }
}

impl ToTokens for HashMapify {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let kv_pairs = &self.entries;
    let ty = &self.value_type;

    let output = quote! {{
      let mut hash_map = std::collections::HashMap::<String, #ty>::new();
      
      #(
        hash_map.insert(#kv_pairs);
      )*
      
      hash_map
    }};

    tokens.extend::<TokenStream>(output);
  }
}
