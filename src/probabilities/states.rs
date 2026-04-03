/// Initial state before any rules have been applied.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Initial;

/// State after resolving the number of attacks from the profile characteristic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Attacked {
    pub attacks: u32,
}

/// State after the hit roll (and any crit effects).
///
/// - `hits`: successful hits to be wound-rolled.
/// - `wounds`: auto-wounds from crit effects (e.g. CritAutoWoundRule), bypass wound roll.
/// - `mortal_wounds`: mortal wounds from crit effects (e.g. CritMortalWoundRule), bypass wound+save.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Hit {
    pub hits: u32,
    pub wounds: u32,
    pub mortal_wounds: u32,
}

/// State after the wound roll.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Wounded {
    pub wounds: u32,
    pub mortal_wounds: u32,
}

/// State after the save roll.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Saved {
    pub unsaved_wounds: u32,
    pub mortal_wounds: u32,
}

/// State after converting wounds to damage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Damaged {
    pub damages: u32,
}
