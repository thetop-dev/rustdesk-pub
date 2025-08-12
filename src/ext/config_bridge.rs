use serde_json::json;
use hbb_common::config::{keys, Config};
use crate::{common, ui_interface};

// 隐藏图标，链接弹窗
pub fn hide_model() -> bool {
    // 参考：tray::start_tray
    // 获取隐藏图标配置：https://github.com/rustdesk/rustdesk-server-pro/issues/332
    // let hide_tray = crate::ui_interface::get_builtin_option(hbb_common::config::keys::OPTION_HIDE_TRAY);
    // println!("{:?}", hide_tray);

    // 隐藏链接弹窗（cm）：https://github.com/rustdesk/rustdesk/discussions/6008#discussioncomment-7268951
    // 使用固定密码、只允许密码登录下才生效
    ui_interface::set_option("allow-hide-cm".parse().unwrap(), "Y".parse().unwrap());
    ui_interface::set_option("approve-mode".parse().unwrap(), "password".parse().unwrap());
    let hide_cm = hbb_common::password_security::hide_cm();
    hide_cm
}

pub fn get_id() -> String {
    Config::get_id()
}

// 获取系统信息
pub fn get_sys_info() -> String {
    let mut v = common::get_sysinfo();
    v["id"] = Config::get_id().parse().unwrap();
    v[keys::OPTION_CUSTOM_RENDEZVOUS_SERVER] = json!(Config::get_option(keys::OPTION_CUSTOM_RENDEZVOUS_SERVER));
    v["relay-server"] = json!(Config::get_option("relay-server"));
    v[keys::OPTION_API_SERVER] = json!(Config::get_option(keys::OPTION_API_SERVER));
    v[keys::OPTION_KEY] = json!(Config::get_option(keys::OPTION_KEY));
    // TODO 获取其他信息
    // 固定密码、临时密码、登录密码

    // 更多信息参考：sync::start_hbbs_sync_async
    let ab_name = Config::get_option(keys::OPTION_PRESET_ADDRESS_BOOK_NAME);
    if !ab_name.is_empty() {
        v[keys::OPTION_PRESET_ADDRESS_BOOK_NAME] = json!(ab_name);
    }
    let ab_tag = Config::get_option(keys::OPTION_PRESET_ADDRESS_BOOK_TAG);
    if !ab_tag.is_empty() {
        v[keys::OPTION_PRESET_ADDRESS_BOOK_TAG] = json!(ab_tag);
    }
    let username = ui_interface::get_builtin_option(keys::OPTION_PRESET_USERNAME);
    if !username.is_empty() {
        v[keys::OPTION_PRESET_USERNAME] = json!(username);
    }
    let strategy_name = ui_interface::get_builtin_option(keys::OPTION_PRESET_STRATEGY_NAME);
    if !strategy_name.is_empty() {
        v[keys::OPTION_PRESET_STRATEGY_NAME] = json!(strategy_name);
    }
    let device_group_name = ui_interface::get_builtin_option(keys::OPTION_PRESET_DEVICE_GROUP_NAME);
    if !device_group_name.is_empty() {
        v[keys::OPTION_PRESET_DEVICE_GROUP_NAME] = json!(device_group_name);
    }
    v.to_string()
}
