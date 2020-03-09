//! A shim layer that allows querying certain info about `Arg`,
//! despite the corresponding fields being private.

use super::*;

#[allow(missing_debug_implementations)]
pub struct ArgView<'arg, 'help> {
    name: &'help str,
    help: Option<&'help str>,
    blacklist: Option<&'arg Vec<Id>>,
    settings: ArgFlags,
    r_unless: Option<&'arg Vec<Id>>,
    overrides: Option<&'arg Vec<Id>>,
    groups: Option<&'arg Vec<Id>>,
    requires: Option<&'arg Vec<(Option<&'help str>, Id)>>,
    short: Option<char>,
    long: Option<&'help str>,
    aliases: Option<&'arg Vec<(&'help str, bool)>>, // (name, visible)
    disp_ord: usize,
    unified_ord: usize,
    possible_vals: Option<&'arg Vec<&'help str>>,
    val_names: Option<&'arg VecMap<&'help str>>,
    num_vals: Option<u64>,
    max_vals: Option<u64>,
    min_vals: Option<u64>,
    validator: Option<&'arg Validator>,
    validator_os: Option<&'arg ValidatorOs>,
    val_delim: Option<char>,
    default_vals: Option<&'arg Vec<&'help OsStr>>,
    default_vals_ifs: Option<&'arg VecMap<(Id, Option<&'help OsStr>, &'help OsStr)>>,
    env: Option<&'arg (&'help OsStr, Option<OsString>)>,
    terminator: Option<&'help str>,
    index: Option<u64>,
    r_ifs: Option<&'arg Vec<(Id, &'help str)>>,
    help_heading: Option<&'help str>,
    global: bool,
    exclusive: bool,
}

impl<'arg, 'help> ArgView<'arg, 'help> {
    pub(crate) fn from_ref(arg: &'arg Arg<'help>) -> Self {
        ArgView {
            name: arg.name,
            help: arg.help,
            blacklist: arg.blacklist.as_ref(),
            settings: arg.settings,
            r_unless: arg.r_unless.as_ref(),
            overrides: arg.overrides.as_ref(),
            groups: arg.groups.as_ref(),
            requires: arg.requires.as_ref(),
            short: arg.short,
            long: arg.long,
            aliases: arg.aliases.as_ref(),
            disp_ord: arg.disp_ord,
            unified_ord: arg.unified_ord,
            possible_vals: arg.possible_vals.as_ref(),
            val_names: arg.val_names.as_ref(),
            num_vals: arg.num_vals,
            max_vals: arg.max_vals,
            min_vals: arg.min_vals,
            validator: arg.validator.as_ref(),
            validator_os: arg.validator_os.as_ref(),
            val_delim: arg.val_delim,
            default_vals: arg.default_vals.as_ref(),
            default_vals_ifs: arg.default_vals_ifs.as_ref(),
            env: arg.env.as_ref(),
            terminator: arg.terminator,
            index: arg.index,
            r_ifs: arg.r_ifs.as_ref(),
            help_heading: arg.help_heading,
            global: arg.global,
            exclusive: arg.exclusive,
        }
    }
}

impl<'arg, 'help> ArgView<'arg, 'help> {
    /// The arg index, as passed to [`Arg::index`].
    pub fn index(&self) -> Option<u64> {
        self.index
    }

    /// The heading, as passed to [`Arg::help_heading`].
    pub fn help_heading(&self) -> Option<&'help str> {
        self.help_heading
    }

    /// The help string, as passed to [`Arg::help`].
    pub fn help(&self) -> Option<&'help str> {
        self.help
    }

    /// The name the arg was created with.
    pub fn name(&self) -> &'help str {
        self.name
    }
}
