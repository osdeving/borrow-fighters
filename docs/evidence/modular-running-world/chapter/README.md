# Capítulo nos mapas ampliados

[Trecho nativo de 16 segundos](chapter-native-preview.mp4), com impactos na pilha,
abertura do caminho, duas EPs e a retomada após a vitória. Os cortes vêm de uma
única execução completa de 113,56 segundos, gravada em 1280×720/30 fps; o preview
foi reduzido para 960×540. A execução usa comandos públicos determinísticos do
modo `--review`, janela oculta e áudio desabilitado.

| Cena | Largura | Percurso de Rust observado |
| --- | ---: | ---: |
| Rua | 4608 px | x=2593–4458 |
| Travessa | 4096 px | x=100–3939 |
| Passagem | 3840 px | x=240–3695 |

A revisão terminou em `Complete`, no frame 6813, com `Victory`. A câmera ficou
dentro dos limites dos três mapas em todos os 6814 frames. Os nove destroços
receberam dano e chegaram a zero de vida entre os frames 3902 e 4401; os objetos
de cima assentaram conforme seus apoios foram destruídos. O encontro seguinte
começou com duas EPs de 120/112 HP e terminou com ambas derrotadas no frame 6398.

[Rua e aproximação](street.png) · [Corrida na travessa](lane-running.png) ·
[Pilha intacta](cargo-intact.png) · [Destruição parcial](cargo-breaking.png) ·
[Duas EPs](two-eps.png) · [Passagem liberada](passage-cleared.png) ·
[Conclusão](complete.png).

A inspeção das imagens confirmou escala dos personagens, aproximações às portas,
destruição progressiva e os dois corpos assentados após a vitória. Não foram
feitas alterações cosméticas a partir desta captura. A política terminou com
100 HP; isso comprova a progressão e os contatos, não substitui avaliação humana
de dificuldade ou teste de controle físico.

O [resumo verificável](summary.json) contém limites, fases e resultados; o
[recorte de telemetria](telemetry-events.jsonl) preserva todas as mudanças de
fase/vida e amostras periódicas, em 79 registros. O [manifesto](manifest.json)
registra SHA-256 do binário imutável usado e de `world.json`. A gravação completa
e a telemetria integral permaneceram em `/tmp/borrow-modular-chapter-final/capture`.

Reprodução a partir da raiz, usando um diretório de revisão novo:

```sh
cargo build --no-default-features --features adventure --bin borrow-adventure
target/debug/borrow-adventure --start chapter --review /tmp/borrow-chapter-review --frames 10000 --hidden --mute
```

O save do jogador não é usado: `--review` mantém seu checkpoint dentro do
diretório da captura. Esta evidência foi produzida em uma cópia própria do
binário para que builds paralelos não alterassem o processo em execução.
