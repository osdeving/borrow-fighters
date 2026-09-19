# Assets: fonte → gerado → jogo

Um builder para recortar e empacotar sprites completos, sem rig, deformação,
resize, inferência de poses ou alteração da pintura. Requer Python 3.10+ e Pillow.

```sh
./dev art validate python
./dev art build cpp
./dev art validate --all
# Equivalente sem o atalho:
python3 tools/art/art.py build tools/art/recipes/cpp.json
```

`validate` lê e valida fontes/receitas sem escrever arquivos. `build` executa
as mesmas validações e gera atlas, manifesto `borrow-fighters.sprite.v1`,
proveniência com hashes, recortes e uma prévia com pivots. A prévia serve para
inspeção estática; a aprovação da animação acontece em movimento no jogo/lab.

`--all` inclui as receitas registradas em `tools/art/recipes/*.json` e as
receitas canônicas `assets-src/**/recipe.json`, sem repetir caminhos resolvidos.

Para trabalho novo, usar `assets-src/<domínio>/<ator>/` para fontes e receita,
`assets-generated/<domínio>/<ator>/` para resultados reproduzíveis e
`assets/<domínio>/...` para arquivos aceitos. O campo `runtime` da receita
documenta o destino aceito: **build não promove nem sobrescreve o runtime**.
Após revisar, copiar somente atlas e manifesto necessários para o destino e
atualizar seu catálogo; preservar a referência à fonte/proveniência. Fontes e
receita são versionadas; intermediários podem ser refeitos. Sem migração em
massa: Python/C++ continuam usando suas fontes e caminhos atuais; suas receitas
registradas apenas produzem candidatos separados.

## Nova animação inteira em grid

Salvar a receita junto da fonte PNG transparente. Exemplo de oito desenhos
completos em duas linhas; ajustar dimensões/pivot aos pixels reais:

```json
{
  "schema": "borrow-fighters.art-recipe.v1",
  "character": "npc",
  "scale": 1,
  "generated": "../../../assets-generated/adventure/npc",
  "runtime": "../../../assets/adventure/npc/npc.sprite.json",
  "output_stem": "npc",
  "actions": {
    "walk": {
      "sheet": "walk.png",
      "reviewed": true,
      "loop": true,
      "grid": {"cell": [256, 384], "columns": 4, "rows": 2},
      "pivot": {"x": 128, "y": 370},
      "duration_ms": 100,
      "ground_tolerance": 4
    }
  }
}
```

`grid.indices` seleciona/reordena células em ordem de leitura, a partir de zero.
Cada célula deve ter arte visível e alpha real; o grid deve cobrir exatamente o
PNG. `ground_tolerance` opcional verifica a distância entre o pivot vertical e
o limite inferior visível: usar em caminhada com apoio, omitir em poses aéreas.
Não repara os pés nem inventa contato. Pivots/durações distintos e cortes fora
de um grid usam `frames` explícitos com `name`, `source_rect: {x,y,w,h}`,
`pivot: {x,y}` e `duration_ms`, no formato já existente de `production.json`.
O builder rejeita cortes inválidos, frames vazios/opacos e metadata de combate
fora do recorte; não cria combate quando ele não foi fornecido.

## Saída para a aventura sem reempacotar

`format: "adventure"` usa as mesmas `actions`/`frames`/`grid`, com
`display_height` e `source_height` positivos e uma `stride_pixels` por ação.
Uma única razão `display_height / source_height` define `size` em todos os
frames: não se redimensionam pixels nem se estica cada pose para a mesma altura.
`source_height` é a altura anatômica de referência do ator, não o tamanho da célula.

```sh
./dev art validate augusta-denim
./dev art build augusta-denim
```

O exemplo [Denim](recipes/augusta-denim.json) conserva a folha de dimensões
ímpares com recortes explícitos, sem repack. A saída `attachments.json` traz
`attachments` no contrato da aventura (`image`, `source`, `anchor`, `size`),
`clips` com IDs e passada, durações e hashes das fontes. Os PNGs em `sprites/`
são copiados byte a byte. A integração copia peças/clips para os catálogos
existentes após revisar o candidato; não existe importação do runtime de luta.

## Compatibilidade e histórico

- `build_reviewed_sprite_atlas.py` chama o mesmo núcleo; preserva os comandos
  anteriores que escreviam recortes na pasta de produção.
- `prepare_reviewed_actions.py` continua necessário para preparar as fontes
  históricas com suas escalas explícitas. O novo builder recebe esses PNGs
  preparados e não altera sua arte aceita.
- `build_python_fighter_atlas.py`, `build_python_high_res_fighter_atlas.py`,
  `build_python_start_atlas.py` e `build_cpp_fighter_atlas.py` são ferramentas
  históricas dos primeiros placeholders. Seus algoritmos de chroma, desenho e
  montagem não reproduzem a produção atual; não executar para atualizá-la.
- [Importador antigo da aventura](README-import-adventure.md) continua disponível
  para suas receitas de peças. Novos ciclos Augusta usam desenhos inteiros e o
  builder único acima; corrigir a pintura na fonte, nunca deformá-la no runtime.
