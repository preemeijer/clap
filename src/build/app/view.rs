//! A shim layer that allows querying certain info about `App`,
//! despite the corresponding fields being private.

use super::*;
use crate::INTERNAL_ERROR_MSG;

#[allow(missing_debug_implementations)]
pub struct AppView<'app, 'help> {
    id: Id,
    name: &'app String,
    bin_name: Option<&'app String>,
    author: Option<&'help str>,
    version: Option<&'help str>,
    long_version: Option<&'help str>,
    about: Option<&'help str>,
    long_about: Option<&'help str>,
    more_help: Option<&'help str>,
    pre_help: Option<&'help str>,
    aliases: Option<&'app Vec<(&'help str, bool)>>, // (name, visible)
    usage_str: Option<&'help str>,
    usage: Option<&'app String>,
    help_str: Option<&'help str>,
    disp_ord: usize,
    term_w: Option<usize>,
    max_w: Option<usize>,
    template: Option<&'help str>,
    settings: AppFlags,
    g_settings: AppFlags,
    args: &'app MKeyMap<'help>,
    subcommands: &'app Vec<App<'help>>,
    groups: &'app Vec<ArgGroup<'help>>,
    help_headings: &'app Vec<Option<&'help str>>,
}

impl<'app, 'help> AppView<'app, 'help> {
    pub(crate) fn from_ref(app: &'app App<'help>) -> AppView<'app, 'help> {
        AppView {
            id: app.id,
            name: &app.name,
            bin_name: app.bin_name.as_ref(),
            author: app.author,
            version: app.version,
            long_version: app.long_version,
            about: app.about,
            long_about: app.long_about,
            more_help: app.more_help,
            pre_help: app.pre_help,
            aliases: app.aliases.as_ref(),
            usage_str: app.usage_str,
            usage: app.usage.as_ref(),
            help_str: app.help_str,
            disp_ord: app.disp_ord,
            term_w: app.term_w,
            max_w: app.max_w,
            template: app.template,
            settings: app.settings,
            g_settings: app.g_settings,
            args: &app.args,
            subcommands: &app.subcommands,
            groups: &app.groups,
            help_headings: &app.help_headings,
        }
    }
}

impl<'app, 'help: 'app> AppView<'app, 'help> {
    /// The list of args the given arg conflicts with.
    ///
    /// # Panics
    ///
    /// If `arg` conflicts with some args `self` don't know about,
    /// it will panic.
    pub fn conflicts_with(&self, arg: &'app Arg<'_>) -> Vec<&'app Arg<'help>> {
        arg.blacklist.as_ref().map_or(vec![], |conflict_ids| {
            conflict_ids
                .iter()
                .map(|id| {
                    self.args
                        .args
                        .iter()
                        .find(|arg| arg.id == *id)
                        .expect(INTERNAL_ERROR_MSG)
                })
                .collect()
        })
    }

    pub fn flags(&self) -> impl Iterator<Item = &Arg<'help>> {
        self.args
            .args
            .iter()
            .filter(|a| !a.is_set(ArgSettings::TakesValue) && !a.view().index().is_some())
            .filter(|a| !a.view().help_heading().is_some())
    }

    pub fn args(&'app self) -> impl Iterator<Item = &'app Arg<'help>> {
        self.args.args.iter()
    }

    // pub fn args_mut(&'app mut self) -> impl Iterator<Item = &'app mut Arg<'help>> {
    //     self.args.args.iter_mut()
    // }
}
