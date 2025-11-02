/// Biblioteca NEN-V (Neurónio-Entrada-Núcleo-Vasos)
///
/// Uma arquitetura de rede neural bioinspirada que integra:
/// - Dendritoma: processamento e aprendizado sináptico
/// - Glia: modulação metabólica e homeostase
/// - NENV: neurónios com memória contextual
/// - Network: orquestração da simulação

pub mod dendritoma;
pub mod glia;
pub mod nenv;
pub mod network;

// Re-exporta tipos principais para facilitar uso
pub use dendritoma::Dendritoma;
pub use glia::Glia;
pub use nenv::{NeuronType, NENV};
pub use network::{ConnectivityType, Network};
