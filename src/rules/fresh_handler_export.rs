use std::path::Path;

// Copyright 2020-2023 the Deno authors. All rights reserved. MIT license.
use super::{Context, LintRule};
use crate::handler::{Handler, Traverse};

use deno_ast::view::{Decl, Pat, Program};
use deno_ast::SourceRanged;

#[derive(Debug)]
pub struct FreshHandlerExport;

const CODE: &str = "fresh-handler-export";
const MESSAGE: &str =
  "Fresh middlewares must be exported as \"handler\" but got \"handlers\" instead.";
const HINT: &str = "Did you mean \"handler\"?";

impl LintRule for FreshHandlerExport {
  fn tags(&self) -> &'static [&'static str] {
    &["fresh"]
  }

  fn code(&self) -> &'static str {
    CODE
  }

  fn lint_program_with_ast_view(
    &self,
    context: &mut Context,
    program: Program,
  ) {
    Visitor.traverse(program, context);
  }

  #[cfg(feature = "docs")]
  fn docs(&self) -> &'static str {
    include_str!("../../docs/rules/fresh_handler_export.md")
  }
}

struct Visitor;

impl Handler for Visitor {
  fn export_decl(
    &mut self,
    export_decl: &deno_ast::view::ExportDecl,
    ctx: &mut Context,
  ) {
    // Fresh only considers components in the routes/ folder to be
    // server components.
    let path = Path::new(ctx.file_name());
    if !path
      .components()
      .map(|comp| comp.as_os_str())
      .any(|comp| comp == "routes")
    {
      return;
    }

    let id = match export_decl.decl {
      Decl::Var(var_decl) => {
        if let Some(first) = var_decl.decls.first() {
          let Pat::Ident(name_ident) = first.name else {
            return;
          };
          name_ident.id
        } else {
          return;
        }
      }
      Decl::Fn(fn_decl) => fn_decl.ident,
      _ => return,
    };

    // Fresh middleware handler must be exported as "handler" not "handlers"
    if id.sym().eq("handlers") {
      ctx.add_diagnostic_with_hint(id.range(), CODE, MESSAGE, HINT);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::assert_lint_ok;

  #[test]
  fn fresh_handler_export_name() {
    assert_lint_ok(&FreshHandlerExport, "const handler = {}", "foo.jsx");
    assert_lint_ok(&FreshHandlerExport, "function handler() {}", "foo.jsx");
    assert_lint_ok(&FreshHandlerExport, "export const handler = {}", "foo.jsx");
    assert_lint_ok(
      &FreshHandlerExport,
      "export const handlers = {}",
      "foo.jsx",
    );
    assert_lint_ok(
      &FreshHandlerExport,
      "export function handlers() {}",
      "foo.jsx",
    );

    assert_lint_ok(
      &FreshHandlerExport,
      "export const handler = {}",
      "routes/foo.jsx",
    );
    assert_lint_ok(
      &FreshHandlerExport,
      "export function handler() {}",
      "routes/foo.jsx",
    );
    assert_lint_ok(
      &FreshHandlerExport,
      "export async function handler() {}",
      "routes/foo.jsx",
    );

    assert_lint_err!(FreshHandlerExport, filename: "routes/index.tsx",  r#"export const handlers = {}"#: [
    {
      col: 13,
      message: MESSAGE,
      hint: HINT,
    }]);
    assert_lint_err!(FreshHandlerExport, filename: "routes/index.tsx",  r#"export function handlers() {}"#: [
    {
      col: 16,
      message: MESSAGE,
      hint: HINT,
    }]);
    assert_lint_err!(FreshHandlerExport, filename: "routes/index.tsx",  r#"export async function handlers() {}"#: [
    {
      col: 22,
      message: MESSAGE,
      hint: HINT,
    }]);
  }
}
