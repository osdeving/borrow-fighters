# Input real — mutação e transformação

O [laudo de Rust](rust-input-review.json) registra input XTest no App real, dentro
de um Xephyr separado cujo pai no desktop do host permaneceu oculto. O executável
foi copiado antes do teste; seu SHA-256 está no JSON. Nenhuma tecla ou clique foi
enviado à janela normal do usuário.

1. Desativar ambas as CPUs em Options e escolher Java Street no menu Versus.
2. [Começar a luta em Java Street](rust-java-before.png) e apertar `Y`.
3. Ver [Sirius após o especial](rust-sirius-after.png), com nome/local do HUD
   atualizados; a arena permanece após a restauração da sequência.
4. Apertar `R`: [novo round retorna à seleção Java Street](rust-reset-java.png),
   com HP restaurado. O menu conserva Java Street como preferência.

Os seis checks passaram. O JSON explica a correção do classificador de texto:
a fonte pequena suavizada não contém pixels idênticos à cor nominal, portanto a
comparação final exige máscaras de luminosidade não vazias. A leitura dos nomes
também foi confirmada visualmente; nenhuma alteração do jogo foi necessária.

O áudio foi desabilitado somente neste teste de input. A prova do player e os
testes de World verificam pausa, retomada, contatos e relógios separadamente.

## Python

O [laudo de Python](python-input-review.json) acrescenta dez checks automáticos
aprovados e três inspeções visuais, com uma cópia identificada do novo binário.
No showcase, `Espaço` pausa, `.` avança um quadro, `Enter`/`Home` reiniciam e
`Escape` volta ao menu. O ciclo completo foi observado entre as verificações.

- [Deglutição](python-swallowed.png): alvo ausente, resultado único de 32 HP.
- [Retorno](python-returned-prone.png): C++ reaparece caída enquanto Python salta
  com as pernas dobradas; após recuperação ambas voltam à postura normal.
- [Defesa real de P2](python-guard-returned.png): segurar `U` antes do `Y` mantém
  o resultado em chip de 8 HP (98 → 90). O alvo retorna em guarda, aceita movimento
  quando a sequência termina e `R` restaura a vida/captura.

As imagens de fases usam instantes observados no host; os relógios exatos são
verificados pelos testes determinísticos de World. A arte ainda podia receber
ajustes de enquadramento após este teste; o JSON identifica o executável usado.
