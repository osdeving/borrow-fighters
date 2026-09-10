# 06 — Processo de Release

## Estado atual

O corte em preparação é `v0.1.0-prototype.4`: aventura de Ada/Rust,
primeiro encontro, apresentação, skip e menu de terminal na mesma execução.
A solicitação desta release inclui commit de todo o trabalho e integração à
`main`. Build, publicação e resultado serão registrados no
[diário](worklogs/rust-adventure-prologue.md).
A [ADR 0019](adr/0019-playtest-distribution.md) registra plataformas e distribuição;
a [ADR 0023](adr/0023-story-to-terminal-menu.md) registra a entrada conjunta.
As [notas da versão](releases/v0.1.0-prototype.4.md) explicam a entrega ao jogador.
O conteúdo da [prototype.3](releases/v0.1.0-prototype.3.md) e a correção anterior
de imagens e áudio em caminhos Windows com acentos continuam incluídos.

## Downloads

| Plataforma | Pacotes | Base de compatibilidade |
|---|---|---|
| Windows x86_64 | Instalador Inno Setup e ZIP portátil | Windows 10 1903+ ou Windows 11, driver OpenGL 3.3 |
| Debian/Ubuntu x86_64 | DEB | Debian 12+, Ubuntu 22.04+ e derivados |
| Fedora x86_64 | RPM | Fedora 44 como ambiente de verificação |
| Linux x86_64 | tar.gz portátil | glibc 2.35+, X11/XWayland e driver OpenGL 3.3 |

Windows usa Raylib e CRT estáticos, com instalação por usuário e desinstalador.
O executável incorpora um manifesto com `activeCodePage=UTF-8`, para as APIs
nativas abrirem imagens e áudio em caminhos como `Jogos/ação çãõ`.
Linux inclui bibliotecas redistribuíveis de X11/áudio; libc e drivers gráficos
permanecem no sistema. Os pacotes incluem os assets usados pelo runtime e suas
referências transitivas, incluindo cenas, sprites, áudio e textos da aventura,
sem vídeos de revisão e materiais de produção. O jogo distribuído compõe
`borrow-story`, com aventura e luta habilitadas, e conserva o nome público
`borrow-fighters` (`borrow-fighters.exe` no Windows).
FFmpeg é opcional e não acompanha a release. Sprite Studio é ferramenta separada.

Créditos, textos de licença e fontes correspondentes acompanham os pacotes.
Os textos `LICENSE-MIT`/`LICENSE-APACHE` formalizam a escolha já declarada no
`Cargo.toml`; assets e bibliotecas mantêm seus próprios créditos e termos.

## Versão e branch

Manter a versão idêntica em `Cargo.toml`, `Cargo.lock` e tag, por exemplo:

```text
0.1.0-prototype.4
v0.1.0-prototype.4
release/v0.1.0-prototype.4
```

A tag e o pacote avançam para cada correção publicada; a branch de estabilização
pode continuar com o nome do corte inicial até o merge do PR.

Conventional Commits continuam sendo usados. A branch de release recebe apenas
empacotamento, instruções, correções e estabilização do corte jogável. O retorno
para `main` é feito por PR; não manter uma `develop` ou um fork do jogo.

## Fluxo automatizado

O workflow [Playtest Release](../.github/workflows/release.yml) faz:

1. Validar versão e presença de notas em `docs/releases/vVERSAO.md`.
2. Compilar/testar a entrada conjunta em Ubuntu 22.04 e Windows 2022 com `Cargo.lock`.
3. Executar fmt, testes e Clippy em ambas as plataformas; verificar também
   as combinações isoladas de features e suas fronteiras.
4. Selecionar assets, empacotar dependências e preparar os cinco downloads.
5. Verificar instalação/desinstalação no Windows e carregamento do executável;
   verificar DEB com inicialização gráfica por Xvfb e instalação RPM no Fedora.
6. Em push de `release/*`, disponibilizar os pacotes como artefatos do Actions.
7. Em push de tag, criar release inicialmente em rascunho, anexar todos os
   downloads e `SHA256SUMS.txt`, e então publicar como pré-release.

O job de publicação recebe `contents: write`; builds usam apenas leitura.
Uma release já publicada não é sobrescrita pelo workflow. Correções posteriores
recebem nova versão/tag; um rascunho de tentativa interrompida pode ser retomado.
Os smoke tests automatizados não substituem playtest humano com GPU e gamepads.
No Windows, validar também o carregamento nativo de PNG em caminhos com espaços
e acentos: conferir apenas `std::fs`, JSON ou existência de arquivos não detecta
a diferença de codificação nas chamadas C usadas pelo Raylib.

## Preparar e publicar

1. Criar `release/vVERSAO` a partir do estado jogável revisado.
2. Atualizar versões, changelog, backlog e notas para jogadores.
3. Executar `cargo fmt`, `cargo test --locked --all-targets` e
   `cargo clippy --locked --all-targets --all-features -- -D warnings`.
4. Validar links Markdown, YAML e scripts de empacotamento.
5. Commitar, enviar a branch e abrir PR de retorno para `main`.
6. Acompanhar o workflow e corrigir falhas antes de criar a tag.
7. Criar e enviar a tag anotada; conferir a release e os cinco downloads.
8. Registrar o resultado e recolher feedback dos jogadores.

Exemplo de publicação após os checks:

```sh
git tag -a v0.1.0-prototype.4 -m 'release: publish adventure, opening and terminal menu'
git push origin v0.1.0-prototype.4
```

A solicitação explícita do responsável por publicar a release autoriza esse
fluxo. A revisão do PR preserva a integração posterior em `main`.

## Gerar pacotes localmente

As instruções dos scripts estão em [packaging/README.md](../packaging/README.md).
É preciso ambiente de build e ferramentas de empacotamento apenas para quem
produz os pacotes. O jogador instala/extrai o download pronto.

Para verificar integridade depois do download no Linux:

```sh
sha256sum --check SHA256SUMS.txt --ignore-missing
```

No PowerShell, use `Get-FileHash CAMINHO -Algorithm SHA256` e compare com a linha
correspondente em `SHA256SUMS.txt`.

## Critérios de pronto

- Versão e escopo registrados; changelog e notas coerentes com o código.
- Builds, testes e empacotamento aprovados em ambas as plataformas.
- Executável abre fora do checkout, com assets completos e dados graváveis.
- No Windows, PNG e áudio carregam em uma pasta com espaços e acentos.
- Aventura chega ao menu na mesma janela, com avanço por trecho, skip total
  e confirmação final; `Como jogar` explica como assumir um jogador, reiniciar e sair.
- Créditos e limitações conhecidos acompanham o download.
- Cinco pacotes e checksums disponíveis na mesma GitHub pré-release.
- PR de retorno para `main` aberto, com evidência do que foi verificado.

A configuração [release.yml](../.github/release.yml) preserva categorias para
notas técnicas geradas pelo GitHub. As notas públicas deste playtest são
curadas: primeiro download e controles, depois novidades e limitações.
