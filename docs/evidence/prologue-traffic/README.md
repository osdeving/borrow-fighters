# Rua com trânsito e acidente no poste

Segunda rodada da [entrega 30](../../30-prologue-scene-improvements.md), na
branch `feature/prologue-scene-improvements`, a partir de `3f6d9da`.

[Prévia com som nativo](traffic-and-crash.mp4): tráfego cotidiano, aparição da
EP, buzina, frenagem, carro amassado contra o poste e consequência persistente.
A prévia é uma execução contínua, sem pausas nem substituição de imagens ou
sons. Os comandos de movimento e defesa são enviados à janela do processo.

| Momento | Captura |
|---|---|
| Automóveis, ciclistas e garoto antes da ameaça | [Rua calma](screens/traffic-01-calm.png) |
| Aproximação da EP | [Antes do susto](screens/traffic-04-before-EP.png) |
| Nariz do carro contra o poste | [Contato](screens/traffic-08-impact.png) |
| Impacto na tomada contínua, com fuga e ciclistas | [Quadro da prévia](screens/traffic-impact-preview.png) |
| Capô dobrado e carro parado | [Carro amassado](screens/traffic-09-wreck.png) |
| Garoto já fora; destroços permanecem | [Consequência](screens/traffic-10-persistent-aftermath.png) |
| Revisão da câmera à direita | [Extremo direito](screens/traffic-11-right-camera.png) |
| Reinício restaura o trânsito sem acidente | [Rua reiniciada](screens/traffic-14-restarted-calm.png) |

## Encenação e leitura

Calçada do garoto em y350, automóveis em y393/430, ciclistas em y467/480 e
Rust em y580. O bairro pintado é comprimido acima da nova rua; ciclovia,
canteiro e piso jogável mantêm seus apoios. As rodas animadas usam centros
medidos no atlas, e a arte amassada conserva a escala da íntegra.

O carro do incidente entra continuamente pela esquerda depois da EP. Buzina
no tick 38, pneus no 78 e impacto no 112 usam o relógio compartilhado de 60 Hz.
Marcas de frenagem permanecem no asfalto; poeira e oscilação do poste cedem
lugar ao capô danificado, fragmentos no chão e fumaça leve. Carros regulares
seguem na outra faixa, sem atravessar o veículo parado. Nenhum desses atores
entra na física do combate.

## Verificação

- [Resultados](verification.json): 445 testes conjuntos, 51 aventura isolada,
  395 luta isolada e três core. Fmt e Clippy estrito aprovados.
- 56 fixtures de fronteira, 38 testes do mixer de revisão e dez de
  empacotamento, incluindo PNG/JSON e os três WAVs novos.
- [20 checks nativos](native-checks.json): movimento antes da EP, reação,
  frenagem, pausa integral, retomada, contato, persistência, câmera,
  derrota/retry, skip sem vitória e restart da rua calma.
- [Eventos dos comandos](native-events.jsonl) e [resultado do jogo](result.json).
  As imagens acima selecionam sete das 14 capturas funcionais e um quadro
  extraído da prévia contínua no instante do impacto.
- A primeira tentativa não concluiu a câmera direita porque Rust morreu.
  A segunda usou defesa e avanço durante a recuperação da EP; somente os
  comandos da automação mudaram. Não foram alteradas saúde ou regras do jogo.

O áudio da prévia vem de um sink PulseAudio dedicado ao processo do jogo,
sem captura do som do desktop. O relógio de parede da telemetria permite
alinhar os samples de áudio ao vídeo. [Medição do áudio](native-audio.json).
A revisão por automação e imagens não substitui avaliação humana de ritmo e
controle físico.

## Reproduzir

```sh
cargo run -- --start encounter

cargo build --locked --bin borrow-adventure
python3.13 tools/review/capture_traffic_x11.py \
  --output-directory /tmp/prologue-traffic-review
```

O harness usa áudio nativo por padrão, com `--mute` opcional. Seu MP4 interno
é silencioso; a gravação do sink dedicado foi feita separadamente para esta
prévia. Logs, telemetria completa e fontes locais estão em
`.git/traffic-review/`, conforme o [diário](../../worklogs/prologue-scene-improvements.md).

Atlas, recortes, prompts e procedência do `image_gen` integrado estão em
[street-traffic.png](../../../assets/adventure/street-traffic.png),
[street-traffic.json](../../../assets/adventure/street-traffic.json) e
[registro da arte](../../../assets/adventure/ART-PROVENANCE.md).
Os sons são síntese original reproduzível pelo
[gerador da aventura](../../../assets/adventure/audio/generate_traffic_audio.py).
