//! Shannon entropy of each section's raw bytes, in section-header order.
use std::f64::consts::LN_2;
use pelite::PeFile;

#[derive(serde::Serialize)]
pub struct SectionEntropy { pub entropy: Option<f64>, pub samples: Vec<f64> }

pub fn section_entropy(pe: PeFile<'_>) -> Vec<SectionEntropy> {
	pe.section_headers().iter().map(|section| {
		let bytes = pe.get_section_bytes(section).ok().filter(|bytes| !bytes.is_empty());
		SectionEntropy {
			entropy: bytes.map(shannon_entropy),
			samples: bytes.map(entropy_samples).unwrap_or_default(),
		}
	}).collect()
}

fn shannon_entropy(bytes: &[u8]) -> f64 {
	let mut counts = [0usize; 256];
	for &byte in bytes { counts[byte as usize] += 1; }
	let entropy = counts.into_iter().filter(|&count| count != 0).map(|count| {
		let probability = count as f64 / bytes.len() as f64;
		-probability * (probability.ln() / LN_2)
	}).sum::<f64>();
	if entropy == 0.0 { 0.0 } else { entropy }
}

fn entropy_samples(bytes: &[u8]) -> Vec<f64> {
	if bytes.is_empty() { return Vec::new(); }
	let cells = (bytes.len() / 512).clamp(1, 16);
	let mut samples = Vec::with_capacity(cells);
	for index in 0..cells {
		let start = (index as u64 * bytes.len() as u64 / cells as u64) as usize;
		let end = ((index + 1) as u64 * bytes.len() as u64 / cells as u64) as usize;
		let entropy = shannon_entropy(&bytes[start..end]);
		samples.push(entropy);
	}
	samples
}

#[test]
fn entropy_extremes_and_samples() {
	assert_eq!(shannon_entropy(&[0; 512]), 0.0);
	assert!((shannon_entropy(&(0..=255).collect::<Vec<u8>>()) - 8.0).abs() < f64::EPSILON);
	assert!(entropy_samples(&[]).is_empty());
	let mut bytes = vec![0; 512];
	bytes.extend((0..=255).cycle().take(512));
	assert_eq!(entropy_samples(&bytes), vec![0.0, 8.0]);
	assert_eq!(entropy_samples(&vec![0; 16384]).len(), 16);
}
