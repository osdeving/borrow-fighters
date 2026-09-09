# Bora testar Borrow Fighters?

Esta é uma demo em desenvolvimento: uma luta local com personagens inspirados
em linguagens de programação, cenários brasileiros e especiais exagerados.
Você não precisa instalar Rust, Cargo ou ferramentas de desenvolvimento.

## Abrir o jogo

- **Windows 10/11, 64 bits:** execute o instalador `-setup.exe`. Ele instala
  somente para seu usuário e cria atalhos. Se preferir o ZIP portátil, extraia
  **toda a pasta** e abra `borrow-fighters.exe` dentro dela.
- **Ubuntu 22.04+, Debian 12+ e derivados, 64 bits:** instale o `.deb` com
  `sudo apt install ./borrow-fighters_*.deb` e abra Borrow Fighters no menu.
- **Fedora, 64 bits:** instale o `.rpm` com
  `sudo dnf install ./borrow-fighters-*.rpm` e abra pelo menu.
- **Linux portátil:** extraia o `.tar.gz`, entre na pasta extraída e execute
  `./borrow-fighters`. Mantenha `bin/`, `lib/` e `assets/` juntos.

Linux precisa de desktop com X11 ou XWayland, glibc 2.35+ e driver com OpenGL
3.3. Bibliotecas de janela e ALSA acompanham o pacote; drivers gráficos,
servidor de áudio e componentes do sistema continuam pertencendo ao sistema.
Windows também precisa de driver com OpenGL 3.3.

Os executáveis ainda não têm assinatura digital; o Windows pode mostrar um
aviso de editor desconhecido. Baixe somente da página oficial de releases.

## Sua primeira luta

1. Leia a ajuda de boas-vindas ao abrir. Você pode revisitá-la no menu.
2. Para jogar contra a CPU, desligue **Player 1 usa IA** em `Options` e
   deixe **Player 2 usa IA** ligado. Se ambos estiverem ligados, assista à demo.
3. Escolha `Quick Fight`. Em `Versus Setup`, troque lutadores e arena.
4. Use `A/D` para andar, `W` para pular e `S` para abaixar. `F` dá soco,
   `H` dá soco forte, `V` dá chute e `Q` defende.
5. Experimente `G` (projétil), `T` (especial) e `Y` (cinematográfico).
   `R` reinicia a luta; `Esc` volta ao menu.

No controle Xbox: direcional/analógico move, `A` pula, `X/Y/B` atacam,
`LB/LT` defende, `RB` lança projétil, `RT` faz especial e `LB + RT` faz o
cinematográfico. A IA do jogador precisa estar desligada para assumir o controle.

Para duas pessoas no teclado, desligue também a IA do Player 2 (`C` alterna
durante a luta). P2 usa setas, `O` (soco), `P` (soco forte), `;` (chute),
`U` (defesa), `Ctrl direito` (projétil), `\` (especial) e `]` (cinematográfico).
Alguns teclados limitam teclas simultâneas; dois controles ajudam nesse caso.

`Training > Move Showcase` demonstra golpes e defesas. Use `Tab` para trocar
a situação, `Enter` para repetir e `Esc` para voltar. Em `Options`, você pode
ligar a ajuda de controles durante a luta.

## O que testar e contar para a gente

Jogue algumas lutas com Rust, Duke/Java, C, Python e C++. Experimente os
especiais e nos conte: os comandos foram claros? Algum golpe pareceu injusto?
Você entendeu quando acertou ou bloqueou? Teve travamento, silêncio ou tela preta?

Abra uma issue no repositório indicado na página da release. Inclua a versão
(também em `BUILD-INFO.json`), sistema operacional, placa de vídeo, personagens
e os passos para repetir. Um print ou vídeo curto ajuda bastante.

É uma demo local: não há multiplayer online. Go continua como personagem
experimental fora da seleção pública. Arte, balanceamento e acabamento estão
em evolução; suas observações ajudam a escolher os próximos ajustes.

A gravação opcional depende de `ffmpeg` instalado separadamente; isso não é
necessário para jogar. Créditos de áudio, fontes e bibliotecas acompanham este
pacote em `THIRD_PARTY_NOTICES.md`, `licenses/` e `THIRD_PARTY_SOURCES/`.
