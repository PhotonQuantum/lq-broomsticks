use crate::ast::{Fresh, IdentType};

pub type BareIdent = String;

impl IdentType for BareIdent {}

impl Fresh for BareIdent {
    fn fresh(&self) -> Self {
        let chars = self.chars();
        let initial_offset = chars.clone().count();
        let (suffix, offset, _) = chars.rfold((0, initial_offset, 0), |(i, offset, e), chr| {
            chr.to_digit(10)
                .and_then(|n| Some((n * u32::pow(10, e) + i, offset - 1, e + 1)))
                .unwrap_or((i, offset, e))
        });
        format!(
            "{}{}",
            self.chars().take(offset).collect::<String>(),
            suffix + 1
        )
    }
}
