# 22 — Apresentação, Brasil cotidiano e especiais cinematográficos

## Plano desta rodada

Pedido de 8 de setembro de 2026: elevar o acabamento do jogo, corrigir letras
serrilhadas, animar o vira-lata caramelo da referência, trazer memes brasileiros
discretos aos cenários e criar outro especial para cada personagem, com impacto
visual na tela inteira e colisão local. A expansão foi solicitada explicitamente;
o escopo histórico de dois placeholders já foi superado pelo slice atual.

1. Incorporar fontes redistribuíveis e suavizar sua redução, mantendo acentos,
   contraste, hierarquia e a mesma geometria para desenho e clique dos menus.
2. Melhorar HUD, seleção e orientação dos menus; revisar legibilidade em 720p e
   aplicar filtragem suave aos fundos pintados para sua escala de apresentação.
3. Remover o cachorro antigo do bitmap do Sirius e colocar um caramelo de pelo
   curto em quatro fases de corrida, atravessando as arenas de vez em quando.
4. Criar participação gestual inspirada em “Já acabou, Jéssica?” em São Paulo,
   com figurino reconhecível da referência, e outros detalhes pequenos por arena.
5. Acrescentar seis especiais cinematográficos, preservando os golpes anteriores.
   Antecipação, bloqueio, recuperação e área de contato permanecem verificáveis.
6. Verificar testes, documentação, atlas e cenas reais; registrar evidência.

## Direção de arte e combate

O Brasil aparece no cotidiano e no humor de internet: um caramelo de orelhas
caídas, bilhete informal, comentário de arquibancada, cálculo confuso e gestos
reconhecíveis. Os atores ficam atrás dos lutadores, em escala de cenário. As
participações móveis cedem espaço à apresentação cinematográfica. Não há áudio
do vídeo original nem novos sistemas de NPC, colisão ou missão.

| Personagem | Novo especial | Apresentação da tela |
|---|---|---|
| Rust | Ownership Eclipse | Eclipse de engrenagens, órbitas e conexões de ownership. |
| Duke / Java | JVM Overdrive | Torres da JVM, sobrecarga de threads e faixas de bytecode. |
| Go | Million Goroutines | Canais paralelos cruzam a arena com pacotes coordenados. |
| C | Kernel Panic | Memória quadriculada se fragmenta numa pane vermelha/ciano. |
| Python | Event Horizon | Órbitas azul/amarelo e dados convergem num horizonte de eventos. |
| C++ | Template Singularity | Templates e operadores se expandem em geometria recursiva. |

Os efeitos grandes são apresentação. O único contato ofensivo é a hitbox
próxima ao personagem, durante a janela ativa do `MoveSpec`. O especial pode
errar, ser bloqueado ou ser interrompido. Nenhuma área desenhada pelo renderer
causa dano. Os seis movimentos usam as poses existentes com timing adaptado;
a nova identidade vem da composição, geometria e animação de tela inteira.

Controles: `Y` para P1, `]` para P2 e `LB` segurado + `RT` no controle.
`T`, `\\` e `RT` sozinho mantêm os especiais de assinatura anteriores.

```sh
cargo run --bin borrow-fighters -- --showcase --character rust --move cinematic_special --repeat
cargo run --bin borrow-fighters -- --showcase --character python --move cinematic_special --repeat --reverse
cargo run --bin borrow-fighters -- --lab combat --character go --move cinematic_special
```

O showcase permite repetir, pausar, avançar quadro e espelhar. Os novos
especiais são acessíveis para Go por CLI/ferramentas, preservando o roster público.

## Critérios de aceite

- Fontes e acentos iguais entre máquinas, sem depender de fontes instaladas.
- Nenhum menu cortado; hover/clique acompanha os retângulos visíveis.
- Caramelo curto e brasileiro, pernas animadas e intervalo entre passagens.
- Figurino e gesto reconhecíveis no cameo, sem competir com a luta.
- Seis novas identidades cinematográficas; todos os contatos continuam locais.
- Pausa, interrupção, lados, guarda, KO e restart sem efeitos órfãos.
- `cargo fmt`, `cargo clippy`, `cargo test` e links Markdown aprovados.

Decisão estrutural: [ADR 0015](adr/0015-cinematic-presentation-and-stage-life.md).

## Resultado e verificação

Os seis itens do plano foram implementados. A suíte passou com **300 testes**,
incluindo matrizes dos seis novos golpes com os dois lados, guarda alta/baixa,
metadata/fallback, distância, interrupção, simultaneidade e reinício. Formatação,
Clippy estrito, links Markdown e YAML GitHub foram aprovados.

A [evidência da rodada](evidence/presentation-polish/README.md) reúne capturas
de menu/HUD, fases dos golpes, cenários e comandos para reproduzir a revisão.
O [vídeo do runtime](../assets/showcase/presentation-polish-2026-09-08.mp4) mostra
as seis cinematográficas e o caramelo em movimento; captura sem áudio.

As [fontes e licenças](../assets/fonts/README.md), os
[assets de cenário](../assets/production/stage-life/README.md) e os
[prompts completos](../assets/production/stage-life/prompts.md) acompanham o jogo.

## Próxima maturidade de produto

Esta rodada melhora o acabamento do slice. Para uma release pública profissional,
o próximo corte é playtest com outras pessoas, pausa dedicada, configuração de
resolução/fullscreen e remapeamento de controles, balanceamento por observação,
empacotamento por plataforma e checklist de distribuição. Online, story mode e
novos sistemas de combo continuam fora desta rodada.
