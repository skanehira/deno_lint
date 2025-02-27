// Copyright 2020-2021 the Deno authors. All rights reserved. MIT license.
use deno_ast::get_syntax;
use deno_ast::Diagnostic;
use deno_ast::MediaType;
use deno_ast::ParsedSource;

pub(crate) fn parse_program(
  file_name: &str,
  media_type: MediaType,
  source_code: String,
) -> Result<ParsedSource, Diagnostic> {
  let syntax = get_syntax(media_type);
  deno_ast::parse_program(deno_ast::ParseParams {
    specifier: file_name.to_string(),
    media_type,
    text_info: deno_ast::SourceTextInfo::from_string(source_code),
    capture_tokens: true,
    maybe_syntax: Some(syntax),
    scope_analysis: true,
  })
}
