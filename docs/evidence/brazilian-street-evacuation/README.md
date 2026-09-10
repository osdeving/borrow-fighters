# Rua brasileira e evacuação coletiva

Terceira rodada do prólogo, na branch `feature/prologue-scene-improvements`.
Implementação: **`65d826b`**. Ponto de retorno: **`c2dbbe4`**, commitado antes desta ampliação.
[Escopo](../../31-brazilian-street-evacuation.md) e
[guia para trocar/reaproveitar peças](../../../assets/adventure/street/README.md).

[Assistir à prévia com som nativo](street-evacuation.mp4): 13,73 segundos de
uma execução contínua, com rua calma, aparição da EP, fuga coletiva, acidente
e rua vazia. O vídeo vem do próprio jogo; comandos de movimento e defesa
foram enviados somente à janela do processo. Não há cortes, imagens
substituídas ou reconstrução de sons.

| Momento | Captura |
|---|---|
| Veículos diferentes, bicicleta e Bar e Mercearia Casa Nossa | [Manhã](screens/evacuation-01-calm.png) |
| Ponto de ônibus e aproximação da ameaça | [Antes da EP](screens/evacuation-03-before-EP.png) |
| Ciclistas desmontam; pausa conserva todos os atores | [Desmontagem pausada](screens/evacuation-04-dismount-paused.png) |
| Quadro da prévia contínua durante o caos | [Prévia](screens/evacuation-preview-chaos.png) |
| Pessoas correm, bicicletas ficam e carros aceleram | [Fuga](screens/evacuation-06-running-and-falling-bikes.png) |
| Carro buzina, freia e amassa contra o poste | [Impacto](screens/evacuation-07-car-impact.png) |
| Nenhum novo figurante circula durante a luta | [Rua vazia](screens/evacuation-08-empty-street.png) |
| Ponto completo, bicicletas e destroços persistentes | [Câmera direita](screens/evacuation-09-empty-right-camera.png) |
| Reinício restaura a manhã e remove os destroços | [Reinício](screens/evacuation-12-restarted-calm.png) |

## Peças e encenação

Garoto, ciclistas, bicicletas, veículos e adereços são imagens transparentes
sobrepostas à base pintada. O catálogo define arte, recortes, apoios e escala;
a cena mantém instâncias e posições. Dois cantos de mercearia usam a mesma
peça. Fachada e ponto recebem texto do catálogo português.

Há hatch, sedã, picape, SUV e ônibus circulando, além de uma van disponível
para reaproveitamento. A EP interrompe o movimento cotidiano: carros aceleram
sem voltar; ciclistas freiam, desmontam, deixam a bicicleta e fogem. O garoto
larga a pipa e corre. Em até seis segundos a evacuação termina, preservando
as bicicletas caídas e o veículo acidentado durante o combate.

Motores reagem no tick 0, buzina no 38, bicicletas tocam o chão no 70, pneus
freiam no 78 e carro colide no 112. Todos seguem o mesmo relógio de 60 Hz.
A pausa congela a cena; retry rearma a evacuação e restart restaura a calma.

## Verificação

[Relatório da matriz](verification.json): Fmt e Clippy estrito aprovados;
450 testes conjuntos, 56 de aventura isolada, 395 de luta isolada e três do
core. Dois testes preexistentes que requerem dispositivo de áudio ficaram
ignorados. Fronteiras: 56 testes; mixer: 40; empacotamento: 14.

## Evidência nativa

- [22 verificações aprovadas](native-checks.json): continuidade em 1219 amostras,
  fases dos dois ciclistas, pausa, rua vazia ao tick 361 e ainda vazia no 1397,
  câmera direita, derrota/retry, skip e restart.
- [Comandos enviados](native-events.jsonl) e [resultado](result.json).
- [Troca e reuso de assets](street-modularity-check.json): PNG substituto
  incluído, recorte/apoio preservados e duas instâncias mantidas sem alterar
  os bytes da composição.
- [Staging Linux real](street-linux-stage-check.json): 240 assets e verificação
  de hashes. O staging Windows foi coberto pelos testes com fixtures;
  não havia executável Windows local para testar o pacote real.
- [Áudio nativo](native-audio.json): sink PulseAudio dedicado ao jogo, sem
  áudio do desktop. Remoção do preroll e codificação AAC, preservando o stream
  de vídeo original e sem ajuste de ganho ou duração dos sons. Sem clipping.
  Os cinco cues foram confirmados por correlação, com latência de 10–16 ms.
  A queda da bicicleta exige análise de banda devido à sobreposição da buzina;
  esse filtro foi usado só para medir, sem modificar o som entregue.

A captura funcional tem 56,2 segundos e 12 imagens; a prévia contínua é uma
execução separada, sem F12 ou pausas que interrompam seu ritmo. Fontes locais,
telemetria completa e scripts de alinhamento estão em
`.git/street-chaos-review/`. A revisão automatizada e visual não representa
avaliação humana dos controles físicos ou da qualidade subjetiva do som.

## Reproduzir

```sh
cargo run -- --start encounter
cargo build --locked --bin borrow-adventure
python3.13 tools/review/capture_street_evacuation_x11.py \
  --output-directory /tmp/street-evacuation-review
```

O CLI anterior `capture_traffic_x11.py` encaminha para esta mesma revisão.
Áudio nativo é padrão; `--mute` desliga. O MP4 interno é silencioso, por isso
a prévia entregue inclui a gravação separada do sink dedicado. Para voltar
à segunda rodada, use o commit `c2dbbe4` numa branch de comparação.
