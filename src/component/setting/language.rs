// crates.io
use serde::{Deserialize, Serialize};
// self
use crate::widget::ComboBoxItem;

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
			Self::AfZa => "Afrikaans (South Africa)",
			Self::SqAl => "Albanian (Albania)",
			Self::GswFr => "Alsatian (France)",
			Self::AmEt => "Amharic (Ethiopia)",
			Self::ArDz => "Arabic (Algeria)",
			Self::ArBh => "Arabic (Bahrain)",
			Self::ArEg => "Arabic (Egypt)",
			Self::ArIq => "Arabic (Iraq)",
			Self::ArJo => "Arabic (Jordan)",
			Self::ArKw => "Arabic (Kuwait)",
			Self::ArLb => "Arabic (Lebanon)",
			Self::ArLy => "Arabic (Libya)",
			Self::ArMa => "Arabic (Morocco)",
			Self::ArOm => "Arabic (Oman)",
			Self::ArQa => "Arabic (Qatar)",
			Self::ArSa => "Arabic (Saudi Arabia)",
			Self::ArSy => "Arabic (Syria)",
			Self::ArTn => "Arabic (Tunisia)",
			Self::ArAe => "Arabic (U.A.E.)",
			Self::ArYe => "Arabic (Yemen)",
			Self::HyAm => "Armenian (Armenia)",
			Self::AsIn => "Assamese (India)",
			Self::AzCyrlAz => "Azerbaijani (Cyrillic, Azerbaijan)",
			Self::AzLatnAz => "Azerbaijani (Latin, Azerbaijan)",
			Self::BnBd => "Bangla (Bangladesh)",
			Self::BnIn => "Bangla (India)",
			Self::BaRu => "Bashkir (Russia)",
			Self::EuEs => "Basque (Spain)",
			Self::BeBy => "Belarusian (Belarus)",
			Self::BsCyrlBa => "Bosnian (Cyrillic, Bosnia and Herzegovina)",
			Self::BsLatnBa => "Bosnian (Latin, Bosnia and Herzegovina)",
			Self::BrFr => "Breton (France)",
			Self::BgBg => "Bulgarian (Bulgaria)",
			Self::MyMm => "Burmese (Myanmar)",
			Self::CaEs => "Catalan (Spain)",
			Self::TzmArabMa => "Central Atlas Tamazight (Arabic, Morocco)",
			Self::KuArabIq => "Central Kurdish (Iraq)",
			Self::ChrCherUs => "Cherokee (United States)",
			Self::ZhCn => "Chinese (Simplified, People's Republic of China)",
			Self::ZhSg => "Chinese (Simplified, Singapore)",
			Self::ZhHk => "Chinese (Traditional, Hong Kong S.A.R.)",
			Self::ZhMo => "Chinese (Traditional, Macao S.A.R.)",
			Self::ZhTw => "Chinese (Traditional, Taiwan)",
			Self::CoFr => "Corsican (France)",
			Self::HrHr => "Croatian (Croatia)",
			Self::HrBa => "Croatian (Latin, Bosnia and Herzegovina)",
			Self::CsCz => "Czech (Czech Republic)",
			Self::DaDk => "Danish (Denmark)",
			Self::PrsAf => "Dari (Afghanistan)",
			Self::DvMv => "Divehi (Maldives)",
			Self::NlBe => "Dutch (Belgium)",
			Self::NlNl => "Dutch (Netherlands)",
			Self::DzBt => "Dzongkha (Bhutan)",
			Self::EnAu => "English (Australia)",
			Self::EnBz => "English (Belize)",
			Self::EnCa => "English (Canada)",
			Self::En029 => "English (Caribbean)",
			Self::EnHk => "English (Hong Kong)",
			Self::EnIn => "English (India)",
			Self::EnIe => "English (Ireland)",
			Self::EnJm => "English (Jamaica)",
			Self::EnMy => "English (Malaysia)",
			Self::EnNz => "English (New Zealand)",
			Self::EnPh => "English (Republic of the Philippines)",
			Self::EnSg => "English (Singapore)",
			Self::EnZa => "English (South Africa)",
			Self::EnTt => "English (Trinidad and Tobago)",
			Self::EnAe => "English (United Arab Emirates)",
			Self::EnGb => "English (United Kingdom)",
			Self::EnUs => "English (United States)",
			Self::EnZw => "English (Zimbabwe)",
			Self::EtEe => "Estonian (Estonia)",
			Self::FoFo => "Faroese (Faroe Islands)",
			Self::FilPh => "Filipino (Philippines)",
			Self::FiFi => "Finnish (Finland)",
			Self::FrBe => "French (Belgium)",
			Self::FrCi => "French (Côte d’Ivoire)",
			Self::FrCm => "French (Cameroon)",
			Self::FrCa => "French (Canada)",
			Self::Fr029 => "French (Caribbean)",
			Self::FrCd => "French (Congo, DRC)",
			Self::FrFr => "French (France)",
			Self::FrHt => "French (Haiti)",
			Self::FrLu => "French (Luxembourg)",
			Self::FrMl => "French (Mali)",
			Self::FrMa => "French (Morocco)",
			Self::FrMc => "French (Principality of Monaco)",
			Self::FrRe => "French (Réunion)",
			Self::FrSn => "French (Senegal)",
			Self::FrCh => "French (Switzerland)",
			Self::FyNl => "Frisian (Netherlands)",
			Self::FfLatnNg => "Fulah (Latin, Nigeria)",
			Self::FfLatnSn => "Fulah (Latin, Senegal)",
			Self::GlEs => "Galician (Spain)",
			Self::KaGe => "Georgian (Georgia)",
			Self::DeAt => "German (Austria)",
			Self::DeDe => "German (Germany)",
			Self::DeLi => "German (Liechtenstein)",
			Self::DeLu => "German (Luxembourg)",
			Self::DeCh => "German (Switzerland)",
			Self::ElGr => "Greek (Greece)",
			Self::KlGl => "Greenlandic (Greenland)",
			Self::GnPy => "Guarani (Paraguay)",
			Self::GuIn => "Gujarati (India)",
			Self::HaLatnNg => "Hausa (Latin, Nigeria)",
			Self::HawUs => "Hawaiian (United States)",
			Self::HeIl => "Hebrew (Israel)",
			Self::HiIn => "Hindi (India)",
			Self::HuHu => "Hungarian (Hungary)",
			Self::IsIs => "Icelandic (Iceland)",
			Self::IgNg => "Igbo (Nigeria)",
			Self::IdId => "Indonesian (Indonesia)",
			Self::IuLatnCa => "Inuktitut (Latin, Canada)",
			Self::IuCansCa => "Inuktitut (Syllabics, Canada)",
			Self::GaIe => "Irish (Ireland)",
			Self::ItIt => "Italian (Italy)",
			Self::ItCh => "Italian (Switzerland)",
			Self::JaJp => "Japanese (Japan)",
			Self::KnIn => "Kannada (India)",
			Self::KrLatnNg => "Kanuri (Latin, Nigeria)",
			Self::KsDevaIn => "Kashmiri (Devanagari, India)",
			Self::KkKz => "Kazakh (Kazakhstan)",
			Self::KmKh => "Khmer (Cambodia)",
			Self::QucLatnGt => "K'iche (Latin, Guatemala)",
			Self::RwRw => "Kinyarwanda (Rwanda)",
			Self::SwKe => "Kiswahili (Kenya)",
			Self::KokIn => "Konkani (India)",
			Self::KoKr => "Korean (Korea)",
			Self::KyKg => "Kyrgyz (Kyrgyzstan)",
			Self::LoLa => "Lao (Lao P.D.R.)",
			Self::LaVa => "Latin (Vatican City)",
			Self::LvLv => "Latvian (Latvia)",
			Self::LtLt => "Lithuanian (Lithuania)",
			Self::DsbDe => "Lower Sorbian (Germany)",
			Self::LbLu => "Luxembourgish (Luxembourg)",
			Self::MkMk => "Macedonian (North Macedonia)",
			Self::MsBn => "Malay (Brunei Darussalam)",
			Self::MsMy => "Malay (Malaysia)",
			Self::MlIn => "Malayalam (India)",
			Self::MtMt => "Maltese (Malta)",
			Self::MiNz => "Maori (New Zealand)",
			Self::ArnCl => "Mapudungun (Chile)",
			Self::MrIn => "Marathi (India)",
			Self::MohCa => "Mohawk (Canada)",
			Self::MnMn => "Mongolian (Cyrillic, Mongolia)",
			Self::MnMongMn => "Mongolian (Traditional Mongolian, Mongolia)",
			Self::NeIn => "Nepali (India)",
			Self::NeNp => "Nepali (Nepal)",
			Self::NbNo => "Norwegian, Bokmål (Norway)",
			Self::NnNo => "Norwegian, Nynorsk (Norway)",
			Self::OcFr => "Occitan (France)",
			Self::OrIn => "Odia (India)",
			Self::OmEt => "Oromo (Ethiopia)",
			Self::PsAf => "Pashto (Afghanistan)",
			Self::FaIr => "Persian (Iran)",
			Self::PlPl => "Polish (Poland)",
			Self::PtBr => "Portuguese (Brazil)",
			Self::PtPt => "Portuguese (Portugal)",
			Self::PaIn => "Punjabi (India)",
			Self::PaArabPk => "Punjabi (Islamic Republic of Pakistan)",
			Self::QuzBo => "Quechua (Bolivia)",
			Self::QuzEc => "Quechua (Ecuador)",
			Self::QuzPe => "Quechua (Peru)",
			Self::RoMd => "Romanian (Moldova)",
			Self::RoRo => "Romanian (Romania)",
			Self::RmCh => "Romansh (Switzerland)",
			Self::RuMd => "Russian (Moldova)",
			Self::RuRu => "Russian (Russia)",
			Self::SahRu => "Sakha (Russia)",
			Self::SmnFi => "Sami, Inari (Finland)",
			Self::SmjNo => "Sami, Lule (Norway)",
			Self::SmjSe => "Sami, Lule (Sweden)",
			Self::SeFi => "Sami, Northern (Finland)",
			Self::SeNo => "Sami, Northern (Norway)",
			Self::SeSe => "Sami, Northern (Sweden)",
			Self::SmsFi => "Sami, Skolt (Finland)",
			Self::SmaNo => "Sami, Southern (Norway)",
			Self::SmaSe => "Sami, Southern (Sweden)",
			Self::SaIn => "Sanskrit (India)",
			Self::GdGb => "Scottish Gaelic (United Kingdom)",
			Self::SrCyrlBa => "Serbian (Cyrillic, Bosnia and Herzegovina)",
			Self::SrCyrlMe => "Serbian (Cyrillic, Montenegro)",
			Self::SrCyrlRs => "Serbian (Cyrillic, Serbia)",
			Self::SrLatnBa => "Serbian (Latin, Bosnia and Herzegovina)",
			Self::SrLatnMe => "Serbian (Latin, Montenegro)",
			Self::SrLatnRs => "Serbian (Latin, Serbia)",
			Self::NsoZa => "Sesotho sa Leboa (South Africa)",
			Self::TnBw => "Setswana (Botswana)",
			Self::TnZa => "Setswana (South Africa)",
			Self::SdArabPk => "Sindhi (Islamic Republic of Pakistan)",
			Self::SiLk => "Sinhala (Sri Lanka)",
			Self::SkSk => "Slovak (Slovakia)",
			Self::SlSi => "Slovenian (Slovenia)",
			Self::SoSo => "Somali (Somalia)",
			Self::StZa => "Sotho (South Africa)",
			Self::EsAr => "Spanish (Argentina)",
			Self::EsVe => "Spanish (Bolivarian Republic of Venezuela)",
			Self::EsBo => "Spanish (Bolivia)",
			Self::EsCl => "Spanish (Chile)",
			Self::EsCo => "Spanish (Colombia)",
			Self::EsCr => "Spanish (Costa Rica)",
			Self::EsCu => "Spanish (Cuba)",
			Self::EsDo => "Spanish (Dominican Republic)",
			Self::EsEc => "Spanish (Ecuador)",
			Self::EsSv => "Spanish (El Salvador)",
			Self::EsGt => "Spanish (Guatemala)",
			Self::EsHn => "Spanish (Honduras)",
			Self::Es419 => "Spanish (Latin America)",
			Self::EsMx => "Spanish (Mexico)",
			Self::EsNi => "Spanish (Nicaragua)",
			Self::EsPa => "Spanish (Panama)",
			Self::EsPy => "Spanish (Paraguay)",
			Self::EsPe => "Spanish (Peru)",
			Self::EsPr => "Spanish (Puerto Rico)",
			Self::EsEs => "Spanish (Spain, International Sort)",
			Self::EsUs => "Spanish (United States)",
			Self::EsUy => "Spanish (Uruguay)",
			Self::SvFi => "Swedish (Finland)",
			Self::SvSe => "Swedish (Sweden)",
			Self::SyrSy => "Syriac (Syria)",
			Self::TgCyrlTj => "Tajik (Cyrillic, Tajikistan)",
			Self::TzmLatnDz => "Tamazight (Latin, Algeria)",
			Self::TaIn => "Tamil (India)",
			Self::TaLk => "Tamil (Sri Lanka)",
			Self::TtRu => "Tatar (Russia)",
			Self::TeIn => "Telugu (India)",
			Self::ThTh => "Thai (Thailand)",
			Self::BoCn => "Tibetan (People's Republic of China)",
			Self::TiEr => "Tigrinya (Eritrea)",
			Self::TiEt => "Tigrinya (Ethiopia)",
			Self::TsZa => "Tsonga (South Africa)",
			Self::TrTr => "Turkish (Turkey)",
			Self::TkTm => "Turkmen (Turkmenistan)",
			Self::UkUa => "Ukrainian (Ukraine)",
			Self::HsbDe => "Upper Sorbian (Germany)",
			Self::UrIn => "Urdu (India)",
			Self::UrPk => "Urdu (Islamic Republic of Pakistan)",
			Self::UgCn => "Uyghur (People's Republic of China)",
			Self::UzCyrlUz => "Uzbek (Cyrillic, Uzbekistan)",
			Self::UzLatnUz => "Uzbek (Latin, Uzbekistan)",
			Self::CaEsValencia => "Valencian (Spain)",
			Self::VeZa => "Venda (South Africa)",
			Self::ViVn => "Vietnamese (Vietnam)",
			Self::CyGb => "Welsh (United Kingdom)",
			Self::WoSn => "Wolof (Senegal)",
			Self::XhZa => "Xhosa (South Africa)",
			Self::IiCn => "Yi (People's Republic of China)",
			Self::Yi001 => "Yiddish (World)",
			Self::YoNg => "Yoruba (Nigeria)",
			Self::ZuZa => "Zulu (South Africa)",
		}
	}
}
