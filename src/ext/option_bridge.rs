use hbb_common::config::Config;
use crate::RendezvousMediator;

// 设置自定义id
pub fn change_id(id: &str) {
    // 参考：Config::update_id();
    // TODO 生成的id还要保证在会和服务器是不重复的，如何校验？
    Config::set_id(id);
    restart_rendezvous_mediator();
}

pub fn restart_rendezvous_mediator() {
    RendezvousMediator::restart();
}
