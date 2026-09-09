# Empacotamento da demo

O workflow [`release.yml`](../.github/workflows/release.yml) constrói a mesma
revisão em Windows x86_64/MSVC e Ubuntu 22.04 x86_64. Os scripts usam a biblioteca
padrão do Python e empacotadores nativos; não fazem commit, push ou publicação.

Na raiz do repositório, depois de `cargo build --locked --release`:

```sh
python3 tools/release/package.py stage --target linux-x86_64 \
  --version 0.1.0-prototype.1 --binary target/release/borrow-fighters \
  --output dist/stage
python3 tools/release/package.py linux --stage dist/stage \
  --version 0.1.0-prototype.1 --output dist
python3 tools/release/package.py verify --stage dist/stage
```

Use a versão de `Cargo.toml`, sem `v`. O diretório de staging precisa estar vazio.
Com `CARGO_BUILD_TARGET`, o binário fica em `target/<target>/release/`.
No Windows, use `--target windows-x86_64`, o arquivo `.exe` em `--binary` e o
subcomando `windows`. Compile MSVC com `RUSTFLAGS=-C target-feature=+crt-static`;
o workflow valida a instalação e a carga do executável sem redistribuível VC++.

Artefatos gerados, usando `0.1.0-prototype.1` como exemplo:

| Plataforma | Arquivo |
| --- | --- |
| Windows portátil | `borrow-fighters-0.1.0-prototype.1-windows-x86_64.zip` |
| Windows instalador | `borrow-fighters-0.1.0-prototype.1-windows-x86_64-setup.exe` |
| Linux portátil | `borrow-fighters-0.1.0-prototype.1-linux-x86_64.tar.gz` |
| Debian/Ubuntu | `borrow-fighters_0.1.0~prototype.1_amd64.deb` |
| Fedora | `borrow-fighters-0.1.0~prototype.1-1.x86_64.rpm` |

O instalador Windows usa [Inno Setup](https://jrsoftware.org/isinfo.php), instala
em `%LOCALAPPDATA%\Programs\Borrow Fighters` e cria atalhos. A configuração
[`PrivilegesRequired=lowest`](https://jrsoftware.org/ishelp/topic_setup_privilegesrequired.htm)
permite instalação por usuário. Os pacotes Linux instalam o jogo em
`/opt/borrow-fighters`, o comando em `/usr/bin/borrow-fighters`, o atalho no menu
e o ícone em `hicolor`. Desinstalar remove o aplicativo; dados do usuário ficam
no diretório pessoal controlado pelo runtime.

## Requisitos do ambiente de build

O Windows precisa de Rust MSVC, Visual Studio Build Tools, CMake, LLVM/libclang
e Inno Setup 6. No Ubuntu 22.04, além das dependências Rust/Raylib do CI, o
empacotamento usa `rpm`, `dpkg-deb`, `readelf`, `ldd`, `ldconfig`, `apt-get`,
`libx11-xcb1`, `libxxf86vm1`, `libxrender1`, `libxext6` e `libasound2-data`.

Habilite entradas `deb-src` equivalentes aos repositórios binários e rode
`apt-get update` antes de empacotar Linux. O script baixa **a versão exata** dos
pacotes fonte das bibliotecas incorporadas, incluindo patches da distribuição.
Se um mirror já tiver removido a versão instalada, atualize o runner ou use um
snapshot consistente: a coleta falha em vez de distribuir fontes divergentes.
O passo `stage` funciona sem fontes APT, `rpm` ou ambiente gráfico.

## Conteúdo e verificação

O staging coleta caminhos concretos de `src/`, manifesta os seis personagens e
segue `image` do manifesto e de cada frame, além do manifesto de áudio. Os
campos de procedência `source`, referências artísticas, reviews e vídeos de
showcase não são dependências de execução. Arquivos de produção usados pelos
especiais e pelos cenários são incluídos individualmente. Ausência de um asset
referenciado interrompe o empacotamento.

O pacote inclui [JOGUE-PRIMEIRO.md](JOGUE-PRIMEIRO.md), licenças do projeto,
créditos de áudio, OFL das fontes, avisos e fontes exatas dos crates resolvidos
pelo Cargo. O arquivo de fontes dos crates preserva também os notices de
bibliotecas C embutidos nos headers do Raylib. `BUILD-INFO.json` identifica a
revisão e `PACKAGE-SHA256SUMS.txt` verifica o conteúdo extraído.

Linux coleta dependências transitivas por `ldd` e também bibliotecas X11/ALSA
carregadas por `dlopen`, que não aparecem no `ldd` do jogo. Copia a configuração
ALSA e registra versões, licenças e fontes em `NATIVE-LIBRARIES.json`, `licenses/`
e `THIRD_PARTY_SOURCES/`. O runtime GCC conserva a licença e sua exceção de
linkagem; não é necessário carregar o fonte do compilador dentro do jogo.
As bibliotecas permanecem dinâmicas e substituíveis em `lib/`.

glibc, loader, OpenGL e drivers permanecem no sistema. O launcher usa
`LD_LIBRARY_PATH` apenas no processo do jogo. A validação recusa dependências
não incorporadas, exceto as do sistema, e símbolos de glibc posteriores a 2.35.
Desktop X11 ou XWayland e OpenGL 3.3 continuam necessários; áudio usa o servidor
ou dispositivo do host. A gravação opcional requer `ffmpeg` separado.

`verify` confere assets e hashes e, após empacotar Linux, a resolução de todas
as bibliotecas incorporadas. O workflow também testa instalação/desinstalação,
abertura Linux sob Xvfb e instalação do RPM em Fedora. O teste de carga
`--help` termina com código 2 conforme o parser atual e não abre uma janela.
Teste visual nativo de controles, GPU, áudio e primeira abertura continua
fazendo parte do [processo de release](../docs/06-release-process.md).
