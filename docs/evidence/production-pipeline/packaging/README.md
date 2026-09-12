# Staging Linux da produção de personagens

Em 12/09/2026, os binários atuais `borrow-story` e `borrow-actor-lab` foram
recompilados com `cargo build --locked --bin borrow-story --bin borrow-actor-lab`
e copiados pelo [empacotador](../../../../tools/release/package.py) para uma
pasta temporária chamada `Jogo ação portátil`. O perfil é **debug local** e
inclui as alterações ainda não commitadas sobre `c6a5c9d`; os hashes dos
binários e dos arquivos relevantes estão no [resumo verificável](summary.json).

A rodada final, às 12:12 UTC, inclui a cobertura contínua do quadril do C++
(`body_leg_blend: [-10, 12]`, `leg_alpha_half_width: 0.35`) e os tiles alternados
do piso (`mirror_ground_tiles: true`). O staging e o resumo anteriores foram
preservados em `/tmp`; este registro corresponde aos novos binários e dados.

O staging e `package.py verify` terminaram com código 0. A partir de outra
pasta, sem override de assets e com dados de usuário temporários, passaram:

- `borrow-fighters --start augusta --hidden --mute --frames 3`;
- `borrow-actor-lab --validate`, com stdout contendo JSON válido: C++, 20 clips
  e 12 imagens decodificadas;
- `borrow-actor-lab --hidden --frames 3`.

O pacote contém 383 arquivos de assets. O fechamento de referências da nova
produção contém 60 arquivos: 27 PNGs, 27 JSONs e 6 WAVs. Não contém originais de
arte em `source/`, receitas de importação, capturas ou vídeos de review. Fontes
tipográficas e avisos de licença continuam presentes; fontes de dependências
de terceiros continuam no arquivo exigido pelo empacotamento.

A instrumentação de acessos com `strace` confirmou que os três processos
carregaram todos os seus assets do pacote, sem recorrer ao checkout. Augusta
abriu 27 PNGs distintos; o laboratório abriu somente os 12 PNGs do C++.
`--validate` não abriu fonte ou cenário. Contagens de abertura incluem as
leituras para validação CPU e carregamento, e não medem alocação GPU.

Este teste cobre resolução de caminhos, dependências de conteúdo, hashes e
abertura nativa curta. Usa bibliotecas e driver deste host; não substitui o
build release, teste de glibc, instaladores, áudio ou revisão completa do jogo.
Não houve publicação. Logs brutos e staging ficaram em `/tmp`; o resumo guarda
seus caminhos para consulta nesta sessão.
