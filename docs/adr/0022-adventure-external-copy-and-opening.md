# ADR 0022 — Texto externo e apresentação da aventura

- Status: aceita para o experimento.
- Data: 2026-09-10.
- Contexto: [entrega 28](../28-adventure-texts-and-opening.md).

## Decisão

Manter um catálogo JSON de textos dentro dos assets exclusivos da aventura,
lido em runtime e validado antes de substituir o catálogo ativo. F5 faz recarga
atômica, sem alterar relógios, combate ou estado de pausa. Arquivo ausente ou
inválido na inicialização gera erro com caminho; reload inválido mantém o texto
anterior. Não incorporar cópia compilada dos textos como fallback silencioso.

Adicionar uma etapa explícita de apresentação após o pesar e antes da conclusão.
O domínio determina progressão e skip; o renderer compõe jornais, personagens,
tipografia e ilustrações. Texto e assets da apresentação pertencem à aventura.
Não carregar registros, spritesheets ou regras de combate da luta; cópias de
arte com procedência são independentes, conforme ADR 0021.

## Consequências

Edições editoriais dispensam compilação; novas chaves estruturais e mudanças
de mecânica ainda exigem código. O catálogo valida chaves necessárias e texto
UTF-8, e a apresentação adapta linhas ao espaço disponível. Os pacotes devem
incluir o JSON editável. A leitura do livro da luta permanece independente.

A etapa de apresentação exige atualizar progressão, input, áudio e evidências
da aventura, sem alterar as regras do encontro. A matriz de features e o checker
de dependências continuam protegendo o isolamento entre os jogos.
