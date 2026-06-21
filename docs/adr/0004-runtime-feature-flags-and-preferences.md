# ADR 0004 — Feature flags runtime e tela de preferencias

## Status

Proposto.

## Contexto

O prototipo greybox passou a ter opcoes de playtest que mudam tanto exibicao quanto regra de jogo:

- HUD e ajuda de controles;
- overlays de debug de combate;
- Player 1 e Player 2 controlados por IA ou manual;
- IA com ou sem golpes;
- Player 1 recebendo ou ignorando dano;
- entrada por gamepad quando o ambiente suportar.

Essas opcoes podem crescer rapido. Se cada tela ou sistema criar seu proprio booleano, o codigo fica dificil de entender e os contribuidores nao sabem onde registrar novos experimentos.

## Decisao

Criar um modulo central de feature flags em `src/game/feature_flags.rs`.

Regras:

- toda opcao runtime experimental deve ser registrada como `FeatureFlag`;
- leitura e escrita devem passar por `FeatureFlags::enabled`, `set` ou `toggle`;
- a tela de preferencias deve alterar flags, nao regras de jogo diretamente;
- sistemas de jogo devem consumir flags por API explicita;
- defaults devem favorecer playtest limpo: HUD ligado, ajuda e debug desligados.

Criar tambem uma cena simples de preferencias em `src/scenes/preferences.rs`, aberta no inicio do jogo e acessivel durante a luta.

## Alternativas consideradas

### Booleans soltos no `App`

Prós:

- mais rapido no primeiro commit.

Contras:

- cresce mal;
- mistura UI, regra e configuracao;
- dificulta documentar novas opcoes.

### Config externo em arquivo

Prós:

- persistencia entre execucoes.

Contras:

- adiciona IO e formato cedo demais;
- nao e necessario antes de estabilizar quais flags importam.

## Consequencias

### Positivas

- Contribuidores sabem onde criar uma nova flag.
- A tela de preferencias fica desacoplada das regras.
- Testes podem validar o contrato das flags sem abrir janela.
- Debug visual pode ficar desligado por padrao sem remover ferramentas.

### Negativas

- Mais um conceito para aprender no prototipo.
- Algumas flags temporarias precisarao ser removidas quando a direcao do jogo estabilizar.

## Criterio de revisao

Revisar se:

- as preferencias precisarem persistir em disco;
- houver perfis de treino/playtest;
- flags temporarias comecarem a substituir configuracao real de jogo;
- a tela de preferencias virar menu principal completo.
