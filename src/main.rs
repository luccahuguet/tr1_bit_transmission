/* spell-checker: disable */

#![allow(arithmetic_overflow)]

use bitreader::BitReader;
use plotters::prelude::*;
use std::io;
use std::str;

fn main() {
    println!("\n\n---- Programa Iniciado! -----\n\n");
    aplicacao_transmissora();
}

fn aplicacao_transmissora() {
    let mut mensagem = String::new();

    println!("Digite uma mensagem");

    io::stdin()
        .read_line(&mut mensagem)
        .expect("Falha ao ler a mensagem");

    camada_de_aplicacao_transmissora(&mut mensagem);
}

fn camada_de_aplicacao_transmissora(mensagem: &mut String) {
    println!("Mensagem: {}", mensagem);
    // let mut mensagem2 = mensagem.trim().to_string();
    // mensagem.truncate(mensagem.len() - 1);
    let len = mensagem.trim().len();
    mensagem.truncate(len);
    // assert_eq!(mensagem, "t");
    // let mensagem_truncada: String = mensagem.truncate(mensagem.len() - 1);
    // let mensagem_parseada: String = mensagem.parse();
    let quadro: &[u8] = mensagem.as_bytes();
    // int quadro [] = mensagem; // trabalhar com bits
    camada_fisica_transmissora(quadro);
}

fn camada_fisica_transmissora(quadro: &[u8]) {
    let tipo_de_codificacao = 0;
    let fluxo_bruto_de_bits: &[u8];

    match tipo_de_codificacao {
        0 => fluxo_bruto_de_bits = camada_fisica_transmissora_codificacao_binaria(quadro),
        1 => fluxo_bruto_de_bits = camada_fisica_transmissora_codificacao_manchester(quadro),
        2 => fluxo_bruto_de_bits = camada_fisica_transmissora_codificacao_bipolar(quadro),
        _ => unreachable!(),
    }

    meio_de_comunicacao(fluxo_bruto_de_bits);
}

fn camada_fisica_transmissora_codificacao_binaria(quadro: &[u8]) -> &[u8] {
    return quadro;
}

fn camada_fisica_transmissora_codificacao_manchester(quadro: &[u8]) -> &[u8] {
    return quadro;
}

fn camada_fisica_transmissora_codificacao_bipolar(quadro: &[u8]) -> &[u8] {
    return quadro;
}

fn meio_de_comunicacao(fluxo_bruto_de_bits: &[u8]) {
    let fluxo_bruto_de_bits_ponto_b = fluxo_bruto_de_bits;

    let bit: u8 = fluxo_bruto_de_bits[0];

    let bin = fluxo_bruto_de_bits
        .iter()
        .map(|x| format!("{:08b}", x))
        .collect::<Vec<String>>()
        .join("");

    // println!("Fluxo de bits: {}", bin);

    // for j in 0..fluxo_bruto_de_bits.len() {
    //     println!(
    //         "fluxo_bruto_de_bits[{}] = {:08b}\n",
    //         j,
    //         fluxo_bruto_de_bits.first().unwrap()
    //     );
    // }

    println!("bit = {:08b}\n", bit);

    plot_stuff(fluxo_bruto_de_bits_ponto_b);

    camada_fisica_receptora(fluxo_bruto_de_bits_ponto_b);
}

fn plot_stuff(fluxo_bruto_de_bits: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    println!("Plotting...\n");
    const OUT_FILE_NAME: &'static str = "plotters-doc-data/twoscale.gif";

    let byte: u8 = fluxo_bruto_de_bits[0];

    let root_area = BitMapBackend::gif(OUT_FILE_NAME, (1024, 768), 1_000)?.into_drawing_area();
    let c = 7;

    for i in 0..c {
        root_area.fill(&WHITE)?;

        let root_area = root_area.titled("Image Title", ("sans-serif", 60))?;

        let (upper, _lower) = root_area.split_vertically(512);

        let x_axis1 = 0..8;
        let x_axis2 = 0..8;
        // let x_axis = [1, 0, 0, 1, 0, 1, 0, 1];
        // let x_axis = fluxo_bruto_de_bits;

        let mut cc = ChartBuilder::on(&upper)
            .margin(5)
            .set_all_label_area_size(50)
            .caption("Bit Transmission", ("sans-serif", 40))
            .build_cartesian_2d(0..8, -1..2)?;

        cc.configure_mesh()
            .x_labels(20)
            .y_labels(10)
            .disable_mesh()
            .draw()?;

        let mut y_axis: Vec<i32> = Vec::new();

        for j in 0..c {
            y_axis.push(BitReader::new(fluxo_bruto_de_bits).read_u8(1).unwrap() as i32);
        }

        // println!("y_axis = {:?}", y_axis);
        // println!("first byte = {:08b}", byte);

        println!("Iteration {}", i);

        cc.draw_series(LineSeries::new(
            x_axis1.map(|x| {
                (
                    x as i32,
                    (((byte.wrapping_shl(x)) & (1 << (7 - i))) / (1 << (7 - i))) as i32,
                )
            }),
            &BLACK,
        ))?;

        cc.draw_series(
            (x_axis2.map(|x| {
                (
                    x as i32,
                    (((byte.wrapping_shl(x)) & (1 << (7 - i))) / (1 << (7 - i))) as i32,
                )
            }))
            .map(|point| Circle::new(point, 7, BLACK.filled())),
        )?
        .label("Bit")
        .legend(|(x, y)| Circle::new((x, y), 5, BLACK.filled()));

        cc.configure_series_labels().border_style(&BLACK).draw()?;

        // To avoid the IO failure being ignored silently, we manually call the present function
        root_area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    }
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}

fn camada_fisica_receptora(quadro: &[u8]) {
    let fluxo_bruto_de_bits = quadro;

    camada_de_aplicacao_receptora(fluxo_bruto_de_bits);
}

fn camada_de_aplicacao_receptora(quadro: &[u8]) {
    // let mensagem: String = str::from_utf8( &[quadro]);
    let s = match str::from_utf8(quadro) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    aplicacao_receptora(s.to_string());
}

fn aplicacao_receptora(mensagem: String) {
    println!("\nA mensagem recebida foi: {}", mensagem);
    println!("\n\n---- Programa Encerrrado. -----\n\n");
}
