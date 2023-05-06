use crate::{DisplayFirstLetter, Health};
use enum_iterator::Sequence;

#[derive(Debug, PartialEq, PartialOrd,Default, Clone, Copy, Sequence)]
#[derive(derive_more::Display)]
pub enum UnitType {
    AI,
    Virus,
    Tech,
    Firewall,
    #[default]
    Program,
}

impl DisplayFirstLetter for UnitType {}

impl UnitType {
    pub fn all() -> enum_iterator::All<Self> {
        enum_iterator::all()
    }
    pub fn stats_table(
            legend: Option<&str>, 
            stat_fn: impl Fn (&Self,&Self) -> Health,
            format_header: impl Fn (&String) -> String,
            format_data: impl Fn (&String) -> String,
        ) -> Vec<Vec<String>> {
        let mut result : Vec<Vec<String>> = Vec::new();
        let mut targets = if let Some(legend) = legend {
            vec![legend.to_string()]
        } else {
            vec!["".to_string()]
        };
        targets.extend(Self::all().map(|t|format_header(&t.to_string())));
        result.push(targets);
        for source in Self::all() {
            let mut targets = vec![source.to_string()];
            if Self::all().map(|t|stat_fn(&source,&t)).sum::<Health>() != 0 {
                targets.extend(Self::all().map(|t|format_data(&stat_fn(&source,&t).to_string())));
                result.push(targets);
            }
        }
        result
    }
    pub fn damage_table(
        legend: Option<&str>, 
        format_header: impl Fn (&String) -> String,
        format_data: impl Fn (&String) -> String,
    ) -> Vec<Vec<String>> {
        Self::stats_table(legend, Self::damage_amount, format_header, format_data)
    }
    pub fn repair_table(
        legend: Option<&str>, 
        format_header: impl Fn (&String) -> String,
        format_data: impl Fn (&String) -> String,
    ) -> Vec<Vec<String>> {
        Self::stats_table(legend, Self::repair_amount, format_header, format_data)
    }
    pub fn can_move_back(&self) -> bool {
        use UnitType::*;
        match self {
            Virus | Tech => true,
            _ => false,
        }
    }
    pub fn can_move_while_engaged(&self) -> bool {
        use UnitType::*;
        match self {
            Virus | Tech => true,
            _ => false,
        }
    }
    pub fn initial_health(&self) -> Health {
        9
    }
    pub fn damage_amount(&self, target: &Self) -> Health {
        use UnitType::*;
        match self {
            AI => match target {
                Firewall => 1,
                _ => 3,
            },
            Virus => match target {
                AI => 9,
                Tech | Program => 6,
                Virus | Firewall => 1,
            },
            Tech => match target {
                Virus => 6,
                _ => 1,
            },
            Firewall => match target {
                _ => 1,
            },
            Program => match target {
                Firewall => 1,
                _ => 3,
            },
        }
    }
    pub fn repair_amount(&self, target: &Self) -> Health {
        use UnitType::*;
        match self {
            Tech => match target {
                AI | Firewall | Program => 3,
                _ => 0,
            },
            AI  => match target {
                Virus | Tech => 1,
                _ => 0,
            },
            _ => 0,
        }
    }
    pub fn self_destruct_amount(&self, _target: &Self) -> Health {
        2
    }
}