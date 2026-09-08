# ADR 0012 — Geometria compartilhada dos menus

## Status

Aceito.

## Contexto

Os menus tinham desenho e navegação por teclado/gamepad, mas não respondiam a cliques. Adicionar áreas clicáveis separadas do desenho duplicaria coordenadas e permitiria selecionar uma linha diferente daquela sob o ponteiro.

## Decisão

Compartilhar os retângulos dos menus em [`src/ui/menu_layout.rs`](../../src/ui/menu_layout.rs), sem dependência de Raylib. O renderer usa esses retângulos para desenhar as linhas, e o adaptador de input usa os mesmos limites para identificar a linha sob o mouse.

[`PreferencesMenu`](../../src/scenes/preferences.rs) continua responsável por navegação e ações. Ele recebe a linha identificada e os eventos de movimento/clique, sem consultar Raylib. Mouse parado não substitui seleção feita por teclado/gamepad; clique esquerdo ativa, clique direito volta valores ajustáveis, e cliques fora das linhas não ativam a seleção anterior.

O cursor nativo permanece visível e livre. O overlay `Linker` usa a posição real apenas quando a janela está em foco e o mouse está dentro dela. `show_cursor()` preserva essa posição; `enable_cursor()` centraliza o ponteiro no Raylib 6 e não deve ser chamado por quadro.

## Consequências

- Desenho e cliques acompanham futuras alterações de posição das linhas.
- Navegação e limites clicáveis podem ser testados sem abrir uma janela.
- Os layouts seguem fixos e específicos das páginas atuais; não introduzimos um framework de UI.
- Cursor, foco e fechamento ainda precisam de verificação no runtime gráfico.

O fluxo de uso e verificação está no [guia técnico](../12-technical-combat-guide.md#mouse-e-fechamento-da-janela) e no [playtest](../10-greybox-playtest.md).
