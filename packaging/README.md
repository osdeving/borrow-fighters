# Empacotamento da demo

O workflow [`release.yml`](../.github/workflows/release.yml) constrói a mesma
revisão em Windows x86_64/MSVC e Ubuntu 22.04 x86_64. Os scripts usam a biblioteca
padrão do Python e empacotadores nativos; não fazem commit, push ou publicação.

Na raiz do repositório, depois de `cargo build --locked --release --bin borrow-story`:

```sh
python3 tools/release/package.py stage --target linux-x86_64 \
  --version 0.1.0-prototype.5 --binary target/release/borrow-story \
  --output dist/stage
python3 tools/release/package.py native-packages --stage dist/stage > dist/native-packages.txt
xargs -r sudo apt-get install --only-upgrade -y -- < dist/native-packages.txt
python3 tools/release/package.py linux --stage dist/stage \
  --version 0.1.0-prototype.5 --output dist
python3 tools/release/package.py verify --stage dist/stage
```

Use a versão de `Cargo.toml`, sem `v`. O diretório de staging precisa estar vazio.
O binário de origem deve ser `borrow-story` (ou `borrow-story.exe`): compila com
as features padrão `adventure` e `fighting` e inicia aventura → apresentação →
menu. O staging conserva o nome público `borrow-fighters`/`borrow-fighters.exe`
e os atalhos existentes. Passar o binário isolado de luta é recusado. Para abrir
o menu diretamente, o jogador pode usar `borrow-fighters --menu`.
Com `CARGO_BUILD_TARGET`, o binário fica em `target/<target>/release/`.
No Windows, use `--target windows-x86_64`, o arquivo `.exe` em `--binary` e o
subcomando `windows`. Compile MSVC com `RUSTFLAGS=-C target-feature=+crt-static`;
o workflow valida a instalação e a carga do executável sem redistribuível VC++.
Aponte também `CMAKE_TOOLCHAIN_FILE` para o caminho absoluto de
[static-runtime.cmake](windows/static-runtime.cmake), para que Raylib/GLFW usem
o mesmo runtime estático `/MT` (a política CMP0091 do CMake separa essa escolha
das flags do Rust). O workflow configura as duas opções.

