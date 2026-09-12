# Rua Augusta — revisão nativa

[Vídeo completo com áudio](adventure.mp4), capturado após o acabamento do
quadril e do piso. A execução usa entradas normais da política de revisão,
sem alterar HP ou forçar vitórias: três seguranças, duas erráticas, conversa
com Julia e saída. O resultado foi `Complete`, com 96 HP, em 100,334 segundos.

São 6.020 amostras registradas na telemetria a 60 Hz e 3.010 quadros de
vídeo a 30 fps. Todas as 14 fases aparecem. As falas permanecem na tela por
pelo menos dois segundos; a duração também considera o tamanho do texto.

## Inspeção visual

Os dez PNGs foram abertos e conferidos: [confronto](01-confrontation.png),
[combate](02-guards-combat.png), [fala após os seguranças](03-after-guards.png),
[descida](04-ep-descent.png), [impacto](05-ep-impact.png),
[resgate](06-rescue.png) e [conclusão](07-complete.png). A navegação aparece no
[seletor](08-protagonists.png), [Continuar C++](09-cpp-continue.png) e
[Continuar Rust](10-rust-continue.png).

- [Chegada das EPs](ep-arrival.mp4): queda simultânea pelos lados, contato e
  poeira preservam a continuidade; o chão mantém sua iluminação nas juntas.
- [Combate com as EPs](ep-combat.mp4): inclui o giro. Amostras entre 56,20 e
  56,60 segundos confirmam que o quadril acompanha as coxas, sem a ponta fixa
  de calça presente no draft. A faixa de mistura respeita o alpha da pintura.
- As recuperações de Linker (35,62–36,02 s) e chute (36,75–37,03 s) foram
  conferidas: o retorno mantém a orientação para o alvo. Giros autorados
  continuam completos; recuperações comuns atravessam zero pelo arco curto.

O piso usa `mirror_ground_tiles`; o rig usa `body_leg_blend: [-10, 12]` e
`leg_alpha_half_width: 0.35`. Os contratos e os recursos efetivamente carregados
têm hashes em [review.json](review.json). Drafts anteriores ficaram fora do
repositório.

## Progresso, áudio e reprodução

[menu-checks.json](menu-checks.json) registra a navegação X11 endereçada à
janela do próprio processo. O jogo criou os dois saves em userdata temporário,
sem escrita de perfil pelo harness, e preservou um ao visitar o outro.
Nenhum perfil real foi usado.

O áudio foi reconstruído dos WAVs locais e cues observados: 27 swishes,
21 impactos, dois projéteis e uma aterrissagem. Parry não foi exercitado nesta
execução. Não houve gravação/escuta do dispositivo de áudio nem teste com
controle físico. O relatório separa essas limitações da validação do arquivo.

```sh
cargo build --bin borrow-adventure --bin borrow-story
BORROW_FIGHTERS_DATA_DIR=/tmp/augusta-review-profile \
XDG_DATA_HOME=/tmp/augusta-review-xdg \
target/debug/borrow-adventure --start augusta \
  --review /tmp/augusta-review --frames 24000 --mute --hidden
python3.13 tools/review/mix_augusta_review_audio.py /tmp/augusta-review
python3.13 tools/review/check_independent_chapters_x11.py \
  --output-directory /tmp/augusta-menu-review
```

`ffprobe -count_frames` verificou vídeo/áudio dos três MP4, e a decodificação
integral por FFmpeg terminou sem erros. [load-report.json](load-report.json)
lista cinco atores, 27 texturas e 33.959.752 bytes RGBA base; mipmaps e custos
do driver não entram nessa soma.
