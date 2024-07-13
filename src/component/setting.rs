// std
use std::{borrow::Cow, fs, path::PathBuf};
// crates.io
use app_dirs2::AppDataType;
use async_openai::config::OPENAI_API_BASE;
use serde::{Deserialize, Serialize};
use tracing::Level;
// self
use super::{function::Function, openai::Model};
use crate::{prelude::*, widget::ComboBoxItem, APP_INFO};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Setting {
	pub general: General,
	pub ai: Ai,
	pub chat: Chat,
	pub hotkeys: Hotkeys,
	pub development: Development,
}
impl Setting {
	pub fn path() -> Result<PathBuf> {
		Ok(app_dirs2::get_app_root(AppDataType::UserConfig, &APP_INFO)
			.map(|p| p.join("setting.toml"))?)
	}

	pub fn load() -> Result<Self> {
		let p = Self::path()?;

		tracing::info!("loading from {}", p.display());

		if !p.is_file() {
			tracing::warn!("it looks like you are running AiR for the first time, creating a new setting file from template");

			return Ok(Default::default());
		}

		let s = match fs::read_to_string(p) {
			Ok(s) => s,
			Err(e) => {
				tracing::error!("failed to load the setting due to: {e}");

				return Ok(Default::default());
			},
		};

		// TODO: https://github.com/hack-ink/AiR/issues/62.
		Ok(toml::from_str(&s)?)
	}

