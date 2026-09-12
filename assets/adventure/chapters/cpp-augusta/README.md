# Rua Augusta — capítulo C++

Entrada: `chapter.json`. Física/instâncias: `world.json`. Elenco e desenho:
`world-art.json`. Falas e interface: `texts.json`.
[Guia e decisões](../../../../docs/38-cpp-augusta-production.md).

O módulo da fachada usa o landmark `ground` como apoio e escala 0,9; piso e
atores compartilham o mapa. As juntas são recortes de alvenaria do mesmo
catálogo, colocados como instâncias próprias. O skyline é o único plano com
parallax; editar esse fator não muda limites ou gatilhos. Para ampliar a rua,
alterar sua largura, instâncias e marcos físicos no arquivo de mundo.
O piso usa `mirror_ground_tiles: true` em `world-art.json`: módulos alternados
são espelhados, mantendo o apoio e igualando as bordas pintadas.

Arte original desta etapa, produzida com `imagegen` em 12/09/2026:

| Arquivo | Geração | Briefing |
| --- | --- | --- |
| `source/facades.png` | `4cab5acb-c1db-4d8a-85b7-fb73bc0d9df0` | Três fachadas frontais isoladas: prédio/boteco com toldo verde, bar de música em tijolo, comércio com mural original; noite azul/âmbar |
| `sprites/skyline.png` | `20b63d79-e60f-448d-83ae-374d8c9a12d1` | Skyline paulistano de edifícios médios/altos, caixas-d'água e janelas, sem primeiro plano ou personagens |
| `sprites/ground.png` | `8753a3ca-448d-4e4a-9ddb-50719a577292` | Faixa horizontal de calçamento intertravado, meio-fio e asfalto úmido, luz noturna |
| `source/npcs.png` | `bf3b0166-8f59-457a-a430-2e18892bdabd` | Folha original dos três NPCs, importada por pacotes independentes |

`art-import.json` contém recortes/apoios das fachadas; `art-import-manifest.json`
registra hashes e alpha. Skyline e piso já são as pinturas-fonte selecionadas,
carregadas como texturas: ajustar instâncias não altera esses arquivos.

Referências visuais e geográficas: [roteiro oficial da SPTuris](https://cidadedesaopaulo.com/wp-content/uploads/2025/01/ROTEIRO-4-BAIXO-AUGUSTA.pdf)
e [fotografia da Augusta](https://vejasp.abril.com.br/estabelecimento/rua-augusta/).
As fotos orientaram elementos e iluminação, não são texturas distribuídas.
O bar **Limiar**, o cafetão e os seguranças são fictícios; o capítulo não atribui
os acontecimentos a um estabelecimento real. O mapa é uma composição inspirada
na região, com distâncias ajustadas ao jogo.
