# Simple TOTP echo server

### 🔵 Description

A Simple TOTP echo server <br>

### 🔵 Endpoints 
<br>

`/v1/generate` - `POST`

- body
```json
{
    "url": "otpauth://totp/some_data"
}
```
- response
```json
{
    "code": "414020",
    "ttl": "27"
}
```
<br><br>

`/v1/generate/:secret` - `GET`

- path
```textt
/v1/generate/superDuperSecret
```

- response
```json
{
    "code": "414020",
    "ttl": "27"
}
```

### 🔵 CLI

`--port <PORT>` -  Port [default: 8000] <br>
`--host <HOST>` -  Host [default: 127.0.0.1] <br>
`-h, --help` - Print help <br>
`-V, --version` - Print version <br>
