# Bora testar Borrow Fighters?

Esta é uma demo em desenvolvimento com prólogo, primeiro capítulo jogável
de Rust e luta local. Os personagens são inspirados em linguagens de
programação, com cenários brasileiros e especiais exagerados.
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

A versão `v0.1.0-prototype.5` abre com Ada, o despertar de Assembly, a manhã de
Rust e seu primeiro encontro. Depois, o **Modo História** continua com o
Capítulo 01 — **Depois do silêncio**: moradores, contato com Python pelo
celular, percurso pelo bairro e combate para proteger a passagem da vila.
O Versus mantém seleção livre, pausa/revanche e energia. A correção anterior de
imagens e áudio em pastas com acentos continua incluída. Ao atualizar pelo ZIP,
extraia toda a pasta do novo pacote.

Os executáveis ainda não têm assinatura digital; o Windows pode mostrar um
aviso de editor desconhecido. Baixe somente da página oficial de releases.

## Da aventura ao menu

Na primeira abertura, o jogo começa pelo prólogo. `Enter`/`RB` avança ao próximo trecho e
`Backspace`/`View` pula tudo diretamente para o menu, inclusive no combate ou
na pausa. Ao chegar normalmente ao título final, **Aperte qualquer tecla para
continuar** aguarda uma nova tecla, clique ou botão do controle. Depois de
assistir ou pular o prólogo, as próximas aberturas chegam ao menu. Pular o
primeiro encontro não registra uma vitória jogada.

No encontro, use `A/D` ou setas para andar, `Espaço/W` para pular, `J/F` para
atacar, `K/H` para ataque forte e `Q/L` para defender. No controle: direcional,
`A` para pular, `X/Y` para ataques e `LB` para defesa. `Esc`/`Start` pausa;
`R` tenta novamente após derrota. Nas cenas, `Tab` revela a mensagem e `A`
do controle avança. Para sair, feche a janela ou use `B` do controle na pausa.

Para começar sempre pelo menu ao executar pelo terminal, use
`borrow-fighters.exe --menu` no Windows ou `./borrow-fighters --menu` no Linux
portátil. **Modo História → Rever prólogo** permite assistir à abertura de novo.

## Seu primeiro capítulo

1. Abra **Modo História → Iniciar capítulo**. **Depois do silêncio** acompanha
   Rust depois do primeiro encontro com a EP. A seleção livre de personagens
   continua disponível no Versus.
2. Espere a câmera apresentar a rua e siga o objetivo no alto da tela.
   Aproxime-se do motorista e dos moradores e use `E`/`A` para interagir.
   `Enter`/`RB` avança as falas. Rust se aproxima das pessoas, conversa e
   pega o celular para falar com Python; a tela ampliada aparece ao lado dele.
3. Depois da conversa, siga pela travessa, salte o obstáculo e ajude quem
   espera na passagem. Ataque com `J/F`, golpe forte com `K/H` e defenda com
   `Q/L`. No controle, use `X/Y` e `LB`.
4. O jogo salva nos checkpoints. Você pode sair e usar **Continuar capítulo**
   depois; **Recomeçar capítulo** inicia novamente. `Esc`/`Start` abre a pausa,
   com **Retomar checkpoint** e **Voltar ao menu**. Após uma derrota,
   `R`/`A` tenta o encontro novamente sem repetir todas as conversas.

| Ação na História | Teclado | Controle Xbox |
|---|---|---|
| Mover | `A/D` ou setas | Direcional ou analógico |
| Interagir | `E` | `A` |
| Pular | `Espaço`, `W` ou seta para cima | `B` |
| Ataque / golpe forte | `J/F` / `K/H` | `X` / `Y` |
| Defender | `Q/L` | `LB` |
| Avançar fala ou trecho | `Enter` | `RB` |
| Concluir a encenação atual | `Backspace` | `View` / `Back` |
| Pausar | `Esc` | `Start` |
| Tentar novamente após derrota | `R` | `A` |

No capítulo, `Backspace` conclui a encenação atual; para sair ao menu, use
a pausa. No prólogo, esse comando pula toda a abertura. O botão de pulo
também muda: **B no capítulo**, **A no encontro do prólogo e no Versus**.

## Sua primeira luta

1. No menu, abra **Como jogar** e escolha **Jogar contra CPU** (você é P1),
   **Duelo local** (duas pessoas) ou **Assistir demo** (CPU contra CPU).
   **Versus Setup** também abre a seleção com a configuração atual.
2. Confirme o personagem de **P1**, depois o de **P2**. Com os dois prontos,
   confirme **Lutar**.
3. Na seleção, use `WASD`/setas, controle ou mouse. `Tab` muda o modo e
   `Q/E` muda a arena; você também pode clicar nessas opções. No controle de P1,
   `Select`/`Back` muda o modo e `LB/RB` muda a arena. Contra CPU ou
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

Jogue o capítulo de Rust e teste sair e voltar por **Continuar capítulo**.
Os objetivos e a conversa no celular ficaram claros? Deu para interagir,
saltar e liberar a passagem? A rua, os moradores e o som reagiram à EP?

Depois, jogue algumas lutas com Rust, Duke/Java, C, Python e C++. Experimente os
especiais e nos conte: os comandos foram claros? Algum golpe pareceu injusto?
Você entendeu quando acertou ou bloqueou e quando a energia ficou pronta?
Avançar a aventura e chegar ao menu foi claro? Selecionar personagens, pausar e
pedir revanche funcionou? Teve travamento, silêncio ou tela preta?

Abra uma issue no repositório indicado na página da release. Inclua a versão
(também em `BUILD-INFO.json`), sistema operacional, placa de vídeo, personagens
e os passos para repetir. Um print ou vídeo curto ajuda bastante.

O Modo História contém o prólogo e este primeiro capítulo; a campanha
completa e capítulos de outros protagonistas ainda não estão disponíveis.
É uma demo local: não há multiplayer online. Go continua como personagem
experimental fora da seleção pública. Arte, balanceamento e acabamento estão
em evolução; suas observações ajudam a escolher os próximos ajustes.

A gravação opcional depende de `ffmpeg` instalado separadamente; isso não é
necessário para jogar. Créditos de áudio, fontes e bibliotecas acompanham este
pacote em `THIRD_PARTY_NOTICES.md`, `licenses/` e `THIRD_PARTY_SOURCES/`.