	pub fn save(&self) -> Result<()> {
		let p = Self::path()?;
		let d = p.parent().unwrap();

		if !d.is_dir() {
			fs::create_dir_all(d)?;
		}

		tracing::info!("saving to {}", p.display());

		Ok(fs::write(p, toml::to_string_pretty(self).unwrap())?)
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct General {
	pub font_size: f32,
	pub hide_on_lost_focus: bool,
	pub active_func: Function,
}
impl Default for General {
	fn default() -> Self {
		Self { font_size: 13., hide_on_lost_focus: true, active_func: Default::default() }
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Ai {
	pub api_base: String,
	pub api_key: String,
	pub model: Model,
	pub temperature: f32,
}
impl Default for Ai {
	fn default() -> Self {
		Self {
			api_base: OPENAI_API_BASE.into(),
			api_key: Default::default(),
			model: Model::default(),
			temperature: 0.7,
		}
	}
}

// TODO?: implement a `Prompt` trait.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Chat {
	pub rewrite: Rewrite,
	pub translation: Translation,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Rewrite {
	pub additional_prompt: String,
}
impl Rewrite {
	pub fn prompt(&self) -> Cow<str> {
		const DEFAULT: &str =
			"As a professional writer and language master, assist me in refining text! \
			Amend any grammatical errors and enhance the language to sound more like a native speaker! \
			Text is always provided in format `<AiR>$TEXT</AiR>`! \
			$TEXT can be provided in any style! \
			Discard the `<AiR></AiR>` tag! \
			But keep the indentation and line breaks format! \
			Extract the $TEXT and return the refined $TEXT only!";

		if self.additional_prompt.is_empty() {
			DEFAULT.into()
		} else {
			format!("{DEFAULT} {}", self.additional_prompt).into()
		}
	}
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Translation {
	pub additional_prompt: String,
	pub a: Language,
	pub b: Language,
}
impl Translation {
	// TODO: https://github.com/hack-ink/AiR/issues/41.
	pub fn prompt(&self) -> Cow<str> {
		let default = format!(
			"As a professional translator and language master, assist me in translating text! \
			I provide two languages, {} and {}! \
			Determine which language the text I give is in, and then translate accordingly. \
			Amend any grammatical errors and enhance the language to sound more like a native speaker! \
			Text is always provided in format `<AiR>$TEXT</AiR>`! \
			$TEXT can be provided in any style! \
			Discard the `<AiR></AiR>` tag! \
			But keep the indentation and line breaks format! \
			Extract the $TEXT and return the translated $TEXT only!",
			self.a.as_str(),
			self.b.as_str(),
		);

		if self.additional_prompt.is_empty() {
			default.into()
		} else {
			format!("{default} {}", self.additional_prompt).into()
		}
	}
}
impl Default for Translation {
	fn default() -> Self {
		Self { additional_prompt: Default::default(), a: Language::ZhCn, b: Language::EnGb }
	}
}
// https://www.alchemysoftware.com/livedocs/ezscript/Topics/Catalyst/Language.htm
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Language {
	// Afrikaans (South Africa)
	AfZa,
	// Albanian (Albania)
	SqAl,
	// Alsatian (France)
	GswFr,
	// Amharic (Ethiopia)
	AmEt,
	// Arabic (Algeria)
	ArDz,
	// Arabic (Bahrain)
	ArBh,
	// Arabic (Egypt)
	ArEg,
	// Arabic (Iraq)
	ArIq,
	// Arabic (Jordan)
	ArJo,
	// Arabic (Kuwait)
	ArKw,
	// Arabic (Lebanon)
	ArLb,
	// Arabic (Libya)
	ArLy,
	// Arabic (Morocco)
	ArMa,
	// Arabic (Oman)
	ArOm,
	// Arabic (Qatar)
	ArQa,
	// Arabic (Saudi Arabia)
	ArSa,
	// Arabic (Syria)
	ArSy,
	// Arabic (Tunisia)
	ArTn,
	// Arabic (U.A.E.)
	ArAe,
	// Arabic (Yemen)
	ArYe,
	// Armenian (Armenia)
	HyAm,
	// Assamese (India)
	AsIn,
	// Azerbaijani (Cyrillic, Azerbaijan)
	AzCyrlAz,
	// Azerbaijani (Latin, Azerbaijan)
	AzLatnAz,
	// Bangla (Bangladesh)
	BnBd,
	// Bangla (India)
	BnIn,
	// Bashkir (Russia)
	BaRu,
	// Basque (Spain)
	EuEs,
	// Belarusian (Belarus)
	BeBy,
	// Bosnian (Cyrillic, Bosnia and Herzegovina)
	BsCyrlBa,
	// Bosnian (Latin, Bosnia and Herzegovina)
	BsLatnBa,
	// Breton (France)
	BrFr,
	// Bulgarian (Bulgaria)
	BgBg,
	// Burmese (Myanmar)
	MyMm,
	// Catalan (Spain)
	CaEs,
	// Central Atlas Tamazight (Arabic, Morocco)
	TzmArabMa,
	// Central Kurdish (Iraq)
	KuArabIq,
	// Cherokee (United States)
	ChrCherUs,
	// Chinese (Simplified, People's Republic of China)
	ZhCn,
	// Chinese (Simplified, Singapore)
	ZhSg,
	// Chinese (Traditional, Hong Kong S.A.R.)
	ZhHk,
	// Chinese (Traditional, Macao S.A.R.)
	ZhMo,
	// Chinese (Traditional, Taiwan)
	ZhTw,
	// Corsican (France)
	CoFr,
	// Croatian (Croatia)
	HrHr,
	// Croatian (Latin, Bosnia and Herzegovina)
	HrBa,
	// Czech (Czech Republic)
	CsCz,
	// Danish (Denmark)
	DaDk,
	// Dari (Afghanistan)
	PrsAf,
	// Divehi (Maldives)
	DvMv,
	// Dutch (Belgium)
	NlBe,
	// Dutch (Netherlands)
	NlNl,
	// Dzongkha (Bhutan)
	DzBt,
	// English (Australia)
	EnAu,
	// English (Belize)
	EnBz,
	// English (Canada)
	EnCa,
	// English (Caribbean)
	En029,
	// English (Hong Kong)
	EnHk,
	// English (India)
	EnIn,
	// English (Ireland)
	EnIe,
	// English (Jamaica)
	EnJm,
	// English (Malaysia)
	EnMy,
	// English (New Zealand)
	EnNz,
	// English (Republic of the Philippines)
	EnPh,
	// English (Singapore)
	EnSg,
	// English (South Africa)
	EnZa,
	// English (Trinidad and Tobago)
	EnTt,
	// English (United Arab Emirates)
	EnAe,
	// English (United Kingdom)
	EnGb,
	// English (United States)
	EnUs,
	// English (Zimbabwe)
	EnZw,
	// Estonian (Estonia)
	EtEe,
	// Faroese (Faroe Islands)
	FoFo,
	// Filipino (Philippines)
	FilPh,
	// Finnish (Finland)
	FiFi,
	// French (Belgium)
	FrBe,
	// French (Côte d’Ivoire)
	FrCi,
	// French (Cameroon)
	FrCm,
	// French (Canada)
	FrCa,
	// French (Caribbean)
	Fr029,
	// French (Congo, DRC)
	FrCd,
	// French (France)
	FrFr,
	// French (Haiti)
	FrHt,
	// French (Luxembourg)
	FrLu,
	// French (Mali)
	FrMl,
	// French (Morocco)
	FrMa,
	// French (Principality of Monaco)
	FrMc,
	// French (Réunion)
	FrRe,
	// French (Senegal)
	FrSn,
	// French (Switzerland)
	FrCh,
	// Frisian (Netherlands)
	FyNl,
	// Fulah (Latin, Nigeria)
	FfLatnNg,
	// Fulah (Latin, Senegal)
	FfLatnSn,
	// Galician (Spain)
	GlEs,
	// Georgian (Georgia)
	KaGe,
	// German (Austria)
	DeAt,
	// German (Germany)
	DeDe,
	// German (Liechtenstein)
	DeLi,
	// German (Luxembourg)
	DeLu,
	// German (Switzerland)
	DeCh,
	// Greek (Greece)
	ElGr,
	// Greenlandic (Greenland)
	KlGl,
	// Guarani (Paraguay)
	GnPy,
	// Gujarati (India)
	GuIn,
	// Hausa (Latin, Nigeria)
	HaLatnNg,
	// Hawaiian (United States)
	HawUs,
	// Hebrew (Israel)
	HeIl,
	// Hindi (India)
	HiIn,
	// Hungarian (Hungary)
	HuHu,
	// Icelandic (Iceland)
	IsIs,
	// Igbo (Nigeria)
	IgNg,
	// Indonesian (Indonesia)
	IdId,
	// Inuktitut (Latin, Canada)
	IuLatnCa,
	// Inuktitut (Syllabics, Canada)
	IuCansCa,
	// Irish (Ireland)
	GaIe,
	// Italian (Italy)
	ItIt,
	// Italian (Switzerland)
	ItCh,
	// Japanese (Japan)
	JaJp,
	// Kannada (India)
	KnIn,
	// Kanuri (Latin, Nigeria)
	KrLatnNg,
	// Kashmiri (Devanagari, India)
	KsDevaIn,
	// Kazakh (Kazakhstan)
	KkKz,
	// Khmer (Cambodia)
	KmKh,
	// K'iche (Latin, Guatemala)
	QucLatnGt,
	// Kinyarwanda (Rwanda)
	RwRw,
	// Kiswahili (Kenya)
	SwKe,
	// Konkani (India)
	KokIn,
	// Korean (Korea)
	KoKr,
	// Kyrgyz (Kyrgyzstan)
	KyKg,
	// Lao (Lao P.D.R.)
	LoLa,
	// Latin (Vatican City)
	LaVa,
	// Latvian (Latvia)
	LvLv,
	// Lithuanian (Lithuania)
	LtLt,
	// Lower Sorbian (Germany)
	DsbDe,
	// Luxembourgish (Luxembourg)
	LbLu,
	// Macedonian (North Macedonia)
	MkMk,
	// Malay (Brunei Darussalam)
	MsBn,
	// Malay (Malaysia)
	MsMy,
	// Malayalam (India)
	MlIn,
	// Maltese (Malta)
	MtMt,
	// Maori (New Zealand)
	MiNz,
	// Mapudungun (Chile)
	ArnCl,
	// Marathi (India)
	MrIn,
	// Mohawk (Canada)
	MohCa,
	// Mongolian (Cyrillic, Mongolia)
	MnMn,
	// Mongolian (Traditional Mongolian, Mongolia)
	MnMongMn,
	// Nepali (India)
	NeIn,
	// Nepali (Nepal)
	NeNp,
	// Norwegian, Bokmål (Norway)
	NbNo,
	// Norwegian, Nynorsk (Norway)
	NnNo,
	// Occitan (France)
	OcFr,
	// Odia (India)
	OrIn,
	// Oromo (Ethiopia)
	OmEt,
	// Pashto (Afghanistan)
	PsAf,
	// Persian (Iran)
	FaIr,
	// Polish (Poland)
	PlPl,
	// Portuguese (Brazil)
	PtBr,
	// Portuguese (Portugal)
	PtPt,
	// Punjabi (India)
	PaIn,
	// Punjabi (Islamic Republic of Pakistan)
	PaArabPk,
	// Quechua (Bolivia)
	QuzBo,
	// Quechua (Ecuador)
	QuzEc,
	// Quechua (Peru)
	QuzPe,
	// Romanian (Moldova)
	RoMd,
	// Romanian (Romania)
	RoRo,
	// Romansh (Switzerland)
	RmCh,
	// Russian (Moldova)
	RuMd,
	// Russian (Russia)
	RuRu,
	// Sakha (Russia)
	SahRu,
	// Sami, Inari (Finland)
	SmnFi,
	// Sami, Lule (Norway)
	SmjNo,
	// Sami, Lule (Sweden)
	SmjSe,
	// Sami, Northern (Finland)
	SeFi,
	// Sami, Northern (Norway)
	SeNo,
	// Sami, Northern (Sweden)
	SeSe,
	// Sami, Skolt (Finland)
	SmsFi,
	// Sami, Southern (Norway)
	SmaNo,
	// Sami, Southern (Sweden)
	SmaSe,
	// Sanskrit (India)
	SaIn,
	// Scottish Gaelic (United Kingdom)
	GdGb,
	// Serbian (Cyrillic, Bosnia and Herzegovina)
	SrCyrlBa,
	// Serbian (Cyrillic, Montenegro)
	SrCyrlMe,
	// Serbian (Cyrillic, Serbia)
	SrCyrlRs,
	// Serbian (Latin, Bosnia and Herzegovina)
	SrLatnBa,
	// Serbian (Latin, Montenegro)
	SrLatnMe,
	// Serbian (Latin, Serbia)
	SrLatnRs,
	// Sesotho sa Leboa (South Africa)
	NsoZa,
	// Setswana (Botswana)
	TnBw,
	// Setswana (South Africa)
	TnZa,
	// Sindhi (Islamic Republic of Pakistan)
	SdArabPk,
	// Sinhala (Sri Lanka)
	SiLk,
	// Slovak (Slovakia)
	SkSk,
	// Slovenian (Slovenia)
	SlSi,
	// Somali (Somalia)
	SoSo,
	// Sotho (South Africa)
	StZa,
	// Spanish (Argentina)
	EsAr,
	// Spanish (Bolivarian Republic of Venezuela)
	EsVe,
	// Spanish (Bolivia)
	EsBo,
	// Spanish (Chile)
	EsCl,
	// Spanish (Colombia)
	EsCo,
	// Spanish (Costa Rica)
	EsCr,
	// Spanish (Cuba)
	EsCu,
	// Spanish (Dominican Republic)
	EsDo,
	// Spanish (Ecuador)
	EsEc,
	// Spanish (El Salvador)
	EsSv,
	// Spanish (Guatemala)
	EsGt,
	// Spanish (Honduras)
	EsHn,
	// Spanish (Latin America)
	Es419,
	// Spanish (Mexico)
	EsMx,
	// Spanish (Nicaragua)
	EsNi,
	// Spanish (Panama)
	EsPa,
	// Spanish (Paraguay)
	EsPy,
	// Spanish (Peru)
	EsPe,
	// Spanish (Puerto Rico)
	EsPr,
	// Spanish (Spain, International Sort)
	EsEs,
	// Spanish (United States)
	EsUs,
	// Spanish (Uruguay)
	EsUy,
	// Swedish (Finland)
	SvFi,
	// Swedish (Sweden)
	SvSe,
	// Syriac (Syria)
	SyrSy,
	// Tajik (Cyrillic, Tajikistan)
	TgCyrlTj,
	// Tamazight (Latin, Algeria)
	TzmLatnDz,
	// Tamil (India)
	TaIn,
	// Tamil (Sri Lanka)
	TaLk,
	// Tatar (Russia)
	TtRu,
	// Telugu (India)
	TeIn,
	// Thai (Thailand)
	ThTh,
	// Tibetan (People's Republic of China)
	BoCn,
	// Tigrinya (Eritrea)
	TiEr,
	// Tigrinya (Ethiopia)
	TiEt,
	// Tsonga (South Africa)
	TsZa,
	// Turkish (Turkey)
	TrTr,
	// Turkmen (Turkmenistan)
	TkTm,
	// Ukrainian (Ukraine)
	UkUa,
	// Upper Sorbian (Germany)
	HsbDe,
	// Urdu (India)
	UrIn,
	// Urdu (Islamic Republic of Pakistan)
	UrPk,
	// Uyghur (People's Republic of China)
	UgCn,
	// Uzbek (Cyrillic, Uzbekistan)
	UzCyrlUz,
	// Uzbek (Latin, Uzbekistan)
	UzLatnUz,
	// Valencian (Spain)
	CaEsValencia,
	// Venda (South Africa)
	VeZa,
	// Vietnamese (Vietnam)
	ViVn,
	// Welsh (United Kingdom)
	CyGb,
	// Wolof (Senegal)
	WoSn,
	// Xhosa (South Africa)
	XhZa,
	// Yi (People's Republic of China)
	IiCn,
	// Yiddish (World)
	Yi001,
	// Yoruba (Nigeria)
	YoNg,
	// Zulu (South Africa)
	ZuZa,
}
impl ComboBoxItem for Language {
	type Array = [Self; Self::COUNT];

	const COUNT: usize = 250;

	fn all() -> Self::Array {
		[
			Self::AfZa,
			Self::SqAl,
			Self::GswFr,
			Self::AmEt,
			Self::ArDz,
			Self::ArBh,
			Self::ArEg,
			Self::ArIq,
			Self::ArJo,
			Self::ArKw,
			Self::ArLb,
			Self::ArLy,
			Self::ArMa,
			Self::ArOm,
			Self::ArQa,
			Self::ArSa,
			Self::ArSy,
			Self::ArTn,
			Self::ArAe,
			Self::ArYe,
			Self::HyAm,
			Self::AsIn,
			Self::AzCyrlAz,
			Self::AzLatnAz,
			Self::BnBd,
			Self::BnIn,
			Self::BaRu,
			Self::EuEs,
			Self::BeBy,
			Self::BsCyrlBa,
			Self::BsLatnBa,
			Self::BrFr,
			Self::BgBg,
			Self::MyMm,
			Self::CaEs,
			Self::TzmArabMa,
			Self::KuArabIq,
			Self::ChrCherUs,
			Self::ZhCn,
			Self::ZhSg,
			Self::ZhHk,
			Self::ZhMo,
			Self::ZhTw,
			Self::CoFr,
			Self::HrHr,
			Self::HrBa,
			Self::CsCz,
			Self::DaDk,
			Self::PrsAf,
			Self::DvMv,
			Self::NlBe,
			Self::NlNl,
			Self::DzBt,
			Self::EnAu,
			Self::EnBz,
			Self::EnCa,
			Self::En029,
			Self::EnHk,
			Self::EnIn,
			Self::EnIe,
			Self::EnJm,
			Self::EnMy,
			Self::EnNz,
			Self::EnPh,
			Self::EnSg,
			Self::EnZa,
			Self::EnTt,
			Self::EnAe,
			Self::EnGb,
			Self::EnUs,
			Self::EnZw,
			Self::EtEe,
			Self::FoFo,
			Self::FilPh,
			Self::FiFi,
			Self::FrBe,
			Self::FrCi,
			Self::FrCm,
			Self::FrCa,
			Self::Fr029,
			Self::FrCd,
			Self::FrFr,
			Self::FrHt,
			Self::FrLu,
			Self::FrMl,
			Self::FrMa,
			Self::FrMc,
			Self::FrRe,
			Self::FrSn,
			Self::FrCh,
			Self::FyNl,
			Self::FfLatnNg,
			Self::FfLatnSn,
			Self::GlEs,
			Self::KaGe,
			Self::DeAt,
			Self::DeDe,
			Self::DeLi,
			Self::DeLu,
			Self::DeCh,
			Self::ElGr,
			Self::KlGl,
			Self::GnPy,
			Self::GuIn,
			Self::HaLatnNg,
			Self::HawUs,
			Self::HeIl,
			Self::HiIn,
			Self::HuHu,
			Self::IsIs,
			Self::IgNg,
			Self::IdId,
			Self::IuLatnCa,
			Self::IuCansCa,
			Self::GaIe,
			Self::ItIt,
			Self::ItCh,
			Self::JaJp,
			Self::KnIn,
			Self::KrLatnNg,
			Self::KsDevaIn,
			Self::KkKz,
			Self::KmKh,
			Self::QucLatnGt,
			Self::RwRw,
			Self::SwKe,
			Self::KokIn,
			Self::KoKr,
			Self::KyKg,
			Self::LoLa,
			Self::LaVa,
			Self::LvLv,
			Self::LtLt,
			Self::DsbDe,
			Self::LbLu,
			Self::MkMk,
			Self::MsBn,
			Self::MsMy,
			Self::MlIn,
			Self::MtMt,
			Self::MiNz,
			Self::ArnCl,
			Self::MrIn,
			Self::MohCa,
			Self::MnMn,
			Self::MnMongMn,
			Self::NeIn,
			Self::NeNp,
			Self::NbNo,
			Self::NnNo,
			Self::OcFr,
			Self::OrIn,
			Self::OmEt,
			Self::PsAf,
			Self::FaIr,
			Self::PlPl,
			Self::PtBr,
			Self::PtPt,
			Self::PaIn,
			Self::PaArabPk,
			Self::QuzBo,
			Self::QuzEc,
			Self::QuzPe,
			Self::RoMd,
			Self::RoRo,
			Self::RmCh,
			Self::RuMd,
			Self::RuRu,
			Self::SahRu,
			Self::SmnFi,
			Self::SmjNo,
			Self::SmjSe,
			Self::SeFi,
			Self::SeNo,
			Self::SeSe,
			Self::SmsFi,
			Self::SmaNo,
			Self::SmaSe,
			Self::SaIn,
			Self::GdGb,
			Self::SrCyrlBa,
			Self::SrCyrlMe,
			Self::SrCyrlRs,
			Self::SrLatnBa,
			Self::SrLatnMe,
			Self::SrLatnRs,
			Self::NsoZa,
			Self::TnBw,
			Self::TnZa,
			Self::SdArabPk,
			Self::SiLk,
			Self::SkSk,
			Self::SlSi,
			Self::SoSo,
			Self::StZa,
			Self::EsAr,
			Self::EsVe,
			Self::EsBo,
			Self::EsCl,
			Self::EsCo,
			Self::EsCr,
			Self::EsCu,
			Self::EsDo,
			Self::EsEc,
			Self::EsSv,
			Self::EsGt,
			Self::EsHn,
			Self::Es419,
			Self::EsMx,
			Self::EsNi,
			Self::EsPa,
			Self::EsPy,
			Self::EsPe,
			Self::EsPr,
			Self::EsEs,
			Self::EsUs,
			Self::EsUy,
			Self::SvFi,
			Self::SvSe,
			Self::SyrSy,
			Self::TgCyrlTj,
			Self::TzmLatnDz,
			Self::TaIn,
			Self::TaLk,
			Self::TtRu,
			Self::TeIn,
			Self::ThTh,
			Self::BoCn,
			Self::TiEr,
			Self::TiEt,
			Self::TsZa,
			Self::TrTr,
			Self::TkTm,
			Self::UkUa,
			Self::HsbDe,
			Self::UrIn,
			Self::UrPk,
			Self::UgCn,
			Self::UzCyrlUz,
			Self::UzLatnUz,
			Self::CaEsValencia,
			Self::VeZa,
			Self::ViVn,
			Self::CyGb,
			Self::WoSn,
			Self::XhZa,
			Self::IiCn,
			Self::Yi001,
			Self::YoNg,
			Self::ZuZa,
		]
	}

	fn as_str(&self) -> &'static str {
		match self {
			Self::AfZa => "af-ZA",
			Self::SqAl => "sq-AL",
			Self::GswFr => "gsw-FR",
			Self::AmEt => "am-ET",
			Self::ArDz => "ar-DZ",
			Self::ArBh => "ar-BH",
			Self::ArEg => "ar-EG",
			Self::ArIq => "ar-IQ",
			Self::ArJo => "ar-JO",
			Self::ArKw => "ar-KW",
			Self::ArLb => "ar-LB",
			Self::ArLy => "ar-LY",
			Self::ArMa => "ar-MA",
			Self::ArOm => "ar-OM",
			Self::ArQa => "ar-QA",
			Self::ArSa => "ar-SA",
			Self::ArSy => "ar-SY",
			Self::ArTn => "ar-TN",
			Self::ArAe => "ar-AE",
			Self::ArYe => "ar-YE",
			Self::HyAm => "hy-AM",
			Self::AsIn => "as-IN",
			Self::AzCyrlAz => "az-Cyrl-AZ",
			Self::AzLatnAz => "az-Latn-AZ",
			Self::BnBd => "bn-BD",
			Self::BnIn => "bn-IN",
			Self::BaRu => "ba-RU",
			Self::EuEs => "eu-ES",
			Self::BeBy => "be-BY",
			Self::BsCyrlBa => "bs-Cyrl-BA",
			Self::BsLatnBa => "bs-Latn-BA",
			Self::BrFr => "br-FR",
			Self::BgBg => "bg-BG",
			Self::MyMm => "my-MM",
			Self::CaEs => "ca-ES",
			Self::TzmArabMa => "tzm-Arab-MA",
			Self::KuArabIq => "ku-Arab-IQ",
			Self::ChrCherUs => "chr-Cher-US",
			Self::ZhCn => "zh-CN",
			Self::ZhSg => "zh-SG",
			Self::ZhHk => "zh-HK",
			Self::ZhMo => "zh-MO",
			Self::ZhTw => "zh-TW",
			Self::CoFr => "co-FR",
			Self::HrHr => "hr-HR",
			Self::HrBa => "hr-BA",
			Self::CsCz => "cs-CZ",
			Self::DaDk => "da-DK",
			Self::PrsAf => "prs-AF",
			Self::DvMv => "dv-MV",
			Self::NlBe => "nl-BE",
			Self::NlNl => "nl-NL",
			Self::DzBt => "dz-BT",
			Self::EnAu => "en-AU",
			Self::EnBz => "en-BZ",
			Self::EnCa => "en-CA",
			Self::En029 => "en-029",
			Self::EnHk => "en-HK",
			Self::EnIn => "en-IN",
			Self::EnIe => "en-IE",
			Self::EnJm => "en-JM",
			Self::EnMy => "en-MY",
			Self::EnNz => "en-NZ",
			Self::EnPh => "en-PH",
			Self::EnSg => "en-SG",
			Self::EnZa => "en-ZA",
			Self::EnTt => "en-TT",
			Self::EnAe => "en-AE",
			Self::EnGb => "en-GB",
			Self::EnUs => "en-US",
			Self::EnZw => "en-ZW",
			Self::EtEe => "et-EE",
			Self::FoFo => "fo-FO",
			Self::FilPh => "fil-PH",
			Self::FiFi => "fi-FI",
			Self::FrBe => "fr-BE",
			Self::FrCi => "fr-CI",
			Self::FrCm => "fr-CM",
			Self::FrCa => "fr-CA",
			Self::Fr029 => "fr-029",
			Self::FrCd => "fr-CD",
			Self::FrFr => "fr-FR",
			Self::FrHt => "fr-HT",
			Self::FrLu => "fr-LU",
			Self::FrMl => "fr-ML",
			Self::FrMa => "fr-MA",
			Self::FrMc => "fr-MC",
			Self::FrRe => "fr-RE",
			Self::FrSn => "fr-SN",
			Self::FrCh => "fr-CH",
			Self::FyNl => "fy-NL",
			Self::FfLatnNg => "ff-Latn-NG",
			Self::FfLatnSn => "ff-Latn-SN",
			Self::GlEs => "gl-ES",
			Self::KaGe => "ka-GE",
			Self::DeAt => "de-AT",
			Self::DeDe => "de-DE",
			Self::DeLi => "de-LI",
			Self::DeLu => "de-LU",
			Self::DeCh => "de-CH",
			Self::ElGr => "el-GR",
			Self::KlGl => "kl-GL",
			Self::GnPy => "gn-PY",
			Self::GuIn => "gu-IN",
			Self::HaLatnNg => "ha-Latn-NG",
			Self::HawUs => "haw-US",
			Self::HeIl => "he-IL",
			Self::HiIn => "hi-IN",
			Self::HuHu => "hu-HU",
			Self::IsIs => "is-IS",
			Self::IgNg => "ig-NG",
			Self::IdId => "id-ID",
			Self::IuLatnCa => "iu-Latn-CA",
			Self::IuCansCa => "iu-Cans-CA",
			Self::GaIe => "ga-IE",
			Self::ItIt => "it-IT",
			Self::ItCh => "it-CH",
			Self::JaJp => "ja-JP",
			Self::KnIn => "kn-IN",
			Self::KrLatnNg => "kr-Latn-NG",
			Self::KsDevaIn => "ks-Deva-IN",
			Self::KkKz => "kk-KZ",
			Self::KmKh => "km-KH",
			Self::QucLatnGt => "quc-Latn-GT",
			Self::RwRw => "rw-RW",
			Self::SwKe => "sw-KE",
			Self::KokIn => "kok-IN",
			Self::KoKr => "ko-KR",
			Self::KyKg => "ky-KG",
			Self::LoLa => "lo-LA",
			Self::LaVa => "la-VA",
			Self::LvLv => "lv-LV",
			Self::LtLt => "lt-LT",
			Self::DsbDe => "dsb-DE",
			Self::LbLu => "lb-LU",
			Self::MkMk => "mk-MK",
			Self::MsBn => "ms-BN",
			Self::MsMy => "ms-MY",
			Self::MlIn => "ml-IN",
			Self::MtMt => "mt-MT",
			Self::MiNz => "mi-NZ",
			Self::ArnCl => "arn-CL",
			Self::MrIn => "mr-IN",
			Self::MohCa => "moh-CA",
			Self::MnMn => "mn-MN",
			Self::MnMongMn => "mn-Mong-MN",
			Self::NeIn => "ne-IN",
			Self::NeNp => "ne-NP",
			Self::NbNo => "nb-NO",
			Self::NnNo => "nn-NO",
			Self::OcFr => "oc-FR",
			Self::OrIn => "or-IN",
			Self::OmEt => "om-ET",
			Self::PsAf => "ps-AF",
			Self::FaIr => "fa-IR",
			Self::PlPl => "pl-PL",
			Self::PtBr => "pt-BR",
			Self::PtPt => "pt-PT",
			Self::PaIn => "pa-IN",
			Self::PaArabPk => "pa-Arab-PK",
			Self::QuzBo => "quz-BO",
			Self::QuzEc => "quz-EC",
			Self::QuzPe => "quz-PE",
			Self::RoMd => "ro-MD",
			Self::RoRo => "ro-RO",
			Self::RmCh => "rm-CH",
			Self::RuMd => "ru-MD",
			Self::RuRu => "ru-RU",
			Self::SahRu => "sah-RU",
			Self::SmnFi => "smn-FI",
			Self::SmjNo => "smj-NO",
			Self::SmjSe => "smj-SE",
			Self::SeFi => "se-FI",
			Self::SeNo => "se-NO",
			Self::SeSe => "se-SE",
			Self::SmsFi => "sms-FI",
			Self::SmaNo => "sma-NO",
			Self::SmaSe => "sma-SE",
			Self::SaIn => "sa-IN",
			Self::GdGb => "gd-GB",
			Self::SrCyrlBa => "sr-Cyrl-BA",
			Self::SrCyrlMe => "sr-Cyrl-ME",
			Self::SrCyrlRs => "sr-Cyrl-RS",
			Self::SrLatnBa => "sr-Latn-BA",
			Self::SrLatnMe => "sr-Latn-ME",
			Self::SrLatnRs => "sr-Latn-RS",
			Self::NsoZa => "nso-ZA",
			Self::TnBw => "tn-BW",
			Self::TnZa => "tn-ZA",
			Self::SdArabPk => "sd-Arab-PK",
			Self::SiLk => "si-LK",
			Self::SkSk => "sk-SK",
			Self::SlSi => "sl-SI",
			Self::SoSo => "so-SO",
			Self::StZa => "st-ZA",
			Self::EsAr => "es-AR",
			Self::EsVe => "es-VE",
			Self::EsBo => "es-BO",
			Self::EsCl => "es-CL",
			Self::EsCo => "es-CO",
			Self::EsCr => "es-CR",
			Self::EsCu => "es-CU",
			Self::EsDo => "es-DO",
			Self::EsEc => "es-EC",
			Self::EsSv => "es-SV",
			Self::EsGt => "es-GT",
			Self::EsHn => "es-HN",
			Self::Es419 => "es-419",
			Self::EsMx => "es-MX",
			Self::EsNi => "es-NI",
			Self::EsPa => "es-PA",
			Self::EsPy => "es-PY",
			Self::EsPe => "es-PE",
			Self::EsPr => "es-PR",
			Self::EsEs => "es-ES",
			Self::EsUs => "es-US",
			Self::EsUy => "es-UY",
			Self::SvFi => "sv-FI",
			Self::SvSe => "sv-SE",
			Self::SyrSy => "syr-SY",
			Self::TgCyrlTj => "tg-Cyrl-TJ",
			Self::TzmLatnDz => "tzm-Latn-DZ",
			Self::TaIn => "ta-IN",
			Self::TaLk => "ta-LK",
			Self::TtRu => "tt-RU",
			Self::TeIn => "te-IN",
			Self::ThTh => "th-TH",
			Self::BoCn => "bo-CN",
			Self::TiEr => "ti-ER",
			Self::TiEt => "ti-ET",
			Self::TsZa => "ts-ZA",
			Self::TrTr => "tr-TR",
			Self::TkTm => "tk-TM",
			Self::UkUa => "uk-UA",
			Self::HsbDe => "hsb-DE",
			Self::UrIn => "ur-IN",
			Self::UrPk => "ur-PK",
			Self::UgCn => "ug-CN",
			Self::UzCyrlUz => "uz-Cyrl-UZ",
			Self::UzLatnUz => "uz-Latn-UZ",
			Self::CaEsValencia => "ca-ES-valencia",
			Self::VeZa => "ve-ZA",
			Self::ViVn => "vi-VN",
			Self::CyGb => "cy-GB",
			Self::WoSn => "wo-SN",
			Self::XhZa => "xh-ZA",
			Self::IiCn => "ii-CN",
			Self::Yi001 => "yi-001",
			Self::YoNg => "yo-NG",
			Self::ZuZa => "zu-ZA",
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Hotkeys {
	pub rewrite: String,
	pub rewrite_directly: String,
	pub translate: String,
	pub translate_directly: String,
}
impl Default for Hotkeys {
	fn default() -> Self {
		Self {
			rewrite: "CTRL+T".into(),
			rewrite_directly: "CTRL+Y".into(),
			translate: "CTRL+U".into(),
			translate_directly: "CTRL+I".into(),
		}
	}
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Development {
	pub log_level: LogLevel,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LogLevel {
	Trace,
	Debug,
	Info,
	Warn,
	Error,
}
impl Default for LogLevel {
	fn default() -> Self {
		Self::Warn
	}
}
impl From<LogLevel> for Level {
	fn from(l: LogLevel) -> Self {
		match l {
			LogLevel::Trace => Level::TRACE,
			LogLevel::Debug => Level::DEBUG,
			LogLevel::Info => Level::INFO,
			LogLevel::Warn => Level::WARN,
			LogLevel::Error => Level::ERROR,
		}
	}
}
impl ComboBoxItem for LogLevel {
	type Array = [Self; Self::COUNT];

	const COUNT: usize = 5;

	fn all() -> Self::Array {
		[Self::Trace, Self::Debug, Self::Info, Self::Warn, Self::Error]
	}

	fn as_str(&self) -> &'static str {
		match self {
			Self::Trace => "Trace",
			Self::Debug => "Debug",
			Self::Info => "Info",
			Self::Warn => "Warn",
			Self::Error => "Error",
		}
	}
}
