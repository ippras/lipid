//! Разбор записей жирных кислот из текста.
//!
//! Обратная операция к [`Delta`](super::display::Delta) и
//! [`Omega`](super::display::Omega): `FattyAcid::from_str` понимает обе нотации
//! и распространённые варианты их записи.
//!
//! | Запись                      | Результат                                              |
//! |-----------------------------|--------------------------------------------------------|
//! | `18:0`                      | насыщенная кислота                                     |
//! | `18:2`                      | две связи, положения неизвестны                        |
//! | `18:2Δ9c,12c`               | Δ-нотация: положения от карбоксильного конца, геометрия |
//! | `18:3ω3`, `18:3n-3`         | ω-нотация: положение последней связи от метильного конца |
//!
//! В ω-нотации известна только последняя связь, поэтому остальные ставятся через
//! 3 атома углерода (метиленпрерванная система), все связи двойные и `cis`.
//! Так устроены природные ω-3 и ω-6 кислоты: `20:4ω6` даёт `5,8,11,14`.

use super::{FattyAcid, Unsaturated};
use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

/// Маркеры Δ-нотации: `Δ` (U+0394), `δ`, `∆` (U+2206, часто попадает из буфера
/// обмена вместо U+0394), `d`.
const DELTA: char = 'Δ';
/// Маркеры ω-нотации: `ω` (U+03C9), `Ω` (U+03A9), `Ω` (U+2126, знак «ом»), `w`, `n`.
const OMEGA: char = ω;
/// Тире между `n` и номером: `n-3`, `n−3`, `n–3`.
const DASH: char = '-';
/// Шаг между двойными связями в метиленпрерванной системе.
const STEP: u32 = 3;
/// Связь, которую подразумевает ω-нотация: двойная, `cis`.
const DOUBLE_CIS: Unsaturated = Unsaturated {
    index: None,
    triple: Some(false),
    parity: Some(false),
};

/// Ошибка разбора жирной кислоты
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// Пустая строка
    Empty,
    /// Ожидалось `<углерод>:<число связей>`
    Header(String),
    /// Не число или число слишком большое
    Number(String),
    /// Неизвестное обозначение связи
    Bond(String),
    /// Лишний или нераспознанный текст
    Unexpected(String),
    /// Заявленное число связей не совпало с перечисленными
    Count { expected: usize, found: usize },
    /// Связей больше, чем позволяет цепь
    TooManyBonds { count: usize, carbon: u8 },
    /// Положение связи вне цепи (допустимо `1..carbon`)
    Index { index: u32, carbon: u8 },
    /// ω-позиция не помещается в цепь
    Omega {
        omega: u32,
        carbon: u8,
        count: usize,
    },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty fatty acid"),
            Self::Header(rest) => {
                write!(f, "expected `<carbon>:<unsaturated>`, found `{rest}`")
            }
            Self::Number(number) => write!(f, "invalid number `{number}`"),
            Self::Bond(kind) => write!(
                f,
                "unknown bond kind `{kind}`, expected one of: c, t, z, e, a, o, u",
            ),
            Self::Unexpected(rest) => write!(f, "unexpected `{rest}`"),
            Self::Count { expected, found } => {
                write!(f, "{expected} bonds declared, but {found} listed")
            }
            Self::TooManyBonds { count, carbon } => {
                write!(
                    f,
                    "{count} bonds do not fit into a chain of {carbon} carbons"
                )
            }
            Self::Index { index, carbon } => write!(
                f,
                "bond position {index} is outside of the chain, expected 1..{carbon}",
            ),
            Self::Omega {
                omega,
                carbon,
                count,
            } => write!(
                f,
                "ω{omega} with {count} bonds does not fit into a chain of {carbon} carbons",
            ),
        }
    }
}

impl std::error::Error for ParseError {}

