use helix_term::{
    commands::MappableCommand,
    config::{Config, ConfigLoadError},
    keymap::KeyTrie,
};
use helix_view::{document::Mode, input::parse_macro};

use super::*;

fn vim_config() -> Config {
    let config = r#"
        [editor]
        keymap = "vim"
    "#
    .to_owned();

    Config::load(Ok(&config), Err(ConfigLoadError::default())).unwrap()
}

fn assert_normal_command(config: &Config, keys: &str, expected: MappableCommand) {
    let keys = parse_macro(keys).unwrap();
    let normal = config.keys.get(&Mode::Normal).unwrap();

    assert_eq!(
        normal.search(&keys),
        Some(&KeyTrie::MappableCommand(expected))
    );
}

fn assert_normal_sequence(config: &Config, keys: &str, expected: &[MappableCommand]) {
    let keys = parse_macro(keys).unwrap();
    let normal = config.keys.get(&Mode::Normal).unwrap();

    assert_eq!(
        normal.search(&keys),
        Some(&KeyTrie::Sequence(expected.into()))
    );
}

#[test]
fn vim_profile_maps_representative_vim_keys() {
    let config = vim_config();

    assert_normal_command(&config, "0", MappableCommand::goto_line_start);
    assert_normal_command(&config, "^", MappableCommand::goto_first_nonwhitespace);
    assert_normal_command(&config, "$", MappableCommand::goto_line_end);
    assert_normal_command(&config, "gg", MappableCommand::goto_file_start);
    assert_normal_command(&config, "G", MappableCommand::goto_line);
    assert_normal_command(&config, "<C-r>", MappableCommand::redo);
}

#[test]
fn vim_profile_maps_lazyvim_workflow_aliases() {
    let config = vim_config();

    assert_normal_command(&config, "<space><space>", MappableCommand::file_picker);
    assert_normal_command(&config, "<space>/", MappableCommand::global_search);
    assert_normal_command(&config, "H", MappableCommand::goto_previous_buffer);
    assert_normal_command(&config, "L", MappableCommand::goto_next_buffer);
    assert_normal_command(&config, "<space>bb", MappableCommand::buffer_picker);
    assert_normal_command(&config, "]d", MappableCommand::goto_next_diag);
    assert_normal_command(&config, "<space>xx", MappableCommand::diagnostics_picker);
    assert_normal_command(&config, "gd", MappableCommand::goto_definition);
    assert_normal_command(&config, "gr", MappableCommand::goto_reference);
    assert_normal_command(&config, "<space>a", MappableCommand::code_action);
    assert_normal_command(
        &config,
        "<space>ss",
        MappableCommand::lsp_or_syntax_symbol_picker,
    );
    assert_normal_command(&config, "]g", MappableCommand::goto_next_change);
    assert_normal_command(&config, "<space>gg", MappableCommand::changed_file_picker);
    assert_normal_command(&config, "<C-h>", MappableCommand::jump_view_left);
    assert_normal_command(&config, "<space>wd", MappableCommand::wclose);
}

#[test]
fn vim_profile_maps_linewise_operator_shortcuts() {
    let config = vim_config();

    assert_normal_sequence(
        &config,
        "dd",
        &[
            MappableCommand::extend_to_line_bounds,
            MappableCommand::delete_selection,
        ],
    );
    assert_normal_sequence(
        &config,
        "yy",
        &[
            MappableCommand::extend_to_line_bounds,
            MappableCommand::yank,
        ],
    );
    assert_normal_sequence(
        &config,
        "cc",
        &[
            MappableCommand::extend_to_line_bounds,
            MappableCommand::change_selection,
        ],
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn vim_profile_preserves_default_keymap_until_selected() -> anyhow::Result<()> {
    test((
        indoc! {"\
            #[t|]#wo
            "},
        "$",
        indoc! {"\
            #[t|]#wo
            "},
    ))
    .await?;

    test_with_config(
        AppBuilder::new().with_config(vim_config()),
        (
            indoc! {"\
                #[t|]#wo
                "},
            "$",
            indoc! {"\
                tw#[o|]#
                "},
        ),
    )
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn vim_profile_user_keys_merge_over_profile() -> anyhow::Result<()> {
    let config = r#"
        [editor]
        keymap = "vim"

        [keys.normal]
        "$" = "goto_line_start"
    "#
    .to_owned();
    let config = Config::load(Ok(&config), Err(ConfigLoadError::default())).unwrap();

    test_with_config(
        AppBuilder::new().with_config(config),
        (
            indoc! {"\
                tw#[o|]#
                "},
            "$",
            indoc! {"\
                #[t|]#wo
                "},
        ),
    )
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn vim_profile_redo_uses_ctrl_r() -> anyhow::Result<()> {
    test_with_config(
        AppBuilder::new().with_config(vim_config()),
        ("#[|]#", "iabc<esc>u<C-r>", "abc#[|\n]#"),
    )
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn vim_profile_dd_deletes_current_line() -> anyhow::Result<()> {
    test_with_config(
        AppBuilder::new().with_config(vim_config()),
        (
            indoc! {"\
                one
                #[t|]#wo
                three
                "},
            "dd",
            indoc! {"\
                one
                #[t|]#hree
                "},
        ),
    )
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn vim_profile_yy_p_pastes_current_line() -> anyhow::Result<()> {
    test_with_config(
        AppBuilder::new().with_config(vim_config()),
        (
            indoc! {"\
                one
                #[t|]#wo
                three
                "},
            "yyp",
            indoc! {"\
                one
                two
                #[two
                |]#three
                "},
        ),
    )
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn vim_profile_cc_changes_current_line() -> anyhow::Result<()> {
    test_with_config(
        AppBuilder::new().with_config(vim_config()),
        (
            indoc! {"\
                one
                #[t|]#wo
                three
                "},
            "ccreplacement<esc>",
            indoc! {"\
                one
                replacement#[
                |]#three
                "},
        ),
    )
    .await?;

    Ok(())
}
