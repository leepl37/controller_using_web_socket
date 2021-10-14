Web_socket_version
-프론트 단 javascript 로 웹 소켓 통신(이사님과 통신), 실시간으로 데이터를 주고 받아 차량 정보를
나타내며 데이터값에 따라 차단기를 열고 닫기를 구현한다.



*사용 툴/언어 
Intelli J/백단:Rust/프론트:Javascript

*실행방법
terminal 창에서 cargo watch -x run (main에 bind 부분에서 포트번호 설정가능)

*빌드(WSL/Docker 사용)
cargo build --release (wsl ubuntu Terminal 에서 빌드해야 도커 위에서 실행가능) 

*Dockerfile
폴더 디렉토리의 dockerfile에 작성된 내용대로 빌드한다. 
docker build -t 아이디/빌드파일:01 . ex) docker build -t leepl37/cloud_server_for_japan:01


*세부 라이브러리
[dependencies]
url = "2.2.2"
actix-web = "3.3.2"
tera = "1.5.0"
actix-files = "0.5.0"
serde = "1.0"
config = "0.10.1"
actix-web-actors = "3.0.0"
actix = "0.10.0"
dotenv = "0.15.0"
log = "0.4.0"
env_logger = "0.8.3"
sha256 = "1.0.2"
actix-http = "2.2.0"
actix-cors = "0.5.4"