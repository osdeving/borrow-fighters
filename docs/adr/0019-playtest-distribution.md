# ADR 0019 — Distribuição do playtest sem toolchain

## Status

Aceita para `v0.1.0-prototype.1`; complementada em `v0.1.0-prototype.2`
para caminhos Unicode no Windows.

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
plataforma ficam nas [notas atuais](../releases/v0.1.0-prototype.2.md);
as [notas iniciais](../releases/v0.1.0-prototype.1.md) permanecem como histórico.

## Complemento — Caminhos Unicode no Windows

O playtest de `v0.1.0-prototype.1` revelou que uma pasta como `Jogo ação çãõ`
era lida corretamente por Rust, mas as APIs C de imagem e áudio interpretavam
os caminhos UTF-8 na página de código local do Windows. O fallback preservava
a luta com blocos e linhas do cenário, escondendo a falha de carregamento.

O executável passa a incorporar [`packaging/windows/app.manifest`](../../packaging/windows/app.manifest),
com `activeCodePage=UTF-8`, pelo [`build.rs`](../../build.rs).
A configuração vale para o processo do jogo, sem pedir alteração da
localidade do sistema. Windows 10 versão 1903+ ou Windows 11 passam a ser a
base mínima: [a documentação da Microsoft](https://learn.microsoft.com/en-us/windows/apps/design/globalizing/use-utf8-code-page)
define o suporte à página de código UTF-8 por manifesto a partir dessa versão.
O requisito gráfico continua OpenGL 3.3; Linux mantém glibc 2.35+, X11/XWayland
e driver OpenGL 3.3.

A regressão em [`tests/runtime_paths.rs`](../../tests/runtime_paths.rs) atravessa
a API nativa `Image::load_image` do Raylib em caminhos com espaços e acentos,
sem precisar de uma janela ou GPU. Testar só existência de arquivo ou JSON por
`std::fs` não cobre esta falha. No teste nativo do pacote em `Jogo ação çãõ`,
a aplicação do manifesto restaurou 58 texturas, 86 sons e oito músicas.
Essa evidência complementa os testes de instalação e a inspeção visual da luta.
Cada novo pacote deve preservar o manifesto no executável e a regressão nativa.

## Alternativas consideradas

Distribuir apenas código-fonte mantém a barreira de compilação. Distribuir
somente o executável quebra o carregamento dos assets. Copiar todo o checkout
produz downloads grandes com arquivos que o jogador não usa. AppImage e
Flatpak podem ser avaliados depois do primeiro feedback sobre instalação.

## Referências

- [Processo de release](../06-release-process.md).
- [Inno Setup: instalação por usuário](https://jrsoftware.org/ishelp/topic_setup_privilegesrequired.htm).
- [GitHub CLI: criar uma release](https://cli.github.com/manual/gh_release_create).
