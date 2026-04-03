# Age of Sigmar Damage Calculator - REST API

FastAPI-based REST API for calculating combat damage probabilities in Age of Sigmar.

## Formal API Specification

### Endpoint

**POST** `/api/calculate`

**Content-Type:** `application/json`

### Request Schema

```json
{
  "attacks_characteristic": "<string>",
  "to_hit": <integer>,
  "to_wound": <integer>,
  "rend": <integer>,
  "damage_characteristic": "<string>",
  "save": <integer>,
  "ward": <integer|null>,
  "hit_rule_type": "<string>"
}
```

### Field Specifications

#### `attacks_characteristic` (string, required)

Number of attacks. Accepts fixed values or dice notation.

**Format regex:** `^(?:\d+|(?:[1-9]\d*)?D[36](?:\+\d+)?)$`

**Valid formats:**
- Fixed value: `"0"`, `"1"`, `"10"`
- Simple die: `"D3"`, `"D6"` (case-sensitive)
- Multiple dice: `"2D3"`, `"2D6"`, `"50D6"`
- Dice with modifier: `"D3+1"`, `"2D6+3"`, `"50D6+5"`

**Constraints:**
- N (multiplier): integer between 1 and 50 when present
- M (modifier): non-negative integer
- Allowed die faces: 3, 6 only
- Case-sensitive: 'D' must be uppercase
- No spaces allowed

#### `to_hit` (integer, required)

Target number for hit rolls.

**Type:** `integer`
**Range:** `2-6` (inclusive)
**Meaning:** A roll of this value or higher on a d6 scores a hit. Lower is better (2+ is easier than 6+).

#### `to_wound` (integer, required)

Target number for wound rolls.

**Type:** `integer`
**Range:** `2-6` (inclusive)
**Meaning:** A roll of this value or higher on a d6 inflicts a wound. Lower is better.

#### `rend` (integer, required)

Rend/AP (armor penetration) value.

**Type:** `integer`
**Range:** `≥0`
**Meaning:** Reduces save characteristic. 0 means no rend. Higher is better for attacker.

#### `damage_characteristic` (string, required)

Damage per successful hit. Accepts fixed values or dice notation.

**Format regex:** `^(?:\d+|(?:[1-9]\d*)?D[36](?:\+\d+)?)$`

**Valid formats:**
- Fixed value: `"0"`, `"1"`, `"10"`
- Simple die: `"D3"`, `"D6"`
- Multiple dice: `"2D3"`, `"2D6"`
- Dice with modifier: `"D3+1"`, `"2D6+3"`

**Constraints:**
- N (multiplier): integer between 1 and 50 when present
- M (modifier): non-negative integer
- Allowed die faces: 3, 6 only
- Case-sensitive: 'D' must be uppercase
- No spaces allowed

#### `save` (integer, required)

Save characteristic of the target.

**Type:** `integer`
**Range:** `2-7` (inclusive)
**Meaning:** Target number for armor save rolls. 7 means no save possible. Lower is better (2+ is best save).

#### `ward` (integer or null, optional)

Ward/invulnerable save characteristic.

**Type:** `integer | null`
**Range:** `2-7` (inclusive) or `null`
**Default:** `null`
**Meaning:** Target number for ward save rolls. 7 means no ward effect. `null` or 7 means no ward save. Lower is better.

#### `hit_rule_type` (string, optional)

Special hit rule modifier.

**Type:** `string` (enum)
**Default:** `"normal"`
**Allowed values:**
- `"normal"` - Standard hit and wound sequence
- `"crit_auto_wound"` - Unmodified 6 to hit automatically wounds (skips wound roll)
- `"crit_mortal_wound"` - Unmodified 6 to hit inflicts mortal wounds (skips wound and save)
- `"crit_double_hit"` - Unmodified 6 to hit counts as 2 hits

**Important:** Values are case-sensitive and must match exactly.

### Response Schema

```json
{
  "probabilities": [
    {"damage": <integer>, "probability": <float>},
    ...
  ],
  "mean_damage": <float>,
  "max_damage": <integer>
}
```

#### Response Fields

- `probabilities` (array): Complete probability distribution
  - `damage` (integer): Damage value
  - `probability` (float): Probability of this damage (0.0-1.0)
  - Probabilities sum to 1.0 (within floating point precision)
  - Sorted by damage value (ascending)

- `mean_damage` (float): Expected damage value (mathematical expectation)

- `max_damage` (integer): Maximum possible damage in the distribution

### HTTP Status Codes

- **200 OK**: Successful calculation
- **400 Bad Request**: Invalid characteristic format or calculation error
- **422 Unprocessable Entity**: JSON validation error (invalid types, out of range values)

### Error Response Format

```json
{
  "detail": "<error message>"
}
```

## Setup

1. Install dependencies:
```bash
.venv/bin/pip install -r api/requirements.txt
```

2. Ensure the Rust Python package is built:
```bash
.venv/bin/maturin develop
```

## Running the Server

```bash
.venv/bin/uvicorn api.main:app --host 0.0.0.0 --port 8001
```

Or with auto-reload for development:
```bash
.venv/bin/uvicorn api.main:app --host 0.0.0.0 --port 8001 --reload
```

## API Endpoints

### POST /api/calculate

Calculate combat damage probabilities.

