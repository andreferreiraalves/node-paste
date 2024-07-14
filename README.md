# Roadmap:

<br>

- [x] <s>Método para retornar a informação salva;</s>
- [x] <s>Método para atualizar a informação salva, ele já retorna o código caso não exista na base;</s>
- [ ] Página com layout igual ao Paste.SH
- [x] DotEnv para configurações da api/interface
- [ ] Deploy em ambiente local com containers
- [ ] Registro de logs em um serviço específico
- [ ] Client desktop, será necessário detalhá-lo


cargo add actix-web
cargo add actix-cors
cargo add serde --features derive
cargo add serde_json
cargo add chrono --features serde
cargo add env_logger
cargo add dotenv
cargo add uuid --features "serde v4"
cargo add sqlx --features "runtime-async-std-native-tls postgres chrono uuid"


cargo install sqlx-cli
sqlx migrate add -r init
sqlx migrate run
sqlx migrate revert
