# ADR 0023 — Apresentação seguida do menu, com domínios independentes

- Status: aceita para o experimento, por pedido do usuário.
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

## Verificação

Testar aventura, luta, ambas e core; resultados explícitos de conclusão/saída;
menu inerte na primeira opção e demais destinos; mouse alinhado com o desenho.
Capturar o final e a chegada ao menu na janela real, conferir o efeito binário
e o cursor, além de verificar que Go não é carregado pela apresentação.
