#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentationEntry {
    pub section: String,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Default)]
pub struct DocsCompiler {
    entries: Vec<DocumentationEntry>,
}

impl DocsCompiler {
    pub fn new() -> Self { Self { entries: Vec::new() } }
    pub fn add_doc(&mut self, section: &str, title: &str, content: &str) {
        self.entries.push(DocumentationEntry {
            section: section.to_string(),
            title: title.to_string(),
            content: content.to_string(),
        });
    }
    pub fn find_by_section(&self, section: &str) -> Vec<&DocumentationEntry> {
        self.entries.iter().filter(|e| e.section == section).collect()
    }
    pub fn entry_count(&self) -> usize { self.entries.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_docs_compiler_aaa() {
        // Arrange
        let mut compiler = DocsCompiler::new();
        // Act
        compiler.add_doc("Admin", "Networking", "Configure interface via agy");
        compiler.add_doc("Admin", "Security", "Enable pledge and unveil");
        compiler.add_doc("User", "Desktop", "Customize theme and panels");
        // Assert
        assert_eq!(compiler.entry_count(), 3);
        let admin_docs = compiler.find_by_section("Admin");
        assert_eq!(admin_docs.len(), 2);
        assert_eq!(admin_docs[0].title, "Networking");
    }
}
