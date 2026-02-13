import init, { compute_combat_damage } from './pkg/rs_aos_stats.js';

let chart = null;

// Initialize the WASM module
async function initWasm() {
    const loadingEl = document.getElementById('loading');
    const errorEl = document.getElementById('error-message');

    try {
        loadingEl.classList.add('active');
        await init();
        loadingEl.classList.remove('active');
        setupEventListeners();
        updateCalculation();
    } catch (error) {
        console.error('Failed to initialize WASM:', error);
        loadingEl.classList.remove('active');
        errorEl.textContent = `Failed to load calculator: ${error.message}`;
        errorEl.classList.add('active');
    }
}

function setupEventListeners() {
    // Attack stats
    const attacks = document.getElementById('attacks');
    const toHit = document.getElementById('to-hit');
    const toWound = document.getElementById('to-wound');
    const rend = document.getElementById('rend');
    const damage = document.getElementById('damage');

    // Defense stats
    const save = document.getElementById('save');
    const ward = document.getElementById('ward');

    // Hit rule
    const hitRule = document.getElementById('hit-rule');

    // Update displays and recalculate on change
    attacks.addEventListener('input', (e) => {
        document.getElementById('attacks-value').textContent = e.target.value;
        updateCalculation();
    });

    toHit.addEventListener('input', (e) => {
        document.getElementById('to-hit-value').textContent = `${e.target.value}+`;
        updateCalculation();
    });

    toWound.addEventListener('input', (e) => {
        document.getElementById('to-wound-value').textContent = `${e.target.value}+`;
        updateCalculation();
    });

    rend.addEventListener('input', (e) => {
        document.getElementById('rend-value').textContent = e.target.value;
        updateCalculation();
    });

    damage.addEventListener('input', (e) => {
        document.getElementById('damage-value').textContent = e.target.value;
        updateCalculation();
    });

    save.addEventListener('input', (e) => {
        document.getElementById('save-value').textContent = `${e.target.value}+`;
        updateCalculation();
    });

    ward.addEventListener('input', (e) => {
        const value = parseInt(e.target.value);
        document.getElementById('ward-value').textContent = value >= 7 ? 'None' : `${value}+`;
        updateCalculation();
    });

    hitRule.addEventListener('change', () => {
        updateCalculation();
    });
}

function updateCalculation() {
    const errorEl = document.getElementById('error-message');

    try {
        // Get all input values
        const attacks = parseInt(document.getElementById('attacks').value);
        const toHit = parseInt(document.getElementById('to-hit').value);
        const toWound = parseInt(document.getElementById('to-wound').value);
        const rend = parseInt(document.getElementById('rend').value);
        const damage = parseInt(document.getElementById('damage').value);
        const save = parseInt(document.getElementById('save').value);
        const wardValue = parseInt(document.getElementById('ward').value);
        const ward = wardValue >= 7 ? undefined : wardValue;
        const hitRule = document.getElementById('hit-rule').value;

        // Call WASM function
        const result = compute_combat_damage(
            attacks,
            toHit,
            toWound,
            rend,
            damage,
            save,
            ward,
            hitRule
        );

        // Update statistics display
        document.getElementById('mean-damage').textContent = result.mean_damage.toFixed(2);
        document.getElementById('max-damage').textContent = result.max_damage;

        const hitRuleNames = {
            'normal': 'Normal',
            'crit_auto_wound': 'Crit Auto-Wound',
            'crit_mortal_wound': 'Crit MW',
            'crit_double_hit': 'Crit Double Hit'
        };
        document.getElementById('current-rule').textContent = hitRuleNames[hitRule];

        // Show stats summary
        document.getElementById('stats-summary').style.display = 'grid';

        // Update chart
        updateChart(result);

        // Hide error if it was showing
        errorEl.classList.remove('active');

    } catch (error) {
        console.error('Calculation error:', error);
        errorEl.textContent = `Calculation error: ${error.message}`;
        errorEl.classList.add('active');
    }
}

function updateChart(result) {
    const ctx = document.getElementById('damage-chart').getContext('2d');

    // Prepare data for Chart.js
    const labels = result.probabilities.map(p => p.damage.toString());
    const data = result.probabilities.map(p => (p.probability * 100).toFixed(2));

    // Create gradient
    const gradient = ctx.createLinearGradient(0, 0, 0, 400);
    gradient.addColorStop(0, 'rgba(102, 126, 234, 0.8)');
    gradient.addColorStop(1, 'rgba(118, 75, 162, 0.2)');

    if (chart) {
        // Update existing chart
        chart.data.labels = labels;
        chart.data.datasets[0].data = data;
        chart.update();
    } else {
        // Create new chart
        chart = new Chart(ctx, {
            type: 'bar',
            data: {
                labels: labels,
                datasets: [{
                    label: 'Probability (%)',
                    data: data,
                    backgroundColor: gradient,
                    borderColor: 'rgba(102, 126, 234, 1)',
                    borderWidth: 2,
                    borderRadius: 5,
                }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    legend: {
                        display: false
                    },
                    title: {
                        display: true,
                        text: 'Damage Probability Distribution',
                        font: {
                            size: 18,
                            weight: 'bold'
                        },
                        color: '#333'
                    },
                    tooltip: {
                        callbacks: {
                            label: function(context) {
                                return `Probability: ${context.parsed.y.toFixed(2)}%`;
                            }
                        }
                    }
                },
                scales: {
                    y: {
                        beginAtZero: true,
                        title: {
                            display: true,
                            text: 'Probability (%)',
                            font: {
                                size: 14,
                                weight: 'bold'
                            }
                        },
                        grid: {
                            color: 'rgba(0, 0, 0, 0.05)'
                        }
                    },
                    x: {
                        title: {
                            display: true,
                            text: 'Damage',
                            font: {
                                size: 14,
                                weight: 'bold'
                            }
                        },
                        grid: {
                            display: false
                        }
                    }
                }
            }
        });
    }
}

// Initialize when the page loads
initWasm();
