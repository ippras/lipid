use super::{FattyAcid, Unsaturated};
use std::fmt::{Display, Formatter, Result, Write as _};

impl FattyAcid {
    pub fn delta(&self) -> Delta<&Self> {
        Delta(self)
    }

    pub fn id(&self) -> Id<&Self> {
        Id(self)
    }

    pub fn iupac(&self) -> Iupac<&Self> {
        Iupac(self)
    }
}

/// Iupac
#[derive(Clone, Debug, Default)]
pub struct Iupac<T>(pub(super) T);

impl Display for Iupac<&FattyAcid> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // 1. Определяем корень
        let root = match self.0.carbon {
            1 => "meth",
            2 => "eth",
            3 => "prop",
            4 => "but",
            5 => "pent",
            6 => "hex",
            7 => "hept",
            8 => "oct",
            9 => "non",
            10 => "dec",
            11 => "undec",
            12 => "dodec",
            13 => "tridec",
            14 => "tetradec",
            15 => "pentadec",
            16 => "hexadec",
            17 => "heptadec",
            18 => "octadec",
            19 => "nonadec",
            20 => "icos",
            21 => "henicos",
            22 => "docos",
            23 => "tricos",
            24 => "tetracos",
            25 => "pentacos",
            26 => "hexacos",
            27 => "heptacos",
            28 => "octacos",
            29 => "nonacos",
            30 => "triacont",
            31 => "hentriacont",
            32 => "dotriacont",
            33 => "tritriacont",
            34 => "tetratriacont",
            35 => "pentatriacont",
            36 => "hexatriacont",
            37 => "heptatriacont",
            38 => "octatriacont",
            39 => "nonatriacont",
            40 => "tetracont",
            _ => unimplemented!(),
        };

        // 2. Формируем префикс стереохимии (например: "(6Z,9Z,12Z)-")
        let mut stereo = self
            .0
            .unsaturated
            .iter()
            .filter_map(|unsaturated| Some((unsaturated.index?, unsaturated.parity?)))
            .peekable();

        if stereo.peek().is_some() {
            write!(f, "(")?;
            let mut first = true;
            for (index, parity) in stereo {
                if !first {
                    write!(f, ",")?;
                }
                let parity = if parity { 'E' } else { 'Z' };
                write!(f, "{index}{parity}")?;
                first = false;
            }
            write!(f, ")-")?;
        }

        // Считаем количество двойных и тройных связей
        let ene_count = self
            .0
            .unsaturated
            .iter()
            .filter(|unsaturated| unsaturated.triple.is_some_and(|triple| !triple))
            .count();
        let yne_count = self
            .0
            .unsaturated
            .iter()
            .filter(|unsaturated| unsaturated.triple.is_some_and(|triple| triple))
            .count();

        // Если связей нет — это насыщенная кислота
        if ene_count == 0 && yne_count == 0 {
            return write!(f, "{root}anoic");
        }

        // 3. Добавляем корень и соединительную 'a'
        let a = if ene_count > 1 || (ene_count == 0 && yne_count > 1) {
            "a"
        } else {
            ""
        };
        write!(f, "{root}{a}")?;

        // Вспомогательное замыкание для форматирования локантов (например: "-6,9,12-")
        let write_locants = |f: &mut Formatter<'_>, is_triple: bool| -> std::fmt::Result {
            let mut locants_iter = self
                .0
                .unsaturated
                .iter()
                .filter(|unsaturated| unsaturated.triple.is_some_and(|triple| triple) == is_triple)
                .filter_map(|unsaturated| unsaturated.index)
                .peekable();

            if locants_iter.peek().is_some() {
                write!(f, "-")?;
                let mut first = true;
                for index in locants_iter {
                    if !first {
                        write!(f, ",")?;
                    }
                    write!(f, "{index}")?;
                    first = false;
                }
                write!(f, "-")?;
            }
            Ok(())
        };

        // 4. Добавляем двойные связи (ene)
        if ene_count > 0 {
            write_locants(f, false)?;
            write!(f, "{}", format_multiplier(ene_count))?;
            if yne_count > 0 {
                write!(f, "en")?; // Если дальше идут тройные связи
            } else {
                write!(f, "enoic")?;
            }
        }

        // 5. Добавляем тройные связи (yne)
        if yne_count > 0 {
            write_locants(f, true)?;
            write!(f, "{}ynoic", format_multiplier(yne_count))?;
        }

        Ok(())
    }
}

/// Вспомогательная функция для множителей ИЮПАК (IUPAC P-14.2.1).
pub fn format_multiplier(n: usize) -> &'static str {
    match n {
        1 => "",
        2 => "di",
        3 => "tri",
        4 => "tetra",
        5 => "penta",
        6 => "hexa",
        7 => "hepta",
        8 => "octa",
        9 => "nona",
        10 => "deca",
        _ => unimplemented!(),
    }
}

/// Delta
#[derive(Clone, Debug, Default)]
pub struct Delta<T>(pub(super) T);

impl Display for Delta<&FattyAcid> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(&self.0.carbon, f)?;
        f.write_char(':')?;
        Display::fmt(&self.0.unsaturated.len(), f)?;
        let mut iter = self.0.unsaturated.iter();
        if let Some(unsaturated) = iter.next() {
            f.write_char('Δ')?;
            Display::fmt(&Delta(unsaturated), f)?;
            for unsaturated in iter {
                f.write_char(',')?;
                Display::fmt(&Delta(unsaturated), f)?;
            }
        }
        Ok(())
    }
}

impl Display for Delta<&Unsaturated> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.0.index {
            None => f.write_char('0')?,
            Some(index) => Display::fmt(&index, f)?,
        }
        match self.0.triple {
            None => f.write_char('u')?, // Unsaturated
            Some(false) => match self.0.parity {
                None => f.write_char('o')?,        // Olefinic
                Some(false) => f.write_char('c')?, // Cis
                Some(true) => f.write_char('t')?,  // Trans
            },
            Some(true) => f.write_char('a')?, // Acetylenic
        }
        Ok(())
    }
}

/// Id
#[derive(Clone, Debug, Default)]
pub struct Id<T>(T);

impl Display for Id<&FattyAcid> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_char('c')?;
        Display::fmt(&self.0.carbon, f)?;
        for unsaturated in &self.0.unsaturated {
            Display::fmt(&Id(unsaturated), f)?;
        }
        Ok(())
    }
}

impl Display for Id<&Unsaturated> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.0.triple {
            None => f.write_char('u')?, // Unsaturated
            Some(false) => match self.0.parity {
                None => f.write_char('o')?,        // Olefinic
                Some(false) => f.write_char('c')?, // Cis
                Some(true) => f.write_char('t')?,  // Trans
            },
            Some(true) => f.write_char('a')?, // Acetylenic
        }
        match self.0.index {
            None => f.write_char('0')?,
            Some(index) => Display::fmt(&index, f)?,
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Kind {
    Delta,
    #[default]
    Id,
}
