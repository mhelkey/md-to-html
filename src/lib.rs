use std::path::{PathBuf, Path};
use clap::Parser;

#[derive(Default)]
pub struct Builder {
    informational_message: bool,
    informational_message_override: Option<String>,
    image_link: bool,
    image_link_dir: Option<PathBuf>,
}

impl Builder {
    pub fn with_informational_message(&mut self, informational_message: bool) -> &mut Self {
        self.informational_message = informational_message;
        self
    }

    pub fn with_informational_message_override(
        &mut self,
        informational_message_override: String,
    ) -> &mut Self {
        self.informational_message_override = Some(informational_message_override);
        self
    }

    pub fn with_image_link(&mut self, image_link: bool) -> &mut Self {
        self.image_link = image_link;
        self
    }

    pub fn with_image_link_dir(&mut self, image_link_dir: PathBuf) -> &mut Self {
        self.image_link_dir = Some(image_link_dir);
        self
    }

    pub fn build(self) -> MarkDownCompiler {
        MarkDownCompiler {
            informational_message: self.informational_message,
            informational_message_override: self.informational_message_override,
            image_link: self.image_link,
            image_link_dir: self.image_link_dir,
			default_compile_dir: None,
        }
    }
}


#[derive(Parser, Clone, Debug)]
pub struct MarkDownCompiler {
	#[arg(long)]
    /// Include a small information blurb at the end of generated HTML
    informational_message: bool,

	#[arg(long)]
    /// Optional informational message override
    informational_message_override: Option<String>,

	#[arg(long)]
    /// Tries to link images without an URL to images
    image_link: bool,

	#[arg(long)]
    /// If linking images, supply optional directory.
    image_link_dir: Option<PathBuf>,

	#[arg(long)]
    /// Default directory to compile. Used for CLI
    default_compile_dir: Option<PathBuf>,
}

impl MarkDownCompiler {
	pub fn builder() -> Builder {
		Builder::default()
	}

	pub fn set_default_compile_dir(&mut self, path: PathBuf) -> &mut Self {
		self.default_compile_dir = Some(path);
		self
	}

	pub fn get_default_compiler_dir<'a>(&'a self) -> Option<&'a PathBuf> {
		self.default_compile_dir.as_ref()
	}

	pub fn compile_default(self) -> anyhow::Result<()> {
		if self.default_compile_dir.is_none() {
			return Err(anyhow::anyhow!("No default directory provided to compile"));
		}

        let path = self.default_compile_dir.clone().unwrap();
        self.compile(&path)?;

		Ok(())
	}

    pub fn compile(self, directory: &Path) -> anyhow::Result<()> {
		tracing::info!("Compiling to html in {}", directory.display());
		Ok(())
	}
}