impl FromStr for FattyAcid {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cursor = Cursor(s);
        if cursor.rest().is_empty() {
            return Err(ParseError::Empty);
        }
        // Необязательный префикс: `C18:1`
        cursor.eat(&['C', 'c']);
        let carbon = match cursor.number()? {
            Some(carbon) => {
                u8::try_from(carbon).map_err(|_| ParseError::Number(carbon.to_string()))?
            }
            None => return Err(cursor.header()),
        };
        if !cursor.eat(&[':']) {
            return Err(cursor.header());
        }
        let Some(count) = cursor.number()? else {
            return Err(cursor.header());
        };
        let count = count as usize;
        if count > usize::from(carbon.saturating_sub(1)) {
            return Err(ParseError::TooManyBonds { count, carbon });
        }
        let unsaturated = if count == 0 {
            Vec::new()
        } else {
            bonds(&mut cursor, carbon, count)?
        };
        if !cursor.rest().is_empty() {
            return Err(cursor.unexpected());
        }
        Ok(FattyAcid {
            carbon,
            unsaturated,
        })
    }
}

/// Часть записи после `<углерод>:<число связей>`
fn bonds(cursor: &mut Cursor, carbon: u8, count: usize) -> Result<Vec<Unsaturated>, ParseError> {
    // `18:2`: связи есть, а где они — неизвестно
    if cursor.rest().is_empty() {
        return Ok(vec![Unsaturated::default(); count]);
    }
    let parenthesized = cursor.eat(&['(']);
    let unsaturated = if cursor.eat(&[DELTA]) {
        delta(cursor, carbon)?
    } else if cursor.eat(&[OMEGA]) {
        omega(cursor, carbon, count)?
    } else {
        return Err(cursor.unexpected());
    };
    if parenthesized && !cursor.eat(&[')']) {
        return Err(cursor.unexpected());
    }
    if unsaturated.len() != count {
        return Err(ParseError::Count {
            expected: count,
            found: unsaturated.len(),
        });
    }
    Ok(unsaturated)
}

/// Δ-нотация: `9c,12c`, `9Z;12Z`, `9cis,12cis`, `9,12`
fn delta(cursor: &mut Cursor, carbon: u8) -> Result<Vec<Unsaturated>, ParseError> {
    let mut unsaturated = Vec::new();
    loop {
        let Some(index) = cursor.number()? else {
            return Err(cursor.unexpected());
        };
        // `0` — положение неизвестно (так его печатает `Delta`)
        let index = match index {
            0 => None,
            index if index < u32::from(carbon) => Some(index as u8),
            index => return Err(ParseError::Index { index, carbon }),
        };
        unsaturated.push(bond(index, cursor.letters())?);
        if !cursor.eat(&[',', ';']) {
            return Ok(unsaturated);
        }
    }
}

/// ω-нотация: `3`, `-3`, `?`. Маркер (`ω`, `n`, ...) уже прочитан.
fn omega(cursor: &mut Cursor, carbon: u8, count: usize) -> Result<Vec<Unsaturated>, ParseError> {
    cursor.eat(&DASHES);
    // `Omega` печатает `?`, когда положение определить нельзя
    if cursor.eat(&['?']) {
        return Ok(vec![Unsaturated::default(); count]);
    }
    let Some(omega) = cursor.number()? else {
        return Err(cursor.unexpected());
    };
    // Последняя связь на позиции `carbon − ω`, остальные через 3 атома назад.
    // Первая связь должна остаться в цепи: позиция не меньше 1.
    let span = STEP * (count as u32 - 1);
    let last = u32::from(carbon)
        .checked_sub(omega)
        .filter(|&last| omega != 0 && span < last)
        .ok_or(ParseError::Omega {
            omega,
            carbon,
            count,
        })?;
    let first = last - span;
    Ok((0..count as u32)
        .map(|k| Unsaturated {
            index: Some((first + STEP * k) as u8),
            ..DOUBLE_CIS
        })
        .collect())
}

