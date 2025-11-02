#!/usr/bin/env python3
"""
Script de Visualização para o Protótipo NEN-V v2

Gera gráficos para análise do experimento de habituação:
1. Atividade de disparo do neurónio alvo
2. Energia do neurónio alvo ao longo do tempo
3. Atividade total da rede
4. Energia média da rede

Uso: python visualize.py
"""

import pandas as pd
import matplotlib.pyplot as plt
import sys

def main():
    # Lê o arquivo CSV gerado pela simulação
    try:
        df = pd.read_csv('habituation_log.csv')
    except FileNotFoundError:
        print("❌ Erro: Arquivo 'habituation_log.csv' não encontrado!")
        print("Execute primeiro: cargo run --release")
        sys.exit(1)

    # Configuração de estilo
    plt.style.use('seaborn-v0_8-darkgrid')
    fig, axes = plt.subplots(2, 2, figsize=(14, 10))
    fig.suptitle('NEN-V Protótipo v2 - Experimento de Habituação',
                 fontsize=16, fontweight='bold')

    # === Gráfico 1: Disparo do Neurónio Alvo ===
    ax1 = axes[0, 0]
    ax1.plot(df['time'], df['target_firing'], 'o-', color='#2E86AB',
             markersize=3, linewidth=1.5, label='Neurónio 55 (alvo)')
    ax1.axvspan(10, 100, alpha=0.2, color='orange', label='Período de estímulo')
    ax1.set_xlabel('Tempo (passos)', fontsize=11)
    ax1.set_ylabel('Disparando (1=Sim, 0=Não)', fontsize=11)
    ax1.set_title('A) Atividade de Disparo - Neurónio Alvo', fontsize=12, fontweight='bold')
    ax1.set_ylim(-0.1, 1.1)
    ax1.legend(loc='upper right')
    ax1.grid(True, alpha=0.3)

    # === Gráfico 2: Energia do Neurónio Alvo ===
    ax2 = axes[0, 1]
    ax2.plot(df['time'], df['target_energy'], color='#A23B72', linewidth=2)
    ax2.axvspan(10, 100, alpha=0.2, color='orange')
    ax2.axhline(y=100, color='green', linestyle='--', alpha=0.5, label='Energia máxima')
    ax2.axhline(y=0, color='red', linestyle='--', alpha=0.5, label='Energia mínima')
    ax2.set_xlabel('Tempo (passos)', fontsize=11)
    ax2.set_ylabel('Energia', fontsize=11)
    ax2.set_title('B) Energia do Neurónio Alvo', fontsize=12, fontweight='bold')
    ax2.legend(loc='lower right')
    ax2.grid(True, alpha=0.3)

    # === Gráfico 3: Atividade Total da Rede ===
    ax3 = axes[1, 0]
    ax3.plot(df['time'], df['total_firing'], color='#F18F01', linewidth=2)
    ax3.axvspan(10, 100, alpha=0.2, color='orange')
    ax3.set_xlabel('Tempo (passos)', fontsize=11)
    ax3.set_ylabel('Número de Neurónios Disparando', fontsize=11)
    ax3.set_title('C) Atividade Total da Rede (100 neurónios)', fontsize=12, fontweight='bold')
    ax3.grid(True, alpha=0.3)

    # === Gráfico 4: Energia Média da Rede ===
    ax4 = axes[1, 1]
    ax4.plot(df['time'], df['avg_energy'], color='#6A994E', linewidth=2)
    ax4.axvspan(10, 100, alpha=0.2, color='orange', label='Período de estímulo')
    ax4.axhline(y=100, color='green', linestyle='--', alpha=0.5)
    ax4.set_xlabel('Tempo (passos)', fontsize=11)
    ax4.set_ylabel('Energia Média', fontsize=11)
    ax4.set_title('D) Energia Média da Rede', fontsize=12, fontweight='bold')
    ax4.legend(loc='lower right')
    ax4.grid(True, alpha=0.3)

    # Ajusta layout
    plt.tight_layout()

    # Salva a figura
    output_file = 'habituation_analysis.png'
    plt.savefig(output_file, dpi=300, bbox_inches='tight')
    print(f"✅ Gráficos salvos em: {output_file}")

    # Mostra estatísticas
    print("\n📊 Estatísticas do Experimento:")
    print(f"   • Total de passos: {len(df)}")
    print(f"   • Disparos do neurónio alvo: {df['target_firing'].sum()}")
    print(f"   • Energia inicial do alvo: {df['target_energy'].iloc[0]:.1f}")
    print(f"   • Energia mínima do alvo: {df['target_energy'].min():.1f}")
    print(f"   • Energia final do alvo: {df['target_energy'].iloc[-1]:.1f}")
    print(f"   • Pico de atividade da rede: {df['total_firing'].max()} neurónios")

    # Análise de habituação
    during_stimulus = df[(df['time'] > 10) & (df['time'] < 100)]
    print(f"\n🧠 Durante o estímulo (t=11 a t=99):")
    print(f"   • Energia média do alvo: {during_stimulus['target_energy'].mean():.1f}")
    print(f"   • Taxa de disparo do alvo: {during_stimulus['target_firing'].mean()*100:.1f}%")

    # Mostra o gráfico
    plt.show()

if __name__ == '__main__':
    main()
