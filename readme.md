# HTTP CLI

HTTP CLI — это инструмент командной строки для отправки HTTP-запросов (GET, POST, PUT, DELETE) с возможностью добавления заголовков и отправки файлов. Утилита написана на Rust с использованием библиотек `clap` для обработки аргументов командной строки и `reqwest` для выполнения HTTP-запросов.

## Установка

Для использования HTTP CLI необходимо установить Rust и Cargo. После этого выполните следующие команды:

```sh
git clone <репозиторий>
cd <репозиторий>
cargo build --release


http_client get http://example.com
http_client get example.com -H "User-Agent=MyClient"


http_client post http://example.com -d '{"key": "value"}' -H "Content-Type=application/json"
http_client post example.com -f file.txt


http_client put http://example.com -d '{"update": true}'
http_client put example.com -f update.txt

http_client delete http://example.com

Параметры
-H, --headers: Добавление заголовков в формате ключ=значение.

-d, --data: Отправка данных в теле запроса (обычно для POST и PUT).

-f, --file: Отправка файла в теле запроса (обычно для POST и PUT).

http_client get http://example.com -H "Authorization=Bearer token"

http_client post http://example.com -d '{"name": "John", "age": 30}' -H "Content-Type=application/json"

http_client post http://example.com -f data.txt

http_client put http://example.com -d '{"status": "active"}' -H "Content-Type=application/json"

http_client delete http://example.com