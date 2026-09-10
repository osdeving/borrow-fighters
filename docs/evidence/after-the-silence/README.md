# Depois do silêncio — primeiro capítulo

Implementação **`2a33f13`**, branch `feature/prologue-scene-improvements`.
A versão anterior permanece em **`81c50e6`**.
[Escopo](../../33-after-the-silence.md),
[arquitetura](../../adr/0027-chapter-spatial-direction.md) e
[diário](../../worklogs/after-the-silence.md).

[Assistir ao capítulo com som nativo](chapter-01.mp4): **93,67 segundos, 2.810 frames**. A execução mantém Rust,
os moradores e os gestos no cenário: motorista, mercearia, apresentação de
Python, telefone, travessa e liberação da passagem. A conversa ampliada mostra
as três mensagens solicitadas; Rust guarda o aparelho antes de recuperar o
controle. Não há páginas de graphic novel no capítulo.


| Momento | Captura da execução final |
|---|---|
| Consequências na rua | [Abertura](screens/01-street-aftermath.png) |
| Rust verifica o motorista | [Motorista](screens/02-driver.png) |
| Lojista na porta aberta | [Mercearia](screens/03-shop.png) |
| Três mensagens e aparelho independente | [Conversa com Python](screens/08-phone-message-3.png) |
| Caminho físico e obstáculo | [Salto](screens/11-lane-jump.png) |
| Rust e moradora visíveis no mesmo plano | [Travessa](screens/12-lane-resident.png) |
| Continuação pelo save | [Submenu](screens/06-continue-submenu.png) |
| Seleção livre preservada | [Versus](screens/09-versus-selection.png) |

As imagens do capítulo são frames extraídos da prévia, sem recompor a cena;
as do menu são capturas da janela própria. O percurso da prévia não usa F12,
pausa ou retry e não gera PNGs durante a gravação.

## Estrutura editável

A encenação usa fases de 60 Hz, trajetos interpolados, clips de poses e pontos
de encaixe para acessórios. O aparelho é independente do corpo; a conversa
ampliada lê o mesmo relógio. Fonte, cores, texto, frames e recortes podem ser
alterados separadamente. [Guia de edição](../../../assets/adventure/chapter/README.md).

`world.json` define os três espaços, limites, apoios, obstáculo, posições de
pessoas, regiões de interação e rotas. Os triggers e o debug F3 leem esses dados.
A moradora sai completamente da mercearia antes de a porta descer; na travessa,
Rust se posiciona a 100 px da pessoa antes de falar, mantendo ambos visíveis.

O host recebe uma solicitação do menu e abre a campanha na mesma janela. O
save guarda checkpoints e a procedência da vitória do prólogo. Uma entrada
pulada ou direta não inventa vitória jogada. O personagem da campanha é Rust;
a seleção livre continua em Versus.

## Verificação

[Matriz de testes](verification.json): 494 passaram com todas as features;
100 de aventura isolada, 394 de luta e três do core. Dois testes preexistentes
da luta exigem dispositivo de áudio e permaneceram ignorados. O ajuste do
primeiro frame do replay passou novamente por oito testes focais, Clippy
completo e rebuild. Formatação, 56 testes de fronteiras e 19 de pacote passaram.

[Percurso funcional](functional-checks.json) e [eventos](functional-events.jsonl):
pausa durante o telefone, bloqueio de input acumulado, ordem das mensagens,
rotas, obstáculo, salto, derrota real, retry na passagem, vitória e checkpoint
final. São eventos de teclado endereçados à janela de um processo próprio;
o harness não injeta posição, saúde ou save. A captura funcional precede os
ajustes finais de enquadramento da travessa e navegação do submenu; a prévia
e a revisão final do host verificam a versão final.


[Revisão final do host](host-checks.json): **21 checks** confirmam primeira
abertura, pulo sem vitória fabricada, abertura posterior no menu, cliques em
História/Iniciar/Continuar, dois checkpoints, replay desde o primeiro trecho
de Ada, Versus e retorno. [Inspeção das dez capturas](host-visual-review.json).
A [prévia final](preview-checks.json) percorre novamente o capítulo e verifica
a nova aproximação da travessa com os dois personagens separados.

[Inventário do pacote](asset-inventory.json): 263 assets existentes foram
resolvidos e verificados por hash, incluindo 15 do capítulo (cerca de 6,1 MB).
O coletor percorre todos os frames do novo catálogo. Os testes cobrem staging
Linux/Windows e dependências faltantes; não foi criado outro instalador ou
uma cópia integral de staging, preservando espaço.


## Som nativo

[Gravação](native-recording.json) e [medição](native-audio.json): sink exclusivo,
PCM do próprio jogo, remoção apenas do preroll de 4,334 s e AAC estéreo a 48 kHz.
O stream de vídeo foi copiado sem recodificar e tem o mesmo hash antes/depois.
Os três sinais do telefone foram encontrados nos ticks 210/510/720, com
correlação 0,91–0,97 e latência medida de 3,7–9,9 ms, preservada no vídeo.
Não houve normalização, substituição da trilha, cortes ou mudança de velocidade.
PCM e AAC não apresentam clipping; pico normalizado abaixo de 0,10.
O MKA temporário foi removido depois da verificação e o sink próprio descarregado.

## Windows

[Limpeza registrada](windows-cleanup.json): sete diretórios temporários dos
nossos testes de Windows e um instalador obsoleto liberaram **517,65 MiB** no
C:, de 4,653 para 5,158 GiB naquele momento. Arquivos pessoais, saves, perfil
do Codex e discos virtuais ativos foram preservados. O resultado é do Windows,
sem contar a remoção de arquivos dentro do WSL como espaço recuperado no C:.

## Reproduzir

```sh
cargo run -- --menu
cargo run -- --start chapter
cargo build --locked --bin borrow-adventure --bin borrow-story
python3.13 tools/review/capture_chapter_x11.py \
  --functional --output-directory /tmp/chapter-functional
python3.13 tools/review/check_chapter_host_x11.py \
  --output-directory /tmp/chapter-host
python3.13 tools/review/record_chapter_preview.py \
  --output-directory /tmp/chapter-preview \
  --video-output /tmp/chapter-preview.mp4
```

Os harnesses usam diretórios novos e saves isolados. A revisão automática
`--review` também possui save próprio. As verificações de janela foram feitas
em Linux/X11; não incluem controle físico nem aprovação visual humana. Os
logs completos e a telemetria estão em `.git/story-chapter-review/`.
