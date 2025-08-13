use hbb_common::config::Config;
use crate::{ui_interface, RendezvousMediator};

// 设置自定义id
pub fn change_id(id: &str) -> String {
    // 参考：Config::update_id();
    // TODO 生成的id还要保证在会和服务器是不重复的，如何校验？
    if !hbb_common::is_valid_custom_id(&id) {
        return "Invalid format".to_string();
    }
    Config::set_id(id);
    restart_rendezvous_mediator();
    "OK".to_string()
    // ui_interface::change_id_shared(id.parse().unwrap_or("".to_string()), Config::get_id())
}

pub fn restart_rendezvous_mediator() {
    RendezvousMediator::restart();
}
