use codespan_reporting::{
    diagnostic::Diagnostic,
    files::Files,
    term::{
        self,
        termcolor::{Buffer, BufferWriter, ColorChoice},
        Config,
    },
};

pub struct ReportManager {
    config: Config,
    writer: Buffer,
}

impl Default for ReportManager {
    fn default() -> Self {
        Self {
            config: Default::default(),
            writer: BufferWriter::stderr(ColorChoice::Auto).buffer(),
        }
    }
}

impl std::fmt::Debug for ReportManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReportManager")
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

impl ReportManager {
    pub fn emit<'f, Id>(
        &mut self,
        files: &'f impl Files<'f, FileId = Id>,
        diag: &Diagnostic<Id>,
    ) -> String {
        term::emit(&mut self.writer, &self.config, files, diag).unwrap();
        String::from_utf8(self.writer.as_slice().to_vec()).unwrap()
    }
}
