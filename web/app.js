import init, { compute_combat_damage_with_dice } from './pkg/rs_aos_stats.js';

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
    const attacksType = document.getElementById('attacks-type');
    const attacksN = document.getElementById('attacks-n');
    const attacksM = document.getElementById('attacks-m');
    const toHit = document.getElementById('to-hit');
    const toWound = document.getElementById('to-wound');
    const rend = document.getElementById('rend');
    const damageType = document.getElementById('damage-type');
    const damageN = document.getElementById('damage-n');
    const damageM = document.getElementById('damage-m');

    // Defense stats
    const save = document.getElementById('save');
    const ward = document.getElementById('ward');

    // Hit rule
    const hitRule = document.getElementById('hit-rule');

    // Attacks type dropdown - show/hide relevant controls
    attacksType.addEventListener('change', () => {
        const type = attacksType.value;
        const nRow = document.getElementById('attacks-n-row');
        const mRow = document.getElementById('attacks-m-row');
        const nLabel = nRow.querySelector('label');

        if (type === 'fixed') {
            nRow.style.display = 'block';
            mRow.style.display = 'none';
            attacksN.min = 1;
            attacksN.max = 40;
            attacksN.value = 10;
            document.getElementById('attacks-n-value').textContent = '10';
            nLabel.innerHTML = 'Value: <span class="value-display" id="attacks-n-value">10</span>';
        } else if (type === 'D3' || type === 'D6') {
            nRow.style.display = 'none';
            mRow.style.display = 'none';
        } else if (type === 'ND3' || type === 'ND6') {
            nRow.style.display = 'block';
            mRow.style.display = 'block';
            attacksN.min = 1;
            attacksN.max = 40;
            nLabel.innerHTML = 'N (multiplier): <span class="value-display" id="attacks-n-value">' + attacksN.value + '</span>';
        }

        updateAttacksNotation();
        updateCalculation();
    });

    // Damage type dropdown - show/hide relevant controls
    damageType.addEventListener('change', () => {
        const type = damageType.value;
        const nRow = document.getElementById('damage-n-row');
        const mRow = document.getElementById('damage-m-row');
        const nLabel = nRow.querySelector('label');

        if (type === 'fixed') {
            nRow.style.display = 'block';
            mRow.style.display = 'none';
            damageN.min = 1;
            damageN.max = 10;
            damageN.value = 1;
            document.getElementById('damage-n-value').textContent = '1';
            nLabel.innerHTML = 'Value: <span class="value-display" id="damage-n-value">1</span>';
        } else if (type === 'D3' || type === 'D6') {
            nRow.style.display = 'none';
            mRow.style.display = 'none';
        } else if (type === 'ND3' || type === 'ND6') {
            nRow.style.display = 'block';
            mRow.style.display = 'block';
            damageN.min = 1;
            damageN.max = 10;
            nLabel.innerHTML = 'N (multiplier): <span class="value-display" id="damage-n-value">' + damageN.value + '</span>';
        }

        updateDamageNotation();
        updateCalculation();
    });

    // Update displays and recalculate on change
    attacksN.addEventListener('input', (e) => {
        document.getElementById('attacks-n-value').textContent = e.target.value;
        updateAttacksNotation();
        updateCalculation();
    });

    attacksM.addEventListener('input', (e) => {
        document.getElementById('attacks-m-value').textContent = e.target.value;
        updateAttacksNotation();
        updateCalculation();
    });

    damageN.addEventListener('input', (e) => {
        document.getElementById('damage-n-value').textContent = e.target.value;
        updateDamageNotation();
        updateCalculation();
    });

    damageM.addEventListener('input', (e) => {
        document.getElementById('damage-m-value').textContent = e.target.value;
        updateDamageNotation();
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

    // Initialize visibility state for attacks and damage controls
    initializeControlVisibility();

    // Initialize notation displays
    updateAttacksNotation();
    updateDamageNotation();
}

function initializeControlVisibility() {
    // Initialize attacks controls
    const attacksType = document.getElementById('attacks-type').value;
    const attacksNRow = document.getElementById('attacks-n-row');
    const attacksMRow = document.getElementById('attacks-m-row');
    const attacksNLabel = attacksNRow.querySelector('label');

    if (attacksType === 'fixed') {
        attacksNRow.style.display = 'block';
        attacksMRow.style.display = 'none';
        document.getElementById('attacks-n').value = 10;
        document.getElementById('attacks-n-value').textContent = '10';
        attacksNLabel.innerHTML = 'Value: <span class="value-display" id="attacks-n-value">10</span>';
    } else if (attacksType === 'D3' || attacksType === 'D6') {
        attacksNRow.style.display = 'none';
        attacksMRow.style.display = 'none';
    } else if (attacksType === 'ND3' || attacksType === 'ND6') {
        attacksNRow.style.display = 'block';
        attacksMRow.style.display = 'block';
        attacksNLabel.innerHTML = 'N (multiplier): <span class="value-display" id="attacks-n-value">' + document.getElementById('attacks-n').value + '</span>';
    }

    // Initialize damage controls
    const damageType = document.getElementById('damage-type').value;
    const damageNRow = document.getElementById('damage-n-row');
    const damageMRow = document.getElementById('damage-m-row');
    const damageNLabel = damageNRow.querySelector('label');

    if (damageType === 'fixed') {
        damageNRow.style.display = 'block';
        damageMRow.style.display = 'none';
        document.getElementById('damage-n').value = 1;
        document.getElementById('damage-n-value').textContent = '1';
        damageNLabel.innerHTML = 'Value: <span class="value-display" id="damage-n-value">1</span>';
    } else if (damageType === 'D3' || damageType === 'D6') {
        damageNRow.style.display = 'none';
        damageMRow.style.display = 'none';
    } else if (damageType === 'ND3' || damageType === 'ND6') {
        damageNRow.style.display = 'block';
        damageMRow.style.display = 'block';
        damageNLabel.innerHTML = 'N (multiplier): <span class="value-display" id="damage-n-value">' + document.getElementById('damage-n').value + '</span>';
    }
}

function updateAttacksNotation() {
    const type = document.getElementById('attacks-type').value;
    const n = parseInt(document.getElementById('attacks-n').value);
    const m = parseInt(document.getElementById('attacks-m').value);
    let notation = '';

    if (type === 'fixed') {
        notation = n.toString();
    } else if (type === 'D3') {
        notation = 'D3';
    } else if (type === 'D6') {
        notation = 'D6';
    } else if (type === 'ND3') {
        notation = `${n}D3`;
        if (m > 0) notation += `+${m}`;
    } else if (type === 'ND6') {
        notation = `${n}D6`;
        if (m > 0) notation += `+${m}`;
    }

    document.getElementById('attacks-notation').textContent = notation;
}

function updateDamageNotation() {
    const type = document.getElementById('damage-type').value;
    const n = parseInt(document.getElementById('damage-n').value);
    const m = parseInt(document.getElementById('damage-m').value);
    let notation = '';

    if (type === 'fixed') {
        notation = n.toString();
    } else if (type === 'D3') {
        notation = 'D3';
    } else if (type === 'D6') {
        notation = 'D6';
    } else if (type === 'ND3') {
        notation = `${n}D3`;
        if (m > 0) notation += `+${m}`;
    } else if (type === 'ND6') {
        notation = `${n}D6`;
        if (m > 0) notation += `+${m}`;
    }

    document.getElementById('damage-notation').textContent = notation;
}

function buildAttacksCharacteristic() {
    const type = document.getElementById('attacks-type').value;
    const n = parseInt(document.getElementById('attacks-n').value);
    const m = parseInt(document.getElementById('attacks-m').value);

    if (type === 'fixed') {
        return n.toString();
    } else if (type === 'D3') {
        return 'D3';
    } else if (type === 'D6') {
        return 'D6';
    } else if (type === 'ND3') {
        return m > 0 ? `${n}D3+${m}` : `${n}D3`;
    } else if (type === 'ND6') {
        return m > 0 ? `${n}D6+${m}` : `${n}D6`;
    }

    return '10';
}

function buildDamageCharacteristic() {
    const type = document.getElementById('damage-type').value;
    const n = parseInt(document.getElementById('damage-n').value);
    const m = parseInt(document.getElementById('damage-m').value);

    if (type === 'fixed') {
        return n.toString();
    } else if (type === 'D3') {
        return 'D3';
    } else if (type === 'D6') {
        return 'D6';
    } else if (type === 'ND3') {
        return m > 0 ? `${n}D3+${m}` : `${n}D3`;
    } else if (type === 'ND6') {
        return m > 0 ? `${n}D6+${m}` : `${n}D6`;
    }

    return '1';
}

function updateCalculation() {
    const errorEl = document.getElementById('error-message');

    try {
        // Get all input values
        const toHit = parseInt(document.getElementById('to-hit').value);
        const toWound = parseInt(document.getElementById('to-wound').value);
        const rend = parseInt(document.getElementById('rend').value);
        const save = parseInt(document.getElementById('save').value);
        const wardValue = parseInt(document.getElementById('ward').value);
        const ward = wardValue >= 7 ? undefined : wardValue;
        const hitRule = document.getElementById('hit-rule').value;

        // Build characteristic strings
        const attacksCharacteristic = buildAttacksCharacteristic();
        const damageCharacteristic = buildDamageCharacteristic();

        // Call WASM function with dice characteristics
        const result = compute_combat_damage_with_dice({
            attacks_characteristic: attacksCharacteristic,
            to_hit: toHit,
            to_wound: toWound,
            rend,
            damage_characteristic: damageCharacteristic,
            save,
            ward,
            hit_rule_type: hitRule
        });

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
