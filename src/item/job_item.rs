use resume_cv_proc_macro::{CvElementBuilder, CvSectionItem};

use crate::writer::{
    latex_writer::{write_latex_command_call, LatexWriter, SectionItemLatexWriter},
    Writer,
};

#[derive(Debug, CvElementBuilder, CvSectionItem)]
pub struct JobItem {
    #[cv_element_builder(text_with_attributes)]
    pub role: String,
    #[cv_element_builder(text_with_attributes)]
    pub company: String,
    #[cv_element_builder(text_with_attributes)]
    pub where_: String,
    #[cv_element_builder(text_with_attributes)]
    pub when: String,
    #[cv_element_builder(text_with_attributes)]
    pub topics: Option<String>,
    #[cv_element_builder(text_with_attributes)]
    pub details: Option<String>,
}

impl LatexWriter for JobItem {
    fn latex_write(&self, f: &mut Writer) -> std::io::Result<()> {
        write_latex_command_call(
            f,
            "itemjob",
            &[
                &self.role,
                &self.company,
                &self.where_,
                &self.when,
                self.topics.as_deref().unwrap_or(""),
                self.details.as_deref().unwrap_or(""),
            ],
        )
    }
}

impl SectionItemLatexWriter for JobItem {
    const SECTION_COMMAND: &'static str = "sectionjob";
}
