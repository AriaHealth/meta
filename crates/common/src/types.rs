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
