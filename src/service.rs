pub mod service {
    use std::collections::HashMap;
    use regex::Regex;
    use crate::my_reader;
    use crate::config;
    use chrono::{NaiveDateTime, Utc};

    pub struct HttpService {
        regex: Regex,
        responce: String,
        buffer: String,
    }

    impl HttpService {
        pub fn new() -> Self {
            Self {
               regex: Regex::new(r#"\[(.+)]\s*"T.+"\s*\d{3,3}\s*".*"\s*"(.+)"\s*"(.*)"\s*$"#).expect("Static regex must be compiling"),
                responce: String::new(),
                buffer: String::new(),
            }
        }

        /**
        Some - answer
        Error - 500 String
         */
        pub fn run(&mut self, http_request: Vec<String>) -> Result<&str, &str> {
            if let Some(request_line) = http_request.first() {
                return if request_line.contains("GET /metrics HTTP/1") || request_line.contains("GET /metrics/ HTTP/1") {
                    let reader = my_reader::my_reader::BufReader::open(config::PATH_LOG_FILE);

                    let now = Utc::now().naive_utc();

                    if let Ok(mut str_reader) = reader {
                        self.responce.clear();

                        let mut start1 = 0;
                        let mut error_wasm_2 = 0;
                        //   let mut  start_proxy_check_3 = 0;
                        let mut start_direct_4 = 0;
                        let mut success_loaded_any_script_5 = 0;
                        let mut wasm_success_6 = 0;
                        let mut errors_wasm: HashMap<String, u32> = HashMap::new();

                        while let Some(a_line) = str_reader.read_line(&mut self.buffer) {
                            if let Ok(line) = a_line {
                                for (_, [date, text, _]) in self.regex.captures_iter(line.trim()).map(|c| c.extract()) {
                                    if let Ok(no_timezone) = NaiveDateTime::parse_from_str(date, "%d/%b/%Y:%H:%M:%S %z") {
                                        let diff = now.signed_duration_since(no_timezone);
                                        if diff.num_seconds() < 24 * 3600 {
                                            match text.trim().chars().next() {
                                                Some(ch) => {
                                                    match ch {
                                                        x if x == '1' => { start1 += 1; }
                                                        x if x == '2' => {
                                                            error_wasm_2 += 1;

                                                            let key: String = text.trim().chars().skip(2).take(100).collect();
                                                            let cnt = errors_wasm.get(key.as_str()).unwrap_or(&0);
                                                            let cnt = *cnt + 1;
                                                            errors_wasm.insert(key, cnt);
                                                        }
                                                        x if x == '4' => { start_direct_4 += 1; }
                                                        x if x == '5' => { success_loaded_any_script_5 += 1; }
                                                        x if x == '6' => { wasm_success_6 += 1; }
                                                        _ => (),
                                                    }
                                                }
                                                None => (),
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        let failure = start1 - success_loaded_any_script_5 - wasm_success_6;
                        self.responce = format!("# HELP widget_start_count Total number of loading.\n# TYPE widget_start_count gauge\nwidget_start_count {start1}\n\
                        # HELP widget_wasm_success_count Success number of wasm loading.\n# TYPE widget_wasm_success_count gauge\nwidget_wasm_success_count {wasm_success_6}\n\
                        # HELP widget_wasm_error_count Error number of wasm loading.\n# TYPE widget_wasm_error_count gauge\nwidget_wasm_error_count {error_wasm_2}\n\
                        # HELP widget_proxy_error_count Error number of proxy loading.\n# TYPE widget_proxy_error_count gauge\nwidget_proxy_error_count {start_direct_4}\n\
                        # HELP widget_failure_count Error number of unshow.\n# TYPE widget_failure_count gauge\nwidget_failure_count {failure}");

                        if errors_wasm.len() > 0 {
                            self.responce.push_str("\n# HELP widget_wasm_reason_count Reasons of wasm errors.\n# TYPE widget_wasm_reason_count gauge");
                        }

                        for (key, value) in errors_wasm {
                            self.responce.push_str(format!("\nwidget_wasm_reason_count{{reason=\"{key}\"}} {value}").as_str());
                        }

                        self.responce.push_str("\n");
                        Ok(self.responce.as_str())
                    } else {
                        Err("202 ")
                    }
                } else {
                    Err("404 Not found")
                }
            } else {
                return Err("400 Not HTTP request");
            }
        }
    }
}