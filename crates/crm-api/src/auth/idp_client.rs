pub struct UserInfo {
    pub sub: String,
    pub email: String,
}

pub fn exchange_code(code: &str, _state: &str) -> Result<UserInfo, String> {
    let client_id = "d5c002f01d1dff402d01439fe3b37918";
    let client_secret = "e3b9943ad933d477f7517e81e725a08ed4be88881803b482cd8c01666e22608d01b0b9dc489a3825";
    let redirect_uri = "https://akurai-crm.olibuijr.com/auth/callback";

    let basic = base64_encode(&format!("{}:{}", client_id, client_secret));
    let token_body = format!(
        "grant_type=authorization_code&code={}&redirect_uri={}",
        urlencode(code),
        urlencode(redirect_uri),
    );

    let response = http_post(
        "127.0.0.1", 3500, "/token",
        &[("Authorization", &format!("Basic {}", basic)),
          ("Content-Type", "application/x-www-form-urlencoded")],
        &token_body,
    )?;

    let token_json = akurai_json::parse(&response.body)
        .map_err(|e| format!("token parse: {e}"))?;

    let access_token = token_json
        .get("access_token")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "no access_token in token response".to_string())?
        .to_string();

    let userinfo_response = http_get(
        "127.0.0.1", 3500, "/userinfo",
        &[("Authorization", &format!("Bearer {}", access_token))],
    )?;

    let info = akurai_json::parse(&userinfo_response.body)
        .map_err(|e| format!("userinfo parse: {e}"))?;

    let sub = info
        .get("sub")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "no sub in userinfo".to_string())?
        .to_string();

    let email = info
        .get("email")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    Ok(UserInfo { sub, email })
}

#[allow(dead_code)]
struct HttpResponse {
    status: u16,
    body: String,
}

fn http_get(host: &str, port: u16, path: &str, headers: &[(&str, &str)]) -> Result<HttpResponse, String> {
    let mut req = format!("GET {} HTTP/1.1\r\nHost: {}:{}\r\n", path, host, port);
    for (k, v) in headers {
        req.push_str(&format!("{}: {}\r\n", k, v));
    }
    req.push_str("Connection: close\r\n\r\n");
    send_http(host, port, &req)
}

fn http_post(host: &str, port: u16, path: &str, headers: &[(&str, &str)], body: &str) -> Result<HttpResponse, String> {
    let mut req = format!(
        "POST {} HTTP/1.1\r\nHost: {}:{}\r\nContent-Length: {}\r\n",
        path, host, port, body.len()
    );
    for (k, v) in headers {
        req.push_str(&format!("{}: {}\r\n", k, v));
    }
    req.push_str("Connection: close\r\n\r\n");
    req.push_str(body);
    send_http(host, port, &req)
}

fn send_http(host: &str, port: u16, request: &str) -> Result<HttpResponse, String> {
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::time::Duration;

    let addr = format!("{}:{}", host, port);
    let stream = TcpStream::connect_timeout(
        &addr.parse().map_err(|e| format!("addr parse: {e}"))?,
        Duration::from_secs(5),
    )
    .map_err(|e| format!("connect: {e}"))?;

    stream
        .set_read_timeout(Some(Duration::from_secs(10)))
        .map_err(|e| format!("set timeout: {e}"))?;

    let mut wr = &stream;
    wr.write_all(request.as_bytes())
        .map_err(|e| format!("write: {e}"))?;

    let mut buf = Vec::new();
    let mut rd = &stream;
    rd.read_to_end(&mut buf)
        .map_err(|e| format!("read: {e}"))?;

    let response = String::from_utf8_lossy(&buf).to_string();

    let status = response
        .lines()
        .next()
        .and_then(|line| line.split(' ').nth(1))
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(0);

    let body = if let Some(pos) = response.find("\r\n\r\n") {
        response[pos + 4..].to_string()
    } else {
        String::new()
    };

    if !(200..300).contains(&status) {
        return Err(format!("HTTP {}: {}", status, body.chars().take(200).collect::<String>()));
    }

    Ok(HttpResponse { status, body })
}

fn base64_encode(input: &str) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = input.as_bytes();
    let mut result = String::new();

    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let triple = (b0 << 16) | (b1 << 8) | b2;

        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);

        if chunk.len() > 1 {
            result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(CHARS[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }

    result
}

fn urlencode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for &b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(b as char);
            }
            b' ' => result.push('+'),
            _ => result.push_str(&format!("%{:02X}", b)),
        }
    }
    result
}