/// Одна Δ-связь: положение и обозначение (`c`, `t`, `a`, ...)
fn bond(index: Option<u8>, kind: &str) -> Result<Unsaturated, ParseError> {
    let (triple, parity) = match kind.to_lowercase().as_str() {
        // Голое число: связь есть, геометрия не указана (как `o` в `Delta`)
        "" | "o" => (Some(false), None),
        "u" => (None, None),
        "c" | "z" | "cis" => (Some(false), Some(false)),
        "t" | "e" | "trans" => (Some(false), Some(true)),
        "a" => (Some(true), None),
        _ => return Err(ParseError::Bond(kind.to_owned())),
    };
    Ok(Unsaturated {
        index,
        triple,
        parity,
    })
}

/// Курсор по необработанной части строки
struct Cursor<'a>(&'a str);

impl<'a> Cursor<'a> {
    /// Необработанная часть без ведущих пробелов (включая неразрывные)
    fn rest(&mut self) -> &'a str {
        self.0 = self.0.trim_start();
        self.0
    }

    /// Съедает один из символов
    fn eat(&mut self, chars: &[char]) -> bool {
        match self.rest().strip_prefix(chars) {
            Some(rest) => {
                self.0 = rest;
                true
            }
            None => false,
        }
    }

    /// Съедает слово без учёта регистра
    fn eat_word(&mut self, word: &str) -> bool {
        let rest = self.rest();
        match rest.get(..word.len()) {
            Some(head) if head.eq_ignore_ascii_case(word) => {
                self.0 = &rest[word.len()..];
                true
            }
            _ => false,
        }
    }

    /// Читает число; `None`, если дальше не цифра
    fn number(&mut self) -> Result<Option<u32>, ParseError> {
        let rest = self.rest();
        let end = rest
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(rest.len());
        if end == 0 {
            return Ok(None);
        }
        let (digits, tail) = rest.split_at(end);
        let number = digits
            .parse()
            .map_err(|_| ParseError::Number(digits.to_owned()))?;
        self.0 = tail;
        Ok(Some(number))
    }

    /// Читает буквы, идущие сразу за числом
    fn letters(&mut self) -> &'a str {
        let end = self
            .0
            .find(|c: char| !c.is_alphabetic())
            .unwrap_or(self.0.len());
        let (letters, tail) = self.0.split_at(end);
        self.0 = tail;
        letters
    }

    fn header(&mut self) -> ParseError {
        ParseError::Header(self.rest().to_owned())
    }

    fn unexpected(&mut self) -> ParseError {
        ParseError::Unexpected(self.rest().to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Кислота с `cis`-двойными связями
    fn cis(carbon: u8, indices: &[u8]) -> FattyAcid {
        FattyAcid {
            carbon,
            unsaturated: indices
                .iter()
                .map(|&index| Unsaturated {
                    index: Some(index),
                    ..DOUBLE_CIS
                })
                .collect(),
        }
    }

    fn parse(s: &str) -> FattyAcid {
        s.parse()
            .unwrap_or_else(|error| panic!("`{s}` is not parsed: {error}"))
    }

    #[test]
    fn delta_variants() {
        let expected = cis(18, &[9, 12]);
        for s in [
            "18:2Δ9c,12c",
            "18:2 Δ9c, 12c",
            "  18:2 Δ 9c , 12c\n",
            "C18:2Δ9c,12c",
            "c18:2Δ9c,12c",
            "18:2δ9c,12c",
            "18:2\u{2206}9c,12c",
            "18:2\u{a0}Δ9c,12c",
            "18:2D9c;12c",
            "18:2d9c,12c",
            "18:2delta9c,12c",
            "18:2 Delta 9c,12c",
            "18:2Δ9Z,12Z",
            "18:2Δ9cis,12cis",
            "18:2Δ9C,12C",
            "18:2 (Δ9c,12c)",
        ] {
            assert_eq!(parse(s), expected, "{s:?}");
        }
    }

    #[test]
    fn delta_bonds() {
        let acid = parse("20:5Δ5t,8o,11,14u,17a");
        let bond = |index, triple, parity| Unsaturated {
            index,
            triple,
            parity,
        };
        assert_eq!(
            acid,
            FattyAcid {
                carbon: 20,
                unsaturated: vec![
                    bond(Some(5), Some(false), Some(true)),
                    bond(Some(8), Some(false), None),
                    bond(Some(11), Some(false), None),
                    bond(Some(14), None, None),
                    bond(Some(17), Some(true), None),
                ],
            },
        );
        // `0` — положение неизвестно
        assert_eq!(parse("18:1Δ0c").unsaturated[0].index, None);
        assert_eq!(parse("18:1Δ9E").unsaturated[0].parity, Some(true));
    }

    #[test]
    fn omega_natural_acids() {
        for (s, expected) in [
            ("18:1ω9", cis(18, &[9])),
            ("16:1ω7", cis(16, &[9])),
            ("22:1ω9", cis(22, &[13])),
            ("24:1ω9", cis(24, &[15])),
            ("18:2ω6", cis(18, &[9, 12])),
            ("18:3ω3", cis(18, &[9, 12, 15])),
            ("18:3ω6", cis(18, &[6, 9, 12])),
            ("20:4ω6", cis(20, &[5, 8, 11, 14])),
            ("20:5ω3", cis(20, &[5, 8, 11, 14, 17])),
            ("22:6ω3", cis(22, &[4, 7, 10, 13, 16, 19])),
        ] {
            assert_eq!(parse(s), expected, "{s}");
        }
    }

    #[test]
    fn omega_variants() {
        let expected = cis(18, &[9, 12, 15]);
        for s in [
            "18:3ω3",
            "18:3 ω3",
            "18:3ω-3",
            "18:3 ω-3",
            "18:3Ω3",
            "18:3\u{2126}3",
            "18:3n-3",
            "18:3 n-3",
            "18:3 n3",
            "18:3N-3",
            "18:3n\u{2212}3",
            "18:3n\u{2013}3",
            "18:3w3",
            "18:3 (n-3)",
            "18:3 (ω-3)",
            "18:3 omega 3",
            "18:3 Omega-3",
            "C18:3 n-3",
        ] {
            assert_eq!(parse(s), expected, "{s:?}");
        }
    }

    #[test]
    fn without_positions() {
        assert_eq!(parse("18:0"), cis(18, &[]));
        assert_eq!(parse("0:0"), FattyAcid::default());
        let unknown = FattyAcid {
            carbon: 18,
            unsaturated: vec![Unsaturated::default(); 2],
        };
        assert_eq!(parse("18:2"), unknown);
        // То, что печатает `Omega`, когда положение неизвестно
        assert_eq!(parse("18:2ω?"), unknown);
        assert_eq!(unknown.omega().to_string(), "18:2ω?");
        assert_eq!(parse("18:2n-?"), unknown);
    }

    /// Всё, что печатает `Delta`, читается обратно без потерь
    #[test]
    fn delta_round_trip() {
        fn bonds(carbon: u8) -> Vec<Unsaturated> {
            let mut bonds = Vec::new();
            let indices = [
                None,
                Some(1),
                Some(carbon / 2),
                Some(carbon.saturating_sub(1)),
            ];
            for index in indices {
                if index.is_some_and(|index| index == 0 || index >= carbon) {
                    continue;
                }
                // Чётность имеет смысл только у двойной связи
                for (triple, parities) in [
                    (None, &[None][..]),
                    (Some(false), &[None, Some(false), Some(true)][..]),
                    (Some(true), &[None][..]),
                ] {
                    for &parity in parities {
                        bonds.push(Unsaturated {
                            index,
                            triple,
                            parity,
                        });
                    }
                }
            }
            bonds
        }

        fn sequences(bonds: &[Unsaturated], len: usize) -> Vec<Vec<Unsaturated>> {
            let mut sequences = vec![Vec::new()];
            for _ in 0..len {
                sequences = sequences
                    .iter()
                    .flat_map(|prefix| {
                        bonds.iter().map(move |&bond| {
                            let mut sequence = prefix.clone();
                            sequence.push(bond);
                            sequence
                        })
                    })
                    .collect();
            }
            sequences
        }

        let mut checked = 0;
        for carbon in 0..=26u8 {
            let bonds = bonds(carbon);
            for len in 0..=usize::from(carbon.saturating_sub(1)).min(3) {
                for unsaturated in sequences(&bonds, len) {
                    let acid = FattyAcid {
                        carbon,
                        unsaturated,
                    };
                    let s = acid.delta().to_string();
                    assert_eq!(parse(&s), acid, "{s}");
                    checked += 1;
                }
            }
        }
        assert!(checked > 100_000, "{checked}");
    }

    /// Всё, что печатает `Omega` для метиленпрерванных `cis`-кислот, читается обратно
    #[test]
    fn omega_round_trip() {
        for acid in [
            cis(18, &[9]),
            cis(18, &[9, 12]),
            cis(18, &[9, 12, 15]),
            cis(18, &[6, 9, 12]),
            cis(20, &[5, 8, 11, 14]),
            cis(20, &[5, 8, 11, 14, 17]),
            cis(22, &[4, 7, 10, 13, 16, 19]),
            cis(24, &[15]),
            cis(18, &[]),
        ] {
            let s = acid.omega().to_string();
            assert_eq!(parse(&s), acid, "{s}");
        }
    }

    #[test]
    fn errors() {
        let text = |s: &str| ParseError::Unexpected(s.to_owned());
        let header = |s: &str| ParseError::Header(s.to_owned());
        for (s, expected) in [
            ("", ParseError::Empty),
            (" \t ", ParseError::Empty),
            ("abc", header("abc")),
            ("18", header("")),
            ("18:", header("")),
            ("18:x", header("x")),
            ("c18c9c12", header("c9c12")),
            ("300:1", ParseError::Number("300".into())),
            (
                "18:99999999999999999999",
                ParseError::Number("99999999999999999999".into()),
            ),
            (
                "1:1",
                ParseError::TooManyBonds {
                    count: 1,
                    carbon: 1,
                },
            ),
            (
                "18:18",
                ParseError::TooManyBonds {
                    count: 18,
                    carbon: 18,
                },
            ),
            (
                "18:2Δ9c",
                ParseError::Count {
                    expected: 2,
                    found: 1,
                },
            ),
            (
                "18:1Δ9c,12c",
                ParseError::Count {
                    expected: 1,
                    found: 2,
                },
            ),
            (
                "18:1Δ18c",
                ParseError::Index {
                    index: 18,
                    carbon: 18,
                },
            ),
            ("18:1Δ9q", ParseError::Bond("q".into())),
            ("18:1Δ9cc", ParseError::Bond("cc".into())),
            ("18:0Δ9c", text("Δ9c")),
            ("18:0ω9", text("ω9")),
            ("18:1ω9 extra", text("extra")),
            ("18:1 foo", text("foo")),
            ("18:1Δ", text("")),
            ("18:1ω", text("")),
            ("18:2 (Δ9c,12c", text("")),
            (
                "18:1ω0",
                ParseError::Omega {
                    omega: 0,
                    carbon: 18,
                    count: 1,
                },
            ),
            (
                "18:1ω18",
                ParseError::Omega {
                    omega: 18,
                    carbon: 18,
                    count: 1,
                },
            ),
            (
                "18:3ω16",
                ParseError::Omega {
                    omega: 16,
                    carbon: 18,
                    count: 3,
                },
            ),
        ] {
            let error = s.parse::<FattyAcid>().expect_err(s);
            assert_eq!(error, expected, "{s:?}");
            assert!(!error.to_string().is_empty());
        }
    }
}
