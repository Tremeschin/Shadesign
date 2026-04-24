use crate::*;

/// Binary resources
/// - Language words from pypi/wordfreq
#[derive(Embed)]
#[folder="$CARGO_MANIFEST_DIR/shadesign/resources"]
struct Resources;

#[derive(SmartDefault, ValueEnum, Clone)]
pub enum Language {
    German,
    Greek,
    #[default]
    English,
    Spanish,
    French,
    Italian,
    Japanese,
    Korean,
    Dutch,
    Polish,
    Portuguese,
    Russian,
    Chinese,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::German     => "de",
            Language::Greek      => "el",
            Language::English    => "en",
            Language::Spanish    => "es",
            Language::French     => "fr",
            Language::Italian    => "it",
            Language::Japanese   => "ja",
            Language::Korean     => "ko",
            Language::Dutch      => "nl",
            Language::Polish     => "pl",
            Language::Portuguese => "pt",
            Language::Russian    => "ru",
            Language::Chinese    => "zh",
        }
    }
}


#[derive(clap::Subcommand)]
pub enum WordFactory {

    /// (Input) Load words from a text file
    File {
        #[arg(short='i', long="input")]
        name: String,
    },

    /// (Input) Load words from wordfreq database
    Wordfreq {

        /// List of languages to load words by frequency
        lang: Vec<Language>,

        /// How many words to use per language (warn: O(N^2) complexity!)
        #[arg(short, long, default_value_t=50000)]
        count: usize,
    }
}

impl WordFactory {
    pub fn get(&self) -> Vec<String> {
        match self {
            Self::File{name} => {
                read_to_string(name)
                    .expect("Unable to read file")
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            }

            Self::Wordfreq{lang, count: total} => {
                lang.clone().into_iter().flat_map(|lang| {

                    // Read bundled raw data
                    let bytes = Resources::get(&format!("frequency/{}.txt.gz", lang.code()))
                        .expect("Missing resource file").data.to_vec();

                    // Decompress, yield lines
                    BufReader::new(GzDecoder::new(bytes.as_slice())).lines()
                        .map(|s| s.unwrap()).collect::<Vec<String>>()
                        .into_iter().take(*total)

                }).collect()
            }
        }
    }
}