**Request Body:**
```json
{
  "attacks_characteristic": "10",      // Number or dice notation (e.g., "D6", "2D6+3")
  "to_hit": 3,                         // To Hit roll (2-6)
  "to_wound": 3,                       // To Wound roll (2-6)
  "rend": 1,                           // Rend value (0+)
  "damage_characteristic": "1",        // Damage per hit, number or dice notation
  "save": 4,                           // Save value (2-7)
  "ward": null,                        // Ward save (2-7, or null)
  "hit_rule_type": "normal"            // Hit rule: "normal", "crit_auto_wound", "crit_mortal_wound", "crit_double_hit"
}
```

**Response:**
```json
{
  "probabilities": [
    {"damage": 0, "probability": 0.0298},
    {"damage": 1, "probability": 0.1254},
    ...
  ],
  "mean_damage": 2.963,
  "max_damage": 10
}
```

### GET /

Get API information.

### GET /health

Health check endpoint.

### GET /docs

Interactive API documentation (Swagger UI).

### GET /redoc

Alternative API documentation (ReDoc).

## Example Usage

### Using curl:

```bash
curl -X POST http://localhost:8001/api/calculate \
  -H "Content-Type: application/json" \
  -d '{
    "attacks_characteristic": "10",
    "to_hit": 3,
    "to_wound": 3,
    "rend": 1,
    "damage_characteristic": "1",
    "save": 4,
    "ward": null,
    "hit_rule_type": "normal"
  }'
```

### With dice notation:

```bash
curl -X POST http://localhost:8001/api/calculate \
  -H "Content-Type: application/json" \
  -d '{
    "attacks_characteristic": "2D6+3",
    "to_hit": 3,
    "to_wound": 3,
    "rend": 1,
    "damage_characteristic": "D3",
    "save": 4,
    "ward": 5,
    "hit_rule_type": "crit_double_hit"
  }'
```

### Using Python:

```python
import requests

response = requests.post(
    "http://localhost:8001/api/calculate",
    json={
        "attacks_characteristic": "2D6",
        "to_hit": 3,
        "to_wound": 3,
        "rend": 1,
        "damage_characteristic": "D3",
        "save": 4,
        "ward": None,
        "hit_rule_type": "normal"
    }
)

result = response.json()
print(f"Mean damage: {result['mean_damage']:.2f}")
print(f"Max damage: {result['max_damage']}")
```

## Dice Notation

The API supports the following formats for attacks and damage characteristics:

- **Fixed values**: `"10"`, `"1"`
- **Simple dice**: `"D3"`, `"D6"`
- **Multiple dice**: `"2D6"`, `"3D3"`
- **Dice with modifier**: `"2D6+3"`, `"3D3+2"`

## Hit Rule Types

- `normal`: Standard hit and wound sequence
- `crit_auto_wound`: Critical hits (6+ to hit) automatically wound
- `crit_mortal_wound`: Critical hits deal mortal wounds
- `crit_double_hit`: Critical hits count as 2 hits

## CORS

CORS is enabled for all origins, allowing the API to be called from web applications.

## Quick Reference for Automation

### Valid Characteristic Patterns

| Type | Pattern | Examples | Notes |
|------|---------|----------|-------|
| Fixed | `^\d+$` | `"1"`, `"10"`, `"40"` | String containing integer |
| Simple die | `^D[36]$` | `"D3"`, `"D6"` | Uppercase D only |
| Multiple dice | `^\d+D[36]$` | `"2D6"`, `"10D3"`, `"50D6"` | N between 1 and 50 |
| Dice + modifier | `^\d*D[36]\+\d+$` | `"D3+1"`, `"2D6+3"`, `"50D6+5"` | N between 1 and 50, modifier M non-negative |

### Parameter Constraints Table

| Parameter | Type | Range | Default | Required |
|-----------|------|-------|---------|----------|
| `attacks_characteristic` | string | See patterns above | - | Yes |
| `to_hit` | integer | 2-6 | - | Yes |
| `to_wound` | integer | 2-6 | - | Yes |
| `rend` | integer | ≥0 | - | Yes |
| `damage_characteristic` | string | See patterns above | - | Yes |
| `save` | integer | 2-7 | - | Yes |
| `ward` | integer\|null | 2-7 or null | null | No |
| `hit_rule_type` | string enum | See below | `"normal"` | No |

### Hit Rule Type Enum

```
"normal" | "crit_auto_wound" | "crit_mortal_wound" | "crit_double_hit"
```

Case-sensitive. Must match exactly.

### Response Guarantees

- `probabilities` array is always non-empty
- `probabilities` is sorted by damage value (ascending)
- Sum of all `probability` values equals 1.0 (±1e-10 floating point tolerance)
- `damage` values are non-negative integers
- `probability` values are in range [0.0, 1.0]
- `mean_damage` is always non-negative
- `max_damage` equals the highest `damage` value in `probabilities`
- **Critical hit rules always increase mean damage** compared to `"normal"` rule

### Expected Damage Increases from Crit Rules

Based on comprehensive testing, crit rules provide the following typical damage increases compared to normal:

| Hit Rule Type | Typical Increase | Edge Case (6+ to hit) |
|---------------|------------------|-----------------------|
| `crit_auto_wound` | +10-12.5% | +50% |
| `crit_mortal_wound` | +25-31% | +200% |
| `crit_double_hit` | +20-25% | +100% |

**Notes:**
- Edge cases (6+ to hit) show dramatic increases because only crits succeed
- `crit_mortal_wound` provides highest damage increase due to bypassing saves
- All crit rules scale with the number of attacks and damage per hit
- Test coverage: 24 test cases across 8 different scenarios verified

To verify: Run `python3 api/test_crit_rules.py`
