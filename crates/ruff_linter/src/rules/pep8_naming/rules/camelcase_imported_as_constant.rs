use ruff_python_ast::{Alias, Stmt};

use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_stdlib::str::{self};
use ruff_text_size::Ranged;

use crate::rules::pep8_naming::helpers;
use crate::settings::types::IdentifierPattern;

/// ## What it does
/// Checks for `CamelCase` imports that are aliased to constant-style names.
///
/// ## Why is this bad?
/// [PEP 8] recommends naming conventions for classes, functions,
/// constants, and more. The use of inconsistent naming styles between
/// import and alias names may lead readers to expect an import to be of
/// another type (e.g., confuse a Python class with a constant).
///
/// Import aliases should thus follow the same naming style as the member
/// being imported.
///
/// ## Example
/// ```python
/// from example import MyClassName as MY_CLASS_NAME
/// ```
///
/// Use instead:
/// ```python
/// from example import MyClassName
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/
#[violation]
pub struct CamelcaseImportedAsConstant {
    name: String,
    asname: String,
}

impl Violation for CamelcaseImportedAsConstant {
    #[derive_message_formats]
    fn message(&self) -> String {
        let CamelcaseImportedAsConstant { name, asname } = self;
        format!("Camelcase `{name}` imported as constant `{asname}`")
    }
}

/// N814
pub(crate) fn camelcase_imported_as_constant(
    name: &str,
    asname: &str,
    alias: &Alias,
    stmt: &Stmt,
    ignore_names: &[IdentifierPattern],
) -> Option<Diagnostic> {
    if ignore_names
        .iter()
        .any(|ignore_name| ignore_name.matches(name))
    {
        return None;
    }

    if helpers::is_camelcase(name)
        && !str::is_cased_lowercase(asname)
        && str::is_cased_uppercase(asname)
        && !helpers::is_acronym(name, asname)
    {
        let mut diagnostic = Diagnostic::new(
            CamelcaseImportedAsConstant {
                name: name.to_string(),
                asname: asname.to_string(),
            },
            alias.range(),
        );
        diagnostic.set_parent(stmt.start());
        return Some(diagnostic);
    }
    None
}
