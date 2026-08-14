pub struct AstProgram {
    pub(crate) text_section: Option<AstTextSection>,
    pub(crate) header_section: Option<AstHeaderSection>,
}

pub struct AstTextSection {}

pub struct AstHeaderSection {}
