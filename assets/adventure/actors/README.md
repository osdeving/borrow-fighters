# Pacotes de personagens da aventura

Cada pasta é um pacote aberto pelo jogo e pelo `borrow-actor-lab`. O arquivo de
entrada é `character.json`; rig, clips, combate e texturas são relativos à pasta.
Não há dependência do catálogo da luta. [Guia de produção](../../../docs/38-cpp-augusta-production.md).

## Produzir ou corrigir

1. Fixar identidade, proporções e vistas na folha de modelo.
2. Preservar a pintura selecionada em `source/`; registrar origem e briefing.
3. Editar `import.json`: recortes explícitos e landmarks. Executar o importador
   e conferir o resultado sobre fundo diferente do chroma.
4. Ajustar pivôs/dimensões no `rig.json`, poses/curvas no `clips.json` e janelas
   de contato no `combat.json`. Os JSONs são a fonte editável dessas decisões.
5. Executar `--validate`, rever clips nos dois sentidos, transições, apoios,
   contatos e vídeo. Guardar uma revisão com snapshot dos arquivos consumidos.
6. Referenciar o pacote em `world-art.json` quando o personagem entrar no mapa.

```sh
python3 tools/art/import_adventure_actor.py --recipe assets/adventure/actors/cpp/import.json --check
cargo run --bin borrow-actor-lab -- --actor assets/adventure/actors/cpp/character.json --validate
cargo run --bin borrow-actor-lab -- --actor assets/adventure/actors/cpp/character.json --clip run
```

Novas IDs de personagem e golpe precisam ser únicas ao reunir pacotes na mesma
simulação. Os pacotes Julia/cafetão usam a mesma estrutura, mas no capítulo são
atores civis: não entram na simulação de dano. As regras do resgate ficam no
capítulo, não na folha de sprites. Python ainda precisa de seu pacote de arte
e movimento; o laboratório já aceita outro caminho sem mudar o renderer.

## Fontes de arte desta etapa

Arte produzida para Borrow Fighters em 12/09/2026 com `imagegen`, usando a C++
original do próprio projeto como identidade. O traje segue a biografia da
aventura: blusa branca fechada, calça preta, botas práticas, luvas e bolsa.
As quatro vistas e peças preservam essas escolhas; nenhum desenho foi gerado
novamente para corrigir os ciclos após a primeira montagem.

| Fonte selecionada | Identificador de geração | Uso |
| --- | --- | --- |
| `cpp/source/model-turnaround.png` | `0fe8b56b-e87f-4c61-b1f3-be11c9781142` | Modelo adulto, quatro vistas; referência, fora do runtime |
| `cpp/source/torso-parts.png` | `24e5a4df-ed28-458c-be19-36f0dbd69a79` | Tronco/cabeça em quatro vistas, chroma magenta |
| `cpp/source/limb-parts.png` | `4643be22-2bcf-41be-a97d-6325c9089a1f` | Pernas contínuas, braço/antebraço, botas e bolsa |
| `julia/source/npcs.png`, também em `broker`/`security` | `bf3b0166-8f59-457a-a430-2e18892bdabd` | Folha original de nove poses; cópias locais mantêm cada pacote independente |
| `erratic/sprites/poses.png` | Arte anterior da aventura | Cópia exata de `assets/adventure/erratic.png` |

Briefing da C++: folha técnica ortográfica, adulta, proporção consistente,
pintura anime/comic com contornos legíveis; torso sem braços e limbs separados
para rig, calça com pregas discretas, botas sem salto alto. Não incluir texto,
cenário ou sombra nas peças. As tentativas selecionadas estão preservadas;
novas gerações de IA não são determinísticas. O export das fontes preservadas é.

Briefing dos NPCs: Julia adulta de 22 anos, cabelo curto cacheado, jaqueta roxa,
calça escura, tênis e bolsa; poses de espera/corrida/conversa. Antagonista adulto
com blazer cinza e camisa vinho, poses de impedir/apontar/fugir. Segurança adulto
com polo escura, calça cargo e botas, poses de guarda/soco/derrota sem ferimentos
gráficos. Pessoas fictícias. Fontes/receitas/revisões ficam fora da distribuição.
