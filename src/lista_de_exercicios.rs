pub fn lista_de_exercícios(
    número_do_exercício: u32
) -> String {
    let exercícios_descrições: Vec<&str> = vec![
        // 1
        "Um programa que lê dois números inteiro e mostra a soma entre os mesmos.",
        
        // 2
        "Um programa que lê a entrada do teclado e mostra no terminal o seu tipo primitivo, e outras as informação possíveis sobre o que foi digitado.",
        
        // 3
        "Um programa que lê um número inteiro e mostra na tela o seu sucessor e seu antecessor.",
        
        // 4
        "Um programa que lê um número inteiro e mostra o seu dobro, triplo e a raiz quadrada.",
        
        // 5
        "Um programa que lê duas notas de um aluno(a), calcula e mostra a média das notas.",
        
        // 6
        "Um programa que lê um valor em metros e o exibe convertido em todos os tipos a seguir:\n\nkm <- hm <- dam <- m -> dm -> cm -> mm",
        
        // 7
        "Um programa que lê um número inteiro e mostra no terminal a sua tabuada.",
        
        // 8
        "Um programa que lê quanto dinheiro uma pessoa tem na carteira e mostra quantos Dólares ela pode comprar.\n\nConsidere US$1,00 = R$3,27",
        
        // 9
        "Um programa que lê a largura e a altura de uma parade em metros, calcula a sua área e a quantidade de tinta necessária para pintá-la, sabendo que cada litro de tinta pinta uma área de 2m².",
        
        // 10
        "Um programa que lê o preço de um produto e mostra seu novo preço com 5% de desconto.",
        
        // 11
        "Um programa que lê o salário de um funcionário e mostra o seu novo salário com 15% de aumento.",
        
        // 12
        "Um programa que converte uma temperatura em °C para °F.",
        
        // 13
        "Um programa que pergunta a quantidade de Km percorridos por um carro alugado e a quantidade de dias pelos quais ele foi alugado.\n O programa irá calcular o preço à ser pago, sabendo que o carro custa R$60.00 por dia e R$0.15 por Km rodado.",
        
        // 14
        "Um programa que lê um número Real e mostra na tela a sua porção inteira.\nex: 6.127 -> 6",
        
        // 15
        "Um programa que lê o comprimento do cateto oposto e do cateto adjacente de um triângulo retângulo, e depois calcula o comprimento da hipotenusa.",
        
        // 16
        "Um programa que lê um ângulo qualquer e mostra no terminal o valor do seno, cosseno e tangente desse ângulo.",
        
        // 17
        "Contexto:\n Um professor quer sortear um dos seus quatro alunos para apagar o quadro.\nExercício:\n Um programa que ajude ele, lendo o nome dos quatro alunos e retornando o nome do escolhido.",
        
        // 18
        "Contexto:\n O mesmo professor do desafio anterior (ex_017) quer sortear a ordem da  apresentação dos trabalhos dos alunos.\n\nExercício:\n Um programa que lê o nome de quatro alunos e mostre em ordem sorteada os nomes para a apresentação.",
        
        // 19
        "Um programa que lê o nome completo de uma pessoa e mostra:\n\n- O nome com todas as letras maiúsculas e minúsculas.\n- Quantas letras o nome todo possui (sem considerar espaços).\n- Quantas letras tem o primeiro nome.",
        
        // 20
        "Um programa que lê um número de 0 a 9999 e mostra no terminal cada um dos dígitos separados.\n\nExemplo:\n 1834\n unidade.: 4\n dezena..: 3\n centena.: 8\n milhar..: 1",
        
        // 21
        "Um programa que lê o nome de uma cidade e retorna se ela começa ou não com o nome \"SANTO\".",
        
        // 22
        "Um programa que lê o nome de uma pessoa e retorna se ela tem \"SILVA\" no nome.",
        
        // 23
        "Um programa que lê uma frase pelo teclado mostra:\n\n- Quantas vezes aparece a letra \"A\";\n- Em que posição o \"A\" aparece pela primeira vez;\n- Em que posição o \"A\" aparece pela última vez.",
        
        // 24
        "Um programa que lê o nome completo de uma pessoa, mostrando em seguida o primeiro e o último nome separadamente.\n\nExemplo:\n\"Ana Maria de Souza\"\n- primeiro.: Ana\n- último...: Souza",
        
        // 25
        "Um programa que faça o computador \"pensar\" em um número inteiro entre 0 e 5 e peça para o usuário tentar descobrir qual foi o número escolhido pelo computador. O programa deverá escrever na tela se o usuário venceu ou perdeu.",
        
        // 26
        "Um programa que lê a velocidade de um carro, se o mesmo ultrapassar 80Km/h, mostre uma mensagem dizendo que ele foi multado.\n A multa vai custar R$7,00 por cada quilometro acima do limite.",
        
        // 27
        "Um programa que lê um número inteiro e mostra na tela se ele é PAR ou ÍMPAR.",
        
        // 28
        "Um programa que pergunta a distância de uma viagem em Km. Calcula o preço da passagem, cobrando  R$0,50 por Km para viagens de até 200Km e R$0,45 para viagens mais longas.",
        
        // 29
        "Um programa que lê um ano qualquer e mostra no terminal se ele é BISSEXTO.",
        
        // 30
        "Um programa que lê três números, e mostra qual é o MAIOR e qual é o MENOR entre eles.",
        
        // 31
        "Um programa que pergunta o salário de um funcinário e calcule o valor do seu aumento. Para salários superiores a R$1.250,00, calcule um aumento de 10%. Para os inferiores ou iguais, o aumento é de 15%.",
        
        // 32
        "Um programa que lê o comprimento de três retas e retorna se elas podem ou não formar um triângulo.",
        
        // 33
        "Um programa para aprovar o empréstimo bancário para a compra de uma casa. Pergunte o valor da casa, o salário do comprador e em quantos anos ele vai pagar.\n A prestação mensal não pode exceder 30% do salário ou então o empréstimo será negado.",
        
        // 34
        "Um programa que lê um número inteiro e pergunta ao usuário qual base quer converter:\n- 1 para binário\n- 2 para octal\n- 3 para hexadecimal",
        
        // 35
        "Um programa que lê dois números inteiros e compara-os, retornando no terminal:\n\n- O primeiro valor maior;\n- O segundo valor maior;\n- Não existe valor maior, os dois são iguais.",
        
        // 36
        "Um programa que lê o ano de nascimento de um jovem e informa, de acordo com a sua idade, se ele ainda vai se alistar ao serviço militar, se é a hora de se alistar ou se já passou do tempo do alistamento.\n O programa também deverá mostrar o tempo que falta ou que passou do prazo.",
        
        // 37
        "Um programa que lê duas notas de um aluno e calcula sua média, mostrando uma mensagem no final, de acordo com a média atingida:\n- Média abaixo de 5.0: Reprovado\n- Média entre 5.0 e 6.9: Recuperação\n- Média 7.0 ou superior: Aprovado",
        
        // 38
        "A Confederação Nacional de Natação precisa de um programa que leia o ano de nascimento de um atleta e mostra sua categoria, de acordo com a idade:\n\n- Até 9 anos: MIRIM\n- Até 14 anos: INFANTIL\n- Até 19 anos: JUNIOR\n- Até 25 anos: SÊNIOR\n- Acima: MASTER",
        
        // 39
        "Refaça o EX_032 dos triângulos acrescentando recurso de mostrar que tipo de triângulo será formado:\n\n- Equilátero: todos os lados iguais\n- Isósceles: dois lados iguais\n- Escaleno: todos os lados diferentes",
        
        // 40
        "Desenvolva uma lógica que lê o peso e a altura de uma pessoa, calcula o seu IMC e mostra seu status corporal, de acordo com a tabela abaixo:\n\n- Abaixo de  18.5: Abaixo do peso\n- Entre 18.5 e 25: Peso ideal\n- Entre 25 e 30: Sobrepeso\n- Entre 30 e 40: Obesidade\n- Acima de 40: Obesidade mórbida",
        
        // 41
        "Um programa que calcula o valor a ser pago por um produto, considerando o seu preço normal e condição de pagamento:\n\n- À vista dinheiro/cheque: 10% desconto\n- À vista no cartão: 5% desconto\n- Em até 2x no cartão: preço normal\n- 3x ou mais no cartão: 20% de juros",
        
        // 42
        "Um programa que faça o usuário jogar JOKENPÔ com o computador.",
        
        // 43
        "Um programa que mostra no terminal uma contagem regressiva para o estoura de fogos de artifícios, indo de 10 até 0, com uma pausa de 1 segundo entre eles.",
        
        // 44
        "Um programa que mostra no terminal todos os números pares que estão no intervalo entre 1 e 50.",
        
        // 45
        "Um programa que calcula a soma entre todos os números ímpares que são múltiplos de três e que se encontram no intervalo de 1 até 500.",
        
        // 46
        "Refaça o Ex_007, mostrando a tabuada de um número que o usuário escolher, só que agora utilizando uma respetição com pergunta sobre continuar.",
        
        // 47
        "Um programa que lê seis números inteiros e mostre a soma apenas daqueles que forem pares. Se o valor digitado for ímpar, desconsidere-o.",
        
        // 48
        "Um programa que lê o primeiro termo e a razão de uma PA. No final, mostra os 10 primeiros termos dessa progressão.",
        
        // 49
        "Um programa que lê um número inteiro e retorna se ele é ou não um número primo.",
        
        // 50
        "Um programa que lê uma frase qualquer e retorna se ela é um palíndromo, desconsiderando os espaços.\n\nExemplos:\n- apos a sopa\n- a sacada da casa\n- a torre da derrota\n- o lobo ama o bolo\n- anotaram a data da maratona",
        
        // 51
        "Um programa que lê o ano de nascimento de sete pessoas. No final, mostra quantas pessoas ainda não atingiram a maioridade e quantas já são maiores.",
        
        // 52
        "Um programa que lê o peso de cinco pessoas. No final, mostra qual foi o maior e o menor peso lido.",
        
        // 53
        "Um programa que lê o nome, idade e gênero de 4 pessoas. No final do programa, motra:\n\n- A média de idade do grupo;\n- Qual é o nome do homem mais velho;\n- Quantas mulheres têm menos de 20 anos.",
        
        // 54
        "Melhore o jogo do Ex_025 onde o computador vai \"pensar\" em um número entre 0 e 10. Só que agora o jogador vai tentar adivinhar até acertar, mostrando no final quantos palpites foram necessários para vencer.",

        // 55
        "Crie um programa que leia dois números inteiros e mostre um menu como o do exemplo abaixo:\n    [ 1 ] Somar\n    [ 2 ] Multiplicar\n    [ 3 ] Qual é o Maior\n    [ 4 ] Informar Novos Números\n    [ 5 ] Fechar o programa\nSeu programa deverá realizar a operação solicitada em cada caso.",
        
        // 56
        "Um programa que lê um número inteiro e mostre o seu fatorial.\n\n Exemplo:\n5! = 5 x 4 x 3 x 2 x 1 = 120",
        
        // 57
        "Melhore o EX_048, perguntando para o usuário se ele quer mostrar mais alguns termos. O programa encerra quando ele disser que quer mostrar 0 termos.",
        
        // 58
        "Um programa que lê um número inteiro x e mostra no terminal os x primeiros elementos da Sequência de Fibonacci.\n\n Exemplo:\n0 -> 1 -> 1 -> 2 -> 3 -> 5 -> 8",
        
        // 59
        "Um programa que lê vários números inteiros digitados. O programa só vai parar quando o usuário digitar o valor 999, que é a condição de parada. No final, mostra quantos números foram digitados e qual foi a soma entre eles (desconsiderando a flag.",
        
        // 60
        "Um programa que lê vários números inteiros digitados. No final da execução, mostra a média entre todos os valores e qual foi o maior e o menor valor registrado. O programa deve perguntar ao usuário se ele quer ou não continuar a digitar valores.",
        
        // 61
        "Um programa que mostra a tabuada de vários números, um de cada vez, para cada valor digitado pelo usuário. O programa será interrompido quando o número solicitado for negativo.",
        
        // 62
        "Um programa que joga par ou ímpar com o usuário. O jogo só será interrompido quando o usuário PERDER, mostrando o total de n vitórias consecutivas que o mesmo conquistou no final do jogo.",
        
        // 63
        "Um programa que lê a idade e o gênero de várias pessoas. A cada pessoa cadastrada, o programa deverá perguntar se o usuário quer ou não adicionar mais uma pessoa.\nNo final mostra:\n\n1º - Quantas pessoas tem mais de 18 anos;\n2º - Quantos homens foram cadastrados;\n3º - Quantas mulheres tem menos 20 anos."
    ];

    return exercícios_descrições[(número_do_exercício - 1) as usize].to_string();
}