O executável também incorpora `activeCodePage=UTF-8` no manifesto do Windows
para que o Raylib abra imagens e áudio quando a pasta contém acentos. Esse
recurso exige **Windows 10 versão 1903 (build 18362) ou posterior, incluindo
Windows 11**, tanto no instalador quanto no ZIP portátil. O instalador verifica
essa versão mínima. A [documentação da Microsoft](https://learn.microsoft.com/en-us/windows/apps/design/globalizing/use-utf8-code-page)
descreve a configuração por processo; o pacote não altera a localidade do sistema.

Artefatos gerados, usando `0.1.0-prototype.5` como exemplo:

| Plataforma | Arquivo |
| --- | --- |
| Windows portátil | `borrow-fighters-0.1.0-prototype.5-windows-x86_64.zip` |
| Windows instalador | `borrow-fighters-0.1.0-prototype.5-windows-x86_64-setup.exe` |
| Linux portátil | `borrow-fighters-0.1.0-prototype.5-linux-x86_64.tar.gz` |
| Debian/Ubuntu | `borrow-fighters_0.1.0~prototype.5_amd64.deb` |
| Fedora | `borrow-fighters-0.1.0~prototype.5-1.x86_64.rpm` |

O instalador Windows usa [Inno Setup](https://jrsoftware.org/isinfo.php), instala
em `%LOCALAPPDATA%\Programs\Borrow Fighters` e cria atalhos. A configuração
[`PrivilegesRequired=lowest`](https://jrsoftware.org/ishelp/topic_setup_privilegesrequired.htm)
permite instalação por usuário. Os pacotes Linux instalam o jogo em
`/opt/borrow-fighters`, o comando em `/usr/bin/borrow-fighters`, o atalho no menu
e o ícone em `hicolor`. Desinstalar remove o aplicativo; dados do usuário ficam
no diretório pessoal controlado pelo runtime.

## Requisitos do ambiente de build

O Windows precisa de Rust MSVC, Visual Studio Build Tools, CMake, LLVM/libclang
e Inno Setup 6. O CI também usa `mt.exe`, do Windows SDK, para extrair e conferir
o manifesto incorporado ao executável entregue. No Ubuntu 22.04, além das dependências Rust/Raylib do CI, o
empacotamento usa `rpm`, `dpkg-deb`, `readelf`, `ldd`, `ldconfig`, `apt-get`,
`libx11-xcb1`, `libxxf86vm1`, `libxrender1`, `libxext6`, `libasound2-data` e
`libpulse0`.

Habilite entradas `deb-src` equivalentes aos repositórios binários e rode
`apt-get update` antes de empacotar Linux. O script baixa **a versão exata** dos
pacotes fonte das bibliotecas incorporadas, incluindo patches da distribuição.
`native-packages` usa a mesma resolução de dependências do empacotador e imprime
somente os nomes dos pacotes binários instalados que fornecem bibliotecas e
dados a incorporar. O CI atualiza essa lista limitada antes da cópia para que
as versões instaladas correspondam às fontes ainda disponíveis nos mirrors.
Não faz uma atualização geral do sistema. Se um mirror já tiver removido uma
versão necessária, a coleta falha em vez de distribuir fontes divergentes.
O passo `stage` funciona sem fontes APT, `rpm` ou ambiente gráfico.

## Conteúdo e verificação

O staging coleta caminhos concretos de `src/`, manifesta os seis personagens e
segue `image` do manifesto e de cada frame, além do manifesto de áudio. Os
carregadores da aventura acrescentam nomes locais de cenas, poses e áudio;
`opening/roster.json` fornece somente os retratos de `characters[].image`.
Os catálogos `street/catalog.json` e `chapter/catalog.json` acrescentam o PNG
de cada frame, inclusive poses posteriores e imagens compartilhadas. Cada
atlas é copiado uma vez. O capítulo inclui também mundo, textos, estilo do
celular, cenário do beco e efeitos de áudio referenciados pelo runtime; os
JSONs de produção, prompts e geradores permanecem no repositório.
Textos externos, três fontes próprias, trilha da apresentação e imagens de
C++/Python acompanham a sequência. O retrato de Go não usado pela apresentação
fica fora desse conjunto. Os
campos de procedência `source`, referências artísticas, reviews e vídeos de
showcase não são dependências de execução. Arquivos de produção usados pelos
especiais e pelos cenários são incluídos individualmente. Ausência de um asset
referenciado interrompe o empacotamento.

O pacote inclui [JOGUE-PRIMEIRO.md](JOGUE-PRIMEIRO.md), licenças do projeto,
créditos de áudio, OFL das fontes dos dois domínios, procedência da arte da
aventura, avisos e fontes exatas dos crates resolvidos
pelo Cargo. O arquivo de fontes dos crates preserva também os notices de
bibliotecas C embutidos nos headers do Raylib. `BUILD-INFO.json` identifica a
revisão e o binário Cargo `borrow-story`; `PACKAGE-SHA256SUMS.txt` verifica o
conteúdo extraído. `verify` também recusa staging de versões antigas sem a
identificação da composição.

Linux coleta dependências transitivas por `ldd` e também bibliotecas X11, ALSA
e PulseAudio carregadas por `dlopen`, que não aparecem no `ldd` do jogo. Copia a configuração
ALSA e registra versões, licenças e fontes em `NATIVE-LIBRARIES.json`, `licenses/`
e `THIRD_PARTY_SOURCES/`. O runtime GCC conserva a licença e sua exceção de
linkagem; não é necessário carregar o fonte do compilador dentro do jogo.
As bibliotecas permanecem dinâmicas e substituíveis em `lib/`.

glibc, loader, OpenGL e drivers permanecem no sistema. O launcher usa
`LD_LIBRARY_PATH` apenas no processo do jogo. A validação recusa dependências
não incorporadas, exceto as do sistema, e símbolos de glibc posteriores a 2.35.
Desktop X11 ou XWayland e OpenGL 3.3 continuam necessários. O cliente PulseAudio
incorporado atende também ao servidor de compatibilidade
[pipewire-pulse](https://docs.pipewire.org/page_module_protocol_pulse.html) do
host; ALSA permanece como alternativa. O pacote inclui `libpulsecommon` e as
demais dependências do cliente, além de um alias `libpulse.so` que mantém a
primeira tentativa de carregamento do miniaudio dentro do bundle. O servidor
de áudio ou dispositivo continua no host. A gravação opcional requer `ffmpeg`
separado.

`verify` confere assets e hashes e, após empacotar Linux, a resolução de todas
as bibliotecas incorporadas. O workflow também testa instalação/desinstalação,
abertura Linux sob Xvfb e instalação do RPM em Fedora. O teste de carga
`--help` termina com código 0 e não abre uma janela. Os testes do coletor e
staging rodam com `python3 -m unittest discover -s tools/release -p 'test_*.py'`:
Cobrem assets dinâmicos, retratos e poses transitivas, ausências, hashes, licenças e
nome público do executável em caminhos com acentos.
No Windows, o instalador e o ZIP são abertos em pastas com espaços e acentos
(`Jogo ação instalado` e `Jogo ação portátil`): o CI confere os assets e seus
hashes, compara o executável com o staging e exige `activeCodePage=UTF-8` no
manifesto extraído de cada cópia. Essa verificação recusa o executável sem a
configuração responsável pelo carregamento de arquivos nesses caminhos.
Teste visual nativo de controles, GPU, áudio e primeira abertura continua
fazendo parte do [processo de release](../docs/06-release-process.md).
