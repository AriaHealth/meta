#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::cmp::{Eq, PartialEq};

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[repr(u16)]
pub enum Country {
  Afghanistan,
  ÅlandIslands,
  Albania,
  Algeria,
  AmericanSamoa,
  Andorra,
  Angola,
  Anguilla,
  Antarctica,
  AntiguaAndBarbuda,
  Argentina,
  Armenia,
  Aruba,
  Australia,
  Austria,
  Azerbaijan,
  Bahamas,
  Bahrain,
  Bangladesh,
  Barbados,
  Belarus,
  Belgium,
  Belize,
  Benin,
  Bermuda,
  Bhutan,
  BoliviaPlurinationalStateOf,
  BonaireSintEustatiusAndSaba,
  BosniaAndHerzegovina,
  Botswana,
  BouvetIsland,
  Brazil,
  BritishIndianOceanTerritory,
  BruneiDarussalam,
  Bulgaria,
  BurkinaFaso,
  Burundi,
  CaboVerde,
  Cambodia,
  Cameroon,
  Canada,
  CaymanIslands,
  CentralAfricanRepublic,
  Chad,
  Chile,
  China,
  ChristmasIsland,
  CocosKeelingIslands,
  Colombia,
  Comoros,
  Congo,
  CongoDemocraticRepublicOfThe,
  CookIslands,
  CostaRica,
  CôteDIvoire,
  Croatia,
  Cuba,
  Curaçao,
  Cyprus,
  Czechia,
  Denmark,
  Djibouti,
  Dominica,
  DominicanRepublic,
  Ecuador,
  Egypt,
  ElSalvador,
  EquatorialGuinea,
  Eritrea,
  Estonia,
  Eswatini,
  Ethiopia,
  FalklandIslandsMalvinas,
  FaroeIslands,
  Fiji,
  Finland,
  France,
  FrenchGuiana,
  FrenchPolynesia,
  FrenchSouthernTerritories,
  Gabon,
  Gambia,
  Georgia,
  Germany,
  Ghana,
  Gibraltar,
  Greece,
  Greenland,
  Grenada,
  Guadeloupe,
  Guam,
  Guatemala,
  Guernsey,
  Guinea,
  GuineaBissau,
  Guyana,
  Haiti,
  HeardIslandAndMcDonaldIslands,
  HolySee,
  Honduras,
  HongKong,
  Hungary,
  Iceland,
  India,
  Indonesia,
  IranIslamicRepublicOf,
  Iraq,
  Ireland,
  IsleOfMan,
  Israel,
  Italy,
  Jamaica,
  Japan,
  Jersey,
  Jordan,
  Kazakhstan,
  Kenya,
  Kiribati,
  KoreaDemocraticPeopleSRepublicOf,
  KoreaRepublicOf,
  Kuwait,
  Kyrgyzstan,
  LaoPeopleSDemocraticRepublic,
  Latvia,
  Lebanon,
  Lesotho,
  Liberia,
  Libya,
  Liechtenstein,
  Lithuania,
  Luxembourg,
  Macao,
  Madagascar,
  Malawi,
  Malaysia,
  Maldives,
  Mali,
  Malta,
  MarshallIslands,
  Martinique,
  Mauritania,
  Mauritius,
  Mayotte,
  Mexico,
  MicronesiaFederatedStatesOf,
  MoldovaRepublicOf,
  Monaco,
  Mongolia,
  Montenegro,
  Montserrat,
  Morocco,
  Mozambique,
  Myanmar,
  Namibia,
  Nauru,
  Nepal,
  Netherlands,
  NewCaledonia,
  NewZealand,
  Nicaragua,
  Niger,
  Nigeria,
  Niue,
  NorfolkIsland,
  NorthMacedonia,
  NorthernMarianaIslands,
  Norway,
  Oman,
  Pakistan,
  Palau,
  PalestineStateOf,
  Panama,
  PapuaNewGuinea,
  Paraguay,
  Peru,
  Philippines,
  Pitcairn,
  Poland,
  Portugal,
  PuertoRico,
  Qatar,
  Réunion,
  Romania,
  RussianFederation,
  Rwanda,
  SaintBarthélemy,
  SaintHelenaAscensionAndTristanDaCunha,
  SaintKittsAndNevis,
  SaintLucia,
  SaintMartinFrenchPart,
  SaintPierreAndMiquelon,
  SaintVincentAndTheGrenadines,
  Samoa,
  SanMarino,
  SaoTomeAndPrincipe,
  SaudiArabia,
  Senegal,
  Serbia,
  Seychelles,
  SierraLeone,
  Singapore,
  SintMaartenDutchPart,
  Slovakia,
  Slovenia,
  SolomonIslands,
  Somalia,
  SouthAfrica,
  SouthGeorgiaAndTheSouthSandwichIslands,
  SouthSudan,
  Spain,
  SriLanka,
  Sudan,
  Suriname,
  SvalbardAndJanMayen,
  Sweden,
  Switzerland,
  SyrianArabRepublic,
  TaiwanProvinceOfChina,
  Tajikistan,
  TanzaniaUnitedRepublicOf,
  Thailand,
  TimorLeste,
  Togo,
  Tokelau,
  Tonga,
  TrinidadAndTobago,
  Tunisia,
  Turkey,
  Turkmenistan,
  TurksAndCaicosIslands,
  Tuvalu,
  Uganda,
  Ukraine,
  UnitedArabEmirates,
  UnitedKingdomOfGreatBritainAndNorthernIreland,
  UnitedStatesOfAmerica,
  UnitedStatesMinorOutlyingIslands,
  Uruguay,
  Uzbekistan,
  Vanuatu,
  VenezuelaBolivarianRepublicOf,
  VietNam,
  VirginIslandsBritish,
  VirginIslandsUS,
  WallisAndFutuna,
  WesternSahara,
  Yemen,
  Zambia,
  Zimbabwe,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Region {
  Africa,
  Americas,
  Asia,
  Europe,
  Oceania,
  Antarctica,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum SubRegion {
  AustraliaAndNewZealand,
  CentralAsia,
  Antarctica,
  EasternAsia,
  EasternEurope,
  LatinAmericaAndTheCaribbean,
  Melanesia,
  Micronesia,
  NorthernAfrica,
  NorthernAmerica,
  NorthernEurope,
  Polynesia,
  SouthEasternAsia,
  SouthernAsia,
  SouthernEurope,
  SubSaharanAfrica,
  WesternAsia,
  WesternEurope,
}

pub trait CountryTrait<To> {
  fn of_country(country: Country) -> To;
}

impl CountryTrait<Region> for Region {
  fn of_country(country: Country) -> Region {
    match country {
      Country::Antarctica => Region::Antarctica,
      Country::Algeria => Region::Africa,
      Country::Angola => Region::Africa,
      Country::Benin => Region::Africa,
      Country::Botswana => Region::Africa,
      Country::BritishIndianOceanTerritory => Region::Africa,
      Country::BurkinaFaso => Region::Africa,
      Country::Burundi => Region::Africa,
      Country::CaboVerde => Region::Africa,
      Country::Cameroon => Region::Africa,
      Country::CentralAfricanRepublic => Region::Africa,
      Country::Chad => Region::Africa,
      Country::Comoros => Region::Africa,
      Country::Congo => Region::Africa,
      Country::CongoDemocraticRepublicOfThe => Region::Africa,
      Country::CôteDIvoire => Region::Africa,
      Country::Djibouti => Region::Africa,
      Country::Egypt => Region::Africa,
      Country::EquatorialGuinea => Region::Africa,
      Country::Eritrea => Region::Africa,
      Country::Eswatini => Region::Africa,
      Country::Ethiopia => Region::Africa,
      Country::FrenchSouthernTerritories => Region::Africa,
      Country::Gabon => Region::Africa,
      Country::Gambia => Region::Africa,
      Country::Ghana => Region::Africa,
      Country::Guinea => Region::Africa,
      Country::GuineaBissau => Region::Africa,
      Country::Kenya => Region::Africa,
      Country::Lesotho => Region::Africa,
      Country::Liberia => Region::Africa,
      Country::Libya => Region::Africa,
      Country::Madagascar => Region::Africa,
      Country::Malawi => Region::Africa,
      Country::Mali => Region::Africa,
      Country::Mauritania => Region::Africa,
      Country::Mauritius => Region::Africa,
      Country::Mayotte => Region::Africa,
      Country::Morocco => Region::Africa,
      Country::Mozambique => Region::Africa,
      Country::Namibia => Region::Africa,
      Country::Niger => Region::Africa,
      Country::Nigeria => Region::Africa,
      Country::Réunion => Region::Africa,
      Country::Rwanda => Region::Africa,
      Country::SaintHelenaAscensionAndTristanDaCunha => Region::Africa,
      Country::SaoTomeAndPrincipe => Region::Africa,
      Country::Senegal => Region::Africa,
      Country::Seychelles => Region::Africa,
      Country::SierraLeone => Region::Africa,
      Country::Somalia => Region::Africa,
      Country::SouthAfrica => Region::Africa,
      Country::SouthSudan => Region::Africa,
      Country::Sudan => Region::Africa,
      Country::TanzaniaUnitedRepublicOf => Region::Africa,
      Country::Togo => Region::Africa,
      Country::Tunisia => Region::Africa,
      Country::Uganda => Region::Africa,
      Country::WesternSahara => Region::Africa,
      Country::Zambia => Region::Africa,
      Country::Zimbabwe => Region::Africa,
      Country::Anguilla => Region::Americas,
      Country::AntiguaAndBarbuda => Region::Americas,
      Country::Argentina => Region::Americas,
      Country::Aruba => Region::Americas,
      Country::Bahamas => Region::Americas,
      Country::Barbados => Region::Americas,
      Country::Belize => Region::Americas,
      Country::Bermuda => Region::Americas,
      Country::BoliviaPlurinationalStateOf => Region::Americas,
      Country::BonaireSintEustatiusAndSaba => Region::Americas,
      Country::BouvetIsland => Region::Americas,
      Country::Brazil => Region::Americas,
      Country::Canada => Region::Americas,
      Country::CaymanIslands => Region::Americas,
      Country::Chile => Region::Americas,
      Country::Colombia => Region::Americas,
      Country::CostaRica => Region::Americas,
      Country::Cuba => Region::Americas,
      Country::Curaçao => Region::Americas,
      Country::Dominica => Region::Americas,
      Country::DominicanRepublic => Region::Americas,
      Country::Ecuador => Region::Americas,
      Country::ElSalvador => Region::Americas,
      Country::FalklandIslandsMalvinas => Region::Americas,
      Country::FrenchGuiana => Region::Americas,
      Country::Greenland => Region::Americas,
      Country::Grenada => Region::Americas,
      Country::Guadeloupe => Region::Americas,
      Country::Guatemala => Region::Americas,
      Country::Guyana => Region::Americas,
      Country::Haiti => Region::Americas,
      Country::Honduras => Region::Americas,
      Country::Jamaica => Region::Americas,
      Country::Martinique => Region::Americas,
      Country::Mexico => Region::Americas,
      Country::Montserrat => Region::Americas,
      Country::Nicaragua => Region::Americas,
      Country::Panama => Region::Americas,
      Country::Paraguay => Region::Americas,
      Country::Peru => Region::Americas,
      Country::PuertoRico => Region::Americas,
      Country::SaintBarthélemy => Region::Americas,
      Country::SaintKittsAndNevis => Region::Americas,
      Country::SaintLucia => Region::Americas,
      Country::SaintMartinFrenchPart => Region::Americas,
      Country::SaintPierreAndMiquelon => Region::Americas,
      Country::SaintVincentAndTheGrenadines => Region::Americas,
      Country::SintMaartenDutchPart => Region::Americas,
      Country::SouthGeorgiaAndTheSouthSandwichIslands => Region::Americas,
      Country::Suriname => Region::Americas,
      Country::TrinidadAndTobago => Region::Americas,
      Country::TurksAndCaicosIslands => Region::Americas,
      Country::UnitedStatesOfAmerica => Region::Americas,
      Country::Uruguay => Region::Americas,
      Country::VenezuelaBolivarianRepublicOf => Region::Americas,
      Country::VirginIslandsBritish => Region::Americas,
      Country::VirginIslandsUS => Region::Americas,
      Country::Afghanistan => Region::Asia,
      Country::Armenia => Region::Asia,
      Country::Azerbaijan => Region::Asia,
      Country::Bahrain => Region::Asia,
      Country::Bangladesh => Region::Asia,
      Country::Bhutan => Region::Asia,
      Country::BruneiDarussalam => Region::Asia,
      Country::Cambodia => Region::Asia,
      Country::China => Region::Asia,
      Country::Cyprus => Region::Asia,
      Country::Georgia => Region::Asia,
      Country::HongKong => Region::Asia,
      Country::India => Region::Asia,
      Country::Indonesia => Region::Asia,
      Country::IranIslamicRepublicOf => Region::Asia,
      Country::Iraq => Region::Asia,
      Country::Israel => Region::Asia,
      Country::Japan => Region::Asia,
      Country::Jordan => Region::Asia,
      Country::Kazakhstan => Region::Asia,
      Country::KoreaDemocraticPeopleSRepublicOf => Region::Asia,
      Country::KoreaRepublicOf => Region::Asia,
      Country::Kuwait => Region::Asia,
      Country::Kyrgyzstan => Region::Asia,
      Country::LaoPeopleSDemocraticRepublic => Region::Asia,
      Country::Lebanon => Region::Asia,
      Country::Macao => Region::Asia,
      Country::Malaysia => Region::Asia,
      Country::Maldives => Region::Asia,
      Country::Mongolia => Region::Asia,
      Country::Myanmar => Region::Asia,
      Country::Nepal => Region::Asia,
      Country::Oman => Region::Asia,
      Country::Pakistan => Region::Asia,
      Country::PalestineStateOf => Region::Asia,
      Country::Philippines => Region::Asia,
      Country::Qatar => Region::Asia,
      Country::SaudiArabia => Region::Asia,
      Country::Singapore => Region::Asia,
      Country::SriLanka => Region::Asia,
      Country::SyrianArabRepublic => Region::Asia,
      Country::TaiwanProvinceOfChina => Region::Asia,
      Country::Tajikistan => Region::Asia,
      Country::Thailand => Region::Asia,
      Country::TimorLeste => Region::Asia,
      Country::Turkey => Region::Asia,
      Country::Turkmenistan => Region::Asia,
      Country::UnitedArabEmirates => Region::Asia,
      Country::Uzbekistan => Region::Asia,
      Country::VietNam => Region::Asia,
      Country::Yemen => Region::Asia,
      Country::ÅlandIslands => Region::Europe,
      Country::Albania => Region::Europe,
      Country::Andorra => Region::Europe,
      Country::Austria => Region::Europe,
      Country::Belarus => Region::Europe,
      Country::Belgium => Region::Europe,
      Country::BosniaAndHerzegovina => Region::Europe,
      Country::Bulgaria => Region::Europe,
      Country::Croatia => Region::Europe,
      Country::Czechia => Region::Europe,
      Country::Denmark => Region::Europe,
      Country::Estonia => Region::Europe,
      Country::FaroeIslands => Region::Europe,
      Country::Finland => Region::Europe,
      Country::France => Region::Europe,
      Country::Germany => Region::Europe,
      Country::Gibraltar => Region::Europe,
      Country::Greece => Region::Europe,
      Country::Guernsey => Region::Europe,
      Country::HolySee => Region::Europe,
      Country::Hungary => Region::Europe,
      Country::Iceland => Region::Europe,
      Country::Ireland => Region::Europe,
      Country::IsleOfMan => Region::Europe,
      Country::Italy => Region::Europe,
      Country::Jersey => Region::Europe,
      Country::Latvia => Region::Europe,
      Country::Liechtenstein => Region::Europe,
      Country::Lithuania => Region::Europe,
      Country::Luxembourg => Region::Europe,
      Country::Malta => Region::Europe,
      Country::MoldovaRepublicOf => Region::Europe,
      Country::Monaco => Region::Europe,
      Country::Montenegro => Region::Europe,
      Country::Netherlands => Region::Europe,
      Country::NorthMacedonia => Region::Europe,
      Country::Norway => Region::Europe,
      Country::Poland => Region::Europe,
      Country::Portugal => Region::Europe,
      Country::Romania => Region::Europe,
      Country::RussianFederation => Region::Europe,
      Country::SanMarino => Region::Europe,
      Country::Serbia => Region::Europe,
      Country::Slovakia => Region::Europe,
      Country::Slovenia => Region::Europe,
      Country::Spain => Region::Europe,
      Country::SvalbardAndJanMayen => Region::Europe,
      Country::Sweden => Region::Europe,
      Country::Switzerland => Region::Europe,
      Country::Ukraine => Region::Europe,
      Country::UnitedKingdomOfGreatBritainAndNorthernIreland => Region::Europe,
      Country::AmericanSamoa => Region::Oceania,
      Country::Australia => Region::Oceania,
      Country::ChristmasIsland => Region::Oceania,
      Country::CocosKeelingIslands => Region::Oceania,
      Country::CookIslands => Region::Oceania,
      Country::Fiji => Region::Oceania,
      Country::FrenchPolynesia => Region::Oceania,
      Country::Guam => Region::Oceania,
      Country::HeardIslandAndMcDonaldIslands => Region::Oceania,
      Country::Kiribati => Region::Oceania,
      Country::MarshallIslands => Region::Oceania,
      Country::MicronesiaFederatedStatesOf => Region::Oceania,
      Country::Nauru => Region::Oceania,
      Country::NewCaledonia => Region::Oceania,
      Country::NewZealand => Region::Oceania,
      Country::Niue => Region::Oceania,
      Country::NorfolkIsland => Region::Oceania,
      Country::NorthernMarianaIslands => Region::Oceania,
      Country::Palau => Region::Oceania,
      Country::PapuaNewGuinea => Region::Oceania,
      Country::Pitcairn => Region::Oceania,
      Country::Samoa => Region::Oceania,
      Country::SolomonIslands => Region::Oceania,
      Country::Tokelau => Region::Oceania,
      Country::Tonga => Region::Oceania,
      Country::Tuvalu => Region::Oceania,
      Country::UnitedStatesMinorOutlyingIslands => Region::Oceania,
      Country::Vanuatu => Region::Oceania,
      Country::WallisAndFutuna => Region::Oceania,
    }
  }
}

impl CountryTrait<SubRegion> for SubRegion {
  fn of_country(country: Country) -> SubRegion {
    match country {
      Country::Antarctica => SubRegion::Antarctica,
      Country::Australia => SubRegion::AustraliaAndNewZealand,
      Country::ChristmasIsland => SubRegion::AustraliaAndNewZealand,
      Country::CocosKeelingIslands => SubRegion::AustraliaAndNewZealand,
      Country::HeardIslandAndMcDonaldIslands => SubRegion::AustraliaAndNewZealand,
      Country::NewZealand => SubRegion::AustraliaAndNewZealand,
      Country::NorfolkIsland => SubRegion::AustraliaAndNewZealand,
      Country::Kazakhstan => SubRegion::CentralAsia,
      Country::Kyrgyzstan => SubRegion::CentralAsia,
      Country::Tajikistan => SubRegion::CentralAsia,
      Country::Turkmenistan => SubRegion::CentralAsia,
      Country::Uzbekistan => SubRegion::CentralAsia,
      Country::China => SubRegion::EasternAsia,
      Country::HongKong => SubRegion::EasternAsia,
      Country::Japan => SubRegion::EasternAsia,
      Country::KoreaDemocraticPeopleSRepublicOf => SubRegion::EasternAsia,
      Country::KoreaRepublicOf => SubRegion::EasternAsia,
      Country::Macao => SubRegion::EasternAsia,
      Country::Mongolia => SubRegion::EasternAsia,
      Country::TaiwanProvinceOfChina => SubRegion::EasternAsia,
      Country::Belarus => SubRegion::EasternEurope,
      Country::Bulgaria => SubRegion::EasternEurope,
      Country::Czechia => SubRegion::EasternEurope,
      Country::Hungary => SubRegion::EasternEurope,
      Country::MoldovaRepublicOf => SubRegion::EasternEurope,
      Country::Poland => SubRegion::EasternEurope,
      Country::Romania => SubRegion::EasternEurope,
      Country::RussianFederation => SubRegion::EasternEurope,
      Country::Slovakia => SubRegion::EasternEurope,
      Country::Ukraine => SubRegion::EasternEurope,
      Country::Anguilla => SubRegion::LatinAmericaAndTheCaribbean,
      Country::AntiguaAndBarbuda => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Argentina => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Aruba => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Bahamas => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Barbados => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Belize => SubRegion::LatinAmericaAndTheCaribbean,
      Country::BoliviaPlurinationalStateOf => SubRegion::LatinAmericaAndTheCaribbean,
      Country::BonaireSintEustatiusAndSaba => SubRegion::LatinAmericaAndTheCaribbean,
      Country::BouvetIsland => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Brazil => SubRegion::LatinAmericaAndTheCaribbean,
      Country::CaymanIslands => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Chile => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Colombia => SubRegion::LatinAmericaAndTheCaribbean,
      Country::CostaRica => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Cuba => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Curaçao => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Dominica => SubRegion::LatinAmericaAndTheCaribbean,
      Country::DominicanRepublic => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Ecuador => SubRegion::LatinAmericaAndTheCaribbean,
      Country::ElSalvador => SubRegion::LatinAmericaAndTheCaribbean,
      Country::FalklandIslandsMalvinas => SubRegion::LatinAmericaAndTheCaribbean,
      Country::FrenchGuiana => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Grenada => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Guadeloupe => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Guatemala => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Guyana => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Haiti => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Honduras => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Jamaica => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Martinique => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Mexico => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Montserrat => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Nicaragua => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Panama => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Paraguay => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Peru => SubRegion::LatinAmericaAndTheCaribbean,
      Country::PuertoRico => SubRegion::LatinAmericaAndTheCaribbean,
      Country::SaintBarthélemy => SubRegion::LatinAmericaAndTheCaribbean,
      Country::SaintKittsAndNevis => SubRegion::LatinAmericaAndTheCaribbean,
      Country::SaintLucia => SubRegion::LatinAmericaAndTheCaribbean,
      Country::SaintMartinFrenchPart => SubRegion::LatinAmericaAndTheCaribbean,
      Country::SaintVincentAndTheGrenadines => SubRegion::LatinAmericaAndTheCaribbean,
      Country::SintMaartenDutchPart => SubRegion::LatinAmericaAndTheCaribbean,
      Country::SouthGeorgiaAndTheSouthSandwichIslands => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Suriname => SubRegion::LatinAmericaAndTheCaribbean,
      Country::TrinidadAndTobago => SubRegion::LatinAmericaAndTheCaribbean,
      Country::TurksAndCaicosIslands => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Uruguay => SubRegion::LatinAmericaAndTheCaribbean,
      Country::VenezuelaBolivarianRepublicOf => SubRegion::LatinAmericaAndTheCaribbean,
      Country::VirginIslandsBritish => SubRegion::LatinAmericaAndTheCaribbean,
      Country::VirginIslandsUS => SubRegion::LatinAmericaAndTheCaribbean,
      Country::Fiji => SubRegion::Melanesia,
      Country::NewCaledonia => SubRegion::Melanesia,
      Country::PapuaNewGuinea => SubRegion::Melanesia,
      Country::SolomonIslands => SubRegion::Melanesia,
      Country::Vanuatu => SubRegion::Melanesia,
      Country::Guam => SubRegion::Micronesia,
      Country::Kiribati => SubRegion::Micronesia,
      Country::MarshallIslands => SubRegion::Micronesia,
      Country::MicronesiaFederatedStatesOf => SubRegion::Micronesia,
      Country::Nauru => SubRegion::Micronesia,
      Country::NorthernMarianaIslands => SubRegion::Micronesia,
      Country::Palau => SubRegion::Micronesia,
      Country::UnitedStatesMinorOutlyingIslands => SubRegion::Micronesia,
      Country::Algeria => SubRegion::NorthernAfrica,
      Country::Egypt => SubRegion::NorthernAfrica,
      Country::Libya => SubRegion::NorthernAfrica,
      Country::Morocco => SubRegion::NorthernAfrica,
      Country::Sudan => SubRegion::NorthernAfrica,
      Country::Tunisia => SubRegion::NorthernAfrica,
      Country::WesternSahara => SubRegion::NorthernAfrica,
      Country::Bermuda => SubRegion::NorthernAmerica,
      Country::Canada => SubRegion::NorthernAmerica,
      Country::Greenland => SubRegion::NorthernAmerica,
      Country::SaintPierreAndMiquelon => SubRegion::NorthernAmerica,
      Country::UnitedStatesOfAmerica => SubRegion::NorthernAmerica,
      Country::ÅlandIslands => SubRegion::NorthernEurope,
      Country::Denmark => SubRegion::NorthernEurope,
      Country::Estonia => SubRegion::NorthernEurope,
      Country::FaroeIslands => SubRegion::NorthernEurope,
      Country::Finland => SubRegion::NorthernEurope,
      Country::Guernsey => SubRegion::NorthernEurope,
      Country::Iceland => SubRegion::NorthernEurope,
      Country::Ireland => SubRegion::NorthernEurope,
      Country::IsleOfMan => SubRegion::NorthernEurope,
      Country::Jersey => SubRegion::NorthernEurope,
      Country::Latvia => SubRegion::NorthernEurope,
      Country::Lithuania => SubRegion::NorthernEurope,
      Country::Norway => SubRegion::NorthernEurope,
      Country::SvalbardAndJanMayen => SubRegion::NorthernEurope,
      Country::Sweden => SubRegion::NorthernEurope,
      Country::UnitedKingdomOfGreatBritainAndNorthernIreland => SubRegion::NorthernEurope,
      Country::AmericanSamoa => SubRegion::Polynesia,
      Country::CookIslands => SubRegion::Polynesia,
      Country::FrenchPolynesia => SubRegion::Polynesia,
      Country::Niue => SubRegion::Polynesia,
      Country::Pitcairn => SubRegion::Polynesia,
      Country::Samoa => SubRegion::Polynesia,
      Country::Tokelau => SubRegion::Polynesia,
      Country::Tonga => SubRegion::Polynesia,
      Country::Tuvalu => SubRegion::Polynesia,
      Country::WallisAndFutuna => SubRegion::Polynesia,
      Country::BruneiDarussalam => SubRegion::SouthEasternAsia,
      Country::Cambodia => SubRegion::SouthEasternAsia,
      Country::Indonesia => SubRegion::SouthEasternAsia,
      Country::LaoPeopleSDemocraticRepublic => SubRegion::SouthEasternAsia,
      Country::Malaysia => SubRegion::SouthEasternAsia,
      Country::Myanmar => SubRegion::SouthEasternAsia,
      Country::Philippines => SubRegion::SouthEasternAsia,
      Country::Singapore => SubRegion::SouthEasternAsia,
      Country::Thailand => SubRegion::SouthEasternAsia,
      Country::TimorLeste => SubRegion::SouthEasternAsia,
      Country::VietNam => SubRegion::SouthEasternAsia,
      Country::Afghanistan => SubRegion::SouthernAsia,
      Country::Bangladesh => SubRegion::SouthernAsia,
      Country::Bhutan => SubRegion::SouthernAsia,
      Country::India => SubRegion::SouthernAsia,
      Country::IranIslamicRepublicOf => SubRegion::SouthernAsia,
      Country::Maldives => SubRegion::SouthernAsia,
      Country::Nepal => SubRegion::SouthernAsia,
      Country::Pakistan => SubRegion::SouthernAsia,
      Country::SriLanka => SubRegion::SouthernAsia,
      Country::Albania => SubRegion::SouthernEurope,
      Country::Andorra => SubRegion::SouthernEurope,
      Country::BosniaAndHerzegovina => SubRegion::SouthernEurope,
      Country::Croatia => SubRegion::SouthernEurope,
      Country::Gibraltar => SubRegion::SouthernEurope,
      Country::Greece => SubRegion::SouthernEurope,
      Country::HolySee => SubRegion::SouthernEurope,
      Country::Italy => SubRegion::SouthernEurope,
      Country::Malta => SubRegion::SouthernEurope,
      Country::Montenegro => SubRegion::SouthernEurope,
      Country::NorthMacedonia => SubRegion::SouthernEurope,
      Country::Portugal => SubRegion::SouthernEurope,
      Country::SanMarino => SubRegion::SouthernEurope,
      Country::Serbia => SubRegion::SouthernEurope,
      Country::Slovenia => SubRegion::SouthernEurope,
      Country::Spain => SubRegion::SouthernEurope,
      Country::Angola => SubRegion::SubSaharanAfrica,
      Country::Benin => SubRegion::SubSaharanAfrica,
      Country::Botswana => SubRegion::SubSaharanAfrica,
      Country::BritishIndianOceanTerritory => SubRegion::SubSaharanAfrica,
      Country::BurkinaFaso => SubRegion::SubSaharanAfrica,
      Country::Burundi => SubRegion::SubSaharanAfrica,
      Country::CaboVerde => SubRegion::SubSaharanAfrica,
      Country::Cameroon => SubRegion::SubSaharanAfrica,
      Country::CentralAfricanRepublic => SubRegion::SubSaharanAfrica,
      Country::Chad => SubRegion::SubSaharanAfrica,
      Country::Comoros => SubRegion::SubSaharanAfrica,
      Country::Congo => SubRegion::SubSaharanAfrica,
      Country::CongoDemocraticRepublicOfThe => SubRegion::SubSaharanAfrica,
      Country::CôteDIvoire => SubRegion::SubSaharanAfrica,
      Country::Djibouti => SubRegion::SubSaharanAfrica,
      Country::EquatorialGuinea => SubRegion::SubSaharanAfrica,
      Country::Eritrea => SubRegion::SubSaharanAfrica,
      Country::Eswatini => SubRegion::SubSaharanAfrica,
      Country::Ethiopia => SubRegion::SubSaharanAfrica,
      Country::FrenchSouthernTerritories => SubRegion::SubSaharanAfrica,
      Country::Gabon => SubRegion::SubSaharanAfrica,
      Country::Gambia => SubRegion::SubSaharanAfrica,
      Country::Ghana => SubRegion::SubSaharanAfrica,
      Country::Guinea => SubRegion::SubSaharanAfrica,
      Country::GuineaBissau => SubRegion::SubSaharanAfrica,
      Country::Kenya => SubRegion::SubSaharanAfrica,
      Country::Lesotho => SubRegion::SubSaharanAfrica,
      Country::Liberia => SubRegion::SubSaharanAfrica,
      Country::Madagascar => SubRegion::SubSaharanAfrica,
      Country::Malawi => SubRegion::SubSaharanAfrica,
      Country::Mali => SubRegion::SubSaharanAfrica,
      Country::Mauritania => SubRegion::SubSaharanAfrica,
      Country::Mauritius => SubRegion::SubSaharanAfrica,
      Country::Mayotte => SubRegion::SubSaharanAfrica,
      Country::Mozambique => SubRegion::SubSaharanAfrica,
      Country::Namibia => SubRegion::SubSaharanAfrica,
      Country::Niger => SubRegion::SubSaharanAfrica,
      Country::Nigeria => SubRegion::SubSaharanAfrica,
      Country::Réunion => SubRegion::SubSaharanAfrica,
      Country::Rwanda => SubRegion::SubSaharanAfrica,
      Country::SaintHelenaAscensionAndTristanDaCunha => SubRegion::SubSaharanAfrica,
      Country::SaoTomeAndPrincipe => SubRegion::SubSaharanAfrica,
      Country::Senegal => SubRegion::SubSaharanAfrica,
      Country::Seychelles => SubRegion::SubSaharanAfrica,
      Country::SierraLeone => SubRegion::SubSaharanAfrica,
      Country::Somalia => SubRegion::SubSaharanAfrica,
      Country::SouthAfrica => SubRegion::SubSaharanAfrica,
      Country::SouthSudan => SubRegion::SubSaharanAfrica,
      Country::TanzaniaUnitedRepublicOf => SubRegion::SubSaharanAfrica,
      Country::Togo => SubRegion::SubSaharanAfrica,
      Country::Uganda => SubRegion::SubSaharanAfrica,
      Country::Zambia => SubRegion::SubSaharanAfrica,
      Country::Zimbabwe => SubRegion::SubSaharanAfrica,
      Country::Armenia => SubRegion::WesternAsia,
      Country::Azerbaijan => SubRegion::WesternAsia,
      Country::Bahrain => SubRegion::WesternAsia,
      Country::Cyprus => SubRegion::WesternAsia,
      Country::Georgia => SubRegion::WesternAsia,
      Country::Iraq => SubRegion::WesternAsia,
      Country::Israel => SubRegion::WesternAsia,
      Country::Jordan => SubRegion::WesternAsia,
      Country::Kuwait => SubRegion::WesternAsia,
      Country::Lebanon => SubRegion::WesternAsia,
      Country::Oman => SubRegion::WesternAsia,
      Country::PalestineStateOf => SubRegion::WesternAsia,
      Country::Qatar => SubRegion::WesternAsia,
      Country::SaudiArabia => SubRegion::WesternAsia,
      Country::SyrianArabRepublic => SubRegion::WesternAsia,
      Country::Turkey => SubRegion::WesternAsia,
      Country::UnitedArabEmirates => SubRegion::WesternAsia,
      Country::Yemen => SubRegion::WesternAsia,
      Country::Austria => SubRegion::WesternEurope,
      Country::Belgium => SubRegion::WesternEurope,
      Country::France => SubRegion::WesternEurope,
      Country::Germany => SubRegion::WesternEurope,
      Country::Liechtenstein => SubRegion::WesternEurope,
      Country::Luxembourg => SubRegion::WesternEurope,
      Country::Monaco => SubRegion::WesternEurope,
      Country::Netherlands => SubRegion::WesternEurope,
      Country::Switzerland => SubRegion::WesternEurope,
    }
  }
}
