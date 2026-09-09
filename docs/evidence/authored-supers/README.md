# Revisão dos supers autorais — 2026-09-09

[Vídeo dos quatro roteiros com áudio](../../../assets/showcase/authored-supers-2026-09-09.mp4).
O `World` real controla atores, captura, contatos, queda e eventos de áudio.
O vídeo registra essa execução; sua trilha é mixada a partir dos eventos reais
e dos arquivos/volumes do manifesto, preservando pausa e retomada da música.
A reprodução pelo `AudioPlayer` real foi verificada separadamente. A mixagem
da demonstração evita o atraso variável de captura PulseAudio no ambiente remoto.

## Imagens e regras

[Revisão das fases](phase-review.json): oito execuções completas, quatro
personagens nos dois sentidos, com 94 capturas. Quinze PNGs representativos
ficam no repositório; `retained_image` identifica quais foram preservados.
Os demais registros mantêm frame, fase, posição e HP, reproduzíveis pelo exemplo.

| Sequência | Evidência principal | Resultado sem defesa |
|---|---|---|
| Rust | [Construção](rust-right-hit-128-RustBuild.png), [mutação](rust-right-hit-225-RustCharge.png), [apagão](rust-right-hit-069-RustBlackout.png) | 28 HP, queda e restauração. |
| Duke | [Lixo](duke-right-hit-094-DukeTrashFall.png), [coleta](duke-right-hit-216-DukeCollect.png), [impacto espelhado](duke-left-hit-324-contact-264.png) | 12 coletas e queda gigante de 32 HP. |
| Old C | [Terminais](c-right-hit-116-CTerminalStorm.png), [tela azul](c-right-hit-202-CBlueScreen.png), [BIOS](c-right-hit-261-CBios.png), [reboot](c-right-hit-301-CReboot.png) | BIOS cobre todos os elementos; volta com alvo caído, 32 HP. |
| C++ | [Tiro no pé](cpp-right-hit-104-footshot.png), [pulinhos espelhados](cpp-left-hit-146-CppHop.png), [corrida](cpp-right-hit-232-CppCharge.png), [chute](cpp-right-hit-274-contact-214.png), [final](cpp-right-hit-358-CppFinisher.png) | Deslocamento até o alvo distante, oito contatos e final, 30 HP. |

[Revisão de guarda](guard-review.json): outras oito execuções nos dois sentidos,
com 7/8/8/9 HP de chip em Rust/Duke/C/C++, respectivamente. Testes determinísticos
também cobrem piso de 1 HP, invencibilidade de treino, alvos aéreos, pedidos
simultâneos, suspensão de projéteis, reset e KO anunciado após a restauração.

## Áudio e controles

[Streams de música](audio-stream-review.json): posição idêntica antes/durante a
pausa, retomada do mesmo cursor e cancelamento interrompendo efeitos, com as
chamadas de player usadas em luta, Lab e showcase. Cues específicos continuam
tocando durante a pausa. Esse teste usa dispositivo de áudio real e é ignorado
na suíte padrão para não exigir hardware em CI; foi executado separadamente.

[Laudo do vídeo](audio-video-review.json): H.264/AAC estéreo de 32,1 s, sem clipping. Correlação dos quatro `SuperStart` encontrou desvio máximo de 2,7 ms; as quatro janelas internas sem cues ficaram em silêncio digital, com música presente antes e depois.

O [registro da mixagem](offline-audio-mix.json) detalha os 61 eventos, bindings, rotação global de clips, interrupções do mesmo `Sound`, corte na entrada e cursor da música. O [script](../../../tools/audio/mix_authored_super_review.py) usa os volumes do manifesto e multiplicador de música 0,5, igual ao App padrão. Para reservar margem à soma de impactos simultâneos, aplica atenuação global constante de -2.29 dB: mantém as proporções e a dinâmica, sem compressor/limitador. O pico final decodificado é −1,973 dBFS. Esta trilha foi montada offline; a validação do dispositivo real acima é separada.

[Controles da aplicação](controls/README.md): 41 verificações com XTest e janela
Xephyr isolada, incluindo pausa, passo, repetição, troca de lados, reset,
defesa, saída, fonte Barlow e gravação com REC oculto durante BIOS.

[Reel de vozes e efeitos](../../../assets/audio/audition.html),
[fontes/licenças](../../../assets/audio/ATTRIBUTION.md) e
[produção com hashes](../../../assets/audio/production-2026-09-09.json).
Os arquivos/bindings anteriores de Duke/Python foram preservados. A análise
automatizada verifica funcionamento e sinal; julgamento subjetivo de timbre
depende de ouvir o reel.

## Reproduzir

Requer sessão gráfica; `--audio` reproduz pelo dispositivo real, em uma passagem
separada da gravação. O exemplo se recusa a sobrescrever um vídeo existente.

```sh
cargo run --example capture_authored_super_review -- --both-sides --output /tmp/super-frames
cargo run --example capture_authored_super_review -- --guard --both-sides --no-snapshots --output /tmp/super-guard
cargo run --example capture_authored_super_review -- --no-snapshots --output /tmp/super-movie --video /tmp/super-movie.mp4
cargo run --example capture_authored_super_review -- --audio --no-snapshots --output /tmp/super-audio
python3 tools/audio/mix_authored_super_review.py \
  --video /tmp/super-movie.mp4 \
  --events /tmp/super-movie/authored-super-review.json \
  --output /tmp/super-movie-with-audio.mp4 \
  --report /tmp/super-offline-audio-mix.json
```

A mixagem requer Python 3 com NumPy, além de `ffmpeg` e `ffprobe` no PATH. Ela copia o vídeo sem recodificar, adiciona AAC e recusa sobrescrever o arquivo de saída. Use o JSON produzido junto do vídeo silencioso; os cues incluem personagem, golpe, binding e frame do evento.

Gravação e exportação de PNGs são passagens separadas. O exemplo grava a 60 frames
por segundo do relógio de combate e lê RGBA em blocos. O JSON registra os frames
dos cues para conferir sincronização. Interpretação e limites do balanceamento estão no
[plano e catálogo da rodada](../../23-authored-super-sequences.md).
