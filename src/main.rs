/* spell-checker: disable */

use plotters::prelude::*;
use std::io;
use std::str;

fn main() {
    println!("\n\n---- Programa Iniciado! -----\n\n");
    aplicacao_transmissora();
}

fn aplicacao_transmissora() {
    // declaramos um variavel string mutável sem um valor definido
    let mut mensagem = String::new();

    println!("Digite uma mensagem");

    io::stdin()
        .read_line(&mut mensagem)
        .expect("Falha ao ler a mensagem");

    camada_de_aplicacao_transmissora(&mut mensagem);
}

fn camada_de_aplicacao_transmissora(mensagem: &mut String) {
    println!("Mensagem recebida no input: {}", mensagem);

    // Aqui removemos espaços em branco e /n's
    let trimmed_len = mensagem.trim().len();
    mensagem.truncate(trimmed_len);

    // Inserimos cada char da mensagem em uma posição do vetor
    let mut quadro: Vec<u8> = Vec::new();
    for ch in mensagem.chars() {
        quadro.push(ch as u8);
    }

    println!("Mensagem em bytes:");
    for i in 0..quadro.len() {
        println!("BYTE {} = {:08b}", i, quadro[i]);
    }

    println!("\nnúmero total de bytes: {}", quadro.len());

    // Chamamos a função de plotar os bits
    let res = plot_byte_signal(&quadro, 0, "Camada de Aplicacao Transmissora");
    // Observe como se dá o tratamento de exceções
    match res {
        Ok(()) => {} //println!("\n---- Plot realizado com Sucesso -----"),
        Err(e) => println!("An error ocurred: {}", e),
    }

    camada_fisica_transmissora(quadro);
}

fn camada_fisica_transmissora(quadro: Vec<u8>) {
    let tipo_de_codificacao = 2;
    let fluxo_bruto_de_bits: Vec<u8>;

    // Para um "switch", temos a ferramenta match, com caracteristicas mais funcionais
    match tipo_de_codificacao {
        0 => fluxo_bruto_de_bits = camada_fisica_transmissora_codificacao_binaria(quadro),
        1 => fluxo_bruto_de_bits = camada_fisica_transmissora_codificacao_manchester(quadro),
        2 => fluxo_bruto_de_bits = camada_fisica_transmissora_codificacao_bipolar(quadro),
        _ => unreachable!(),
    }
    let res = plot_byte_signal(
        &fluxo_bruto_de_bits,
        tipo_de_codificacao,
        "Camada Fisica Transmissora",
    );

    match res {
        Ok(()) => {} //println!("\n---- Plot realizado com Sucesso -----"),
        Err(e) => println!("An error ocurred: {}", e),
    }

    meio_de_comunicacao(fluxo_bruto_de_bits, tipo_de_codificacao);
}

fn camada_fisica_transmissora_codificacao_binaria(quadro: Vec<u8>) -> Vec<u8> {
    return quadro;
}

fn camada_fisica_transmissora_codificacao_manchester(quadro: Vec<u8>) -> Vec<u8> {
    let mut manchester_array: Vec<u8> = Vec::new();
    let mut manchester_byte1: u8;
    let mut manchester_byte2: u8;
    println!("\nBytes na codificação manchester:");
    // Para cada byte...
    for k in 0..quadro.len() {
        manchester_byte1 = 0;
        manchester_byte2 = 0;
        // Calulando dois bytes de Manchester consecutivos
        for i in 0..4 {
            let j = i * 2;

            let byte_half1 = quadro[k] & 1 << (7 - i);
            if byte_half1 == 1 << (7 - i) {
                // Se é 1 escreve 10
                manchester_byte1 = manchester_byte1 | (1 << (7 - j));
            } else {
                // Se é 0 escreve 01
                manchester_byte1 = manchester_byte1 | (1 << (7 - j - 1));
            }

            let byte_half2 = quadro[k] & 1 << i;
            if byte_half2 == 1 << i {
                // Se é 1 escreve 10
                manchester_byte2 = manchester_byte2 | (1 << j + 1);
            } else {
                // Se é 0 escreve 01
                manchester_byte2 = manchester_byte2 | (1 << j);
            }
        }
        println!("\nmanchester byte 1 = {:08b}", manchester_byte1);
        println!("manchester byte 2 = {:08b}", manchester_byte2);
        manchester_array.push(manchester_byte1);
        manchester_array.push(manchester_byte2);
    }

    return manchester_array;
}

