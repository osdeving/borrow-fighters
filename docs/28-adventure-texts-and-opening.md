# 28 — Textos editáveis, manhã e apresentação do universo

Continuação autorizada na branch `feature/rust-adventure-prologue`, em 10 de
setembro de 2026. O usuário pediu editar o texto exibido sem recompilar, corrigir
o encaixe de Rust na cama e criar uma apresentação empolgante com manchetes,
personagens, logo em movimento e subtítulo.

**Estado:** implementado e verificado. [Vídeo da apresentação](evidence/adventure-opening/opening.mp4),
[manhã corrigida](evidence/adventure-opening/morning.mp4) e
[evidências completas](evidence/adventure-opening/README.md). Passaram 417 testes
com ambos os modos e 26 checks pela janela, incluindo edição/reload no mesmo
binário, JSON inválido preservando texto e pausa/skip/replay da apresentação.

## Sequência e conteúdo

A apresentação entra **depois do primeiro combate e do pesar de Rust**, conforme
resposta do usuário. Ada → manhã → encontro → pesar continuam nessa ordem.
Depois, jornais apresentam a tensão social, C++ e Python recebem cenas de
contexto, Rust/Duke/Old C/Go são apresentados e o logo fecha a montagem.
O experimento oferece entrada direta e replay da apresentação, sem exigir
repetir o combate para revisá-la.

C++ é uma mulher adulta que trabalhou como profissional do sexo antes de
despertar para o Linker e tornar-se heroína. A história respeita sua pessoa
antes e depois desse despertar. Python é professora universitária e ensina
humanos sobre EPs. As manchetes são jornais dentro da ficção: medo e conflito
convivem com educação e relatos de solidariedade, preservando a diversidade
moral das EPs. Não ampliar a campanha nem alterar regras de combate.

## Texto e apresentação

Os textos da aventura, incluindo legendas, terminal, manchetes, nomes, ajuda,
menus e subtítulo, passam para `assets/adventure/texts/pt-BR.json`. A aplicação
lê o arquivo ao abrir; F5 recarrega sem reiniciar nem recompilar. JSON inválido
no reload conserva o último catálogo válido e informa a falha. A luta mantém
seu livro em `assets/lore/story.json`; não importa o catálogo da aventura.
[ADR 0022](adr/0022-adventure-external-copy-and-opening.md).

Rust usa apoios por pose: corpo sobre a área útil do colchão ao dormir, quadril
apoiado ao sentar e pés no chão ao levantar. Conferir no renderer todas as poses
e suas transições. A apresentação combina ilustrações de origem, montagem dos
seis personagens, tipografia animada, recortes de jornal e trilha própria.
Os textos são desenhados pelo jogo, nunca gravados nas ilustrações.

## Verificação

- Editar JSON com o mesmo binário e conferir a mudança na janela; F5 válido e
  inválido, preservação de acentos e conteúdo anterior em caso de erro.
- Capturar as poses deitado, levantando o tronco, sentado, espreguiçando e em pé;
  revisar o suporte no colchão e no chão com a mesma escala.
- Confirmar vitória → pesar → apresentação → conclusão, pular/pausar/repetir
  apresentação e ausência de dano ou controle indevido durante cinematográficas.
- Registrar vídeo e quadros das manchetes, C++, Python, elenco e logo/subtítulo.
- Fmt, Clippy, testes e checker de fronteiras; docs/links atualizados. Manter
  diário e commits por etapa para retomada após interrupções do WSL.
