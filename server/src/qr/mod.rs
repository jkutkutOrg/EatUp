use qrcode_generator::QrCodeEcc;

fn print_qr(qr: Vec<Vec<bool>>) {
    let empty = "██";
    let filled = "  ";

    let qr_arr = qr.iter().map(|row| {
        row.iter().map(|cell| {
            if *cell {
                filled
            } else {
                empty
            }
        }).collect::<String>()
    }).collect::<Vec<String>>();

    let qr_string = qr_arr.iter().map(|row| {
        format!("{}{}{}", empty, row, empty)
    }).collect::<Vec<String>>().join("\n");

    let border_row = vec![empty; qr.len() + 2].join("");

    println!("{}\n{}\n{}", border_row, qr_string, border_row);
}

pub fn generate(content: &str, output_file: &str) {
    qrcode_generator::to_png_to_file(
        content,
        QrCodeEcc::High,
        1024,
        output_file
    ).unwrap();
}

pub fn generate_with_debug(content: &str, output_file: &str) {
    generate(content, output_file);
    print_qr(qrcode_generator::to_matrix(content, QrCodeEcc::Low).unwrap())
}