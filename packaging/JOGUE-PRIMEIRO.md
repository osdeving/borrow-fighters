# Bora testar Borrow Fighters?

Esta é uma demo em desenvolvimento: uma luta local com personagens inspirados
em linguagens de programação, cenários brasileiros e especiais exagerados.
Você não precisa instalar Rust, Cargo ou ferramentas de desenvolvimento.

## Abrir o jogo

- **Windows 10 (versão 1903+) ou Windows 11, 64 bits:** execute o instalador `-setup.exe`. Ele instala
  somente para seu usuário e cria atalhos. Se preferir o ZIP portátil, extraia
  **toda a pasta** e abra `borrow-fighters.exe` dentro dela.
- **Ubuntu 22.04+, Debian 12+ e derivados, 64 bits:** instale o `.deb` com
  `sudo apt install ./borrow-fighters_*.deb` e abra Borrow Fighters no menu.
- **Fedora, 64 bits:** instale o `.rpm` com
  `sudo dnf install ./borrow-fighters-*.rpm` e abra pelo menu.
- **Linux portátil:** extraia o `.tar.gz`, entre na pasta extraída e execute
  `./borrow-fighters`. Mantenha `bin/`, `lib/` e `assets/` juntos.

Linux precisa de desktop com X11 ou XWayland, glibc 2.35+ e driver com OpenGL
3.3. Bibliotecas de janela, ALSA e cliente PulseAudio acompanham o pacote;
o cliente também funciona com a compatibilidade PulseAudio do PipeWire. Drivers gráficos,
servidor de áudio e componentes do sistema continuam pertencendo ao sistema.
Windows também precisa de driver com OpenGL 3.3.

A versão `v0.1.0-prototype.2` corrige imagens e áudio ao instalar ou extrair
o jogo em pastas com acentos. Se a primeira versão mostrou lutadores em blocos
ou cenários sem imagens, baixe o pacote atualizado e extraia toda a pasta.

Os executáveis ainda não têm assinatura digital; o Windows pode mostrar um
aviso de editor desconhecido. Baixe somente da página oficial de releases.

## Sua primeira luta

1. No guia de boas-vindas, escolha **Jogar contra CPU** (você é P1),
   **Duelo local** (duas pessoas) ou **Assistir demo** (CPU contra CPU).
   Você pode reabrir o guia em **Como jogar**.
2. Confirme o personagem de **P1**, depois o de **P2**. Com os dois prontos,
   confirme **Lutar**. Pelo menu principal, `Quick Fight` e `Versus Setup`
   também abrem a seleção.
3. Na seleção, use `WASD`/setas, controle ou mouse. `Tab` muda o modo e
   `Q/E` muda a arena; você também pode clicar nessas opções. Contra CPU ou
   na demo, `Enter` confirma o lado ativo. No duelo local, P1 usa `WASD` + `F`
   e P2 usa setas + `Enter`; com mouse, clique nas prévias P1/P2 para trocar
   o lado ativo. `Esc`/`B` desfaz uma confirmação antes de voltar ao menu.
4. Espere o **Fight!**. Use `A/D` para andar, `W` para pular e `S` para abaixar.
   `F` dá soco, `H` dá soco forte, `V` dá chute e `Q` defende.
   Segure `S + Q` contra rasteiras; defenda em pé contra golpes por cima.
5. Experimente `G` (projétil) e `T` (especial). A barra de energia começa em
   **50/100** e enche ao acertar, receber golpes ou defender. Com **100**,
   `Y` ativa o cinematográfico e gasta a barra. Errar golpes ou esperar
   não gera energia.
6. `Esc` ou `Menu`/`Start` do controle abre **Pausa**: **Continuar**,
   **Reiniciar**, **Trocar personagens** ou **Menu**. `R` reinicia rapidamente
   fora desses menus. Ao terminar a luta, escolha **Revanche**,
   **Trocar personagens** ou **Menu** abaixo dos lutadores.

No controle Xbox: direcional/analógico move, `A` pula, `X/Y/B` atacam,
`LB/LT` defende, `RB` lança projétil, `RT` faz especial e `LB + RT` faz o
cinematográfico com energia cheia. Na seleção, `A` confirma; dois controles
podem escolher seus lados. Com ambos prontos, `Start` inicia a luta.

No duelo local, P2 luta com setas, `O` (soco), `P` (soco forte), `;` ou `/`
(chute), `U` (defesa), `Ctrl direito` (projétil), `\` (especial) e `]`
(cinematográfico). Alguns teclados limitam teclas simultâneas; dois controles
ajudam nesse caso. A IA precisa estar desligada para controlar o respectivo
jogador; **Duelo local** faz isso para os dois. `C`/`View` alterna a CPU de P2
fora da pausa.

`Training > Move Showcase` demonstra golpes e defesas. Use `Tab` para trocar
a situação, `Enter` para repetir e `Esc` para voltar. Os cinematográficos são
livres de custo no **Move Showcase** e no **Combat Lab**. Em `Options`, você
pode ligar a ajuda de controles durante a luta.

## O que testar e contar para a gente

Jogue algumas lutas com Rust, Duke/Java, C, Python e C++. Experimente os
especiais e nos conte: os comandos foram claros? Algum golpe pareceu injusto?
Você entendeu quando acertou ou bloqueou e quando a energia ficou pronta?
Selecionar personagens, pausar e pedir revanche foi claro? Teve travamento,
silêncio ou tela preta?

Abra uma issue no repositório indicado na página da release. Inclua a versão
(também em `BUILD-INFO.json`), sistema operacional, placa de vídeo, personagens
e os passos para repetir. Um print ou vídeo curto ajuda bastante.

É uma demo local: não há multiplayer online. Go continua como personagem
experimental fora da seleção pública. Arte, balanceamento e acabamento estão
em evolução; suas observações ajudam a escolher os próximos ajustes.

A gravação opcional depende de `ffmpeg` instalado separadamente; isso não é
necessário para jogar. Créditos de áudio, fontes e bibliotecas acompanham este
pacote em `THIRD_PARTY_NOTICES.md`, `licenses/` e `THIRD_PARTY_SOURCES/`.
