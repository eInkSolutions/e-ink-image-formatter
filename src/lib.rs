pub fn project_tag() -> &'static str {
    "e-ink-image-formatter core"
}

#[cfg(test)]
mod tests {
    use super::project_tag;

    #[test]
    fn exposes_core_project_tag() {
        assert_eq!(project_tag(), "e-ink-image-formatter core");
    }
}
