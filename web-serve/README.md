# Web Api

Um exemplo simples de web serve para uso em api rest

Rodar o server
``` rust
cargo run
```

``` bash
curl 127.0.0.1:8080 \
-H "Content-Type: application/json" \
-XGET -d '{"echo":"SUA MENSAGEM A SER RETORNADA"}'
```

Ira retornar um json com a seguinte estrutura
``` json
{
  "echo": "SUA MENSAGEM A SER RETORNADA"
}
```