fn camada_fisica_transmissora_codificacao_bipolar(quadro: Vec<u8>) -> Vec<u8> {
    let mut bipolar_array: Vec<u8> = Vec::new();
    println!("\nBytes na codificação Bipolar:");
    // para cada byte...
    for k in 0..quadro.len() {
        let mut n_ones_is_pair = true;
        for h in 0..2 {
            bipolar_array.push(0);
            for i in (0..4).map(|x| x + h * 4) {
                let j = (i * 2) % 8;

                let current_bit = quadro[k] & 1 << (7 - i);

                if current_bit == 1 << (7 - i) {
                    if n_ones_is_pair {
                        // Se é nr de 1s é par escreve 01 (1)
                        bipolar_array[k * 2 + h] = bipolar_array[k * 2 + h] | (1 << (7 - j - 1));
                    } else {
                        // Se é nr de 1s é impar escreve 10 (-1)
                        bipolar_array[k * 2 + h] = bipolar_array[k * 2 + h] | (1 << (7 - j));
                    }
                } else {
                    // Se é 0 "escreve" 00
                }
                if current_bit == 1 << (7 - i) {
                    n_ones_is_pair = !n_ones_is_pair;
                }
            }
            // Para printar os bits da codificação bipolar no terminal, isto é, além da simulação
            println!(
                "bipolar_array[{}] = {:08b}",
                k * 2 + h,
                bipolar_array[k * 2 + h]
            );
        }
    }

    return bipolar_array;
}

fn meio_de_comunicacao(fluxo_bruto_de_bits: Vec<u8>, tipo_de_codificacao: u8) {
    let fluxo_bruto_de_bits_ponto_b = fluxo_bruto_de_bits;

    camada_fisica_receptora(fluxo_bruto_de_bits_ponto_b, tipo_de_codificacao);
}

// Aqui temos nossa função de plotar bytes, uma das dificuldades do projeto
fn plot_byte_signal(
    fluxo_bruto_de_bits: &Vec<u8>,
    tipo_de_codificacao: u8,
    graph_title: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\nPlotting...\n");

    let file_name = format!("plotters-doc-data/{}.gif", graph_title.replace(" ", "_"));
    let out_file_name = &file_name;

    let nome_da_codificacao = match tipo_de_codificacao {
        0 => "Binária (ou sem codificação)",
        1 => "Manchester",
        2 => "Bipolar",
        _ => unreachable!(),
    };

    println!("Tipo de codificação: {}", nome_da_codificacao);

    // Estabelecemos as dimensões do plot
    let root_area = BitMapBackend::gif(out_file_name, (1024, 768), 1_000)?.into_drawing_area();
    // Se aumentarmos o número de iterações, vemos os bits em movimento
    //, mas como os bits que chegam são zeros (e não o próximo byte), não acresenta em nada na simulação
    let n_iterations = 1;

    let mut byte: u8;

    // Para cada byte...
    for j in 1..fluxo_bruto_de_bits.len() + 1 {
        byte = fluxo_bruto_de_bits[j - 1];

        // Para cada iteração...
        for i in 0..n_iterations {
            // pintamos a região de branco
            root_area.fill(&WHITE)?;

            // Settamos o título do gráfico
            let root_area = root_area.titled(graph_title, ("sans-serif", 60))?;

            // Estabelecemos a área de desenho
            let (upper, _lower) = root_area.split_vertically(512);

            // Criamos labels
            let mut cc = ChartBuilder::on(&upper)
                .margin(5)
                .set_all_label_area_size(50)
                .caption(
                    format!("Codificação {}", nome_da_codificacao),
                    ("sans-serif", 40),
                )
                .build_cartesian_2d(0..8, -1..2)?;

            // Criamos a mesh
            cc.configure_mesh()
                .x_labels(20)
                .y_labels(10)
                .disable_mesh()
                .draw()?;

            let x_axis1 = 0..8;
            let x_axis2 = 0..8;

            // Plotamos somente as retas
            cc.draw_series(LineSeries::new(
                x_axis1.map(|x| {
                    (
                        x as i32,
                        (((byte.wrapping_shl(x)) & (1 << (7 - (i % 8)))) / (1 << (7 - (i % 8))))
                            as i32,
                    )
                }),
                &BLACK,
            ))?;

            // Plotamos também um circulo para cada bit, para fins estéticos
            cc.draw_series(
                (x_axis2.map(|x| {
                    (
                        x as i32,
                        (((byte.wrapping_shl(x)) & (1 << (7 - (i % 8)))) / (1 << (7 - (i % 8))))
                            as i32,
                    )
                }))
                .map(|point| Circle::new(point, 7, BLACK.filled())),
            )?
            .label(format!("BYTE {} \n", j))
            .legend(|(x, y)| Circle::new((x, y), 5, BLACK.filled()));
            cc.configure_series_labels().border_style(&BLACK).draw()?;

            // To avoid the IO failure being ignored silently, we manually call the present function
            root_area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        }
    }
    println!("Result has been saved to {}", out_file_name);

    Ok(())
}

