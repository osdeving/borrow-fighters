# Chegada cinematográfica, vizinhança e ambiente da manhã

Quarta rodada do prólogo, na branch `feature/prologue-scene-improvements`.
Implementação: **`f90a0d1`** e **`6c513e1`**. A versão aprovada anterior continua
em **`bdd9ae6`**, como ponto de retorno e comparação.
[Escopo](../../32-cinematic-neighbourhood-arrival.md),
[diário](../../worklogs/cinematic-neighbourhood-arrival.md) e
[guia de peças substituíveis](../../../assets/adventure/street/README.md).

[Assistir à cena com som nativo](street-arrival-and-shelter.mp4): 30,73 segundos
de uma execução contínua, começando no quarto. Aos 14 s, a câmera encontra a
pipa, desce pela vizinhança e chega a Rust; depois vêm exploração, EP, fuga,
abrigo e fechamento da mercearia. Não há cortes, pausas, frames substituídos
ou reconstrução da trilha sonora. O vídeo preserva os 922 frames da captura.

| Momento | Captura |
|---|---|
| Céu e pipa antes da entrega de controle | [Início da tomada](screens/arrival-01-kite.png) |
| Letreiro legível, lojista, cliente e caramelo | [Descida da câmera](screens/arrival-02-descent.png) |
| Exploração liberada e ônibus proporcional aos carros | [Rua calma](screens/arrival-04-playable-neighbourhood.png) |
| Último visitante passando atrás do batente | [Entrada na mercearia](screens/neighbours-04-last-visitor-entering.png) |
| Lojista puxa a porta depois que todos entram | [Porta baixando](screens/neighbours-06-shutter-lowering.png) |
| Porta encostada no chão, bicicletas e carro amassado persistentes | [Loja fechada](screens/neighbours-07-closed-shop.png) |
| Ponto de ônibus vazio e ausência de retorno dos figurantes | [Câmera direita](screens/neighbours-08-right-camera.png) |

## Encenação e assets

A chegada dura seis segundos; o ambiente segue animado enquanto ações de
combate aguardam. O enquadramento final coincide com a câmera jogável.
`Enter`/`RB` termina a tomada e libera exploração. Pausa congela câmera e
vizinhança; retry acordado dispensa a chegada, e restart restaura a manhã.

O letreiro usa texto renderizado com Barlow Condensed SemiBold, mipmaps e
filtragem trilinear. Ônibus de aproximadamente 440×129 pixels conserva sua
proporção original, comparado aos automóveis de 147–172 pixels de largura.

Há dois moradores no ponto, uma cliente na mercearia e um lojista. O caramelo
fareja, caminha e senta no passeio. Após a EP, clientes correm até a entrada
real da loja e são ocultados progressivamente pelo batente. O lojista espera
o último visitante, puxa a porta de enrolar e fica coberto por ela. A porta
desce por recorte da imagem, sem esticar as lâminas, até fechar toda a abertura.
O cão foge pela calçada e não retorna durante a luta.

Moradores, lojista, cão e porta são novos PNGs transparentes, separados da
fachada. Recortes, escala e apoios ficam no catálogo; uma das identidades é
reutilizada em duas instâncias. Arte, prompts e procedência estão nos guias
de [moradores/porta](../../../assets/adventure/street/NEIGHBOURS.md) e
[caramelo](../../../assets/adventure/street/CARAMELO.md).

## Verificação funcional e empacotamento

[Matriz aprovada](verification.json): fmt e Clippy com `-D warnings`, 461
testes conjuntos, 67 de aventura isolada, 395 de luta isolada e três do core.
Dois testes preexistentes que exigem dispositivo de áudio ficaram ignorados.
Fronteiras: 56 testes; mixer: 47; pacote: 14. Código, assets e binário usado
na captura permaneceram iguais durante a matriz.

- [24 verificações nativas](native-checks.json): bloqueio de input antecipado,
  câmera contínua em 370 amostras, pausa/retomada, liberação de controles,
  trajetos dos moradores e cão em 1077 amostras, espera do lojista,
  fechamento, rua vazia, câmera direita, derrota/retry, skip e restart.
- [Eventos enviados à janela](native-events.jsonl) e [resultado](result.json).
  A execução funcional durou 65,24 s; gerou 2258 registros, 14 screenshots
  e vídeo de 62 s. Auditoria da telemetria confirmou ausência de moradores,
  cão e tráfego e porta fechada em 1311 amostras entre os ticks 361–2039.
- A prévia contínua é outra execução: [cinco checks](preview-native-checks.json),
  [eventos](preview-native-events.jsonl) e [resultado](preview-result.json).
  Não usa F12 nem pausa para produzir seus frames.
- [Staging Linux real](street-linux-stage-check.json): 248 assets verificados
  por hash, 59 frames de catálogo resolvidos e os novos arquivos incluídos.
  O campo de commit registra o HEAD anterior à integração, pois o staging
  verificou o workspace completo antes do commit `6c513e1`. Windows foi coberto
  pelos testes com fixtures, sem executável Windows local para staging real.

## Som capturado no jogo

[Relatório de áudio nativo](native-audio.json): sink PulseAudio exclusivo
para o jogo, sem mistura com áudio do desktop. Removido somente o preroll;
AAC estéreo a 48 kHz, sem normalização, reconstrução ou alteração temporal.
O stream comprimido de vídeo tem o mesmo SHA-256 antes e depois do mux.

O quarto contém ar e pássaros; a rua usa ar e trânsito leve em streams
separados. A reação faz o trânsito diminuir até parar aos seis segundos.
Na janela de 29,32–30,12 s, o áudio capturado corresponde ao ar, com
contribuição de trânsito praticamente zero. Efeitos de combate continuam.

Oito efeitos confirmados: aceleração no tick 0, latido no 8, buzina no 38,
bicicleta no 70, frenagem no 78, colisão no 112, porta rolando no 270 e
contato com o piso no 330. Latência nativa preservada: 12,8–17,5 ms. Latido
e bicicleta usaram separação de banda apenas na análise por sobreposição
de outros sons; o áudio exportado não recebeu esses filtros. Pico 0,238,
sem clipping no PCM capturado ou no AAC decodificado.

## Reproduzir

```sh
cargo run -- --start morning
cargo build --locked --bin borrow-adventure
python3.13 tools/review/capture_neighbourhood_x11.py \
  --output-directory /tmp/neighbourhood-review
```

O harness habilita áudio por padrão e aceita `--mute`. Seu gravador interno
produz vídeo silencioso; a prévia entregue usa gravação separada do sink.
Fontes completas, logs, telemetria e scripts de alinhamento permanecem em
`.git/neighbourhood-review/`. A validação visual e automatizada não substitui
playtest humano de controle físico, timbre e equilíbrio nos alto-falantes.
Para comparar a versão aprovada anterior, abra `bdd9ae6` em outra branch.
