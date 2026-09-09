# ADR 0020 — Seleção visual, fluxo de partida e energia

## Status

Aceita para a rodada autorizada em [26](../26-playtest-visual-completion.md).

## Contexto

A seleção por linhas não apresenta bem o elenco. A pausa implícita no menu
geral e o reinício por Start confundem a sequência das partidas. Cinematográficos
estão livres para demonstração, mas agora precisam de barra de energia no versus.
A produção de reações também deve alcançar todos os personagens já existentes.

## Decisão

- Criar estado puro e pequeno de seleção com dois cursores, confirmações,
  random resolvido uma vez por confirmação, vagas futuras e arena. Reutilizar
  retratos e atlas atuais nos previews; nenhum framework de telas ou UI.
- Manter uma cena explícita de seleção e estado de pausa/resultado próprio
  da partida. `App` controla transições; domínio não conhece botões Raylib.
- Compartilhar apenas helpers de apresentação realmente usados: painéis,
  progressão de transições e efeitos de foco/confirmação. Relógio visual de UI
  não pode avançar o relógio de combate quando a partida estiver pausada.
- `World` possui a energia dos dois lutadores, com política `Metered` ou
  `Unlimited`. Construtores de baixo nível continuam livres para preservar
  laboratório, showcases e capturas determinísticas; `App` habilita `Metered`
  explicitamente para partidas normais, inclusive `--fight`.
- Energia começa em 50/100, cinematográfico custa 100 e contatos normais
  concedem carga limitada. Não há ganho por erro nem pelos contatos do próprio
  cinematográfico. Cobrar uma única vez, somente depois de aceitar o comando.
- Reiniciar limpa carga e inputs pendentes. Pausar preserva carga, cursores
  de animação e áudio; transições não acumulam simulação atrasada.
- Reações por contato permanecem apresentação: ampliar os atlas e habilitar
  os personagens no contrato existente, mantendo `combat_manifest` e regras
  físicas. A [ADR 0018](0018-contact-reaction-profiles.md) continua válida.
- Busca e eventual fallback da voz Old C ficam no manifesto/créditos de áudio,
  sem alterar a voz de Rust ou trocar o conjunto dos demais personagens.

## Consequências e limites

Há módulos novos por responsabilidade real, em vez de ampliar indefinidamente
os menus de preferências. Os mesmos estados e retângulos precisam dirigir
input e desenho. A seleção é verificável sem GPU; aparência e fluidez ainda
exigem capturas do renderer real. Os parâmetros de energia são provisórios;
esta rodada não altera dano/alcance ou assume balanceamento competitivo.

## Alternativas

Manter o menu por linhas não atende ao pedido de apresentação do elenco.
Uma barra só visual não limitaria o uso dos especiais. Colocar custos diretamente
nos inputs do teclado deixaria CPU, gamepad e ferramentas inconsistentes.
Um framework genérico de telas/efeitos aumentaria o trabalho sem necessidade.
