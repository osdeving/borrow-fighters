# ADR 0023 — Apresentação seguida do menu, com domínios independentes

- Status: aceita; entrada conjunta promovida a padrão por pedido do usuário.
- Data: 2026-09-10.

## Contexto

Após a abertura com Ada, Rust, encontro e apresentação, o usuário quer chegar
ao menu de luta na mesma janela. A independência da ADR 0021 continua necessária:
é possível desenvolver ou compilar um modo sem o outro.

## Decisão

Adicionar `borrow-story`, executável que exige `fighting` e `adventure`, e
`src/presentation.rs`, composição externa habilitada somente com ambas as features.
Ela possui a janela e chama APIs de aplicação, sem regras, sprites ou estado de
combate. A aplicação de luta prepara suas texturas antes do prólogo e devolve
um menu opaco à composição; a aventura não acessa esse objeto nem seus assets.
Isso antecipa o carregamento dos atlas para evitar segundos de tela escura
depois do título. A aventura devolve um resultado explícito ao terminar; seus recursos
locais são liberados antes de iniciar o áudio e o loop do menu de luta. Fechar a janela ou sair
da aventura encerra a sessão; não abre o menu por engano.

`borrow-adventure` e `borrow-fighters` continuam independentes. Nenhum domínio
importa a composição ou o outro domínio. O checker fiscaliza também essa borda;
o core permanece somente `math` e `runtime_paths`.

O menu principal adota as cores e tipografia do título final, moldura de terminal,
cursor de bloco e revelação binária. `Modo História` substitui `Quick Fight` e
permanece sem ação. As demais entradas mantêm seus destinos. A composição abre
diretamente o menu principal, inclusive no primeiro uso; `Como jogar` permanece
acessível, e o onboarding do executável de luta isolado é preservado.

Go sai somente da apresentação; o corte mantém 48 segundos e a trilha atual.
A atualização de imagens e conteúdos do livro `Lore / Roster` fica no backlog.

## Entrada padrão — atualização de 10/09/2026

Com avanço por trechos, skip total e confirmação final implementados, o usuário
adotou a execução conjunta como padrão: `default-run = "borrow-story"` e
`default = ["fighting", "adventure"]`. Assim, `cargo run` abre Ada → Rust →
apresentação → menu, e `cargo run -- --menu` abre diretamente o menu.

Esta atualização substitui somente a escolha de entrada/default da ADR 0021.
O isolamento de código e assets permanece: cada domínio ainda pode ser
compilado sozinho com `--no-default-features`, sua feature e seu `--bin`.
Os comandos específicos de luta, laboratório e viewer usam explicitamente
`cargo run --bin borrow-fighters -- ...`. A composição conserva sua CLI de
história/menu. A distribuição já publicada continua registrada em sua release.

## Verificação

Revisão de navegação: a conclusão natural agora aguarda confirmação antes de
devolver `Completed`. Um pedido explícito de pular tudo devolve `Skipped` e vai
ao menu imediatamente. Sair/fechar/limitar frames continuam distintos e não
iniciam o outro modo. `Enter`/`RB` avança trechos no domínio de aventura sem
fabricar resultado de combate. Teclas anteriores à tela final não a dispensam.
O renderer de revisão determinística pode confirmar após um hold de três
segundos; essa automação não se aplica ao jogador.

Testar aventura, luta, ambas e core; resultados explícitos de conclusão/saída;
menu inerte na primeira opção e demais destinos; mouse alinhado com o desenho.
Capturar o final e a chegada ao menu na janela real, conferir o efeito binário
e o cursor, além de verificar que Go não é carregado pela apresentação.
