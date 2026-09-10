# Editar os textos sem recompilar

Edite [pt-BR.json](pt-BR.json), salve e pressione **F5** na janela da aventura.
A recarga funciona inclusive durante a pausa. Também é possível fechar e abrir
o mesmo executável: os textos são lidos do disco em cada inicialização.

```sh
cargo run --no-default-features --features adventure --bin borrow-adventure
# Com um arquivo de trabalho separado:
target/debug/borrow-adventure --texts /caminho/para/meus-textos.json
# Assistir diretamente à apresentação:
target/debug/borrow-adventure --start opening
```

Não altere `version` nem os nomes das chaves; edite os valores entre aspas.
O arquivo usa UTF-8. Para quebra de linha escreva `\n`; aspas dentro do texto
precisam de `\"`. JSON não aceita comentários ou vírgula depois do último item.

| Chaves | Onde aparecem |
|---|---|
| `window.*` | Título da janela |
| `ada.*` | Legendas, terminal, passagem de tempo e ajuda do prólogo |
| `morning.*` | Manhã de Rust |
| `encounter.*`, `aftermath.*` | Objetivo, comandos e pesar após a luta |
| `ending.*`, `defeat.*`, `pause.*` | Conclusão, derrota, pausa e opções |
| `navigation.*` | Próximo trecho, pular tudo e confirmação final da apresentação |
| `opening.news.*` | Seção, manchete e linha de apoio dos três jornais |
| `opening.cpp.*`, `opening.python.*` | Nomes, papéis e biografias ilustradas |
| `opening.duke.*`, `opening.c.*`, `opening.go.*`, `opening.rust.*` | Apresentação dos demais personagens |
| `opening.logo.*`, `opening.subtitle`, `opening.tagline` | Logo, subtítulo e frase final |
| `editor.*` | Avisos de recarga |

O jogo confirma a recarga. Se o JSON estiver inválido, conservará integralmente
os textos anteriores e mostrará um aviso; o terminal informa caminho e causa.
Corrija, salve e pressione F5 novamente. Na inicialização, arquivo inválido ou
ausente impede abrir a aventura, evitando substituir suas edições silenciosamente.

Jornais e biografias ajustam tamanho/quebra ao espaço; edições excessivas recebem
reticências para não cobrir a cena. Mantenha frases curtas e confira o tempo de
leitura no jogo. O limite técnico é 2.000 caracteres por valor. Recarregar não
reinicia a cena, a música ou o combate. Novas cenas, durações e comandos ainda
são alterações de código.

Para conferir a navegação, `Enter`/`RB` avança ao próximo trecho; `A` no controle
também avança cenas e continua pulando no combate. Em `borrow-story`,
`Backspace`/`View` pula tudo e abre o menu, inclusive durante combate ou pausa.
Na conclusão normal, **Aperte qualquer tecla para continuar** espera uma nova
tecla, clique ou botão; a entrada que avançou o último trecho e teclas mantidas
pressionadas não confirmam. Em `borrow-adventure`, pular tudo conclui localmente.
Para sair durante a aventura, feche a janela ou use `B` do controle na pausa.
[Escopo da navegação](../../../docs/29-story-terminal-menu.md).

Os textos do livro do jogo de luta ficam separadamente em
[assets/lore/story.json](../../lore/story.json). A apresentação da aventura não
importa esse livro. Em uma distribuição, inclua a pasta `assets/adventure/`
completa ao lado da raiz de assets selecionada pelo executável.