fn camada_fisica_receptora(quadro: Vec<u8>, tipo_de_codificacao: u8) {
    let fluxo_bruto_de_bits: Vec<u8>;

    match tipo_de_codificacao {
        0 => fluxo_bruto_de_bits = camada_fisica_receptora_codificacao_binaria(quadro),
        1 => fluxo_bruto_de_bits = camada_fisica_receptora_codificacao_manchester(quadro),
        2 => fluxo_bruto_de_bits = camada_fisica_receptora_codificacao_bipolar(quadro),
        _ => unreachable!(),
    }

    let res = plot_byte_signal(&fluxo_bruto_de_bits, 0, "Camada Fisica Receptora");
    match res {
        Ok(()) => {} //println!("\n---- Plot realizado com Sucesso -----"),
        Err(e) => println!("An error ocurred: {}", e),
    }

    camada_de_aplicacao_receptora(fluxo_bruto_de_bits);
}

fn camada_fisica_receptora_codificacao_binaria(quadro: Vec<u8>) -> Vec<u8> {
    return quadro;
}

fn camada_fisica_receptora_codificacao_manchester(quadro: Vec<u8>) -> Vec<u8> {
    let mut manchester_decodification: Vec<u8> = Vec::new();
    let mut manchester_byte: u8;
    // Como o número de bytes dobrou, e podemos percorrer apenas metade do quadro, assim o fazemos
    for k in 0..(quadro.len() / 2) {
        let g = k * 2;
        manchester_byte = 0;
        for i in 0..4 {
            let j = i * 2;

            let byte_half1 = quadro[g] & 3 << (7 - j - 1);

            if byte_half1 == 1 << (7 - j - 1) {
                manchester_byte = manchester_byte | (1 << (7 - i));
            }

            let byte_half2 = quadro[g + 1] & 3 << (7 - j - 1);

            if byte_half2 == 1 << (7 - j - 1) {
                manchester_byte = manchester_byte | (1 << (7 - 4 - i));
            }
        }
        manchester_decodification.push(!manchester_byte);
    }
    return manchester_decodification;
}

fn camada_fisica_receptora_codificacao_bipolar(quadro: Vec<u8>) -> Vec<u8> {
    let mut bipolar_decoded_array: Vec<u8> = Vec::new();
    for k in 0..(quadro.len() / 2) {
        bipolar_decoded_array.push(0);
        for h in 0..2 {
            for i in 0..4 {
                let j = i * 2;
                let current_bit = quadro[k * 2 + h] & 3 << (7 - j - 1);

                if current_bit == 1 << (7 - j) || current_bit == 1 << (7 - j - 1) {
                    // Se é 01 (1) ou 10 (-1), escreve 1
                    bipolar_decoded_array[k] = bipolar_decoded_array[k] | (1 << (7 - (i + h * 4)));
                } else {
                    // Se é 0 "escreve" 00
                }
            }
        }
    }
    return bipolar_decoded_array;
}

fn camada_de_aplicacao_receptora(quadro: Vec<u8>) {
    // Abaixo um trecho de código que imprime o quadro recebido, se necessário
    // for i in 0..quadro.len() {
    //     println!("BYTE {} = {:08b}", i, quadro[i]);
    // }

    let s = match str::from_utf8(&quadro) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    aplicacao_receptora(s.to_string());
}

fn aplicacao_receptora(mensagem: String) {
    println!("\nA mensagem recebida foi: {}", mensagem);
    println!("\n---- Programa Encerrrado. -----\n\n");
}
