# ADR 0019 — Distribuição do playtest sem toolchain

## Status

Aceita para `v0.1.0-prototype.1`.

## Contexto

O protótipo já permite playtest, mas exigir Rust, compilador C e dependências
de desenvolvimento impede que outras pessoas simplesmente baixem e joguem.
O runtime também procura assets no checkout e começa com CPU contra CPU.

## Decisão

- Estabilizar o estado atual em `release/v0.1.0-prototype.1`, incluindo os
  commits locais existentes, e abrir PR de retorno para `main`.
- Gerar Windows x86_64 com MSVC, Raylib e runtime C estáticos: instalador
  Inno Setup por usuário, sem elevação, e ZIP portátil com a mesma árvore.
- Gerar Linux x86_64 no Ubuntu 22.04: tar.gz portátil, DEB para Debian/Ubuntu
  e RPM para Fedora. Os pacotes nativos usam a mesma árvore em
  `/opt/borrow-fighters` e incluem atalho no menu de aplicativos.
- Incluir assets alcançáveis pelos manifestos, créditos, licenças e fontes
  correspondentes das bibliotecas redistribuídas. Referências, capturas e
  material de produção sem uso em runtime ficam fora dos downloads.
- No Linux, incluir bibliotecas de X11/áudio carregadas dinamicamente e
  dependências transitivas redistribuíveis. A libc, o servidor de janelas e
  os drivers OpenGL permanecem responsabilidades do sistema.
- Resolver assets a partir da instalação, com fallback para o checkout no
  desenvolvimento. Gravações e marcador de boas-vindas usam dados do usuário.
- Exibir um guia breve na primeira abertura e permitir reabri-lo pelo menu.
  Oferecer jogar contra CPU, duelo local e assistir. A entrada normal inicia
  P1 manual contra CPU; os defaults do domínio e ferramentas CLI são preservados.
- Builds de `release/*` geram artefatos para verificação. Tags versionadas
  geram uma GitHub pré-release somente após checks e pacotes aprovados no CI.
  As notas são escritas para jogadores e acompanham cada versão.

## Consequências

Não é necessário instalar Rust, CMake, FFmpeg ou ferramentas de compilação
para jogar. FFmpeg continua opcional para a ferramenta de gravação.
O instalador Windows não tem assinatura comercial nesta rodada, então o
sistema pode informar que o editor é desconhecido. Os pacotes Linux ainda
exigem um desktop e driver com OpenGL compatível; não são binários universais
para qualquer distribuição, arquitetura ou versão antiga da libc.

Sem macOS, autoatualizador, servidor online, AppImage, remapeamento de input
ou mudanças de combate neste corte. As limitações e a verificação por
plataforma ficam nas [notas da versão](../releases/v0.1.0-prototype.1.md).

## Alternativas consideradas

Distribuir apenas código-fonte mantém a barreira de compilação. Distribuir
somente o executável quebra o carregamento dos assets. Copiar todo o checkout
produz downloads grandes com arquivos que o jogador não usa. AppImage e
Flatpak podem ser avaliados depois do primeiro feedback sobre instalação.

## Referências

- [Processo de release](../06-release-process.md).
- [Inno Setup: instalação por usuário](https://jrsoftware.org/ishelp/topic_setup_privilegesrequired.htm).
- [GitHub CLI: criar uma release](https://cli.github.com/manual/gh_release_create).
