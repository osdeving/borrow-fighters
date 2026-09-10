# Revisão — apresentação seguida do menu de terminal

Entrega na branch `feature/rust-adventure-prologue`, em 10/09/2026.
[Escopo](../../29-story-terminal-menu.md) e
[decisão de composição](../../adr/0023-story-to-terminal-menu.md).

## Desenho

- [Menu principal](terminal-menu.png): marca creme/dourada, subtítulo, moldura e primeira opção reservada.
- [Revelação binária](binary-reveal.png): números formando a opção selecionada.
- [Cursor de bloco](block-cursor.png): indicador verde-água à esquerda, com piscada temporal.
- [Título com cinco personagens](opening-five-characters.png): Go ausente da montagem final.

Capturas do renderer Raylib, em 1280×720, revisadas visualmente. Não são mockups.
A captura do título mantém os assets locais da aventura; os cinco retratos
preservam seus hashes, e o renderer da abertura não carrega `go.png`.

## Verificação automatizada

Matriz: 421 testes com ambos os modos, 395 somente luta, 27 aventura/core e 3
somente core. Fmt, Clippy com warnings como erro e 54 fixtures do checker de
fronteiras aprovados. Modo História inerte foi verificado com comandos de menu
de teclado, mouse e gamepad; isso não equivale a testar um controle físico.

## Janela nativa

Os [14 checks nativos](native/native-checks.json) passaram. O
[vídeo da passagem e navegação](native/title-to-menu-native.mp4) preserva o
mesmo PID e janela entre a apresentação e o menu. Complete → primeiro cabeçalho
do menu visível levou aproximadamente 1,23 s neste host, incluindo fade final
e entrada, em amostragem de cerca de 100 ms. O carregamento antecipado retirou
o intervalo escuro de 5–7 s observado na primeira tentativa.

Enter e Espaço na primeira linha conservaram o menu; Versus abriu a seleção.
Training, Lore, Options e Como jogar foram visitados, com retorno ao principal.
Sair durante a aventura e atingir o limite de frames encerraram o processo
sem entrar no menu. Uma abertura oculta voltou visível ao menu após skip.
O marcador de onboarding não foi criado na chegada; visitar o guia gravou-o
somente no diretório de teste. Catálogo e binário mantiveram seus hashes.
Os 13 screenshots foram conferidos na [revisão visual separada](native/visual-review.json),
preservando o relatório automático original e seus campos de revisão pendente.

```sh
cargo build --all-features --bin borrow-story
python3 tools/review/capture_story_menu_x11.py \
  --executable target/debug/borrow-story \
  --output-directory /tmp/borrow-story-menu-review-new
```

O diretório de saída deve ser novo. A revisão usa X11, FFmpeg, `stdbuf` e
`xwininfo`; a ferramenta existente fornece o adaptador X11 com ownership por PID.

A ferramenta
[capture_story_menu_x11.py](../../../tools/review/capture_story_menu_x11.py) usa
somente a janela pertencente ao PID do processo iniciado, com dados de usuário
em diretório temporário. Não grava a área de trabalho nem altera o catálogo.

O vídeo nativo é amostrado em 10 fps e não inclui áudio. Logs comprovam abertura
e encerramento dos dispositivos; não há alegação de avaliação auditiva humana.
Os recursos de luta são preparados antes da aventura no executável conjunto;
os dois executáveis independentes continuam separados por features e imports.

As tentativas sem sincronização e a tentativa interrompida são intermediários
em `/tmp`, não estão incluídas como evidência aprovada. A rodada arquivada usa
o binário de `0574347`, sem novas alterações de código depois da revisão.
