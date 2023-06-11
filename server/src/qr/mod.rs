use qrcode_generator::QrCodeEcc;

#[cfg(debug_assertions)]
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

fn generate_qr(content: &str, output_file: &str) {
  qrcode_generator::to_png_to_file(
    content,
    QrCodeEcc::High,
    1024,
    output_file
  ).unwrap();
}

pub fn generate(content: &str, output_file: &str) {
  #[cfg(debug_assertions)]
  println!("Generating QR code: {}", output_file);
  generate_qr(content, output_file);
  #[cfg(debug_assertions)]
  print_qr(qrcode_generator::to_matrix(content, QrCodeEcc::Low).unwrap());
}