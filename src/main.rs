/* spell-checker: disable */
use plotters::prelude::*;
use std::io;
use std::str;

fn main() {
    println!("Hello, world!");
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

    plot_stuff(fluxo_bruto_de_bits);

    camada_fisica_receptora(fluxo_bruto_de_bits_ponto_b);
}

fn plot_stuff(fluxo_bruto_de_bits: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    const OUT_FILE_NAME: &'static str = "plotters-doc-data/twoscale.png";

    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .right_y_label_area_size(40)
        .margin(5)
        .caption("Dual Y-Axis Example", ("sans-serif", 50.0).into_font())
        .build_cartesian_2d(0f32..10f32, (0.1f32..1e10f32).log_scale())?
        .set_secondary_coord(0f32..10f32, -1.0f32..1.0f32);

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .y_desc("Log Scale")
        .y_label_formatter(&|x| format!("{:e}", x))
        .draw()?;

    chart
        .configure_secondary_axes()
        .y_desc("Linear Scale")
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            (0..=100).map(|x| (x as f32 / 10.0, (1.02f32).powf(x as f32 * x as f32 / 10.0))),
            &BLUE,
        ))?
        .label("y = 1.02^x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .draw_secondary_series(LineSeries::new(
            (0..=100).map(|x| (x as f32 / 10.0, (x as f32 / 5.0).sin())),
            &RED,
        ))?
        .label("y = sin(2x)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&RGBColor(128, 128, 128))
        .draw()?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
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
    println!("A mensagem recebida foi: {}", mensagem);
}
