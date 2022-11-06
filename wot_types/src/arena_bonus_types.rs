#[derive(PartialEq, Hash, Eq, Copy, Clone, Debug)]
/// Gamemode. It could be a random battle or a frontlines battle etc.
pub enum ArenaBonusType {
    Unknown              = 0,
    Regular              = 1,
    Training             = 2,
    Company              = 3,
    Tournament           = 4,
    Clan                 = 5,
    Tutorial             = 6,
    Cybersport           = 7,
    Historical           = 8,
    EventBattles         = 9,
    Sortie               = 10,
    FortBattle           = 11,
    RatedCybersport      = 12,
    GlobalMap            = 13,
    TournamentRegular    = 14,
    TournamentClan       = 15,
    RatedSandbox         = 16,
    Sandbox              = 17,
    FalloutClassic       = 18,
    FalloutMultiteam     = 19,
    Sortie2              = 20,
    FortBattle2          = 21,
    Ranked               = 22,
    Bootcamp             = 23,
    EpicRandom           = 24,
    EpicRandomTraining   = 25,
    EventBattles2        = 26,
    EpicBattle           = 27,
    EpicBattleTraining   = 28,
    BattleRoyaleSolo     = 29,
    BattleRoyaleSquad    = 30,
    TournamentEvent      = 31,
    Bob                  = 32,
    EventRandom          = 33,
    BattleRoyaleTrnSolo  = 34,
    BattleRoyaleTrnSquad = 35,
    WeekendBrawl         = 36,
    Mapbox               = 37,
    MapsTraining         = 38,

    /// `Rts` is the Art of Strategy Gameode
    Rts                  = 39,
    Rts1x1               = 40,
    RtsBootcamp          = 41,
}

impl ArenaBonusType {
    pub fn from(value: i32) -> Option<ArenaBonusType> {
        use ArenaBonusType::*;
        match value {
            0 => Some(Unknown),
            1 => Some(Regular),
            2 => Some(Training),
            4 => Some(Tournament),
            5 => Some(Clan),
            6 => Some(Tutorial),
            7 => Some(Cybersport),
            9 => Some(EventBattles),
            13 => Some(GlobalMap),
            14 => Some(TournamentRegular),
            15 => Some(TournamentClan),
            16 => Some(RatedSandbox),
            17 => Some(Sandbox),
            18 => Some(FalloutClassic),
            19 => Some(FalloutMultiteam),
            20 => Some(Sortie2),
            21 => Some(FortBattle2),
            22 => Some(Ranked),
            23 => Some(Bootcamp),
            24 => Some(EpicRandom),
            25 => Some(EpicRandomTraining),
            26 => Some(EventBattles2),
            27 => Some(EpicBattle),
            28 => Some(EpicBattleTraining),
            29 => Some(BattleRoyaleSolo),
            30 => Some(BattleRoyaleSquad),
            31 => Some(TournamentEvent),
            32 => Some(Bob),
            33 => Some(EventRandom),
            34 => Some(BattleRoyaleTrnSolo),
            35 => Some(BattleRoyaleTrnSquad),
            36 => Some(WeekendBrawl),
            37 => Some(Mapbox),
            38 => Some(MapsTraining),
            39 => Some(Rts),
            40 => Some(Rts1x1),
            41 => Some(RtsBootcamp),
            _ => None,
        }
    }
}

impl std::fmt::Display for ArenaBonusType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ArenaBonusType::*;
        match self {
            Unknown => write!(f, "Unknown"),
            Regular => write!(f, "Regular"),
            Training => write!(f, "Training"),
            Tournament => write!(f, "Tournament"),
            Clan => write!(f, "Clan"),
            Tutorial => write!(f, "Tutorial"),
            Cybersport => write!(f, "Cybersport"),
            EventBattles => write!(f, "EventBattles"),
            GlobalMap => write!(f, "GlobalMap"),
            TournamentRegular => write!(f, "TournamentRegular"),
            TournamentClan => write!(f, "TournamentClan"),
            RatedSandbox => write!(f, "RatedSandbox"),
            Sandbox => write!(f, "Sandbox"),
            FalloutClassic => write!(f, "FalloutClassic"),
            FalloutMultiteam => write!(f, "FalloutMultiteam"),
            Sortie2 => write!(f, "Sortie2"),
            FortBattle2 => write!(f, "FortBattle2"),
            Ranked => write!(f, "Ranked"),
            Bootcamp => write!(f, "Bootcamp"),
            EpicRandom => write!(f, "EpicRandom"),
            EpicRandomTraining => write!(f, "EpicRandomTraining"),
            EventBattles2 => write!(f, "EventBattles2"),
            EpicBattle => write!(f, "EpicBattle"),
            EpicBattleTraining => write!(f, "EpicBattleTraining"),
            BattleRoyaleSolo => write!(f, "BattleRoyaleSolo"),
            BattleRoyaleSquad => write!(f, "BattleRoyaleSquad"),
            TournamentEvent => write!(f, "TournamentEvent"),
            Bob => write!(f, "Bob"),
            EventRandom => write!(f, "EventRandom"),
            BattleRoyaleTrnSolo => write!(f, "BattleRoyaleTrnSolo"),
            BattleRoyaleTrnSquad => write!(f, "BattleRoyaleTrnSquad"),
            WeekendBrawl => write!(f, "WeekendBrawl"),
            Mapbox => write!(f, "Mapbox"),
            MapsTraining => write!(f, "MapsTraining"),
            Rts => write!(f, "Rts"),
            Rts1x1 => write!(f, "Rts1x1"),
            RtsBootcamp => write!(f, "RtsBootcamp"),
        }
    }
}
